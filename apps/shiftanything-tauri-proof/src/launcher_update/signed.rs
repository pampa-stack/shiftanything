use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;
use tauri::{AppHandle, Context, Url, Wry};
use tauri_plugin_updater::{Update, UpdaterExt};

const ENDPOINT: Option<&str> = option_env!("SHIFT_LAUNCHER_UPDATE_ENDPOINT");
const PUBLIC_KEY: Option<&str> = option_env!("SHIFT_LAUNCHER_UPDATE_PUBKEY");
#[cfg(target_os = "macos")]
const TARGET: &str = "darwin-aarch64";
#[cfg(target_os = "linux")]
const TARGET: &str = "linux-x86_64-deb";
#[cfg(windows)]
const TARGET: &str = "windows-x86_64";

#[derive(Clone)]
pub(super) struct Channel {
    endpoint: Url,
}

fn channel<'a>(endpoint: Option<&str>, public_key: Option<&'a str>) -> Result<(Channel, &'a str), &'static str> {
    let (endpoint, public_key) = match (endpoint, public_key) {
        (None, None) => return Err("channel_disabled"),
        (Some(endpoint), Some(public_key)) => (endpoint, public_key.trim()),
        _ => return Err("channel_invalid"),
    };
    let endpoint = Url::parse(endpoint).map_err(|_| "channel_invalid")?;
    if !trusted_url(&endpoint) || endpoint.query().is_some() {
        return Err("channel_invalid");
    }
    let decoded = STANDARD.decode(public_key).map_err(|_| "channel_invalid")?;
    let decoded = std::str::from_utf8(&decoded).map_err(|_| "channel_invalid")?;
    minisign_verify::PublicKey::decode(decoded).map_err(|_| "channel_invalid")?;
    Ok((Channel { endpoint }, public_key))
}

fn trusted_url(url: &Url) -> bool {
    url.scheme() == "https" && url.has_host() && url.username().is_empty() && url.password().is_none() && url.fragment().is_none()
}

/// Packages come from the same host that serves the signed manifest.
fn trusted_download(endpoint: &Url, download: &Url) -> bool {
    trusted_url(download) && download.host_str() == endpoint.host_str() && download.port_or_known_default() == endpoint.port_or_known_default()
}

pub(super) fn check_packaged_channel() -> Result<(), &'static str> {
    let (channel, _) = channel(ENDPOINT, PUBLIC_KEY)?;
    println!("{TARGET} {}", channel.endpoint);
    Ok(())
}

pub(super) fn configure(context: &mut Context<Wry>, isolated: bool) -> Result<Channel, &'static str> {
    if isolated {
        return Err("channel_disabled");
    }
    let (channel, public_key) = channel(ENDPOINT, PUBLIC_KEY)?;
    installed_path().map_err(|_| "installation_unsupported")?;
    context.config_mut().plugins.0.insert(
        "updater".into(),
        serde_json::json!({
            "pubkey": public_key,
            "requireSignedVersion": true,
        }),
    );
    Ok(channel)
}

pub(super) async fn check(app: &AppHandle, channel: Channel) -> Result<Option<Update>, &'static str> {
    let builder = app.updater_builder();
    // Keep the launcher usable if Windows refuses to start the setup; a successful start exits the process.
    #[cfg(windows)]
    let builder = builder.on_before_exit(|| {});
    let updater = builder
        .endpoints(vec![channel.endpoint.clone()])
        .map_err(|_| "channel_invalid")?
        .timeout(Duration::from_secs(15))
        .target(TARGET)
        .configure_client(|client| client.https_only(true))
        .build()
        .map_err(|_| "channel_invalid")?;
    let mut update = updater.check().await.map_err(|_| "check_failed")?;
    if let Some(update) = &mut update {
        if !trusted_download(&channel.endpoint, &update.download_url) || update.signature.trim().is_empty() {
            return Err("release_invalid");
        }
        update.timeout = Some(Duration::from_secs(180));
    }
    Ok(update)
}

/// A downloaded package whose signature and signed version the updater plugin verified.
/// macOS and Linux apply it at once; Windows keeps the setup until the launcher is idle.
pub(super) struct Staged {
    #[cfg(windows)]
    update: Update,
    #[cfg(windows)]
    bytes: Vec<u8>,
}

pub(super) async fn install(update: Update) -> Result<Staged, &'static str> {
    let path = installed_path().map_err(|_| "installation_unsupported")?;
    let bytes = update.download(|_, _| {}, || {}).await.map_err(|_| "download_or_signature_failed")?;
    #[cfg(windows)]
    {
        let _ = path;
        if !nsis_setup(&bytes) {
            return Err("release_invalid");
        }
        Ok(Staged { update, bytes })
    }
    #[cfg(not(windows))]
    {
        tauri::async_runtime::spawn_blocking(move || apply(&path, &update.version, &bytes)).await.map_err(|_| "installation_failed")??;
        Ok(Staged {})
    }
}

/// Starts the updated launcher. Returns only when that could not start.
pub(super) fn finish(app: &AppHandle, staged: Staged) -> Result<(), &'static str> {
    #[cfg(windows)]
    {
        let _ = app;
        // The plugin runs the setup with /P /UPDATE /R: it waits for this process, replaces the
        // installed files and starts the new version. On success the plugin exits this process.
        staged.update.install(&staged.bytes).map_err(|_| "installation_failed")
    }
    #[cfg(not(windows))]
    {
        let _ = staged;
        app.exit(super::RESTART_EXIT_CODE);
        Ok(())
    }
}

#[cfg(any(windows, test))]
fn nsis_setup(bytes: &[u8]) -> bool {
    // A PE image carrying the NSIS first-header signature. This only filters the format:
    // the plugin's minisign check of the signed version is what authenticates the package.
    const NSIS: &[u8] = b"\xEF\xBE\xAD\xDENullsoftInst";
    bytes.starts_with(b"MZ") && bytes.windows(NSIS.len()).any(|window| window == NSIS)
}

#[cfg(not(windows))]
fn checked_output(binary: &str, args: &[&str], path: &Path) -> Result<String, &'static str> {
    let output = Command::new(binary).args(args).arg(path).output().map_err(|_| "installation_failed")?;
    if !output.status.success() {
        return Err("installation_failed");
    }
    String::from_utf8(output.stdout).map_err(|_| "installation_failed")
}

#[cfg(target_os = "macos")]
fn installed_path() -> Result<PathBuf, &'static str> {
    if !cfg!(target_arch = "aarch64") {
        return Err("installation_unsupported");
    }
    let exe = std::env::current_exe().and_then(std::fs::canonicalize).map_err(|_| "installation_unsupported")?;
    let macos = exe.parent().filter(|p| p.ends_with("Contents/MacOS")).ok_or("installation_unsupported")?;
    let app = macos.parent().and_then(Path::parent).filter(|p| p.extension().is_some_and(|ext| ext == "app")).ok_or("installation_unsupported")?;
    const ID: &str = "it.promptica.shiftanything";
    if plist_value(app, "CFBundleIdentifier")? != ID
        || exe.file_name().and_then(|p| p.to_str()) != Some(plist_value(app, "CFBundleExecutable")?.as_str())
    {
        return Err("installation_unsupported");
    }
    Ok(app.to_owned())
}

#[cfg(target_os = "macos")]
fn plist_value(app: &Path, key: &str) -> Result<String, &'static str> {
    let plist = app.join("Contents/Info.plist");
    checked_output("/usr/bin/plutil", &["-extract", key, "raw", "-o", "-"], &plist).map(|text| text.trim().to_owned())
}

#[cfg(target_os = "macos")]
fn apply(app: &Path, version: &str, bytes: &[u8]) -> Result<(), &'static str> {
    let parent = app.parent().ok_or("installation_unsupported")?;
    let staging = tempfile::tempdir_in(parent).map_err(|_| "installation_permissions")?;
    let archive = staging.path().join("update.tar.gz");
    std::fs::write(&archive, bytes).map_err(|_| "installation_failed")?;
    let status = Command::new("/usr/bin/tar")
        .arg("--mac-metadata")
        .arg("-xzpf")
        .arg(&archive)
        .arg("-C")
        .arg(staging.path())
        .status()
        .map_err(|_| "release_invalid")?;
    if !status.success() {
        return Err("release_invalid");
    }
    let replacement = staging.path().join("Sh*ftAnything.app");
    if !std::fs::symlink_metadata(&replacement).is_ok_and(|metadata| metadata.is_dir())
        || plist_value(&replacement, "CFBundleIdentifier")? != "it.promptica.shiftanything"
        || plist_value(&replacement, "CFBundleShortVersionString")? != version
        || plist_value(&replacement, "CFBundleExecutable")? != "ShiftAnything"
    {
        return Err("release_invalid");
    }
    let source = Command::new("/usr/bin/codesign").args(["-dv", "--verbose=4"]).arg(app).output().map_err(|_| "installation_failed")?;
    let details = String::from_utf8(source.stderr).map_err(|_| "installation_failed")?;
    let team = details
        .lines()
        .find_map(|line| line.strip_prefix("TeamIdentifier="))
        .filter(|team| team.len() == 10 && team.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit()))
        .ok_or("installation_unsupported")?;
    let requirement = format!("=anchor apple generic and identifier \"it.promptica.shiftanything\" and certificate leaf[subject.OU] = \"{team}\"");
    checked_output("/usr/bin/codesign", &["--verify", "--strict", "--deep", "--all-architectures", "-R", &requirement], &replacement)
        .map_err(|_| "release_invalid")?;
    let assessment = Command::new("/usr/sbin/spctl")
        .args(["--assess", "--type", "execute", "--ignore-cache", "--no-cache", "-vv"])
        .arg(&replacement)
        .output()
        .map_err(|_| "release_invalid")?;
    if !assessment.status.success() || !String::from_utf8_lossy(&assessment.stderr).lines().any(|line| line == "source=Notarized Developer ID") {
        return Err("release_invalid");
    }
    let executable = replacement.join("Contents/MacOS/ShiftAnything");
    let description = checked_output("/usr/bin/file", &["-b"], &executable).map_err(|_| "release_invalid")?;
    if !description.contains("Mach-O") || !description.split(|c: char| !c.is_ascii_alphanumeric()).any(|token| token == "arm64") {
        return Err("release_invalid");
    }
    use rustix::fs::{CWD, RenameFlags, renameat_with};
    renameat_with(CWD, app, CWD, &replacement, RenameFlags::EXCHANGE).map_err(|_| "installation_permissions")
}

#[cfg(target_os = "linux")]
fn installed_path() -> Result<PathBuf, &'static str> {
    const PATH: &str = "/usr/bin/shiftanything";
    let exe = std::env::current_exe().and_then(std::fs::canonicalize).map_err(|_| "installation_unsupported")?;
    if !cfg!(target_arch = "x86_64") || exe != Path::new(PATH) || !Path::new("/usr/bin/pkexec").is_file() {
        return Err("installation_unsupported");
    }
    let package = checked_output("/usr/bin/dpkg-query", &["-S"], &exe)?;
    if package.trim() != "shiftanything: /usr/bin/shiftanything" {
        return Err("installation_unsupported");
    }
    if !installed_version(env!("CARGO_PKG_VERSION"))? {
        return Err("installation_unsupported");
    }
    Ok(exe)
}

#[cfg(target_os = "linux")]
fn installed_version(version: &str) -> Result<bool, &'static str> {
    let installed =
        checked_output("/usr/bin/dpkg-query", &["-W", "--showformat=${db:Status-Status}\t${Version}\t${Architecture}"], Path::new("shiftanything"))?;
    Ok(installed == format!("installed\t{version}\tamd64"))
}

#[cfg(target_os = "linux")]
fn apply(_exe: &Path, version: &str, bytes: &[u8]) -> Result<(), &'static str> {
    let staging = tempfile::tempdir().map_err(|_| "installation_failed")?;
    let path = staging.path().join("shiftanything.deb");
    std::fs::write(&path, bytes).map_err(|_| "installation_failed")?;
    for (field, expected) in [("Package", "shiftanything"), ("Architecture", "amd64"), ("Version", version)] {
        let result = Command::new("/usr/bin/dpkg-deb").arg("-f").arg(&path).arg(field).output().map_err(|_| "release_invalid")?;
        if !result.status.success() || String::from_utf8_lossy(&result.stdout).trim() != expected {
            return Err("release_invalid");
        }
    }
    let result = Command::new("/usr/bin/pkexec")
        .args(["/usr/bin/apt-get", "-y", "--no-remove", "--only-upgrade", "install", "--"])
        .arg(path)
        .stdin(std::process::Stdio::null())
        .status()
        .map_err(|_| "installation_failed")?;
    match result.code() {
        Some(0) if installed_version(version)? => Ok(()),
        Some(126) => Err("installation_cancelled"),
        Some(127) => Err("installation_permissions"),
        _ => Err("installation_failed"),
    }
}

#[cfg(windows)]
const UNINSTALL_KEY: &str = r"HKCU\Software\Microsoft\Windows\CurrentVersion\Uninstall\ShiftAnything";

/// Only the per-user NSIS installation registered for this exact executable and version can update itself.
#[cfg(windows)]
fn installed_path() -> Result<PathBuf, &'static str> {
    if !cfg!(target_arch = "x86_64") {
        return Err("installation_unsupported");
    }
    let exe = std::env::current_exe().and_then(std::fs::canonicalize).map_err(|_| "installation_unsupported")?;
    let value = |name: &str| -> Result<String, &'static str> {
        let reg = Path::new(&std::env::var_os("SystemRoot").ok_or("installation_unsupported")?).join(r"System32\reg.exe");
        let reg = reg.to_str().ok_or("installation_unsupported")?.to_owned();
        let output = Command::new(reg).args(["query", UNINSTALL_KEY, "/v", name]).output().map_err(|_| "installation_unsupported")?;
        if !output.status.success() {
            return Err("installation_unsupported");
        }
        registry_value(&String::from_utf8_lossy(&output.stdout), name).ok_or("installation_unsupported")
    };
    let folder = PathBuf::from(value("InstallLocation")?.trim_matches('"'));
    let registered = std::fs::canonicalize(folder.join(value("MainBinaryName")?)).map_err(|_| "installation_unsupported")?;
    let uninstaller = folder.join("uninstall.exe");
    if registered != exe || !uninstaller.is_file() || value("DisplayVersion")? != env!("CARGO_PKG_VERSION") {
        return Err("installation_unsupported");
    }
    Ok(exe)
}

/// Reads one REG_SZ value from `reg query` output.
#[cfg(any(windows, test))]
fn registry_value(output: &str, name: &str) -> Option<String> {
    output.lines().find_map(|line| {
        let rest = line.trim_start().strip_prefix(name)?;
        let rest = rest.strip_prefix(|c: char| c.is_whitespace())?.trim_start().strip_prefix("REG_SZ")?;
        let value = rest.strip_prefix(|c: char| c.is_whitespace())?.trim();
        (!value.is_empty()).then(|| value.to_owned())
    })
}

#[cfg(test)]
mod tests {
    use super::{channel, nsis_setup, registry_value, trusted_download, trusted_url};

    #[test]
    fn channel_fails_closed_for_partial_and_invalid_configurations() {
        for (endpoint, key) in [
            (None, None),
            (Some("https://example.invalid/latest.json"), None),
            (None, Some("bad key")),
            (Some("https://example.invalid/latest.json"), Some("bWluaXNpZ24=")),
        ] {
            assert!(channel(endpoint, key).is_err());
        }
    }

    #[test]
    fn update_request_urls_are_https_and_credential_free() {
        for (url, expected) in [
            ("https://example.invalid/latest.json", true),
            ("https://user:pass@example.invalid/latest.json", false),
            ("https://example.invalid/latest.json#fragment", false),
            ("http://example.invalid/latest.json", false),
            ("file:///tmp/launcher/latest.json", false),
        ] {
            assert_eq!(trusted_url(&url.parse().unwrap()), expected);
        }
    }

    #[test]
    fn packages_must_come_from_the_manifest_host() {
        let endpoint = "https://github.com/pampa-stack/shiftanything/releases/latest/download/latest.json".parse().unwrap();
        for (url, expected) in [
            ("https://github.com/pampa-stack/shiftanything/releases/download/v1.0.0/setup.exe", true),
            ("https://example.invalid/setup.exe", false),
            ("https://github.com:8443/setup.exe", false),
            ("http://github.com/setup.exe", false),
        ] {
            assert_eq!(trusted_download(&endpoint, &url.parse().unwrap()), expected, "{url}");
        }
    }

    #[test]
    fn windows_update_must_be_an_nsis_setup() {
        let mut setup = b"MZ\x90\0".to_vec();
        setup.extend_from_slice(&[0; 64]);
        assert!(!nsis_setup(&setup));
        setup.extend_from_slice(b"\xEF\xBE\xAD\xDENullsoftInst");
        assert!(nsis_setup(&setup));
        assert!(!nsis_setup(&setup[2..]));
    }

    #[test]
    fn registry_values_are_read_by_exact_name() {
        let output = "\r\nHKEY_CURRENT_USER\\Software\\X\r\n    InstallLocation    REG_SZ    \"C:\\Users\\a b\\ShiftAnything\"\r\n    InstallLocationOld    REG_SZ    D:\\x\r\n";
        assert_eq!(registry_value(output, "InstallLocation").as_deref(), Some("\"C:\\Users\\a b\\ShiftAnything\""));
        assert_eq!(registry_value(output, "Install"), None);
        assert_eq!(registry_value(output, "DisplayVersion"), None);
    }
}
