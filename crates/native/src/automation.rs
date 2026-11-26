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
    take_request_in(&root().ok()?)
}

pub(crate) fn take_request_in(root: &std::path::Path) -> Option<Request> {
    if std::fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return None;
    }
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
    publish_in(&root, snapshot);
}

pub(crate) fn publish_in(root: &std::path::Path, snapshot: &Snapshot) {
    if std::fs::symlink_metadata(root).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
        return;
    }
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
    let uv = uv_executable().ok_or("uv is required; install it from https://docs.astral.sh/uv/getting-started/installation/")?;
    let support = root.join("mcp");
    if std::fs::symlink_metadata(&support).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("MCP support directory is a symlink".into());
    }
    std::fs::create_dir_all(&support).map_err(|error| error.to_string())?;
    let script = support.join("mcp_catalogo.py");
    let catalog = support.join("curated.json");
    let comparison_script = support.join("shifter.py");
    let comparison_data = support.join("shifter.json");
    for target in [&script, &catalog, &comparison_script, &comparison_data] {
        if std::fs::symlink_metadata(target).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err("MCP support file is a symlink".into());
        }
    }
    crate::paths::write_atomic(&script, include_bytes!("../../../tools/mcp_catalogo.py"))?;
    crate::paths::write_atomic(&catalog, include_bytes!("../../core/curated.json"))?;
    crate::paths::write_atomic(&comparison_script, include_bytes!("../../../tools/shifter.py"))?;
    crate::paths::write_atomic(&comparison_data, include_bytes!("../../core/shifter.json"))?;
    serde_json::to_string_pretty(&serde_json::json!({"mcpServers": {"shiftanything": {
        "command": uv, "args": ["run", script.to_string_lossy(),
            "--automation-read-root", root.to_string_lossy(), "--automation-write-root", root.to_string_lossy(),
            "--bridge-root", root.to_string_lossy()]
    }}}))
    .map_err(|error| error.to_string())
}

/// Checks local prerequisites and a recent real stdio tool call. Does not read client configs.
pub fn connection() -> Result<bool, String> {
    connection_in(&root()?)
}

pub fn connection_in(root: &std::path::Path) -> Result<bool, String> {
    if uv_executable().is_none() {
        return Err("uv is required; install it from https://docs.astral.sh/uv/getting-started/installation/".into());
    }
    let path = root.join("client.json");
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

/// Finder and GUI clients do not inherit a login shell's PATH. Persist an absolute executable.
fn uv_executable() -> Option<PathBuf> {
    let name = if cfg!(windows) { "uv.exe" } else { "uv" };
    let mut candidates: Vec<PathBuf> = std::env::var_os("PATH")
        .into_iter()
        .flat_map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .filter(|path| path.is_absolute())
        .map(|path| path.join(name))
        .collect();
    if let Some(home) = dirs::home_dir() {
        candidates.push(home.join(".local/bin").join(name));
        candidates.push(home.join(".cargo/bin").join(name));
    }
    #[cfg(unix)]
    for folder in ["/opt/homebrew/bin", "/usr/local/bin", "/usr/bin"] {
        candidates.push(PathBuf::from(folder).join(name));
    }
    uv_from_candidates(candidates)
}
fn uv_from_candidates(candidates: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    candidates.into_iter().find_map(|path| {
        let metadata = std::fs::metadata(&path).ok()?;
        if !metadata.is_file() {
            return None;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 == 0 {
                return None;
            }
        }
        std::fs::canonicalize(path).ok()
    })
}
#[cfg(test)]
mod executable_tests {
    #[test]
    fn uv_candidates_require_an_executable_file_and_store_an_absolute_path() {
        let root = tempfile::tempdir().unwrap();
        let missing = root.path().join("missing");
        assert!(super::uv_from_candidates([missing.clone(), root.path().into()]).is_none());
        let file = root.path().join("uv");
        std::fs::write(&file, "fixture").unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
            assert!(super::uv_from_candidates([file.clone()]).is_none());
            std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let resolved = super::uv_from_candidates([missing, file.clone()]).unwrap();
        assert!(resolved.is_absolute());
        assert_eq!(resolved, std::fs::canonicalize(file).unwrap());
    }
}
