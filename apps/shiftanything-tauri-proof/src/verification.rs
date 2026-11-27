//! Opt-in preparation of the reviewed previous official release, never a webview command.
use artcraft_launcher_core::{
    CraftApp, PackageKind,
    curated::{HANDY, LOCALSEND, OBS_STUDIO},
    version,
};
use artcraft_launcher_native::{http, install, manifest, paths::Paths};
use std::path::Path;

pub fn seed_previous_from_args(root: &Path, args: &[std::ffi::OsString], lifecycle: bool) -> Result<(), String> {
    let mut selected = [
        ("--seed-handy-previous", &HANDY, "0.10.1", "7cdef5de7d1bfcea70813320b897540cbead2190f5280db6d9760a59e5995463"),
        ("--seed-localsend-previous", &LOCALSEND, "1.18.2", "126860d56f6f49b11845f601aac51de27a49b16d2b48102415da91e0e37e5155"),
        ("--seed-obs-previous", &OBS_STUDIO, "33.0.2", "f9cd96511c5efdd3ffd9a0068e3bbc276c61fc451d1004e3eea5658a9cb70488"),
    ]
    .into_iter()
    .filter_map(|(flag, app, version, sha256)| {
        args.iter().position(|arg| arg == flag).map(|position| (args.get(position + 1), app, version, sha256))
    });
    let Some((previous, app, version, sha256)) = selected.next() else {
        return Ok(());
    };
    if !lifecycle || selected.next().is_some() {
        return Err("Previous-release preparation requires --lifecycle and one seed".into());
    }
    let previous = Path::new(previous.ok_or("Missing previous DMG")?);
    seed_previous(root, previous, app, version, sha256)
}

fn seed_previous(root: &Path, previous: &Path, app: &CraftApp, version_text: &str, sha256: &str) -> Result<(), String> {
    let home = root.join("home");
    let configured = std::env::var_os("HOME").ok_or("Missing isolated HOME")?;
    if std::env::var_os("CFFIXED_USER_HOME").as_ref() != Some(&configured)
        || std::path::PathBuf::from(&configured).canonicalize().map_err(|error| error.to_string())? != home
    {
        return Err("Previous-release preparation requires the isolated profile".into());
    }
    if http::sha256_file(previous)? != sha256 {
        return Err(format!("Unexpected previous {} artifact digest", app.name));
    }
    // Reuse the native factory's canonical-path and profile-boundary validation.
    let _boundary = artcraft_launcher_native::NativeBackend::new_isolated_macos_profile(|| {})?;
    let paths = Paths::new()?;
    let apps = home.join("Applications");
    std::fs::create_dir_all(&apps).map_err(|error| error.to_string())?;
    if paths.manifest().exists() || apps.join(format!("{}.app", app.mac_bundle_name())).exists() {
        return Err("Previous-release preparation requires an unused profile".into());
    }
    let old = version::parse(version_text).ok_or("Invalid previous version")?;
    let placed =
        install::macos::install_dmg(app, previous, None, Some(&apps), None, &install::macos::Expect { team: app.mac_team, version: Some(&old) })?;
    let mut manifest = manifest::Manifest::default();
    manifest.apps.insert(
        app.id.into(),
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
    let sentinel = home.join("Library/Application Support").join(app.bundle_id()).join("shift-proof-user-data");
    std::fs::create_dir_all(sentinel.parent().ok_or("Missing sentinel parent")?).map_err(|error| error.to_string())?;
    std::fs::write(sentinel, "synthetic-data-keep").map_err(|error| error.to_string())?;
    println!("PREPARED official {} {version_text}; native digest, signature and version checks passed", app.name);
    Ok(())
}
