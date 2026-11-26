//! Drive the desktop backend against official LocalSend AppImages in a disposable Linux sandbox.
//! See docs/verification/pro-371-linux-localsend.md for the sandbox and evidence.

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn main() {
    eprintln!("This verification example requires Linux x86_64.");
    std::process::exit(2);
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn main() -> Result<(), String> {
    use std::path::Path;

    let root = Path::new("/fixture");
    if !root.join(".shift-localsend-proof").is_file()
        || ![("HOME", "home"), ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"), ("XDG_CONFIG_HOME", "config")]
            .iter()
            .all(|(key, path)| std::env::var_os(key).as_deref() == Some(root.join(path).as_os_str()))
    {
        return Err("Run only in the disposable /fixture sandbox".into());
    }
    linux::run(&std::env::args().nth(1).ok_or("expected install, update, launch-old, launch, or uninstall")?)
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
mod linux {
    use std::path::Path;
    use std::time::{Duration, Instant};

    use artcraft_launcher_core::curated::LOCALSEND;
    use artcraft_launcher_core::platform::pick_asset;
    use artcraft_launcher_core::protocol::{Backend, Command, Event, JobPhase, LatestRelease, Source};
    use artcraft_launcher_core::release::{Release, ReleaseQuery, parse_github_release};
    use artcraft_launcher_core::{Arch, Os, PackageKind, Platform};
    use artcraft_launcher_native::source::{GitHubSource, ReleaseSource};
    use artcraft_launcher_native::{NativeBackend, http, manifest::Manifest, paths};

    const OLD: (&str, u64, &str) = ("1.18.2", 66_471_104, "19f21613d1834f6caa69517113670461a3f10b80a2a315f86548c0663d9c0aaf");
    const NEW: (&str, u64, &str) = ("1.19.0", 61_752_512, "e72ce88383a32beec9701583c22e298037f7683f9590cea7b3a281f58c2ec2b3");

    fn package_name(version: &str) -> String {
        format!("LocalSend-{version}-linux-x86-64.AppImage")
    }

    fn verify_release(release: &Release, (version, size, sha256): (&str, u64, &str)) -> Result<(), String> {
        let platform = Platform { os: Os::Linux, arch: Arch::X64 };
        let (asset, _) = pick_asset(release, &[LOCALSEND.id], platform, &[PackageKind::AppImage]).ok_or("release missing reviewed AppImage")?;
        if release.tag != format!("v{version}")
            || release.prerelease
            || asset.name != package_name(version)
            || asset.size != size
            || asset.sha256.as_deref() != Some(sha256)
        {
            return Err("official release metadata differs from reviewed asset".into());
        }
        println!("OFFICIAL {} bytes={} sha256={sha256}", asset.url, asset.size);
        Ok(())
    }

    fn seed_previous_release(paths: &paths::Paths) -> Result<(), String> {
        let url = format!("https://api.github.com/repos/{}/releases/tags/v{}", LOCALSEND.repo, OLD.0);
        let body = http::Http::new()?.get_text(&url)?.ok_or("missing official release body")?;
        let release = parse_github_release(&body, LOCALSEND.repo)?;
        verify_release(&release, OLD)?;
        // Cache the historical release so the normal Install command selects it without pins.
        let cache = serde_json::json!({"entries": {LOCALSEND.repo: {
            "release": release, "etag": null, "prereleases": false, "fetched_at": artcraft_launcher_native::now()
        }}});
        paths::write_atomic(&paths.release_cache(), &serde_json::to_vec(&cache).map_err(|error| error.to_string())?)
    }

    fn check_latest_release(paths: &paths::Paths) -> Result<(), String> {
        let source = GitHubSource::new(http::Http::new()?, paths.release_cache());
        let answer = source.latest(&ReleaseQuery::for_app(&LOCALSEND, LOCALSEND.repo), false);
        let LatestRelease::Found(release) = answer.latest else {
            return Err(format!("latest release lookup failed: {:?}", answer.latest));
        };
        if answer.source != Source::GitHubApi {
            return Err(format!("expected direct GitHub API response, found {:?}", answer.source));
        }
        verify_release(&release, NEW)?;
        source.save();
        Ok(())
    }

    fn wait_for<T>(backend: &mut NativeBackend, limit: Duration, mut read: impl FnMut(Event) -> Option<T>) -> Result<T, String> {
        let end = Instant::now() + limit;
        while Instant::now() < end {
            if let Some(result) = backend.poll().into_iter().find_map(&mut read) {
                return Ok(result);
            }
            std::thread::sleep(Duration::from_millis(40));
        }
        Err("timed out waiting for LocalSend backend event".into())
    }

    pub fn run(phase: &str) -> Result<(), String> {
        let paths = paths::Paths::new()?;
        if paths.data != Path::new("/fixture/data/shiftanything") || paths.cache != Path::new("/fixture/cache/shiftanything") {
            return Err("launcher storage escaped the disposable sandbox".into());
        }
        let app = LOCALSEND.id.to_owned();
        let new = match phase {
            "install" => {
                seed_previous_release(&paths)?;
                Some(OLD)
            }
            "update" => {
                check_latest_release(&paths)?;
                Some(NEW)
            }
            "launch-old" | "launch" | "uninstall" => None,
            _ => return Err("unknown verification phase".into()),
        };
        let mut backend = NativeBackend::new(|| {});
        let before = wait_for(&mut backend, Duration::from_secs(25), |event| match event {
            Event::Detected { app: id, installed } if id == app => Some(installed),
            _ => None,
        })?;
        let expected_before = match phase {
            "install" => None,
            "update" | "launch-old" => Some(OLD.0),
            _ => Some(NEW.0),
        };
        if before.is_some() != expected_before.is_some()
            || before.as_ref().and_then(|installed| installed.version.as_ref()).map(ToString::to_string).as_deref() != expected_before
        {
            return Err(format!("unexpected initial state: {before:?}"));
        }
        let command = match phase {
            "launch" | "launch-old" => Command::Launch { app: app.clone(), args: vec![] },
            "uninstall" => Command::Uninstall { app: app.clone() },
            _ => Command::Install { app: app.clone() },
        };
        backend.send(command);
        let mut transferred = None;
        let result = wait_for(&mut backend, Duration::from_secs(100), |event| match event {
            Event::Progress { app: id, phase: JobPhase::Downloading { done, total } } if id == app && done == total => {
                transferred = Some(total);
                None
            }
            Event::JobFinished { app: id, result } if id == app => Some(result),
            _ => None,
        })??;
        let manifest = Manifest::load(&paths.manifest());
        match (phase, new) {
            (_, Some((version, size, sha256))) => {
                let record = manifest.apps.get(&app).ok_or("installed AppImage is missing from the launcher manifest")?;
                if transferred != Some(size)
                    || record.version != version
                    || record.kind != PackageKind::AppImage
                    || record.needs_root
                    || !record.path.starts_with("/fixture/home")
                    || record.path.file_name().is_none_or(|name| name != package_name(version).as_str())
                    || http::sha256_file(&record.path)? != sha256
                {
                    return Err(format!("download or recorded AppImage differs from the official package: {record:?}, bytes={transferred:?}"));
                }
                println!("RECORDED {} sha256={sha256}", record.path.display());
            }
            ("uninstall", None) if manifest.apps.contains_key(&app) => return Err("removed AppImage remains in the launcher manifest".into()),
            (_, None) => {}
        }
        if matches!(phase, "update" | "uninstall") && before.as_ref().is_some_and(|installed| Path::new(&installed.location).exists()) {
            return Err("previous executable still exists".into());
        }
        println!("PASS {phase}: {result}");
        Ok(())
    }
}
