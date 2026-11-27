use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use artcraft_launcher_core::protocol::{Backend, Command, Event, JobPhase};
use artcraft_launcher_core::release::parse_github_release;
use artcraft_launcher_core::PackageKind;
use artcraft_launcher_native::{NativeBackend, http, manifest::Manifest, paths};

fn wait<T>(backend: &mut NativeBackend, seconds: u64, read: impl Fn(Event) -> Option<T>) -> Result<T, String> {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < deadline {
        if let Some(value) = backend.poll().into_iter().find_map(&read) {
            return Ok(value);
        }
        std::thread::sleep(Duration::from_millis(40));
    }
    Err("timed out waiting for native event".into())
}

fn main() -> Result<(), String> {
    let root = Path::new("/fixture");
    if !root.join(".shift-official-batch").is_file()
        || ![("HOME", "home"), ("XDG_DATA_HOME", "data"), ("XDG_CACHE_HOME", "cache"), ("XDG_CONFIG_HOME", "config")]
            .iter().all(|(key, rel)| std::env::var_os(key).as_deref() == Some(root.join(rel).as_os_str())) {
        return Err("disposable /fixture environment required".into());
    }
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let [id, phase, old, new, repo] = args.as_slice() else { return Err("expected id, phase, old and new, repo".into()) };
    let paths = paths::Paths::new()?;
    if paths.data != root.join("data/shiftanything") { return Err("unexpected launcher storage".into()) }
    let wanted = match phase.as_str() {
        "install" => Some(old),
        "update" => Some(new),
        _ => None,
    };
    let mut expected = None;
    if let Some(wanted) = wanted {
        let version = wanted.trim_start_matches('v');
        let file = root.join(format!("{wanted}.json"));
        let mut release = parse_github_release(&std::fs::read_to_string(&file).map_err(|e| e.to_string())?, repo)?;
        if release.tag != *wanted || release.prerelease {
            return Err(format!("unexpected upstream release {:?}", release.tag));
        }
        let name = format!("{id}-{version}-linux-x86_64.AppImage");
        let asset = release.assets.iter_mut().find(|a| a.name == name).ok_or("missing exact package from official release")?;
        let canonical = format!("https://github.com/{repo}/releases/download/{wanted}/{name}");
        if asset.url != canonical || asset.size == 0 {
            return Err(format!("bad upstream asset {:?}", asset.url));
        }
        let digest = std::fs::read_to_string(root.join(format!("{wanted}.sha256"))).map_err(|e| e.to_string())?;
        let digest = digest.trim().to_ascii_lowercase();
        if digest.len() != 64 || !digest.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("missing exact SHA-256 from official checksum list".into());
        }
        if asset.sha256.as_ref().is_some_and(|found| found != &digest) { return Err("release API and checksum list disagree".into()) }
        asset.sha256 = Some(digest.clone());
        expected = Some((version.to_owned(), name, digest, asset.size));
        let mut entries = serde_json::Map::new();
        entries.insert(repo.clone(), serde_json::json!({"release": release, "etag": null, "prereleases": false, "fetched_at": artcraft_launcher_native::now()}));
        paths::write_atomic(&paths.release_cache(), &serde_json::to_vec(&serde_json::json!({"entries": entries})).map_err(|e| e.to_string())?)?;
    }
    let mut backend = NativeBackend::new(|| {});
    let previous = wait(&mut backend, 30, |event| match event {
        Event::Detected { app, installed } if app == *id => Some(installed),
        _ => None,
    })?;
    let previous_version = previous.as_ref().and_then(|state| state.version.as_ref()).map(ToString::to_string);
    let expect_previous = match phase.as_str() {
        "install" => None,
        "update" | "launch-old" => Some(old.trim_start_matches('v')),
        "launch" | "uninstall" => Some(new.trim_start_matches('v')),
        _ => return Err("unknown test phase".into()),
    };
    if previous_version.as_deref() != expect_previous {
        return Err(format!("unexpected before state {previous:?} for {phase}"));
    }
    let command = match phase.as_str() {
        "launch" | "launch-old" => Command::Launch { app: id.clone(), args: Vec::new() },
        "uninstall" => Command::Uninstall { app: id.clone() },
        _ => Command::Install { app: id.clone() },
    };
    let start = Instant::now();
    backend.send(command);
    let mut transferred = None;
    let result = loop {
        if start.elapsed() > Duration::from_secs(120) { return Err("native action timed out".into()) }
        let mut result = None;
        for event in backend.poll() {
            match event {
                Event::Progress { app, phase: JobPhase::Downloading { done, total } } if app == *id && done == total => transferred = Some(total),
                Event::JobFinished { app, result: value } if app == *id => result = Some(value),
                _ => {}
            }
        }
        if let Some(result) = result { break result? }
        std::thread::sleep(Duration::from_millis(40));
    };
    let manifest = Manifest::load(&paths.manifest());
    if let Some((version, name, digest, size)) = expected {
        let record = manifest.apps.get(id).ok_or("installed app missing from ownership manifest")?;
        let path = PathBuf::from(&record.path);
        if transferred != Some(size) || record.version != version || record.kind != PackageKind::AppImage
            || record.needs_root || !path.starts_with(root.join("home")) || path.file_name().is_none_or(|file| file != name.as_str())
            || http::sha256_file(&path)? != digest {
            return Err(format!("unexpected installed AppImage {record:?}, bytes transferred {transferred:?}"));
        }
        println!("OFFICIAL {repo} {version} {size} sha256={digest}");
    }
    if phase == "uninstall" && manifest.apps.contains_key(id) { return Err("removed app was still in ownership manifest".into()) }
    if matches!(phase.as_str(), "update" | "uninstall")
        && previous.as_ref().is_some_and(|state| Path::new(&state.location).exists()) {
        return Err("previous executable still exists at the installation path".into());
    }
    println!("PASS {id} {phase} {}ms: {result}", start.elapsed().as_millis());
    Ok(())
}
