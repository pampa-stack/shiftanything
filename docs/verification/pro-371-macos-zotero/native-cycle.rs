use artcraft_launcher_core::curated::ZOTERO;
use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease, Origin, VersionTrack};
use artcraft_launcher_core::release::Release;
use artcraft_launcher_core::{Arch, Os, PackageKind, Platform};
use artcraft_launcher_native::{NativeBackend, manifest::Manifest, paths};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const APP: &str = ZOTERO.id;
const BEFORE: (&str, u64, &str) = ("10.0.8", 192345844, "bf6192a0745ae67efbb7ff4e7f1953d8b78f5eed7083f4eefa12b762ecddcabe");
const AFTER: (&str, u64, &str) = ("10.0.9", 192346587, "fa42a04c33028ee020ccf47e64b3e135ce2fc57ba9a099e9f467f280ea56a387");

fn wait_for<T>(backend: &mut NativeBackend, mut pick: impl FnMut(Event) -> Option<T>) -> Result<T, String> {
    let deadline = Instant::now() + Duration::from_secs(360);
    while Instant::now() < deadline {
        if let Some(result) = backend.poll().into_iter().find_map(&mut pick) {
            return Ok(result);
        }
        std::thread::sleep(Duration::from_millis(30));
    }
    Err("timed out waiting for native event".into())
}

fn assert_package(release: &Release, expected: (&str, u64, &str)) -> Result<(), String> {
    let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
    let (asset, kind) = pick_asset(release, &[APP], platform, &[PackageKind::Dmg]).ok_or("missing official ARM package")?;
    let (tag, size, sha256) = expected;
    let url = format!("https://download.zotero.org/client/release/{tag}/Zotero-{tag}.dmg");
    if release.tag != tag || release.version_label() != tag || asset.url != url || asset.size != size || asset.sha256.as_deref() != Some(sha256) {
        return Err(format!("Unexpected official package {asset:?} for {}", release.tag));
    }
    println!("PASS OFFICIAL {tag} {kind:?} {} {size} {sha256}", asset.url);
    Ok(())
}

fn native_release(backend: &mut NativeBackend) -> Result<Release, String> {
    let latest = wait_for(backend, |event| match event {
        Event::Latest { app, latest, from: Origin::Zotero } if app == APP => Some(latest),
        _ => None,
    })?;
    match latest {
        LatestRelease::Found(release) => Ok(release),
        failure => Err(format!("Zotero release unavailable: {failure:?}")),
    }
}

fn main() -> Result<(), String> {
    let phase = std::env::args().nth(1).ok_or("phase missing")?;
    let home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME missing")?);
    if !home.parent().and_then(Path::file_name).and_then(|p| p.to_str()).is_some_and(|p| p.starts_with("shift-zotero-native-scratch."))
        || !home.join(".shift-zotero-native-proof").is_file()
    {
        return Err("run only in disposable proof home".into());
    }
    let applications = home.join("Applications");
    let bundle = applications.join("Zotero.app");
    let path = paths::Paths::new()?;
    let mut backend = NativeBackend::new_isolated_macos_profile(|| {})?;
    let options = BackendOptions {
        install_locations: InstallLocations { root: Some(applications.display().to_string()), ..Default::default() },
        simultaneous_downloads: 1,
        ..Default::default()
    };
    backend.send(Command::Configure(options.clone()));
    backend.send(Command::Detect);
    let installed = wait_for(&mut backend, |ev| match ev {
        Event::Detected { app, installed } if app == APP => Some(installed),
        _ => None,
    })?;
    let initial = installed.as_ref().and_then(|i| i.version.as_ref()).map(ToString::to_string);
    if installed.as_ref().is_some_and(|i| Path::new(&i.location) != bundle) {
        return Err(format!("Unexpected initial location {installed:?}"));
    }
    let expected = match (phase.as_str(), initial.as_deref()) {
        ("install", None) => {
            backend.send(Command::ListVersions { app: APP.into() });
            let published = wait_for(&mut backend, |event| match event {
                Event::Versions { app, result } if app == APP => Some(result),
                _ => None,
            })??;
            for expected in [BEFORE, AFTER] {
                let release =
                    published.iter().find(|release| release.tag == expected.0).ok_or_else(|| format!("Official tag {} missing", expected.0))?;
                assert_package(release, expected)?;
            }
            backend.send(Command::Configure(BackendOptions { versions: [(APP.into(), VersionTrack::Pinned(BEFORE.0.into()))].into(), ..options }));
            assert_package(&native_release(&mut backend)?, BEFORE)?;
            backend.send(Command::InstallChosen { app: APP.into(), tag: BEFORE.0.into(), fallback: None });
            Some(BEFORE)
        }
        ("update", Some(tag)) if tag == BEFORE.0 => {
            backend.send(Command::CheckForUpdates { force: true });
            assert_package(&native_release(&mut backend)?, AFTER)?;
            backend.send(Command::InstallExpected { apps: vec![APP.into()], installed: true });
            Some(AFTER)
        }
        ("remove", Some(tag)) if tag == AFTER.0 => {
            backend.send(Command::Uninstall { app: APP.into() });
            None
        }
        _ => return Err(format!("Unexpected proof phase {phase} at {initial:?}")),
    };
    let mut done = 0;
    let result = wait_for(&mut backend, |ev| match ev {
        Event::Progress { app, phase: JobPhase::Downloading { done: bytes, .. } } if app == APP => {
            done = done.max(bytes);
            None
        }
        Event::JobFinished { app, result } if app == APP => Some(result),
        _ => None,
    })??;
    let manifest = Manifest::load(&path.manifest());
    if let Some((tag, size, _)) = expected {
        let record = manifest.apps.get(APP).ok_or("missing native receipt")?;
        if done != size || record.version != tag || record.kind != PackageKind::Dmg || record.path != bundle || !bundle.is_dir() {
            return Err(format!("Unexpected native receipt {record:?}, transferred {done}, expected {size}"));
        }
    } else if bundle.exists() || manifest.apps.contains_key(APP) || !home.join(".Trash/Zotero.app").is_dir() {
        return Err("expected Zotero.app only in private Trash with no native record".into());
    }
    println!("PASS NATIVE {phase}: {result}; transferred={done}");
    Ok(())
}
