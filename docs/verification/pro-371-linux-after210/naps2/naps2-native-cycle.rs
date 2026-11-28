use std::path::Path;
use std::time::{Duration, Instant};

use artcraft_launcher_core::curated::{NAPS2, NAPS2_LINUX_FLATPAK_ID};
use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{Backend, Command, Event, JobPhase, LatestRelease, Source};
use artcraft_launcher_core::release::{ReleaseQuery, release_from_sums};
use artcraft_launcher_core::{InstallSource, PackageKind, Platform};
use artcraft_launcher_native::source::{GitHubSource, ReleaseSource};
use artcraft_launcher_native::{
    NativeBackend,
    http::Http,
    paths::{Paths, write_atomic},
};

fn observed(
    backend: &mut NativeBackend,
    limit: Duration,
    read: impl FnMut(Event) -> Option<()>,
) -> Result<(), String> {
    let mut read = read;
    let end = Instant::now() + limit;
    while Instant::now() < end {
        if backend.poll().into_iter().find_map(&mut read).is_some() {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err("Timed out awaiting backend event".into())
}

fn detect(backend: &mut NativeBackend, expected: Option<&str>) -> Result<(), String> {
    backend.send(Command::Detect);
    observed(backend, Duration::from_secs(70), |event| match event {
        Event::Detected { app, installed } if app == NAPS2.id => {
            println!("Detected {installed:?}");
            match (expected, installed) {
                (None, None) => Some(()),
                (Some(version), Some(copy))
                    if copy.source == InstallSource::Flatpak
                        && copy.package.as_deref() == Some(NAPS2_LINUX_FLATPAK_ID)
                        && copy.installation.as_deref() == Some("user")
                        && !copy.externally_managed
                        && copy
                            .version
                            .as_ref()
                            .is_some_and(|v| v.to_string() == version) =>
                {
                    Some(())
                }
                _ => panic!("Unexpected NAPS2 state (expected {expected:?})"),
            }
        }
        _ => None,
    })
}

fn prepare_release(paths: &Paths, current: bool) -> Result<(), String> {
    let (release, expected) = if current {
        let source = GitHubSource::new(Http::new()?, paths.release_cache());
        let answer = source.latest(&ReleaseQuery::for_app(&NAPS2, NAPS2.repo), false);
        if !matches!(answer.source, Source::GitHubApi | Source::GitHubPages) {
            return Err(format!(
                "No current publisher response: {:?}",
                answer.source
            ));
        }
        let LatestRelease::Found(release) = answer.latest else {
            return Err("No current release".into());
        };
        println!("Discovery {:?}", answer.source);
        source.save();
        (release, "v8.6.1")
    } else {
        let tag = "v8.6.0";
        let release = release_from_sums(NAPS2.repo, tag, &[]);
        let cache = serde_json::json!({"entries": {NAPS2.repo: {
            "release": &release, "etag": null, "prereleases": false, "fetched_at": artcraft_launcher_native::now()
        }}});
        write_atomic(
            &paths.release_cache(),
            &serde_json::to_vec(&cache).map_err(|err| err.to_string())?,
        )?;
        (release, tag)
    };
    let (asset, _) = pick_asset(
        &release,
        &[NAPS2.id],
        Platform::current().ok_or("No platform")?,
        &[PackageKind::Flatpak],
    )
    .ok_or("The release recipe did not select its Flatpak")?;
    if release.tag != expected {
        return Err(format!("Unexpected release tag {}", release.tag));
    }
    println!(
        "Release {expected} {} bytes={} hash={:?}",
        asset.url, asset.size, asset.sha256
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let stage = std::env::args().nth(1).ok_or("Missing stage")?;
    let home = std::env::var("HOME").map_err(|err| err.to_string())?;
    let home = Path::new(&home);
    if !home.starts_with("/home/qa") || !home.join(".naps2-proof").is_file() {
        return Err("Need a private launcher proof profile".into());
    }
    let paths = Paths::new()?;
    if !paths.data.starts_with(home.join("data"))
        || !paths.cache.starts_with(home.join("cache"))
        || std::env::var_os("FLATPAK_USER_DIR").as_deref() != Some(home.join("flatpak").as_os_str())
        || std::env::var_os("FLATPAK_SYSTEM_DIR").as_deref()
            != Some(home.join("flatpak-system").as_os_str())
    {
        return Err("Profile dirs escaped private scope".into());
    }
    let id = || NAPS2.id.to_string();
    let (before, after, command) = match stage.as_str() {
        "install" => (
            None,
            Some("8.6.0"),
            Command::InstallExpected {
                apps: vec![id()],
                installed: false,
            },
        ),
        "update" => (
            Some("8.6.0"),
            Some("8.6.1"),
            Command::InstallExpected {
                apps: vec![id()],
                installed: true,
            },
        ),
        "open-old" => (
            Some("8.6.0"),
            Some("8.6.0"),
            Command::Launch {
                app: id(),
                args: vec![],
            },
        ),
        "open-new" => (
            Some("8.6.1"),
            Some("8.6.1"),
            Command::Launch {
                app: id(),
                args: vec![],
            },
        ),
        "remove" => (Some("8.6.1"), None, Command::Uninstall { app: id() }),
        _ => return Err("Unknown stage".into()),
    };
    let downloads = matches!(stage.as_str(), "install" | "update");
    if downloads {
        prepare_release(&paths, stage == "update")?;
    }
    let mut backend = NativeBackend::new(|| {});
    detect(&mut backend, before)?;
    let mut downloaded = false;
    let mut failure = None;
    backend.send(command);
    observed(
        &mut backend,
        Duration::from_secs(230),
        |event| match event {
            Event::Progress {
                app,
                phase: JobPhase::Downloading { done, total },
            } if app == NAPS2.id && done > 0 && done == total => {
                println!("Downloaded {done} bytes through NativeBackend");
                downloaded = true;
                None
            }
            Event::JobFinished { app, result } if app == NAPS2.id => {
                println!("JobFinished {stage}: {result:?}");
                failure = result.err();
                Some(())
            }
            _ => None,
        },
    )?;
    if let Some(reason) = failure {
        return Err(reason);
    }
    if downloads && !downloaded {
        return Err("Expected a new HTTPS download".into());
    }
    if !stage.starts_with("open-") {
        detect(&mut backend, after)?;
    }
    println!("PASS NAPS2 {stage}");
    Ok(())
}
