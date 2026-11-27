//! Reproducible signed-package probe. No changes to installed apps or the personal Trash.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[cfg(target_os = "macos")]
fn main() -> Result<(), String> {
    use artcraft_launcher_core::{InstallSource, Installed, curated::HANDY, version};
    use artcraft_launcher_native::{http, install};
    let source = std::env::args_os().nth(1).ok_or("supply the official Handy_0.10.2_aarch64.dmg path")?;
    let image = std::path::PathBuf::from(source);
    let hash = http::sha256_file(&image)?;
    if hash != "b8b893fbf9e8262abb0e363cb0ac6715cc69eac2fd6ffd8c0785278083a0e3ca" {
        return Err("unexpected package checksum".into());
    }
    let previous_image = std::env::args_os().nth(2).map(std::path::PathBuf::from);
    if let Some(previous) = &previous_image
        && http::sha256_file(previous)? != "7cdef5de7d1bfcea70813320b897540cbead2190f5280db6d9760a59e5995463"
    {
        return Err("unexpected previous package checksum".into());
    }
    let profile = tempfile::tempdir().map_err(|e| e.to_string())?;
    let apps = profile.path().join("Applications");
    let trash = profile.path().join("Trash");
    let home = profile.path().join("home");
    for dir in [&apps, &trash, &home] {
        std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    let previous = if let Some(previous_image) = &previous_image {
        let old_version = version::parse("0.10.1").ok_or("invalid previous version")?;
        let expect = install::macos::Expect { team: HANDY.mac_team, version: Some(&old_version) };
        let placed = install::macos::install_dmg(&HANDY, previous_image, None, Some(&apps), None, &expect)?;
        verify_help(&placed, &home)?;
        println!("PASS signed 0.10.1 install and actual executable --help inside temporary profile");
        Some(Installed {
            version: Some(old_version),
            source: InstallSource::MacApp,
            location: placed.display().to_string(),
            package: None,
            externally_managed: false,
            origin: None,
            installation: None,
        })
    } else {
        None
    };
    let version = version::parse("0.10.2").ok_or("invalid recipe version")?;
    let expect = install::macos::Expect { team: HANDY.mac_team, version: Some(&version) };
    let placed = install::macos::install_dmg(&HANDY, &image, previous.as_ref(), Some(&apps), None, &expect)?;
    println!("PASS signed install inside temporary profile");
    verify_help(&placed, &home)?;
    println!("PASS launch installed official executable (--help; no GUI opened)");
    if previous.is_some() {
        println!("PASS signed official update 0.10.1 → 0.10.2 with expected version/team");
    }
    install::macos::move_to_trash_in(&placed, &trash)?;
    if placed.exists() || !trash.join("Handy.app").exists() {
        return Err("removal did not move the app into the temporary Trash".into());
    }
    println!("PASS native removal into temporary Trash; profile cleaned on exit");
    println!("NOT TESTED: GUI startup, download through UI, MCP approval; update requires optional 0.10.1 argument");
    Ok(())
}
#[cfg(not(target_os = "macos"))]
fn main() {
    eprintln!("This verified recipe probe requires macOS arm64.");
}

#[cfg(target_os = "macos")]
fn verify_help(placed: &std::path::Path, home: &std::path::Path) -> Result<(), String> {
    use artcraft_launcher_native::process;
    let executable = placed.join("Contents/MacOS/handy");
    let mut command = process::command(&executable);
    command
        .arg("--help")
        .env("HOME", home)
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_CACHE_HOME", home.join("cache"));
    let output = process::run_command(command, "Handy --help", std::time::Duration::from_secs(15))?;
    if !output.contains("--start-hidden") {
        return Err("installed program did not return documented CLI help".into());
    }
    Ok(())
}
