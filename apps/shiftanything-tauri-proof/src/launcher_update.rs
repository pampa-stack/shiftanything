//! Publisher-signed updates of the launcher itself. Catalog app updates are separate.
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod signed;

use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, Wry};

const EVENT: &str = "shift-launcher-update";

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Idle,
    Checking,
    Available,
    Downloading,
    Ready,
    Error,
}

#[derive(Clone, Serialize)]
struct Available {
    version: String,
    notes: String,
}

#[derive(Clone, Serialize)]
pub(crate) struct Snapshot {
    current: String,
    available: Option<Available>,
    status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<&'static str>,
}

struct Inner {
    snapshot: Snapshot,
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    pending: Option<tauri_plugin_updater::Update>,
}

impl Inner {
    fn change(&mut self, app: &AppHandle, status: Status, error: Option<&'static str>) -> Snapshot {
        self.snapshot.status = status;
        self.snapshot.error = error;
        let snapshot = self.snapshot.clone();
        let _ = app.emit(EVENT, &snapshot);
        snapshot
    }
}

pub(crate) struct LauncherUpdater {
    inner: Mutex<Inner>,
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    channel: Result<signed::Channel, &'static str>,
}

pub(crate) fn configure(context: &mut tauri::Context<Wry>, isolated: bool) -> LauncherUpdater {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    let channel = signed::configure(context, isolated);
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let _ = isolated;
    LauncherUpdater {
        inner: Mutex::new(Inner {
            snapshot: Snapshot { current: context.package_info().version.to_string(), available: None, status: Status::Idle, error: None },
            #[cfg(any(target_os = "macos", target_os = "linux"))]
            pending: None,
        }),
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        channel,
    }
}

pub(crate) fn start(app: &AppHandle) -> tauri::Result<()> {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    if app.state::<LauncherUpdater>().channel.is_ok() {
        app.plugin(tauri_plugin_updater::Builder::new().build())?;
        let _ = check(app);
        let handle = app.clone();
        std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_secs(3600));
                let _ = check(&handle);
            }
        });
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    let _ = app;
    Ok(())
}

#[tauri::command]
pub(crate) fn launcher_update_state(state: State<'_, LauncherUpdater>) -> Result<Snapshot, String> {
    Ok(state.inner.lock().map_err(|e| e.to_string())?.snapshot.clone())
}

#[tauri::command]
pub(crate) fn launcher_update_check(app: AppHandle) -> Result<Snapshot, String> {
    check(&app)
}

fn check(app: &AppHandle) -> Result<Snapshot, String> {
    let state = app.state::<LauncherUpdater>();
    let mut inner = state.inner.lock().map_err(|e| e.to_string())?;
    if matches!(inner.snapshot.status, Status::Checking | Status::Downloading | Status::Ready) {
        return Ok(inner.snapshot.clone());
    }
    inner.snapshot.available = None;
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    return Ok(inner.change(app, Status::Error, Some("installation_unsupported")));
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let channel = match &state.channel {
            Ok(channel) => channel.clone(),
            Err(error) => return Ok(inner.change(app, Status::Error, Some(error))),
        };
        inner.pending = None;
        let snapshot = inner.change(app, Status::Checking, None);
        let handle = app.clone();
        tauri::async_runtime::spawn(async move {
            let result = signed::check(&handle, channel).await;
            let state = handle.state::<LauncherUpdater>();
            if let Ok(mut inner) = state.inner.lock() {
                match result {
                    Ok(Some(update)) => {
                        inner.snapshot.available = Some(Available {
                            version: update.version.clone(),
                            notes: update.body.as_deref().unwrap_or_default().chars().take(4_000).collect(),
                        });
                        inner.pending = Some(update);
                        inner.change(&handle, Status::Available, None);
                    }
                    Ok(None) => {
                        inner.change(&handle, Status::Idle, None);
                    }
                    Err(error) => {
                        inner.change(&handle, Status::Error, Some(error));
                    }
                }
            }
        });
        Ok(snapshot)
    }
}

#[tauri::command]
pub(crate) fn launcher_update_install(app: AppHandle) -> Result<Snapshot, String> {
    let state = app.state::<LauncherUpdater>();
    let mut inner = state.inner.lock().map_err(|e| e.to_string())?;
    if matches!(inner.snapshot.status, Status::Checking | Status::Downloading | Status::Ready) {
        return Ok(inner.snapshot.clone());
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    return Ok(inner.change(&app, Status::Error, Some("installation_unsupported")));
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let Some(update) = inner.pending.take() else {
            if matches!(inner.snapshot.status, Status::Error) {
                return Ok(inner.snapshot.clone());
            }
            return Ok(inner.change(&app, Status::Error, Some("update_unavailable")));
        };
        let snapshot = inner.change(&app, Status::Downloading, None);
        let handle = app.clone();
        tauri::async_runtime::spawn(async move {
            let result = signed::install(update).await;
            let state = handle.state::<LauncherUpdater>();
            if let Ok(mut inner) = state.inner.lock() {
                match result {
                    Ok(()) => {
                        inner.change(&handle, Status::Ready, None);
                    }
                    Err(error) => {
                        inner.snapshot.available = None;
                        inner.change(&handle, Status::Error, Some(error));
                    }
                }
            }
        });
        Ok(snapshot)
    }
}
