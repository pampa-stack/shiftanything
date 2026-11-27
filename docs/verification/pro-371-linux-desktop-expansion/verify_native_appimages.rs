use std::path::Path;
use std::time::{Duration, Instant};

use artcraft_launcher_core::curated::{OPEN_PENCIL, PASEO, T3_CODE, ULTIMAKER_CURA, VOICE_STUDIO};
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

const VOICE_STUDIO_PACKAGES: [Package; 2] = [
    Package {
        version: "0.5.7",
        name: "VoiceStudio-0.5.7-linux-x64.AppImage",
        size: 267_623_923,
        sha: "39a3e0aae23f2a95cb2aabf4a1ca4a1b9c17931b19571179ace017981e364700",
    },
    Package {
        version: "0.5.8",
        name: "VoiceStudio-0.5.8-linux-x64.AppImage",
        size: 289_429_504,
        sha: "90bf90e0dd057632c76e5b286a2e068f9fb9cb1e1363ebaa215a17cb2e3d430e",
    },
];
const T3_CODE_PACKAGES: [Package; 2] = [
    Package {
        version: "0.1.12",
        name: "T3-Code-x86_64.AppImage",
        size: 255_860_306,
        sha: "66b91551571df6d29b55cb590295edf411cff96c95a8feb3f41c3ca595d722e2",
    },
    Package {
        version: "0.1.13",
        name: "T3-Code-x86_64.AppImage",
        size: 256_003_692,
        sha: "dac1bbe9e93c1a5fb0942332befa998597f6a086232a87766ef5d8235a44f56d",
    },
];
const PASEO_PACKAGES: [Package; 2] = [
    Package {
        version: "0.18.0",
        name: "Paseo-x86_64.AppImage",
        size: 171_962_101,
        sha: "3498024f3a3a7c55c7421a5dd6d85824745e638ddfb612c1909ec1058b03c24e",
    },
    Package {
        version: "0.18.1",
        name: "Paseo-x86_64.AppImage",
        size: 171_958_150,
        sha: "a17d26d6930da647c0eacad3ebcae779bfc4bb47a824236ff790a60c199d85b6",
    },
];

const CURA_PACKAGES: [Package; 2] = [
    Package {
        version: "5.13.0",
        name: "UltiMaker-Cura-5.13.0-linux-X64.AppImage",
        size: 334_154_232,
        sha: "100f068127b2598167f00ba4db0e0699ded45adf97bbab2d22ff171a1e1ecc40",
    },
    Package {
        version: "5.14.0",
        name: "UltiMaker-Cura-5.14.0-linux-X64.AppImage",
        size: 370_960_888,
        sha: "988a2a67cce7ee7b9b9a843aad65ec4754d2ceb0a72e70eeaf884b995b75990f",
    },
];

const OPEN_PENCIL_PACKAGES: [Package; 2] = [
    Package {
        version: "0.16.0",
        name: "OpenPencil_0.16.0_amd64.AppImage",
        size: 96_438_776,
        sha: "a7d893ac35367c8ea000b00890136ce5861a13bf2574b44760f77849cfdf77a2",
    },
    Package {
        version: "0.17.0",
        name: "OpenPencil_0.17.0_amd64.AppImage",
        size: 99_834_360,
        sha: "43da3dbd7a223a103692dd2d9193a86f8ddcb3db7577bd373949aae8455e94fc",
    },
];

fn release_tag(app: &CraftApp, package: &Package) -> String {
    let prefix = if app.id == ULTIMAKER_CURA.id { "" } else { "v" };
    format!("{prefix}{}", package.version)
}

fn verify_release(app: &CraftApp, release: &Release, package: &Package) -> Result<(), String> {
    let host = Platform::current().ok_or("Missing current platform")?;
    let (asset, kind) = pick_asset(release, &[app.id], host, &[PackageKind::AppImage]).ok_or("Recipe rejected published AppImage")?;
    if kind != PackageKind::AppImage
        || release.tag != release_tag(app, package)
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
        let url = format!(
            "https://api.github.com/repos/{}/releases/tags/{}",
            app.repo,
            release_tag(app, package)
        );
        let body = match std::env::var_os("SHIFT_NATIVE_APPIMAGE_PREVIOUS_METADATA") {
            Some(file) => {
                println!("HISTORICAL METADATA GitHub API response supplied from file");
                std::fs::read_to_string(file).map_err(|error| error.to_string())?
            }
            None => http::Http::new()?
                .get_text(&url)?
                .ok_or("Published historical API result missing")?,
        };
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
        if !matches!(answer.source, Source::GitHubApi | Source::GitHubPages) {
            return Err(format!("Need publisher-verified update metadata, got {:?}", answer.source));
        }
        verify_release(app, &release, package)?;
        println!("DISCOVERY {:?}", answer.source);
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
    let home = std::env::var("HOME").map_err(|err| err.to_string())?;
    let home = Path::new(&home);
    if !home.join(".shift-native-appimage-proof").is_file() {
        return Err("Need a disposable Linux profile".into());
    }
    let args: Vec<String> = std::env::args().collect();
    let Some((app, packages)) = args.get(1).and_then(|id| match id.as_str() {
        "openpencil" => Some((OPEN_PENCIL, &OPEN_PENCIL_PACKAGES)),
        "ultimaker-cura" => Some((ULTIMAKER_CURA, &CURA_PACKAGES)),
        "paseo" => Some((PASEO, &PASEO_PACKAGES)),
        "voice-studio" => Some((VOICE_STUDIO, &VOICE_STUDIO_PACKAGES)),
        "t3code" => Some((T3_CODE, &T3_CODE_PACKAGES)),
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
        if !entry.contains(&format!("Exec=\"{}\" %F", record.path.display()))
            || !entry.contains(&format!("Icon={desktop_id}\n"))
            || std::fs::read(&icon).map_err(|err| err.to_string())?
                != artcraft_launcher_core::catalog::app_icon_png(app.id).ok_or("No bundled icon")?
        {
            return Err("Unexpected generated desktop entry or menu icon".into());
        }
        if app.id == ULTIMAKER_CURA.id && !entry.contains("StartupWMClass=UltiMaker-Cura\n") {
            return Err("Generated desktop entry does not match the Cura window class".into());
        }
        if app.id == OPEN_PENCIL.id && !entry.contains("StartupWMClass=OpenPencil\n") {
            return Err("Generated desktop entry does not match the OpenPencil window class".into());
        }
        println!("FILE {} version={} bytes={} sha256={}", record.path.display(), record.version, package.size, package.sha);
        println!("DESKTOP {} ICON {} sha256={}", desktop.display(), icon.display(), http::sha256_file(&icon)?);
    }
    if phase == "remove" && before.as_ref().is_some_and(|installed| Path::new(&installed.location).exists()) {
        return Err("Previous AppImage still exists".into());
    }
    if phase == "remove" && (manifest.apps.contains_key(app.id) || icon.exists() || desktop.exists()) {
        return Err("Removed AppImage left a launcher manifest entry, desktop entry, or icon".into());
    }
    println!("PASS {} {phase} {} ms {result}", app.id, elapsed);
    Ok(())
}
