//! Typed lifecycle adapter. The core approval contract and native installer remain authoritative.
use artcraft_launcher_core::{
    CraftApp, Os, PackageKind, Platform,
    automation::{InventoryEntry, Operation, Request, Snapshot as Inventory},
    curated::verified_linux_appimage,
    platform::{pick_asset, recipe_platform_verified},
    protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease},
    release::{Asset, Release},
    status::Installed,
};
use artcraft_launcher_native::{NativeBackend, automation};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

pub struct NativeState {
    backend: NativeBackend,
    detected: BTreeMap<String, Option<Installed>>,
    latest: BTreeMap<String, LatestRelease>,
    installable: BTreeMap<String, bool>,
    jobs: BTreeMap<String, JobPhase>,
    results: BTreeMap<String, String>,
    active: BTreeMap<String, String>,
    pending: Option<Request>,
    pending_from_ui: bool,
    open_after_install: BTreeSet<String>,
    next_request: u64,
    enabled: bool,
    isolated: bool,
    pub diagnostics: PathBuf,
    pub favorites: crate::favorites::Favorites,
}

#[derive(Serialize)]
pub struct Progress {
    phase: String,
    done: Option<u64>,
    total: Option<u64>,
}

#[derive(Serialize)]
pub struct Snapshot {
    platform: Option<Platform>,
    detected: BTreeMap<String, Option<Installed>>,
    lifecycle_enabled: bool,
    isolated_profile: bool,
    inventory: Inventory,
    download_sizes: BTreeMap<String, u64>,
    pending: Option<Request>,
    offer_open_after_install: bool,
    progress: BTreeMap<String, Progress>,
    favorites: std::collections::BTreeSet<String>,
}

fn compatible_asset<'a>(app: &CraftApp, platform: Platform, release: &'a Release) -> Option<&'a Asset> {
    let allowed = if platform.os == Os::Linux {
        if !verified_linux_appimage(app, platform) {
            return None;
        }
        &[PackageKind::AppImage]
    } else {
        platform.download_kinds()
    };
    let stems: Vec<_> = app.stems().collect();
    let kinds: Vec<_> = allowed.iter().copied().filter(|kind| app.installs(*kind)).collect();
    if !artcraft_launcher_native::install::recipe_system_supported(app) || !recipe_platform_verified(&stems, platform) {
        return None;
    }
    pick_asset(release, &stems, platform, &kinds).map(|(asset, _)| asset)
}

impl NativeState {
    pub fn catalog_media(&self) -> artcraft_launcher_native::screenshots::CatalogMedia {
        self.backend.catalog_media()
    }

    pub fn lifecycle_enabled(&self) -> bool {
        self.enabled
    }

    pub fn new(root: PathBuf, enabled: bool, isolated: bool) -> Result<Self, String> {
        let mut backend = if isolated {
            #[cfg(target_os = "linux")]
            let backend = NativeBackend::new_isolated_linux_profile(&root.join("home"), || {})?;
            #[cfg(not(target_os = "linux"))]
            let backend = NativeBackend::new_isolated_macos_profile(|| {})?;
            backend
        } else {
            NativeBackend::new(|| {})
        };
        if isolated && enabled {
            let applications = root.join("home/Applications");
            if let Err(error) = std::fs::create_dir(&applications)
                && error.kind() != std::io::ErrorKind::AlreadyExists
            {
                return Err(error.to_string());
            }
            let metadata = std::fs::symlink_metadata(&applications).map_err(|error| error.to_string())?;
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                return Err("The isolated Applications directory must be a real directory".into());
            }
            backend.send(Command::Configure(BackendOptions {
                install_locations: InstallLocations { root: Some(root.join("home/Applications").display().to_string()), ..Default::default() },
                simultaneous_downloads: 1,
                ..Default::default()
            }));
        }
        backend.send(Command::Detect);
        if enabled {
            backend.send(Command::CheckForUpdates { force: false });
        }
        Ok(Self {
            backend,
            detected: BTreeMap::new(),
            latest: BTreeMap::new(),
            installable: BTreeMap::new(),
            jobs: BTreeMap::new(),
            results: BTreeMap::new(),
            active: BTreeMap::new(),
            pending: None,
            pending_from_ui: false,
            open_after_install: BTreeSet::new(),
            next_request: 0,
            enabled,
            isolated,
            favorites: crate::favorites::Favorites::load(&root)?,
            diagnostics: root,
        })
    }

    fn poll(&mut self) {
        for event in self.backend.poll() {
            match event {
                Event::Detected { app, installed } => {
                    self.detected.insert(app, installed);
                }
                Event::Latest { app, latest, .. } => {
                    self.latest.insert(app, latest);
                }
                Event::Installable { app, installable } => {
                    self.installable.insert(app, installable);
                }
                Event::Progress { app, phase } => {
                    self.jobs.insert(app, phase);
                }
                Event::JobFinished { app, result } => {
                    let launch = self.open_after_install.remove(&app) && result.is_ok();
                    self.finish_job(&app, result);
                    if launch && let Err(error) = self.open_installed_bundle(&app) {
                        self.results.insert(format!("app:{app}"), format!("installed; could not open: {error}"));
                    }
                }
                Event::Notice { app, result } => {
                    self.open_after_install.remove(&app);
                    self.finish_job(&app, result);
                }
                Event::Occupied { app, .. } | Event::GoBackFallback { app, .. } => {
                    // This proof never consents to overwriting an unrelated destination or downgrading.
                    self.jobs.remove(&app);
                    self.open_after_install.remove(&app);
                    let message = "rejected: destination or version requires additional consent".to_string();
                    if let Some(id) = self.active.remove(&app) {
                        self.results.insert(format!("{id}:{app}"), message.clone());
                    }
                    self.results.insert(format!("app:{app}"), message);
                }
                _ => {}
            }
        }
        while self.results.len() > 256 {
            self.results.pop_first();
        }
    }

    fn finish_job(&mut self, app: &str, result: Result<String, String>) {
        self.jobs.remove(app);
        let message = match result {
            Ok(message) => format!("completed: {message}"),
            Err(error) => format!("failed: {error}"),
        };
        if let Some(id) = self.active.remove(app) {
            self.results.insert(format!("{id}:{app}"), message.clone());
        }
        self.results.insert(format!("app:{app}"), message.clone());
        println!("NATIVE {app}: {message}");
    }

    fn open_installed_bundle(&mut self, app: &str) -> Result<(), String> {
        let bundle = self.detected.get(app).and_then(Option::as_ref).map(|entry| Path::new(&entry.location));
        if bundle.is_none_or(|path| !path.is_dir() || path.extension().is_none_or(|ext| !ext.eq_ignore_ascii_case("app"))) {
            return Err("No macOS app bundle was detected after installation".into());
        }
        let request = Request { id: "open-after-install".into(), operation: Operation::Open, apps: vec![app.into()] };
        self.validate(&request)?;
        self.backend.send(Command::Launch { app: app.into(), args: vec![] });
        Ok(())
    }

    fn inventory(&self) -> Inventory {
        let platform = self.backend.platform();
        Inventory {
            updated_at: self.backend.now(),
            apps: artcraft_launcher_core::curated::NATIVE_APPS
                .iter()
                .map(|app| {
                    let installed = self.detected.get(app.id).and_then(Option::as_ref);
                    let release = match self.latest.get(app.id) {
                        Some(LatestRelease::Found(release)) => Some(release),
                        _ => None,
                    };
                    let has_asset = platform.zip(release).and_then(|(platform, release)| compatible_asset(app, platform, release)).is_some();
                    let installable = self.enabled && has_asset && installed.is_none_or(|_| self.installable.get(app.id) == Some(&true));
                    let update_available = installable
                        && installed.zip(release).is_some_and(|(installed, release)| {
                            !installed.externally_managed
                                && installed
                                    .version
                                    .as_ref()
                                    .zip(release.version.as_ref())
                                    .is_some_and(|(old, new)| artcraft_launcher_core::version::is_update(new, old))
                        });
                    InventoryEntry {
                        id: app.id.into(),
                        detected: self.detected.contains_key(app.id),
                        installed: installed.is_some(),
                        compatible: has_asset,
                        installable,
                        update_available,
                        installed_version: installed.and_then(|entry| entry.version.as_ref()).map(ToString::to_string),
                        latest_version: release.and_then(|entry| entry.version.as_ref()).map(ToString::to_string),
                        job: self.jobs.get(app.id).map(|phase| format!("{phase:?}")),
                    }
                })
                .collect(),
            results: self.results.clone(),
        }
    }

    fn validate(&self, request: &Request) -> Result<(), String> {
        if !self.enabled {
            return Err("Read-only proof".into());
        }
        validate_ui_request(request, &self.inventory())?;
        if self.isolated
            && request.operation == Operation::Open
            && request.apps.iter().any(|id| id == "handy")
            && std::path::Path::new("/tmp/com_pais_handy_si.sock").exists()
        {
            return Err("Handy already has a single-instance socket; personal instance will not be contacted".into());
        }
        Ok(())
    }

    pub fn check_updates(&mut self) -> Result<(), String> {
        if !self.enabled {
            return Err("Native lifecycle is disabled in this profile".into());
        }
        self.backend.send(Command::CheckForUpdates { force: true });
        Ok(())
    }

    pub fn snapshot(&mut self) -> Snapshot {
        self.poll();
        if self.enabled {
            if self.pending.is_none()
                && let Some(request) = automation::take_request()
            {
                let outcome = if self.results.contains_key(&request.id) {
                    Err("request already processed".into())
                } else {
                    request.validate().and_then(|()| self.validate(&request))
                };
                match outcome {
                    Ok(()) => {
                        self.results.insert(request.id.clone(), "awaiting approval".into());
                        self.pending = Some(request);
                        self.pending_from_ui = false;
                    }
                    Err(error) => {
                        self.results.insert(request.id, format!("rejected: {error}"));
                    }
                }
            }
            automation::publish(&self.inventory());
        }
        let platform = self.backend.platform();
        let download_sizes = artcraft_launcher_core::curated::NATIVE_APPS
            .iter()
            .filter_map(|app| {
                let LatestRelease::Found(release) = self.latest.get(app.id)? else { return None };
                let asset = compatible_asset(app, platform?, release)?;
                Some((app.id.to_string(), asset.size))
            })
            .collect();
        Snapshot {
            platform,
            download_sizes,
            detected: self.detected.clone(),
            lifecycle_enabled: self.enabled,
            isolated_profile: self.isolated,
            inventory: self.inventory(),
            pending: self.pending.clone(),
            offer_open_after_install: self.pending.as_ref().is_some_and(|request| can_open_after_install(request, self.pending_from_ui, platform)),
            favorites: self.favorites.ids().clone(),
            progress: self
                .jobs
                .iter()
                .map(|(id, phase)| {
                    let (done, total) = match phase {
                        JobPhase::Downloading { done, total } | JobPhase::Reconnecting { done, total } => (Some(*done), Some(*total)),
                        _ => (None, None),
                    };
                    (id.clone(), Progress { phase: format!("{phase:?}"), done, total })
                })
                .collect(),
        }
    }

    pub fn request(&mut self, operation: Operation, apps: Vec<String>) -> Result<Request, String> {
        self.poll();
        if self.pending.is_some() {
            return Err("Another request awaits approval".into());
        }
        self.next_request = self.next_request.checked_add(1).ok_or("Request limit reached")?;
        let request = Request { id: format!("ui-{}", self.next_request), operation, apps };
        self.validate(&request)?;
        self.results.insert(request.id.clone(), "awaiting approval".into());
        self.pending = Some(request.clone());
        self.pending_from_ui = true;
        Ok(request)
    }

    pub fn decide(&mut self, id: &str, approve: bool, open_after_install: bool) -> Result<(), String> {
        self.poll();
        let request = self.pending.as_ref().filter(|request| request.id == id).ok_or("Unknown pending request")?.clone();
        self.pending = None;
        let from_ui = std::mem::take(&mut self.pending_from_ui);
        if !approve {
            self.results.insert(request.id, "rejected".into());
            return Ok(());
        }
        if let Err(error) = self.validate(&request) {
            self.results.insert(request.id, format!("rejected: {error}"));
            return Err(error);
        }
        let command = match request.operation {
            Operation::Install | Operation::Update | Operation::UpdateAll => {
                Command::InstallExpected { apps: request.apps.clone(), installed: request.operation != Operation::Install }
            }
            Operation::Remove => Command::Uninstall { app: request.apps[0].clone() },
            Operation::Open => Command::Launch { app: request.apps[0].clone(), args: vec![] },
            Operation::Cancel => Command::Cancel { app: request.apps[0].clone() },
        };
        if open_after_install && can_open_after_install(&request, from_ui, self.backend.platform()) {
            self.open_after_install.insert(request.apps[0].clone());
        }
        if request.operation == Operation::Cancel {
            self.open_after_install.remove(&request.apps[0]);
        }
        self.results.insert(request.id.clone(), "approved; native operation requested (not completed)".into());
        for id in request.apps {
            if request.operation != Operation::Open {
                self.jobs.insert(id.clone(), JobPhase::Queued);
            }
            self.results.remove(&format!("app:{id}"));
            self.active.insert(id, request.id.clone());
        }
        println!("APPROVED {} {:?}", request.id, command);
        self.backend.send(command);
        Ok(())
    }
}

fn can_open_after_install(request: &Request, from_ui: bool, platform: Option<Platform>) -> bool {
    from_ui && request.operation == Operation::Install && request.apps.len() == 1 && platform.is_some_and(|target| target.os == Os::MacOs)
}

fn validate_ui_request(request: &Request, inventory: &Inventory) -> Result<(), String> {
    if request.operation != Operation::Install || request.apps.len() <= 1 {
        return request.validate_state(inventory);
    }
    let distinct = request.apps.iter().collect::<std::collections::BTreeSet<_>>();
    if distinct.len() != request.apps.len() || distinct.len() > 32 {
        return Err("Invalid install selection".into());
    }
    for app in &request.apps {
        Request { id: request.id.clone(), operation: Operation::Install, apps: vec![app.clone()] }.validate_state(inventory)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use artcraft_launcher_core::Arch;

    #[test]
    fn only_a_single_macos_install_from_the_window_can_request_a_first_open() {
        let mut request = Request { id: "ui-1".into(), operation: Operation::Install, apps: vec!["kopia".into()] };
        let mac = Some(Platform { os: Os::MacOs, arch: Arch::Arm64 });
        let linux = Some(Platform { os: Os::Linux, arch: Arch::X64 });
        assert!(can_open_after_install(&request, true, mac));
        assert!(!can_open_after_install(&request, false, mac));
        assert!(!can_open_after_install(&request, true, linux));
        assert!(!can_open_after_install(&request, true, None));

        request.apps.push("handy".into());
        assert!(!can_open_after_install(&request, true, mac));
        request.apps.pop();
        request.operation = Operation::Update;
        assert!(!can_open_after_install(&request, true, mac));
    }

    #[test]
    fn linux_install_action_requires_an_appimage_for_the_tested_platform_and_recipe() {
        let platform = Platform { os: Os::Linux, arch: Arch::X64 };
        let photo = CraftApp::by_id("photocraft").unwrap();
        let obsolete = CraftApp::by_id("designcraft").unwrap();
        let image = |name: &str| Release { assets: vec![Asset { name: name.into(), ..Default::default() }], ..Default::default() };
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-x86_64.AppImage")).is_some());
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-aarch64.AppImage")).is_none());
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-x86_64.deb")).is_none());
        assert!(compatible_asset(photo, Platform { arch: Arch::Arm64, ..platform }, &image("photocraft-0.23.1-linux-aarch64.AppImage")).is_none());
        assert!(compatible_asset(obsolete, platform, &image("designcraft-0.21.1-linux-x86_64.AppImage")).is_none());
    }
}
