use artcraft_launcher_core::{Arch, CraftApp, Os, PackageKind, Platform};
use artcraft_launcher_core::catalog::AppKind;
use artcraft_launcher_core::platform::pick_asset;
use artcraft_launcher_core::protocol::{InstallLocations, JobPhase};
use artcraft_launcher_core::release::{Asset, Release};
use artcraft_launcher_core::version;
use artcraft_launcher_native::{detect, http::{Http, Stop}, install::{self, Job}, manifest::Manifest, paths::Paths};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn bad(error: impl std::fmt::Display) -> String { error.to_string() }
fn arg(value: Option<String>) -> Result<String, String> { value.ok_or("missing argument".into()) }
fn static_arg(value: Option<String>) -> Result<&'static str, String> { Ok(Box::leak(arg(value)?.into_boxed_str())) }

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let app = CraftApp {
        id: static_arg(args.next())?,
        name: static_arg(args.next())?,
        repo: static_arg(args.next())?,
        bundle: Some(static_arg(args.next())?),
        mac_team: Some(static_arg(args.next())?),
        tagline: "Package verification in a disposable profile",
        color: [0; 3], ink_light: [0; 3], ink_dark: [0; 3],
        former_repo: None, former_ids: &[], former_names: &[],
        mac_bundle_name: None, kind: AppKind::Community, publisher: "Original publisher",
        msi_publisher: None, former_bundles: &[], site: None,
    };
    let operation = arg(args.next())?;
    let cwd = std::env::current_dir().map_err(bad)?.canonicalize().map_err(bad)?;
    let home = cwd.join(format!("home-{}", app.id));
    let shell_home = PathBuf::from(std::env::var_os("HOME").ok_or("HOME is required")?);
    if !cwd.starts_with("/Volumes/T7-Backup") || !cwd.join(".shift-mac-probe").is_file()
        || shell_home != home || shell_home.canonicalize().map_err(bad)? != home
        || directories_home(&home)? != home
    {
        return Err("Expected disposable proof home on T7".into());
    }
    let apps = home.join("Apps");
    let trash = home.join(".Trash");
    for dir in [&apps, &trash] {
        if dir.canonicalize().map_err(bad)? != *dir {
            return Err("Unexpected install or trash directory".into());
        }
    }
    let marker = home.join("data-to-preserve.txt");
    if std::fs::read(&marker).map_err(bad)? != b"fixture data\n" {
        return Err("Missing synthetic data marker".into());
    }
    let paths = Paths::under(&home.join("launcher"));
    let receipt_path = paths.manifest();
    let mut manifest = Manifest::load(&receipt_path);
    let old = manifest.apps.get(app.id).cloned();
    let bundle = apps.join(format!("{}.app", app.mac_bundle_name()));
    let current = detect::macos::read_bundle(&app, &bundle);
    if (current.is_some() != old.is_some()) || (bundle.exists() != old.is_some()) {
        return Err("Bundle and receipt disagree".into());
    }
    let stop = Stop::default();
    let clock = Instant::now();
    if operation == "remove" {
        let existing = current.as_ref().ok_or("No installed copy")?;
        let old = old.as_ref().ok_or("No prior install record")?;
        if existing.location != bundle.to_string_lossy() || old.path != bundle {
            return Err("Removal path is outside proof profile".into());
        }
        install::uninstall(&app, existing, Some(old), &paths, &stop, &mut |phase| { println!("{phase:?}"); })?;
        manifest.apps.remove(app.id);
        manifest.save(&receipt_path)?;
        let moved = trash.join(format!("{}.app", app.mac_bundle_name()));
        if bundle.exists() || detect::macos::read_bundle(&app, &moved).is_none() || Manifest::load(&receipt_path).apps.contains_key(app.id) {
            return Err("Bundle or receipt still installed or missing from private Trash".into());
        }
        println!("PASS private Trash {}", moved.display());
    } else {
        if !matches!((operation.as_str(), current.is_some()), ("install", false) | ("update", true)) {
            return Err("Operation does not match initial state".into());
        }
        let tag = arg(args.next())?;
        let name = arg(args.next())?;
        let size = arg(args.next())?.parse::<u64>().map_err(bad)?;
        let checksum = arg(args.next())?;
        let expected = version::parse(&tag).ok_or("Expected a valid release version")?;
        let asset = Asset { url: format!("https://github.com/{}/releases/download/{tag}/{name}", app.repo), name, size, sha256: Some(checksum) };
        let release = Release { tag, version: Some(expected.clone()), assets: vec![asset.clone()], ..Release::default() };
        let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        println!("Generic selector matched: {}", pick_asset(&release, &[app.id], platform, &[PackageKind::Dmg]).is_some());
        if let Some(existing) = current.as_ref() {
            let version = existing.version.as_ref().ok_or("No installed version")?;
            if version >= &expected || existing.location != bundle.to_string_lossy() {
                return Err("Old version is not older or outside profile".into());
            }
        }
        let http = Http::new()?;
        let locations = InstallLocations { root: Some(apps.display().to_string()), ..Default::default() };
        let mut downloaded = 0;
        let mut phase = |phase| match phase {
            JobPhase::Downloading { done, .. } => { downloaded = downloaded.max(done); }
            other => println!("{other:?} after {:.3}s", clock.elapsed().as_secs_f64()),
        };
        let receipt = install::install(
            &app, &release, &asset, PackageKind::Dmg, current.as_ref(), old.as_ref(),
            &mut Job { http: &http, paths: &paths, stop: &stop, phase: &mut phase, install_locations: &locations, replace: false,
                       approved_path: None, downgrade: None, batch: None, place: None, put_back: None },
        )?.ok_or("Missing install receipt")?;
        let actual = detect::macos::read_bundle(&app, &bundle).ok_or("Missing installed bundle")?;
        if receipt.path != bundle || receipt.kind != PackageKind::Dmg || receipt.version != expected.to_string()
            || actual.version.as_ref() != Some(&expected) || downloaded != size
        {
            return Err(format!("Unexpected receipt {receipt:?} or download {downloaded} of {size}"));
        }
        manifest.apps.insert(app.id.into(), receipt);
        manifest.save(&receipt_path)?;
        println!("PASS {expected} {downloaded} bytes");
    }
    if std::fs::read(marker).map_err(bad)? != b"fixture data\n" {
        return Err("Synthetic data changed".into());
    }
    println!("PASS {operation} {:.3}s", clock.elapsed().as_secs_f64());
    Ok(())
}

fn directories_home(expected: &Path) -> Result<PathBuf, String> {
    let trash = expected.join(".Trash");
    if !trash.is_dir() { return Err("Missing proof trash".into()); }
    let paths = Paths::new()?;
    paths.data.strip_prefix(expected).map_err(bad)?;
    paths.cache.strip_prefix(expected).map_err(bad)?;
    Ok(expected.to_path_buf())
}
