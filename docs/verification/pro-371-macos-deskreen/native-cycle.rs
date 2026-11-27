use artcraft_launcher_core::curated::DESKREEN;
use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease, VersionTrack};
use artcraft_launcher_core::release::{Release, parse_github_release};
use artcraft_launcher_core::{Arch, Os, PackageKind, Platform};
use artcraft_launcher_native::{NativeBackend, manifest::Manifest, paths::Paths};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const APP: &str = DESKREEN.id;
const BEFORE: &str = "v3.2.15";
const AFTER: &str = "v3.2.16";
const BUNDLE: &str = "Deskreen CE.app";

fn seed_previous(paths: &Paths) -> Result<(), String> {
    let release = parse_github_release(include_str!("previous-release.json"), DESKREEN.repo)?;
    if release.tag != BEFORE || release.prerelease {
        return Err("expected the previous stable release".into());
    }
    package_size(&release)?;
    let cache = serde_json::json!({
        "entries": {},
        "versions": {DESKREEN.repo: [artcraft_launcher_native::now(), [release]]}
    });
    println!("HISTORICAL METADATA official GitHub tag response supplied to the disposable version cache");
    artcraft_launcher_native::paths::write_atomic(&paths.release_cache(), &serde_json::to_vec(&cache).map_err(|error| error.to_string())?)
}

fn wait<T>(backend: &mut NativeBackend, mut accept: impl FnMut(Event) -> Option<T>) -> Result<T, String> {
    let deadline = Instant::now() + Duration::from_secs(150);
    while Instant::now() < deadline {
        if let Some(event) = backend.poll().into_iter().find_map(&mut accept) {
            return Ok(event);
        }
        std::thread::sleep(Duration::from_millis(35));
    }
    Err("native event timed out".into())
}

fn package_size(release: &Release) -> Result<u64, String> {
    let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
    let (asset, _) = pick_asset(release, &[APP], platform, &[PackageKind::Dmg]).ok_or("missing reviewed Deskreen DMG")?;
    println!("PUBLISHER {} {} bytes {} SHA-256 {}", release.tag, asset.size, asset.url, asset.sha256.as_deref().ok_or("missing digest")?);
    Ok(asset.size)
}

fn expect_release(backend: &mut NativeBackend, tag: &str) -> Result<Release, String> {
    wait(backend, |event| match event {
        Event::Latest { app, latest: LatestRelease::Found(release), .. } if app == APP && release.tag == tag => Some(release),
        _ => None,
    })
}

fn main() -> Result<(), String> {
    let phase = std::env::args().nth(1).ok_or("phase missing")?;
    let home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME missing")?);
    if !home.parent().and_then(Path::file_name).and_then(|dir| dir.to_str()).is_some_and(|dir| dir.starts_with("shift-deskreen-native."))
        || !home.join(".deskreen-verification-profile").is_file()
    {
        return Err("use the disposable profile created by run-native-cycle.sh".into());
    }

    let applications = home.join("Applications");
    let bundle = applications.join(BUNDLE);
    let paths = Paths::new()?;
    if phase == "install" {
        seed_previous(&paths)?;
    }
    let mut backend = NativeBackend::new_isolated_macos_profile(|| {})?;
    let options = BackendOptions {
        install_locations: InstallLocations { root: Some(applications.display().to_string()), ..Default::default() },
        simultaneous_downloads: 1,
        ..Default::default()
    };
    backend.send(Command::Configure(options.clone()));
    backend.send(Command::Detect);
    let installed = wait(&mut backend, |event| match event {
        Event::Detected { app, installed } if app == APP => Some(installed),
        _ => None,
    })?;
    if installed.as_ref().is_some_and(|copy| Path::new(&copy.location) != bundle) {
        return Err("Deskreen detected outside the isolated profile".into());
    }
    let initial = installed.as_ref().and_then(|copy| copy.version.as_ref()).map(ToString::to_string);
    let expected = match (phase.as_str(), initial.as_deref()) {
        ("install", None) => {
            backend.send(Command::ListVersions { app: APP.into() });
            let versions = wait(&mut backend, |event| match event {
                Event::Versions { app, result } if app == APP => Some(result),
                _ => None,
            })??;
            let release = versions.iter().find(|release| release.tag == BEFORE).ok_or("reviewed previous release missing")?;
            let size = package_size(release)?;
            backend.send(Command::Configure(BackendOptions { versions: [(APP.into(), VersionTrack::Pinned(BEFORE.into()))].into(), ..options }));
            let _ = expect_release(&mut backend, BEFORE)?;
            backend.send(Command::InstallChosen { app: APP.into(), tag: BEFORE.into(), fallback: None });
            Some((BEFORE, size))
        }
        ("update", Some("3.2.15")) => {
            backend.send(Command::CheckForUpdates { force: true });
            let release = expect_release(&mut backend, AFTER)?;
            let size = package_size(&release)?;
            backend.send(Command::InstallExpected { apps: vec![APP.into()], installed: true });
            Some((AFTER, size))
        }
        ("remove", Some("3.2.16")) => {
            backend.send(Command::Uninstall { app: APP.into() });
            None
        }
        _ => return Err(format!("unexpected phase {phase} at {initial:?}")),
    };
    let mut transferred = 0;
    let result = wait(&mut backend, |event| match event {
        Event::Progress { app, phase: JobPhase::Downloading { done, .. } } if app == APP => {
            transferred = transferred.max(done);
            None
        }
        Event::JobFinished { app, result } if app == APP => Some(result),
        _ => None,
    })??;
    let manifest = Manifest::load(&paths.manifest());
    if let Some((tag, size)) = expected {
        let record = manifest.apps.get(APP).ok_or("missing installation record")?;
        if record.path != bundle || record.version != tag.trim_start_matches('v') || record.kind != PackageKind::Dmg || !bundle.is_dir() {
            return Err(format!("unexpected installation record {record:?}"));
        }
        if transferred != size {
            return Err(format!("expected {size} downloaded bytes, saw {transferred}"));
        }
    } else if bundle.exists() || manifest.apps.contains_key(APP) || !home.join(".Trash").join(BUNDLE).is_dir() {
        return Err("expected only the removed Deskreen bundle in the disposable Trash".into());
    }
    println!("PASS {phase} {result}; downloaded_bytes={transferred}");
    Ok(())
}
