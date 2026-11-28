//! Publisher-signed updates of the launcher itself. Catalog app updates are separate.
#[cfg(signed_updates)]
mod signed;

use crate::session::NativeState;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, Wry};

const EVENT: &str = "shift-launcher-update";
pub(crate) const RESTART_EXIT_CODE: i32 = 75;

pub(crate) fn check_packaged_channel() -> Result<(), &'static str> {
    #[cfg(signed_updates)]
    return signed::check_packaged_channel();
    #[cfg(not(signed_updates))]
    Err("installation_unsupported")
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Idle,
    Checking,
    Available,
    Downloading,
    Ready,
    RestartPending,
    Restarting,
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
    #[cfg(signed_updates)]
    pending: Option<tauri_plugin_updater::Update>,
    #[cfg(signed_updates)]
    staged: Option<signed::Staged>,
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
    #[cfg(signed_updates)]
    channel: Result<signed::Channel, &'static str>,
}

pub(crate) fn configure(context: &mut tauri::Context<Wry>, isolated: bool) -> LauncherUpdater {
    #[cfg(signed_updates)]
    let channel = signed::configure(context, isolated);
    #[cfg(not(signed_updates))]
    let _ = isolated;
    LauncherUpdater {
        inner: Mutex::new(Inner {
            snapshot: Snapshot { current: context.package_info().version.to_string(), available: None, status: Status::Idle, error: None },
            #[cfg(signed_updates)]
            pending: None,
            #[cfg(signed_updates)]
            staged: None,
        }),
        #[cfg(signed_updates)]
        channel,
    }
}

pub(crate) fn start(app: &AppHandle) -> tauri::Result<()> {
    #[cfg(signed_updates)]
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
    #[cfg(not(signed_updates))]
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
    if matches!(inner.snapshot.status, Status::Checking | Status::Downloading | Status::Ready | Status::RestartPending | Status::Restarting) {
        return Ok(inner.snapshot.clone());
    }
    inner.snapshot.available = None;
    #[cfg(not(signed_updates))]
    return Ok(inner.change(app, Status::Error, Some("installation_unsupported")));
    #[cfg(signed_updates)]
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
    if matches!(inner.snapshot.status, Status::Checking | Status::Downloading | Status::Ready | Status::RestartPending | Status::Restarting) {
        return Ok(inner.snapshot.clone());
    }
    #[cfg(not(signed_updates))]
    return Ok(inner.change(&app, Status::Error, Some("installation_unsupported")));
    #[cfg(signed_updates)]
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
            let Ok(mut inner) = state.inner.lock() else {
                return;
            };
            match result {
                Ok(staged) => inner.staged = Some(staged),
                Err(error) => {
                    inner.snapshot.available = None;
                    inner.change(&handle, Status::Error, Some(error));
                    return;
                }
            }
            inner.change(&handle, Status::RestartPending, None);
            drop(inner);
            restart_when_idle(handle);
        });
        Ok(snapshot)
    }
}

#[tauri::command]
pub(crate) fn launcher_update_restart(app: AppHandle) -> Result<Snapshot, String> {
    let state = app.state::<LauncherUpdater>();
    let mut inner = state.inner.lock().map_err(|error| error.to_string())?;
    if !matches!(inner.snapshot.status, Status::Ready) {
        return Ok(inner.snapshot.clone());
    }
    let snapshot = inner.change(&app, Status::RestartPending, None);
    drop(inner);
    restart_when_idle(app);
    Ok(snapshot)
}

fn restart_when_idle(handle: AppHandle) {
    std::thread::spawn(move || {
        let native = handle.state::<Mutex<NativeState>>();
        let state = handle.state::<LauncherUpdater>();
        loop {
            let idle = match native.lock() {
                Ok(mut native) => native.begin_restart(),
                Err(_) => {
                    if let Ok(mut inner) = state.inner.lock() {
                        inner.change(&handle, Status::Ready, Some("restart_failed"));
                    }
                    return;
                }
            };
            if idle {
                #[cfg(signed_updates)]
                {
                    let staged = match state.inner.lock() {
                        Ok(mut inner) => {
                            inner.change(&handle, Status::Restarting, None);
                            inner.staged.take()
                        }
                        Err(_) => None,
                    };
                    let result = staged.ok_or("restart_failed").and_then(|staged| signed::finish(&handle, staged));
                    if let Err(error) = result {
                        // The setup did not start: the current version keeps running.
                        if let Ok(mut native) = native.lock() {
                            native.cancel_restart();
                        }
                        if let Ok(mut inner) = state.inner.lock() {
                            inner.snapshot.available = None;
                            inner.change(&handle, Status::Error, Some(error));
                        }
                    }
                }
                #[cfg(not(signed_updates))]
                handle.exit(RESTART_EXIT_CODE);
                return;
            }
            std::thread::sleep(std::time::Duration::from_millis(350));
        }
    });
}
