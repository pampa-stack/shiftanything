use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, BackendOptions, Command, Event, InstallLocations, JobPhase, LatestRelease, Origin, VersionTrack};
use artcraft_launcher_core::{Arch, CraftApp, Os, Platform, release::Release};
use artcraft_launcher_native::{
    NativeBackend,
    manifest::{Manifest, Record},
    paths,
};
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
        let label = match self.tag {
            "5.3.4.1" => "5.3.4",
            "v5.0.9-2" => "5.0.9",
            tag => tag.trim_start_matches("mac-").trim_start_matches("release-"),
        };
        let version = if self.name.starts_with("kdenlive-") {
            artcraft_launcher_core::release::kdenlive_mac_version(label)
        } else if self.name == "MeetingBar.dmg" {
            artcraft_launcher_core::release::release_tag_version(artcraft_launcher_core::curated::MEETINGBAR.repo, self.tag)
        } else if self.name.starts_with("karing_") {
            artcraft_launcher_core::release::release_tag_version(artcraft_launcher_core::curated::KARING.repo, self.tag)
        } else if self.name == "MacVim.dmg" {
            artcraft_launcher_core::release::release_tag_version(artcraft_launcher_core::curated::MACVIM.repo, self.tag)
        } else if self.name.starts_with("Syncthing-") {
            artcraft_launcher_core::release::syncthing_macos_version(label)
        } else {
            artcraft_launcher_core::version::parse(label)
        };
        artcraft_launcher_core::version::label(&version.expect("canonical package tag"))
    }

    fn assert_release(&self, app: &CraftApp, release: &Release) -> Result<(), String> {
        let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let (asset, kind) = pick_asset(release, &[app.id], platform, &[app.mac_package_kind()])
            .ok_or_else(|| format!("missing official ARM package for {:?} ({:?}), expected {}", release.tag, release.version, self.tag))?;
        if release.tag != self.tag
            || release.version_label() != self.version()
            || asset.name != self.name
            || asset.url != self.url
            || asset.size != self.size
            || asset.sha256.as_deref() != Some(self.digest)
        {
            return Err(format!(
                "Unexpected official package {asset:?} for {} (version {}); expected {} {} {} {} {} {}",
                release.tag,
                release.version_label(),
                self.tag,
                self.version(),
                self.name,
                self.size,
                self.digest,
                self.url,
            ));
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
            matches!(
                from,
                Origin::GitHub
                    | Origin::ArtCraftApi
                    | Origin::Blender
                    | Origin::Gimp
                    | Origin::Krita
                    | Origin::VideoLan
                    | Origin::Inkscape
                    | Origin::Kdenlive
                    | Origin::ITerm2
                    | Origin::Itsycal
                    | Origin::Signal
                    | Origin::Mozilla
                    | Origin::Element
            )
            .then_some(latest)
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
    if ![Path::new("/private/tmp"), Path::new("/Volumes/T7-Backup")].iter().any(|root| parent.parent() == Some(*root))
        || !parent.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.starts_with("shift-air-overnight."))
        || !parent.join(".shift-owned").is_file()
        || !home.join(".shift-overnight-cycle").is_file()
        || ![
            "craft-anything",
            "moonlight",
            "openpencil",
            "airi",
            "sunshine",
            "openttd",
            "rectangle",
            "codeedit",
            "maccy",
            "blender",
            "gimp",
            "netnewswire",
            "krita",
            "vlc",
            "inks",
            "kdenlive",
            "subtitle-edit",
            "openwhispr",
            "deepchat",
            "alttab",
            "amethyst",
            "dockdoor",
            "macvim",
            "hidden-bar",
            "swiftbar",
            "sloth",
            "jellyfin-desktop",
            "iterm2",
            "itsycal",
            "github-desktop",
            "linearmouse",
            "loop-macos",
            "dyad",
            "finicky",
            "hammerspoon",
            "ice",
            "virtualbuddy",
            "easydict",
            "utm",
            "openmtp",
            "invoke-community",
            "mist",
            "coteditor",
            "monitorcontrol",
            "gnucash",
            "floorp",
            "stellarium",
            "macshot",
            "signal-desktop",
            "firefox",
            "element",
            "aionui",
            "raspberry-pi-imager",
            "ruffle",
            "scummvm",
            "wwdc",
            "pika-color",
            "logseq-db",
            "onlyoffice",
            "sigil",
            "syncthing-macos",
            "meetingbar",
            "karing",
            "playcover",
            "material-maker",
            "mullvad-browser",
            "openboard",
            "freeplane",
        ]
        .contains(&app.id)
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
    let fixture_version = match id.as_str() {
        "iterm2" => Some("3.7.8"),
        "github-desktop" => Some("3.7.0"),
        "monitorcontrol" => Some("4.2.0"),
        _ => None,
    };
    if phase == "update-from-fixture" {
        if !bundle.is_dir() || Some(before.version().as_str()) != fixture_version {
            return Err("the signed migration fixture is missing".into());
        }
        // The isolated backend detects only managed copies, and reads the signed bundle's version.
        let mut manifest = Manifest::load(&paths.manifest());
        if manifest.apps.contains_key(id) {
            return Err("migration proof must start with an empty app receipt".into());
        }
        manifest.apps.insert(
            id.clone(),
            Record {
                version: before.version(),
                kind: app.mac_package_kind(),
                path: bundle.clone(),
                installed_at: 0,
                files: vec![],
                needs_root: false,
            },
        );
        manifest.save(&paths.manifest())?;
        println!("SIGNED MIGRATION FIXTURE RECEIPT {}", before.version());
    }
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
    let initial = installed.as_ref().and_then(|installed| installed.version.as_ref()).map(artcraft_launcher_core::version::label);
    if installed.as_ref().is_some_and(|installed| Path::new(&installed.location) != bundle) {
        return Err(format!("Unexpected initial location {installed:?}"));
    }

    if matches!(id.as_str(), "wwdc" | "pika-color" | "logseq-db" | "onlyoffice" | "sigil")
        && phase == "open"
        && initial.as_deref().is_some_and(|installed| installed == before.version() || installed == after.version())
    {
        let args = if id == "logseq-db" {
            vec![format!("--user-data-dir={}", home.join("logseq-user-data").display())]
        } else {
            Vec::new()
        };
        backend.send(Command::Launch { app: id.clone(), args });
        let result = wait_for(&mut backend, |event| match event {
            Event::JobFinished { app, result } if app == *id => Some(result),
            _ => None,
        })??;
        assert_marker(&home)?;
        println!("PASS NATIVE {id} {phase}: {result}");
        return Ok(());
    }

    let expected = match (phase.as_str(), initial.as_deref()) {
        ("install-latest", None) if fixture_version.is_some() => {
            if id == "iterm2" {
                backend.send(Command::ListVersions { app: id.clone() });
                let versions = wait_for(&mut backend, |event| match event {
                    Event::Versions { app, result } if app == *id => Some(result),
                    _ => None,
                })??;
                let [current] = versions.as_slice() else { return Err("iTerm2 exposed a historical package".into()) };
                after.assert_release(app, current)?;
            }
            backend.send(Command::CheckForUpdates { force: true });
            after.assert_release(app, &native_release(&mut backend, id)?)?;
            backend.send(Command::InstallExpected { apps: vec![id.clone()], installed: false });
            Some(after)
        }
        ("install", None) => {
            backend.send(Command::ListVersions { app: id.clone() });
            let versions = wait_for(&mut backend, |event| match event {
                Event::Versions { app, result } if app == *id => Some(result),
                _ => None,
            })??;
            for expected in [&before, &after] {
                let release =
                    versions.iter().find(|release| release.tag == expected.tag).ok_or_else(|| format!("Official tag {} missing", expected.tag))?;
                expected.assert_release(app, release)?;
            }
            backend.send(Command::Configure(BackendOptions {
                install_locations: InstallLocations { root: Some(applications.display().to_string()), ..Default::default() },
                versions: [(id.clone(), VersionTrack::Pinned(before.tag.into()))].into(),
                simultaneous_downloads: 1,
                ..Default::default()
            }));
            before.assert_release(app, &native_release(&mut backend, id)?)?;
            backend.send(Command::InstallChosen { app: id.clone(), tag: before.tag.into(), fallback: None });
            Some(before)
        }
        ("update" | "update-from-fixture", Some(tag)) if tag == before.version() => {
            backend.send(Command::CheckForUpdates { force: true });
            after.assert_release(app, &native_release(&mut backend, id)?)?;
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
            || record.kind != app.mac_package_kind()
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
