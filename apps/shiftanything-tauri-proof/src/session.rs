//! Typed lifecycle adapter. The core approval contract and native installer remain authoritative.
use artcraft_launcher_core::{
    Platform,
    automation::{InventoryEntry, Operation, Request, Snapshot as Inventory},
    platform::{pick_asset, recipe_platform_verified},
    protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease},
    status::Installed,
};
use artcraft_launcher_native::{NativeBackend, automation};
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf};

pub struct NativeState {
    backend: NativeBackend,
    detected: BTreeMap<String, Option<Installed>>,
    latest: BTreeMap<String, LatestRelease>,
    installable: BTreeMap<String, bool>,
    jobs: BTreeMap<String, JobPhase>,
    results: BTreeMap<String, String>,
    active: BTreeMap<String, String>,
    pending: Option<Request>,
    next_request: u64,
    enabled: bool,
    pub diagnostics: PathBuf,
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
    inventory: Inventory,
    pending: Option<Request>,
    progress: BTreeMap<String, Progress>,
}

impl NativeState {
    pub fn new(root: PathBuf, enabled: bool) -> Result<Self, String> {
        let mut backend = NativeBackend::new_isolated_macos_profile(|| {})?;
        if enabled {
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
            next_request: 0,
            enabled,
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
                Event::JobFinished { app, result } | Event::Notice { app, result } => {
                    self.jobs.remove(&app);
                    let message = match result {
                        Ok(message) => format!("completed: {message}"),
                        Err(error) => format!("failed: {error}"),
                    };
                    if let Some(id) = self.active.remove(&app) {
                        self.results.insert(format!("{id}:{app}"), message.clone());
                    }
                    self.results.insert(format!("app:{app}"), message.clone());
                    println!("NATIVE {app}: {message}");
                }
                Event::Occupied { app, .. } | Event::GoBackFallback { app, .. } => {
                    // This proof never consents to overwriting an unrelated destination or downgrading.
                    self.jobs.remove(&app);
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
                    let has_asset = platform.zip(release).is_some_and(|(platform, release)| {
                        let stems: Vec<_> = app.stems().collect();
                        let kinds: Vec<_> = platform.download_kinds().iter().copied().filter(|kind| app.installs(*kind)).collect();
                        recipe_platform_verified(&stems, platform) && pick_asset(release, &stems, platform, &kinds).is_some()
                    });
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
        request.validate_state(&self.inventory())?;
        if request.operation == Operation::Open
            && request.apps.iter().any(|id| id == "handy")
            && std::path::Path::new("/tmp/com_pais_handy_si.sock").exists()
        {
            return Err("Handy already has a single-instance socket; personal instance will not be contacted".into());
        }
        Ok(())
    }

    pub fn snapshot(&mut self) -> Snapshot {
        self.poll();
        if self.enabled {
            if self.pending.is_none()
                && let Some(request) = automation::take_request()
            {
                let outcome = if self.results.contains_key(&request.id) { Err("request already processed".into()) } else { self.validate(&request) };
                match outcome {
                    Ok(()) => {
                        self.results.insert(request.id.clone(), "awaiting approval".into());
                        self.pending = Some(request);
                    }
                    Err(error) => {
                        self.results.insert(request.id, format!("rejected: {error}"));
                    }
                }
            }
            automation::publish(&self.inventory());
        }
        Snapshot {
            platform: self.backend.platform(),
            detected: self.detected.clone(),
            lifecycle_enabled: self.enabled,
            inventory: self.inventory(),
            pending: self.pending.clone(),
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
        Ok(request)
    }

    pub fn decide(&mut self, id: &str, approve: bool) -> Result<(), String> {
        self.poll();
        let request = self.pending.as_ref().filter(|request| request.id == id).ok_or("Unknown pending request")?.clone();
        self.pending = None;
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
