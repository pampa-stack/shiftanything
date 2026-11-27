use std::path::Path;
use std::time::{Duration, Instant};

use artcraft_launcher_core::curated::{OPENSHOT, PENCIL2D};
use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, Command, Event, LatestRelease, Source};
use artcraft_launcher_core::release::{Release, ReleaseQuery, parse_github_release};
use artcraft_launcher_core::{CraftApp, PackageKind, Platform};
use artcraft_launcher_native::source::{GitHubSource, ReleaseSource};
use artcraft_launcher_native::{NativeBackend, http, manifest::Manifest, paths};

struct Package {
    version: &'static str,
    name: &'static str,
    size: u64,
    sha: &'static str,
}

const PENCIL: [Package; 2] = [
    Package {
        version: "0.7.1",
        name: "pencil2d-linux-amd64-v0.7.1.AppImage",
        size: 60471800,
        sha: "02416a6777f3b5aca826285b85d66476c1352d0d8252b55714cc7bc55ba463f6",
    },
    Package {
        version: "0.7.2",
        name: "pencil2d-linux-amd64-v0.7.2.AppImage",
        size: 60471800,
        sha: "4034b036258b6a13d4540911ebce085f31ee18a9dacf438845ac71998d42ff91",
    },
];
const OPEN: [Package; 2] = [
    Package {
        version: "4.1.0",
        name: "OpenShot-v4.1.0-x86_64.AppImage",
        size: 258708672,
        sha: "43af897622628b6ee092d5cfc5b555d50d6253dd32c5562060c9759b041d7e4c",
    },
    Package {
        version: "4.1.1",
        name: "OpenShot-v4.1.1-x86_64.AppImage",
        size: 258921664,
        sha: "662d0c3cec67ddc90df6a8b0c723c0aac0d3bddf0c3c467eff562e818bb8c72f",
    },
];

fn verify_release(app: &CraftApp, release: &Release, package: &Package) -> Result<(), String> {
    let host = Platform::current().ok_or("Missing current platform")?;
    let (asset, kind) = pick_asset(release, &[app.id], host, &[PackageKind::AppImage]).ok_or("Recipe rejected published AppImage")?;
    if kind != PackageKind::AppImage
        || release.tag != format!("v{}", package.version)
        || asset.name != package.name
        || asset.size != package.size
        || asset.sha256.as_deref() != Some(package.sha)
    {
        return Err(format!("Unexpected release metadata {release:?}"));
    }
    println!("OFFICIAL {} bytes={} sha256={}", asset.url, package.size, package.sha);
    Ok(())
}

fn initialize(app: &CraftApp, paths: &paths::Paths, package: &Package, previous: bool) -> Result<(), String> {
    if previous {
        let url = format!("https://api.github.com/repos/{}/releases/tags/v{}", app.repo, package.version);
        let body = http::Http::new()?.get_text(&url)?.ok_or("Published historical API result missing")?;
        let release = parse_github_release(&body, app.repo)?;
        verify_release(app, &release, package)?;
        let cache = serde_json::json!({"entries": {app.repo: {
            "release": release, "etag": null, "prereleases": false, "fetched_at": artcraft_launcher_native::now()
        }}});
        paths::write_atomic(&paths.release_cache(), &serde_json::to_vec(&cache).map_err(|err| err.to_string())?)
    } else {
        let source = GitHubSource::new(http::Http::new()?, paths.release_cache());
        let answer = source.latest(&ReleaseQuery::for_app(app, app.repo), false);
        let LatestRelease::Found(release) = answer.latest else {
            return Err(format!("Latest release unavailable ({:?})", answer.latest));
        };
        if answer.source != Source::GitHubApi {
            return Err(format!("Need direct GitHub API update metadata, got {:?}", answer.source));
        }
        verify_release(app, &release, package)?;
        source.save();
        Ok(())
    }
}

fn poll<T>(backend: &mut NativeBackend, limit: Duration, mut read: impl FnMut(Event) -> Option<T>) -> Result<T, String> {
    let end = Instant::now() + limit;
    while Instant::now() < end {
        if let Some(value) = backend.poll().into_iter().find_map(&mut read) {
            return Ok(value);
        }
        std::thread::sleep(Duration::from_millis(40));
    }
    Err("Timed out waiting for backend event".to_string())
}

fn main() -> Result<(), String> {
    let home = Path::new("/fixture/home");
    if !home.join(".shift-vps2-appimage-proof").is_file() {
        return Err("Need disposable VPS2 Bubblewrap profile".into());
    }
    let args: Vec<String> = std::env::args().collect();
    let Some((app, packages)) = args.get(1).and_then(|id| match id.as_str() {
        "pencil2d" => Some((PENCIL2D, &PENCIL)),
        "openshot-video-editor" => Some((OPENSHOT, &OPEN)),
        _ => None,
    }) else {
        return Err("Unknown proof app".into());
    };
    let phase = args.get(2).ok_or("Missing phase")?.as_str();
    let paths = paths::Paths::new()?;
    let after_package = match phase {
        "install" => Some(&packages[0]),
        "update" => Some(&packages[1]),
        "open-old" | "open-new" | "remove" => None,
        _ => return Err("Unknown phase".into()),
    };
    if let Some(package) = after_package {
        initialize(&app, &paths, package, phase == "install")?;
    }
    let mut backend = NativeBackend::new_isolated_linux_profile(home, || {})?;
    backend.send(Command::Detect);
    let before = poll(&mut backend, Duration::from_secs(30), |event| match event {
        Event::Detected { app: id, installed } if id == app.id => Some(installed),
        _ => None,
    })?;
    let expected = match phase {
        "install" => None,
        "update" | "open-old" => Some(packages[0].version),
        _ => Some(packages[1].version),
    };
    let detected = before.as_ref().and_then(|installed| installed.version.as_ref()).map(ToString::to_string);
    if before.is_some() != expected.is_some() || detected.as_deref() != expected {
        return Err(format!("Unexpected detected state before {phase}: {before:?}"));
    }
    let command = match phase {
        "open-old" | "open-new" => Command::Launch { app: app.id.into(), args: vec![] },
        "remove" => Command::Uninstall { app: app.id.into() },
        _ => Command::InstallExpected { apps: vec![app.id.into()], installed: phase == "update" },
    };
    let start = Instant::now();
    backend.send(command);
    let result = poll(&mut backend, Duration::from_secs(240), |event| match event {
        Event::JobFinished { app: id, result } if id == app.id => Some(result),
        _ => None,
    })??;
    let elapsed = start.elapsed().as_millis();
    let manifest = Manifest::load(&paths.manifest());
    let desktop_id = app.linux_desktop_id();
    let icon = home.join(format!("data/icons/hicolor/128x128/apps/{desktop_id}.png"));
    let desktop = home.join(format!("data/applications/{desktop_id}.desktop"));
    if let Some(package) = after_package {
        let record = manifest.apps.get(app.id).ok_or("AppImage missing from manifest")?;
        if record.version != package.version
            || record.kind != PackageKind::AppImage
            || record.needs_root
            || record.path != home.join("Applications").join(package.name)
            || record.path.metadata().map_err(|err| err.to_string())?.len() != package.size
            || http::sha256_file(&record.path)? != package.sha
            || !record.files.contains(&desktop)
            || !record.files.contains(&icon)
        {
            return Err(format!("Unexpected AppImage record: {record:?}"));
        }
        let entry = std::fs::read_to_string(&desktop).map_err(|err| err.to_string())?;
        if !entry.contains(&format!("Exec=\"{}\" --appimage-extract-and-run %F", record.path.display()))
            || !entry.contains(&format!("Icon={desktop_id}\n"))
            || std::fs::read(&icon).map_err(|err| err.to_string())?
                != artcraft_launcher_core::catalog::app_icon_png(app.id).ok_or("No bundled icon")?
        {
            return Err("Unexpected generated desktop entry or menu icon".into());
        }
        println!("FILE {} version={} bytes={} sha256={}", record.path.display(), record.version, package.size, package.sha);
        println!("DESKTOP {} ICON {} sha256={}", desktop.display(), icon.display(), http::sha256_file(&icon)?);
    }
    if matches!(phase, "update" | "remove") && before.as_ref().is_some_and(|installed| Path::new(&installed.location).exists()) {
        return Err("Previous AppImage still exists".into());
    }
    if phase == "remove" && (manifest.apps.contains_key(app.id) || icon.exists() || desktop.exists()) {
        return Err("Removed AppImage left a launcher manifest entry, desktop entry, or icon".into());
    }
    println!("PASS {} {phase} {} ms {result}", app.id, elapsed);
    Ok(())
}
