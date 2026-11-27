use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease, Origin, VersionTrack};
use artcraft_launcher_core::{Arch, CraftApp, Os, PackageKind, Platform, release::Release};
use artcraft_launcher_native::{NativeBackend, manifest::Manifest, paths};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

struct Package<'a> {
    tag: &'a str,
    name: &'a str,
    size: u64,
    digest: &'a str,
    url: &'a str,
}

impl<'a> Package<'a> {
    fn parse(line: &'a str) -> Result<Self, String> {
        let [tag, name, size, digest, url] = line.split('\t').collect::<Vec<_>>().try_into().map_err(|_| "wrong official release metadata")?;
        Ok(Self { tag, name, size: size.parse().map_err(|_| "invalid official package size")?, digest, url })
    }

    fn version(&self) -> String {
        artcraft_launcher_core::version::parse(self.tag).expect("canonical package tag").to_string()
    }

    fn assert_release(&self, id: &str, release: &Release) -> Result<(), String> {
        let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let (asset, kind) = pick_asset(release, &[id], platform, &[PackageKind::Dmg])
            .ok_or_else(|| format!("missing official ARM package for {:?} ({:?}), expected {}", release.tag, release.version, self.tag))?;
        if release.tag != self.tag
            || release.version_label() != self.version()
            || asset.name != self.name
            || asset.url != self.url
            || asset.size != self.size
            || asset.sha256.as_deref() != Some(self.digest)
        {
            return Err(format!("Unexpected official package {asset:?} for {}", release.tag));
        }
        println!("PASS OFFICIAL {} {kind:?} {} {} {}", self.tag, asset.url, self.size, self.digest);
        Ok(())
    }
}

fn wait_for<T>(backend: &mut NativeBackend, mut pick: impl FnMut(Event) -> Option<T>) -> Result<T, String> {
    let deadline = Instant::now() + Duration::from_secs(400);
    while Instant::now() < deadline {
        if let Some(result) = backend.poll().into_iter().find_map(&mut pick) {
            return Ok(result);
        }
        std::thread::sleep(Duration::from_millis(30));
    }
    Err("timed out waiting for native event".into())
}

fn native_release(backend: &mut NativeBackend, id: &str) -> Result<Release, String> {
    let latest = wait_for(backend, |event| match event {
        Event::Latest { app, latest, from } if app == id => {
            println!("NATIVE ORIGIN {id} {from:?}");
            matches!(from, Origin::GitHub | Origin::ArtCraftApi).then_some(latest)
        }
        _ => None,
    })?;
    match latest {
        LatestRelease::Found(release) => Ok(release),
        failure => Err(format!("Official release unavailable: {failure:?}")),
    }
}

fn assert_marker(home: &Path) -> Result<(), String> {
    if std::fs::read(home.join("synthetic-user-data.txt")).map_err(|error| error.to_string())? != b"synthetic user data\n" {
        return Err("synthetic user data changed".into());
    }
    Ok(())
}

fn main() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [id, phase, app_name] = args.as_slice() else { return Err("expected recipe, phase and bundle".into()) };
    let app = CraftApp::by_id(id).ok_or("no native recipe")?;
    let home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME missing")?).canonicalize().map_err(|error| error.to_string())?;
    let parent = home.parent().ok_or("no proof scratch path")?;
    if !parent.starts_with("/private/tmp")
        || !parent.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.starts_with("shift-air-overnight."))
        || !parent.join(".shift-owned").is_file()
        || !home.join(".shift-overnight-cycle").is_file()
        || !["moonlight", "openpencil", "airi", "sunshine", "openttd"].contains(&app.id)
        || app.mac_team.is_none()
        || app.mac_bundle_name.unwrap_or(app.name) != app_name
    {
        return Err("run only in a disposable proof profile".into());
    }
    assert_marker(&home)?;
    let metadata = std::fs::read_to_string(parent.join(format!("{id}.tsv"))).map_err(|error| error.to_string())?;
    let [before, after] =
        metadata.lines().map(Package::parse).collect::<Result<Vec<_>, _>>()?.try_into().map_err(|_| "expected two official packages")?;
    let start = Instant::now();
    let applications = home.join("Applications");
    let bundle = applications.join(format!("{app_name}.app"));
    let paths = paths::Paths::new()?;
    let mut backend = NativeBackend::new_isolated_macos_profile(|| {})?;
    backend.send(Command::Configure(BackendOptions {
        install_locations: InstallLocations { root: Some(applications.display().to_string()), ..Default::default() },
        simultaneous_downloads: 1,
        ..Default::default()
    }));
    backend.send(Command::Detect);
    let installed = wait_for(&mut backend, |event| match event {
        Event::Detected { app, installed } if app == *id => Some(installed),
        _ => None,
    })?;
    let initial = installed.as_ref().and_then(|installed| installed.version.as_ref()).map(ToString::to_string);
    if installed.as_ref().is_some_and(|installed| Path::new(&installed.location) != bundle) {
        return Err(format!("Unexpected initial location {installed:?}"));
    }

    let expected = match (phase.as_str(), initial.as_deref()) {
        ("install", None) => {
            backend.send(Command::ListVersions { app: id.clone() });
            let versions = wait_for(&mut backend, |event| match event {
                Event::Versions { app, result } if app == *id => Some(result),
                _ => None,
            })??;
            for expected in [&before, &after] {
                let release =
                    versions.iter().find(|release| release.tag == expected.tag).ok_or_else(|| format!("Official tag {} missing", expected.tag))?;
                expected.assert_release(id, release)?;
            }
            backend.send(Command::Configure(BackendOptions {
                install_locations: InstallLocations { root: Some(applications.display().to_string()), ..Default::default() },
                versions: [(id.clone(), VersionTrack::Pinned(before.tag.into()))].into(),
                simultaneous_downloads: 1,
                ..Default::default()
            }));
            before.assert_release(id, &native_release(&mut backend, id)?)?;
            backend.send(Command::InstallChosen { app: id.clone(), tag: before.tag.into(), fallback: None });
            Some(before)
        }
        ("update", Some(tag)) if tag == before.version() => {
            backend.send(Command::CheckForUpdates { force: true });
            after.assert_release(id, &native_release(&mut backend, id)?)?;
            backend.send(Command::InstallExpected { apps: vec![id.clone()], installed: true });
            Some(after)
        }
        ("remove", Some(tag)) if tag == after.version() => {
            backend.send(Command::Uninstall { app: id.clone() });
            None
        }
        _ => return Err(format!("Unexpected proof phase {phase} at {initial:?}")),
    };
    let mut verified_package_bytes = 0;
    let result = wait_for(&mut backend, |event| match event {
        Event::Progress { app, phase: JobPhase::Downloading { done, .. } } if app == *id => {
            verified_package_bytes = verified_package_bytes.max(done);
            None
        }
        Event::JobFinished { app, result } if app == *id => Some(result),
        _ => None,
    })??;
    let manifest = Manifest::load(&paths.manifest());
    if let Some(package) = expected {
        let record = manifest.apps.get(id).ok_or("missing native receipt")?;
        if verified_package_bytes != package.size
            || record.version != package.version()
            || record.kind != PackageKind::Dmg
            || record.path != bundle
            || !bundle.is_dir()
        {
            return Err(format!("Unexpected native receipt {record:?}, package_bytes {verified_package_bytes}, expected {}", package.size));
        }
    } else if bundle.exists() || manifest.apps.contains_key(id) || !home.join(".Trash").join(format!("{app_name}.app")).is_dir() {
        return Err("expected only the isolated Trash bundle and no native record".into());
    }
    assert_marker(&home)?;
    println!("PASS NATIVE {id} {phase}: {result}; package_bytes={verified_package_bytes}; elapsed={:.3}s", start.elapsed().as_secs_f64());
    Ok(())
}
