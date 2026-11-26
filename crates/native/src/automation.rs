//! Fixed-file IPC for the scoped catalog MCP. No executable or caller-supplied path is accepted.
use artcraft_launcher_core::automation::{Request, Snapshot};
use std::io::Read;
use std::path::PathBuf;
use std::sync::Mutex;
static LAST_PUBLISH: Mutex<i64> = Mutex::new(0);
fn root() -> Result<PathBuf, String> {
    let root = crate::paths::Paths::new()?.data.join("automation");
    if std::fs::symlink_metadata(&root).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("automation directory is a symlink".into());
    }
    std::fs::create_dir_all(&root).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
    }
    Ok(root)
}
pub fn take_request() -> Option<Request> {
    let root = root().ok()?;
    let path = root.join("request.json");
    if !std::fs::symlink_metadata(&path).is_ok_and(|m| m.is_file() && !m.file_type().is_symlink() && m.len() <= 8192) {
        return None;
    }
    #[cfg(unix)]
    let file =
        std::fs::File::from(rustix::fs::open(&path, rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::NOFOLLOW, rustix::fs::Mode::empty()).ok()?);
    #[cfg(not(unix))]
    let file = std::fs::File::open(&path).ok()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if file.metadata().ok()?.nlink() != 1 {
            return None;
        }
    }
    let mut bytes = Vec::new();
    file.take(8193).read_to_end(&mut bytes).ok()?;
    // Consume malformed requests too, so they cannot block the queue forever.
    std::fs::remove_file(path).ok()?;
    if bytes.len() > 8192 {
        return None;
    }
    serde_json::from_slice(&bytes).ok()
}
pub fn publish(snapshot: &Snapshot) {
    let mut last = LAST_PUBLISH.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if *last == snapshot.updated_at {
        return;
    }
    *last = snapshot.updated_at;
    let Ok(root) = root() else { return };
    let target = root.join("snapshot.json");
    if std::fs::symlink_metadata(&target).is_ok_and(|m| m.file_type().is_symlink()) {
        return;
    }
    if let Ok(bytes) = serde_json::to_vec(snapshot)
        && let Err(error) = crate::paths::write_atomic(&target, &bytes)
    {
        log::warn!("automation snapshot unavailable: {error}");
    }
}

/// Materialize the audited stdio server in the launcher's own profile, never in a client profile.
pub fn configuration() -> Result<String, String> {
    configuration_in(&root()?)
}

/// Isolated-profile entry point for reproducible verification and portable packaging.
pub fn configuration_in(root: &std::path::Path) -> Result<String, String> {
    if std::fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return Err("automation directory is a symlink".into());
    }
    let support = root.join("mcp");
    if std::fs::symlink_metadata(&support).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("MCP support directory is a symlink".into());
    }
    std::fs::create_dir_all(&support).map_err(|error| error.to_string())?;
    let script = support.join("mcp_catalogo.py");
    let catalog = support.join("curated.json");
    for target in [&script, &catalog] {
        if std::fs::symlink_metadata(target).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("MCP support file is a symlink".into());
        }
    }
    crate::paths::write_atomic(&script, include_bytes!("../../../tools/mcp_catalogo.py"))?;
    crate::paths::write_atomic(&catalog, include_bytes!("../../core/curated.json"))?;
    serde_json::to_string_pretty(&serde_json::json!({"mcpServers": {"shiftanything": {
        "command": "uv", "args": ["run", script.to_string_lossy(),
            "--automation-read-root", root.to_string_lossy(), "--automation-write-root", root.to_string_lossy(),
            "--bridge-root", root.to_string_lossy()]
    }}}))
    .map_err(|error| error.to_string())
}

/// Checks local prerequisites and a recent real stdio tool call. Does not read client configs.
pub fn connection() -> Result<bool, String> {
    if crate::process::which("uv").is_none() {
        return Err("uv is missing from PATH".into());
    }
    let path = root()?.join("client.json");
    if !std::fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() <= 256) {
        return Ok(false);
    }
    #[cfg(unix)]
    let file = std::fs::File::from(
        rustix::fs::open(&path, rustix::fs::OFlags::RDONLY | rustix::fs::OFlags::NOFOLLOW, rustix::fs::Mode::empty())
            .map_err(|error| error.to_string())?,
    );
    #[cfg(not(unix))]
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let mut bytes = Vec::new();
    file.take(257).read_to_end(&mut bytes).map_err(|error| error.to_string())?;
    let value: serde_json::Value = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    Ok(value["last_seen"].as_i64().is_some_and(|seen| crate::now().abs_diff(seen) <= 60))
}
