//! Opt-in preparation of the reviewed previous official release, never a webview command.
use artcraft_launcher_core::{PackageKind, curated::HANDY, version};
use artcraft_launcher_native::{http, install, manifest, paths::Paths};
use std::path::Path;

pub fn seed_previous_handy(root: &Path, previous: &Path) -> Result<(), String> {
    let home = root.join("home");
    let configured = std::env::var_os("HOME").ok_or("Missing isolated HOME")?;
    if std::env::var_os("CFFIXED_USER_HOME").as_ref() != Some(&configured)
        || std::path::PathBuf::from(&configured).canonicalize().map_err(|error| error.to_string())? != home
    {
        return Err("Previous-release preparation requires the isolated profile".into());
    }
    if http::sha256_file(previous)? != "7cdef5de7d1bfcea70813320b897540cbead2190f5280db6d9760a59e5995463" {
        return Err("Unexpected previous Handy artifact digest".into());
    }
    // Reuse the native factory's canonical-path and profile-boundary validation.
    let _boundary = artcraft_launcher_native::NativeBackend::new_isolated_macos_profile(|| {})?;
    let paths = Paths::new()?;
    let apps = home.join("Applications");
    std::fs::create_dir_all(&apps).map_err(|error| error.to_string())?;
    if paths.manifest().exists() || apps.join("Handy.app").exists() {
        return Err("Previous-release preparation requires an unused profile".into());
    }
    let old = version::parse("0.10.1").ok_or("Invalid previous version")?;
    let placed = install::macos::install_dmg(
        &HANDY,
        previous,
        None,
        Some(&apps),
        None,
        &paths,
        &install::macos::Expect { team: HANDY.mac_team, version: Some(&old) },
    )?;
    let mut manifest = manifest::Manifest::default();
    manifest.apps.insert(
        "handy".into(),
        manifest::Record {
            version: old.to_string(),
            kind: PackageKind::Dmg,
            path: placed,
            installed_at: artcraft_launcher_native::now(),
            files: vec![],
            needs_root: false,
        },
    );
    manifest.save(&paths.manifest())?;
    let sentinel = home.join("Library/Application Support/com.pais.handy/shift-proof-user-data");
    std::fs::create_dir_all(sentinel.parent().ok_or("Missing sentinel parent")?).map_err(|error| error.to_string())?;
    std::fs::write(sentinel, "synthetic-data-keep").map_err(|error| error.to_string())?;
    println!("PREPARED official Handy 0.10.1; native digest, signature and version checks passed");
    Ok(())
}
