//! The normal desktop entry point and the opt-in isolated verification profile.
use std::ffi::OsString;
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub(crate) const LANGUAGE_FILE: &str = "language";
const MARKER: &str = ".shiftanything-proof";
const MARKER_CONTENTS: &[u8] = b"shiftanything-tauri-proof-v1\n";
type StartupError = Box<dyn std::error::Error>;

pub(crate) struct Launch {
    pub root: PathBuf,
    pub language: &'static str,
    pub isolated: bool,
    pub capture_window: bool,
    pub lifecycle: bool,
    _desktop_lock: Option<File>,
}

pub(crate) fn start(args: Vec<OsString>) -> Result<Option<Launch>, StartupError> {
    if args.len() == 1 && (args[0] == "--version" || args[0] == "-V") {
        println!("shiftanything {}", env!("CARGO_PKG_VERSION"));
        return Ok(None);
    }
    if args.len() == 1 && args[0] == "--check-update-channel" {
        crate::launcher_update::check_packaged_channel()?;
        return Ok(None);
    }
    #[cfg(windows)]
    if !args.is_empty() {
        return windows_arguments(&args);
    }
    if args.is_empty() || finder_launch(&args) {
        return desktop();
    }
    if args[0] == "--child" {
        return isolated_child(&args);
    }
    spawn_isolated(&args)?;
    Ok(None)
}

#[cfg(windows)]
fn windows_arguments(args: &[OsString]) -> Result<Option<Launch>, StartupError> {
    let manifest = match args {
        [flag, manifest] if flag == "--install-msis" => Path::new(manifest),
        [flag, ..] if flag == "--install-msis" => return Err("Expected --install-msis <manifest>".into()),
        _ => return Err("Isolated verification profiles require macOS or Linux. Use a disposable Windows VM for verification.".into()),
    };
    #[cfg(feature = "windows-lifecycle")]
    {
        // The elevated process does its work before creating a webview or acquiring the desktop instance lock.
        artcraft_launcher_native::install::msi_session::run_elevated(manifest)?;
        Ok(None)
    }
    #[cfg(not(feature = "windows-lifecycle"))]
    {
        let _ = manifest;
        Err("Windows MSI installation is disabled in this build".into())
    }
}

fn finder_launch(args: &[OsString]) -> bool {
    cfg!(target_os = "macos") && args.len() == 1 && args[0].as_encoded_bytes().starts_with(b"-psn_")
}

fn desktop() -> Result<Option<Launch>, StartupError> {
    let root = artcraft_launcher_native::paths::Paths::new()?.data.join("desktop");
    fs::create_dir_all(root.parent().ok_or("Missing launcher data folder")?)?;
    private_directory(&root, true)?;
    let lock_path = root.join("instance.lock");
    if fs::symlink_metadata(&lock_path).is_ok_and(|metadata| !metadata.is_file() || metadata.is_symlink()) {
        return Err("Launcher instance lock must be a regular file".into());
    }
    let lock = OpenOptions::new().write(true).create(true).truncate(false).open(lock_path)?;
    match lock.try_lock() {
        Ok(()) => {}
        Err(TryLockError::WouldBlock) => return Ok(None),
        Err(TryLockError::Error(error)) => return Err(error.into()),
    }
    let language = saved_language(&root).unwrap_or_else(|| preferred_language(sys_locale::get_locales()));
    let lifecycle = cfg!(any(target_os = "macos", target_os = "linux", all(windows, feature = "windows-lifecycle")));
    Ok(Some(Launch { root, language, isolated: false, capture_window: false, lifecycle, _desktop_lock: Some(lock) }))
}

fn preferred_language(locales: impl IntoIterator<Item = impl AsRef<str>>) -> &'static str {
    locales
        .into_iter()
        .find_map(|locale| match locale.as_ref().split(['_', '-', '.', '@']).next() {
            Some(language) if language.eq_ignore_ascii_case("it") => Some("it"),
            Some(language) if language.eq_ignore_ascii_case("en") => Some("en"),
            _ => None,
        })
        .unwrap_or("it")
}

fn saved_language(root: &Path) -> Option<&'static str> {
    let path = root.join(LANGUAGE_FILE);
    if !fs::symlink_metadata(&path).ok()?.file_type().is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    File::open(path).ok()?.take(3).read_to_end(&mut bytes).ok()?;
    match bytes.as_slice() {
        b"it" => Some("it"),
        b"en" => Some("en"),
        _ => None,
    }
}

fn private_directory(path: &Path, allow_existing: bool) -> io::Result<()> {
    let builder = fs::DirBuilder::new();
    #[cfg(unix)]
    let builder = {
        use std::os::unix::fs::DirBuilderExt;
        let mut builder = builder;
        builder.mode(0o700);
        builder
    };
    match builder.create(path) {
        Ok(()) => Ok(()),
        Err(error) if allow_existing && error.kind() == io::ErrorKind::AlreadyExists && fs::symlink_metadata(path)?.file_type().is_dir() => Ok(()),
        Err(error) => Err(error),
    }
}

fn check_marker(root: &Path) -> Result<(), StartupError> {
    let marker = root.join(MARKER);
    if !fs::symlink_metadata(&marker)?.file_type().is_file() {
        return Err("Resume requires a profile created by this proof".into());
    }
    let mut bytes = Vec::new();
    File::open(marker)?.take(65).read_to_end(&mut bytes)?;
    if bytes != MARKER_CONTENTS {
        return Err("Resume requires a profile created by this proof".into());
    }
    Ok(())
}

fn proof_language(value: Option<&OsString>) -> Result<&'static str, StartupError> {
    match value {
        None => Ok("it"),
        Some(value) if value == "it" => Ok("it"),
        Some(value) if value == "en" => Ok("en"),
        Some(_) => Err("Use it or en".into()),
    }
}

fn spawn_isolated(args: &[OsString]) -> Result<(), StartupError> {
    let root = PathBuf::from(&args[0]);
    if !root.is_absolute() {
        return Err("Verification profile must be absolute".into());
    }
    let language = proof_language(args.get(1))?;
    if args.iter().skip(2).any(|argument| argument == "--resume") {
        check_marker(&root)?;
    } else {
        private_directory(&root, false)?;
        fs::write(root.join(MARKER), MARKER_CONTENTS)?;
    }
    let root = root.canonicalize()?;
    let home = root.join("home");
    for path in [&home, &home.join(".Trash"), &home.join("config"), &home.join("data"), &home.join("cache")] {
        private_directory(path, true)?;
    }
    #[cfg(target_os = "linux")]
    private_directory(&home.join("state"), true)?;
    println!("PROOF_ROOT={}", root.display());
    let mut child = std::process::Command::new(std::env::current_exe()?);
    #[cfg(target_os = "linux")]
    child.env("XDG_STATE_HOME", home.join("state")).env("APPIMAGE_EXTRACT_AND_RUN", "1");
    let status = child
        .arg("--child")
        .arg(&root)
        .arg(language)
        .args(args.iter().skip(2))
        .env("HOME", &home)
        .env("CFFIXED_USER_HOME", &home)
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_CACHE_HOME", home.join("cache"))
        .env("CRAFT_VERSIONS_FILE", home.join("Library/Application Support/ShiftAnything/versions.json"))
        .env_remove("ARTCRAFT_LAUNCHER_GITHUB_TOKEN")
        .env_remove("ARTCRAFT_LAUNCHER_DOWNLOAD_BASE")
        .env_remove("ARTCRAFT_LAUNCHER_FEED_URL")
        .status()?;
    if status.success() { Ok(()) } else { Err("Desktop proof failed".into()) }
}

fn isolated_child(args: &[OsString]) -> Result<Option<Launch>, StartupError> {
    let root = PathBuf::from(args.get(1).ok_or("Missing verification profile")?).canonicalize()?;
    check_marker(&root)?;
    let home = root.join("home");
    if home.canonicalize()? != home || std::env::var_os("HOME").as_deref() != Some(home.as_os_str()) {
        return Err("The verification process must use its own home directory".into());
    }
    let language = proof_language(Some(args.get(2).ok_or("Missing verification language")?))?;
    let lifecycle = args.iter().skip(3).any(|arg| arg == "--lifecycle");
    let capture_window = args.iter().skip(3).any(|arg| arg == "--capture-window");
    #[cfg(target_os = "macos")]
    crate::verification::seed_previous_from_args(&root, &args[3..], lifecycle)?;
    #[cfg(not(target_os = "macos"))]
    if args[3..].iter().any(|arg| arg.to_str().is_some_and(|arg| arg.starts_with("--seed-") && arg.ends_with("-previous"))) {
        return Err("Previous DMG preparation requires macOS".into());
    }
    Ok(Some(Launch { root, language, isolated: true, capture_window, lifecycle, _desktop_lock: None }))
}

#[cfg(all(test, windows))]
mod windows_tests {
    use super::*;

    #[test]
    fn isolated_profile_arguments_are_rejected_before_creating_files() -> Result<(), StartupError> {
        let sandbox = tempfile::tempdir()?;
        let profile = sandbox.path().join("verification-profile");
        let profile_arg = profile.as_os_str().to_owned();
        for args in [vec![profile_arg.clone(), "it".into(), "--lifecycle".into()], vec!["--child".into(), profile_arg, "it".into()]] {
            assert!(matches!(start(args), Err(error) if error.to_string().contains("disposable Windows VM")));
            assert!(!profile.exists());
        }
        Ok(())
    }

    #[test]
    fn msi_broker_requires_exact_arguments() -> Result<(), StartupError> {
        let sandbox = tempfile::tempdir()?;
        let manifest = sandbox.path().join("batch manifest.json");
        let manifest_arg = manifest.as_os_str().to_owned();
        for args in [vec!["--install-msis".into()], vec!["--install-msis".into(), manifest_arg.clone(), "--extra".into()]] {
            assert!(matches!(start(args), Err(error) if error.to_string().contains("Expected --install-msis <manifest>")));
        }
        let result = start(vec!["--install-msis".into(), manifest_arg]);
        #[cfg(feature = "windows-lifecycle")]
        assert!(matches!(result, Err(error) if error.to_string().contains("couldn't read the batch manifest")));
        #[cfg(not(feature = "windows-lifecycle"))]
        assert!(matches!(result, Err(error) if error.to_string().contains("disabled in this build")));
        assert_eq!(sandbox.path().read_dir()?.count(), 0);
        Ok(())
    }

    #[cfg(feature = "windows-lifecycle")]
    #[test]
    fn msi_broker_completes_without_a_desktop_launch() -> Result<(), StartupError> {
        let sandbox = tempfile::tempdir()?;
        let manifest = sandbox.path().join("batch manifest.json");
        fs::write(&manifest, r#"{"token":"test-session","msis":[]}"#)?;
        assert!(start(vec!["--install-msis".into(), manifest.as_os_str().to_owned()])?.is_none());
        assert_eq!(fs::read_to_string(manifest.with_extension("results"))?, "test-session");
        assert!(!manifest.with_extension("pid").exists());
        Ok(())
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn desktop_uses_the_first_supported_system_language() {
        assert_eq!(preferred_language(["fr-FR", "en-GB", "it-IT"]), "en");
        assert_eq!(preferred_language(["it_IT.UTF-8", "en-US"]), "it");
        assert_eq!(preferred_language(["EN_us", "it"]), "en");
        assert_eq!(preferred_language(["fr", "C.UTF-8"]), "it");
    }

    #[test]
    fn desktop_language_accepts_only_known_preferences() -> Result<(), StartupError> {
        let root = tempfile::tempdir()?;
        assert_eq!(saved_language(root.path()), None);
        let path = root.path().join(LANGUAGE_FILE);
        fs::write(&path, "en")?;
        assert_eq!(saved_language(root.path()), Some("en"));
        fs::write(&path, "it")?;
        assert_eq!(saved_language(root.path()), Some("it"));
        fs::write(&path, "en-US")?;
        assert_eq!(saved_language(root.path()), None);
        Ok(())
    }

    #[test]
    fn existing_profile_directories_and_markers_must_not_be_symlinks() -> Result<(), StartupError> {
        use std::os::unix::fs::symlink;

        let root = tempfile::tempdir()?;
        let directory = root.path().join("home");
        private_directory(&directory, false)?;
        assert!(private_directory(&directory, false).is_err());
        assert!(private_directory(&directory, true).is_ok());
        let link = root.path().join("linked-home");
        symlink(&directory, &link)?;
        assert!(private_directory(&link, true).is_err());
        let real_marker = root.path().join("other");
        fs::write(&real_marker, MARKER_CONTENTS)?;
        symlink(&real_marker, root.path().join(MARKER))?;
        assert!(check_marker(root.path()).is_err());
        Ok(())
    }
}
