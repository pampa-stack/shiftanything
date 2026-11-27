//! Typed lifecycle adapter. The core approval contract and native installer remain authoritative.
use artcraft_launcher_core::{
    CraftApp, Os, PackageKind, Platform,
    automation::{InventoryEntry, Operation, Request, Snapshot as Inventory},
    curated::verified_linux_appimage,
    platform::{pick_asset, recipe_platform_verified},
    protocol::{Backend, BackendOptions, CANCELLED, Command, Event, INSTALL_CANCELLED, InstallLocations, JobPhase, LatestRelease},
    release::{Asset, Release},
    status::Installed,
};
use artcraft_launcher_native::{NativeBackend, automation};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::OnceLock,
    time::{Duration, Instant},
};

const UPDATE_CHECK_INTERVAL: Duration = Duration::from_secs(15 * 60);

pub struct NativeState {
    backend: NativeBackend,
    detected: BTreeMap<String, Option<Installed>>,
    latest: BTreeMap<String, LatestRelease>,
    installable: BTreeMap<String, bool>,
    jobs: BTreeMap<String, JobPhase>,
    results: BTreeMap<String, String>,
    active: BTreeMap<String, String>,
    cancellations: BTreeMap<String, String>,
    pending: Option<Request>,
    pending_from_ui: bool,
    open_after_install: BTreeSet<String>,
    next_request: u64,
    last_update_check: Instant,
    initial_detection_complete: bool,
    initial_release_check_complete: bool,
    restarting: bool,
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
struct ReleaseAsset {
    tag: String,
    version: String,
    published_at: Option<i64>,
    bytes: u64,
}

#[derive(Serialize)]
pub struct Snapshot {
    platform: Option<Platform>,
    detected: BTreeMap<String, Option<Installed>>,
    lifecycle_enabled: bool,
    isolated_profile: bool,
    inventory: Inventory,
    release_assets: BTreeMap<String, ReleaseAsset>,
    verified_installs: &'static BTreeSet<String>,
    install_checks_pending: BTreeSet<String>,
    pending: Option<Request>,
    offer_open_after_install: bool,
    progress: BTreeMap<String, Progress>,
    favorites: std::collections::BTreeSet<String>,
}

/// Completed package cycles, distinct from today's package availability.
fn verified_installs(platform: Option<Platform>) -> &'static BTreeSet<String> {
    type Evidence = BTreeMap<String, BTreeMap<String, BTreeSet<String>>>;
    static EVIDENCE: OnceLock<Option<Evidence>> = OnceLock::new();
    static NONE: BTreeSet<String> = BTreeSet::new();
    let Some(platform) = platform else { return &NONE };
    let evidence = EVIDENCE.get_or_init(|| match serde_json::from_str(include_str!("../verified-platforms.json")) {
        Ok(evidence) => Some(evidence),
        Err(error) => {
            eprintln!("Invalid embedded package verification evidence: {error}");
            None
        }
    });
    evidence
        .as_ref()
        .and_then(|evidence| evidence.get(&platform.os.label().to_ascii_lowercase()))
        .and_then(|architectures| architectures.get(&platform.arch.label().to_ascii_lowercase()))
        .unwrap_or(&NONE)
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

    pub fn isolated_profile(&self) -> bool {
        self.isolated
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
            cancellations: BTreeMap::new(),
            pending: None,
            pending_from_ui: false,
            open_after_install: BTreeSet::new(),
            next_request: 0,
            last_update_check: Instant::now(),
            initial_detection_complete: false,
            initial_release_check_complete: false,
            restarting: false,
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
                // Stable detection publishes its MCP state after the installed-app scan.
                Event::Mcp { .. } => self.initial_detection_complete = true,
                Event::CheckFinished { .. } => self.initial_release_check_complete = true,
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
                    self.open_after_install.remove(&app);
                    self.record_job_end(&app, "rejected: destination or version requires additional consent".into(), false);
                }
                _ => {}
            }
        }
        while self.results.len() > 256 {
            self.results.pop_first();
        }
    }

    fn finish_job(&mut self, app: &str, result: Result<String, String>) {
        let stopped = result.as_ref().is_err_and(|error| error == CANCELLED);
        let message = match result {
            Ok(message) => format!("completed: {message}"),
            Err(error) if error == CANCELLED || error == INSTALL_CANCELLED => CANCELLED.into(),
            Err(error) => format!("failed: {error}"),
        };
        self.record_job_end(app, message, stopped);
    }

    fn record_job_end(&mut self, app: &str, message: String, stopped: bool) {
        self.jobs.remove(app);
        if let Some(id) = self.active.remove(app) {
            self.results.insert(format!("{id}:{app}"), message.clone());
        }
        if let Some(id) = self.cancellations.remove(app) {
            let outcome = if stopped { "completed: operation cancelled" } else { "failed: native operation ended without confirming cancellation" };
            self.results.insert(format!("{id}:{app}"), outcome.into());
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
        let verified = verified_installs(platform);
        let mut apps: Vec<_> = artcraft_launcher_core::curated::NATIVE_APPS
            .iter()
            .map(|app| {
                let installed = self.detected.get(app.id).and_then(Option::as_ref);
                let release = match self.latest.get(app.id) {
                    Some(LatestRelease::Found(release)) => Some(release),
                    _ => None,
                };
                let has_asset = platform.zip(release).and_then(|(platform, release)| compatible_asset(app, platform, release)).is_some();
                let installable =
                    self.enabled && verified.contains(app.id) && has_asset && installed.is_none_or(|_| self.installable.get(app.id) == Some(&true));
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
                    can_open: None,
                    can_remove: None,
                    installed_version: installed.and_then(|entry| entry.version.as_ref()).map(artcraft_launcher_core::version::label),
                    latest_version: release.and_then(|entry| entry.version.as_ref()).map(artcraft_launcher_core::version::label),
                    job: self.jobs.get(app.id).map(|phase| format!("{phase:?}")),
                }
            })
            .collect();
        for id in artcraft_launcher_core::flathub::IDS {
            let Some(state) = self.backend.flatpak_status(id) else { continue };
            insert_repository_entry(
                &mut apps,
                InventoryEntry {
                    id: id.into(),
                    detected: self.detected.contains_key(id),
                    installed: state.installed.is_some(),
                    compatible: true,
                    installable: self.enabled && verified.contains(id) && state.installable,
                    update_available: self.enabled && verified.contains(id) && state.update_available,
                    can_open: Some(state.can_open),
                    can_remove: Some(state.can_remove),
                    installed_version: state.installed_version_label,
                    latest_version: None,
                    job: self.jobs.get(id).map(|phase| format!("{phase:?}")),
                },
            );
        }
        Inventory { updated_at: self.backend.now(), apps, results: self.results.clone() }
    }

    fn validate(&self, request: &Request) -> Result<(), String> {
        if !self.enabled {
            return Err("Read-only proof".into());
        }
        validate_ui_request(request, &self.inventory())?;
        if request.operation == Operation::Cancel && self.cancellations.contains_key(&request.apps[0]) {
            return Err("Cancellation already requested".into());
        }
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
        self.last_update_check = Instant::now();
        self.backend.send(Command::Detect);
        self.backend.send(Command::CheckForUpdates { force: true });
        Ok(())
    }

    pub fn snapshot(&mut self) -> Snapshot {
        self.poll();
        if self.enabled && !self.restarting {
            if self.pending.is_none() && self.jobs.is_empty() && self.last_update_check.elapsed() >= UPDATE_CHECK_INTERVAL {
                self.last_update_check = Instant::now();
                self.backend.send(Command::CheckForUpdates { force: false });
            }
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
        }
        let inventory = self.inventory();
        if self.enabled {
            automation::publish(&inventory);
        }
        let platform = self.backend.platform();
        let verified = verified_installs(platform);
        let install_checks_pending = if self.enabled {
            inventory
                .apps
                .iter()
                .filter(|app| verified.contains(&app.id) && !app.installed)
                .filter(|app| {
                    let detecting = !self.initial_detection_complete && !app.detected;
                    let checking_release =
                        !self.initial_release_check_complete && !self.latest.contains_key(&app.id) && self.backend.flatpak_status(&app.id).is_none();
                    detecting || checking_release
                })
                .map(|app| app.id.clone())
                .collect()
        } else {
            BTreeSet::new()
        };
        let release_assets = artcraft_launcher_core::curated::NATIVE_APPS
            .iter()
            .filter_map(|app| {
                let LatestRelease::Found(release) = self.latest.get(app.id)? else { return None };
                let asset = compatible_asset(app, platform?, release)?;
                let metadata = ReleaseAsset {
                    tag: release.tag.clone(),
                    version: release.version_label(),
                    published_at: release.published_at,
                    bytes: asset.size,
                };
                Some((app.id.to_string(), metadata))
            })
            .collect();
        Snapshot {
            platform,
            release_assets,
            verified_installs: verified,
            install_checks_pending,
            detected: self.detected.clone(),
            lifecycle_enabled: self.enabled,
            isolated_profile: self.isolated,
            inventory,
            pending: self.pending.clone(),
            offer_open_after_install: self.pending.as_ref().is_some_and(|request| can_open_after_install(request, self.pending_from_ui, platform)),
            favorites: self.favorites.ids().clone(),
            progress: self
                .jobs
                .iter()
                .map(|(id, phase)| {
                    let (phase, done, total) = if self.cancellations.contains_key(id) {
                        ("cancelling", None, None)
                    } else {
                        match phase {
                            JobPhase::Downloading { done, total } => ("downloading", Some(*done), Some(*total)),
                            JobPhase::Reconnecting { done, total } => ("reconnecting", Some(*done), Some(*total)),
                            JobPhase::Verifying => ("verifying", None, None),
                            JobPhase::Queued => ("queued", None, None),
                            JobPhase::Waiting => ("waiting", None, None),
                            JobPhase::Installing => ("installing", None, None),
                            JobPhase::Removing => ("removing", None, None),
                        }
                    };
                    (id.clone(), Progress { phase: phase.to_string(), done, total })
                })
                .collect(),
        }
    }

    pub fn request(&mut self, operation: Operation, apps: Vec<String>) -> Result<Request, String> {
        if self.restarting {
            return Err("Sh*ftAnything is restarting".into());
        }
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

    pub fn begin_restart(&mut self) -> bool {
        self.poll();
        if self.restarting || self.pending.is_some() || !self.jobs.is_empty() || !self.active.is_empty() {
            return false;
        }
        self.restarting = true;
        true
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
        self.results.insert(request.id.clone(), "approved; native operation requested (not completed)".into());
        for id in request.apps {
            if request.operation == Operation::Cancel {
                self.open_after_install.remove(&id);
                self.cancellations.insert(id, request.id.clone());
                continue;
            }
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

/// One inventory row per app: the verified repository takes precedence on its target platform.
fn insert_repository_entry(apps: &mut Vec<InventoryEntry>, repository: InventoryEntry) {
    if let Some(existing) = apps.iter_mut().find(|app| app.id == repository.id) {
        *existing = repository;
    } else {
        apps.push(repository);
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

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn restart_waits_for_catalog_operations_and_reserves_the_idle_session() -> Result<(), Box<dyn std::error::Error>> {
        const PROFILE: &str = "SHIFT_TEST_RESTART_PROFILE";
        let Some(root) = std::env::var_os(PROFILE) else {
            // Exercise the real isolated backend without changing the test runner's environment.
            let profile = tempfile::tempdir()?;
            let root = profile.path().canonicalize()?;
            let home = root.join("home");
            std::fs::create_dir(&home)?;
            let mut child = std::process::Command::new(std::env::current_exe()?);
            child.args(["--exact", "session::tests::restart_waits_for_catalog_operations_and_reserves_the_idle_session"]);
            child.env(PROFILE, &root).env("HOME", &home).env("CFFIXED_USER_HOME", &home);
            for (key, directory) in
                [("XDG_CONFIG_HOME", "config"), ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"), ("XDG_STATE_HOME", "state")]
            {
                std::fs::create_dir(home.join(directory))?;
                child.env(key, home.join(directory));
            }
            let output = child.output()?;
            assert!(output.status.success(), "{}{}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
            return Ok(());
        };
        let mut state = NativeState::new(root.into(), false, true)?;
        let app = "handy".to_owned();
        let id = "ui-1".to_owned();
        state.pending = Some(Request { id: id.clone(), operation: Operation::Install, apps: vec![app.clone()] });
        assert!(!state.begin_restart(), "must wait for the consent dialog");
        state.pending = None;
        for phase in
            [JobPhase::Queued, JobPhase::Downloading { done: 1, total: 2 }, JobPhase::Reconnecting { done: 1, total: 2 }, JobPhase::Installing]
        {
            state.jobs.insert(app.clone(), phase);
            assert!(!state.begin_restart(), "must wait during {phase:?}");
        }
        state.jobs.clear();
        state.active.insert(app.clone(), id);
        assert!(!state.begin_restart(), "must wait for an active native operation without progress");
        state.finish_job(&app, Err(CANCELLED.into()));
        assert!(state.begin_restart(), "cancelled jobs must release the restart");
        assert!(!state.begin_restart(), "the same restart cannot be reserved twice");
        assert_eq!(state.request(Operation::Install, vec![app]).unwrap_err(), "Sh*ftAnything is restarting");
        Ok(())
    }

    #[test]
    fn installation_evidence_is_scoped_to_catalog_app_and_target() {
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        let windows = Platform { os: Os::Windows, arch: Arch::X64 };
        assert!(verified_installs(Some(mac)).contains("naps2"));
        assert!(!verified_installs(Some(linux)).contains("naps2"));
        assert!(verified_installs(Some(linux)).contains("photocraft"));
        assert!(!verified_installs(Some(Platform { arch: Arch::Arm64, ..linux })).contains("photocraft"));
        assert!(verified_installs(Some(windows)).iter().map(String::as_str).eq(["drawio", "keepassxc", "photocraft", "wordcraft"]));
        assert!(verified_installs(Some(Platform { arch: Arch::Arm64, ..windows })).is_empty());
        let catalog = artcraft_launcher_core::curated::apps();
        for id in verified_installs(Some(mac)).iter().chain(verified_installs(Some(linux))).chain(verified_installs(Some(windows))) {
            assert!(catalog.iter().any(|app| &app.id == id), "verification refers to unknown app {id}");
        }
    }

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
    fn repository_entry_replaces_the_asset_row_for_the_same_app() {
        let mut inventory = Inventory::default();
        let asset = InventoryEntry {
            id: "obs".into(),
            detected: true,
            installed: false,
            compatible: false,
            installable: false,
            update_available: false,
            can_open: None,
            can_remove: None,
            installed_version: None,
            latest_version: Some("33.0.3".into()),
            job: None,
        };
        inventory.apps.push(asset);
        insert_repository_entry(
            &mut inventory.apps,
            InventoryEntry {
                id: "obs".into(),
                detected: true,
                installed: true,
                compatible: true,
                installable: true,
                update_available: true,
                can_open: Some(true),
                can_remove: Some(true),
                installed_version: Some("33.0.2".into()),
                latest_version: None,
                job: None,
            },
        );
        assert_eq!(inventory.apps.len(), 1);
        let request = Request { id: "update-obs".into(), operation: Operation::Update, apps: vec!["obs".into()] };
        assert!(validate_ui_request(&request, &inventory).is_ok());
    }

    #[test]
    fn linux_install_action_requires_an_appimage_for_the_tested_platform_and_recipe() {
        let platform = Platform { os: Os::Linux, arch: Arch::X64 };
        let photo = CraftApp::by_id("photocraft").unwrap();
        let designcraft = CraftApp::by_id("designcraft").unwrap();
        let untested = CraftApp::by_id("kopia").unwrap();
        let image = |name: &str| Release { assets: vec![Asset { name: name.into(), ..Default::default() }], ..Default::default() };
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-x86_64.AppImage")).is_some());
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-aarch64.AppImage")).is_none());
        assert!(compatible_asset(photo, platform, &image("photocraft-0.23.1-linux-x86_64.deb")).is_none());
        assert!(compatible_asset(photo, Platform { arch: Arch::Arm64, ..platform }, &image("photocraft-0.23.1-linux-aarch64.AppImage")).is_none());
        assert!(compatible_asset(designcraft, platform, &image("designcraft-0.21.1-linux-x86_64.AppImage")).is_some());
        assert!(compatible_asset(untested, platform, &image("kopia-0.23.1-linux-x86_64.AppImage")).is_none());
    }
}
