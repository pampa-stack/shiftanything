//! A self-contained app folder shipped as a reviewed `.tar.xz` (Linux).
//!
//! The archive is decompressed by the system `xz` and read by the `tar` crate, so no new
//! dependency is needed. Nothing selects [`PackageKind::TarXz`](artcraft_launcher_core::PackageKind)
//! yet: `artcraft_launcher_core::pending` holds the recipe until its lifecycle proof exists.
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Component, Path, PathBuf};
use std::process::Stdio;

use artcraft_launcher_core::pending::ArchiveLayout;

use super::{CraftApp, Paths, SYSTEM_BIN_DIRS, find_in, single_top_folder, swap_into_place};

/// Bounds against decompression bombs (CWE-409); the reviewed Umi-OCR archive has 6,007 entries
/// and 1.1 GB.
const MAX_ENTRIES: usize = 50_000;
const MAX_UNPACKED_BYTES: u64 = 2 * 1024 * 1024 * 1024;

fn plain_relative(path: &Path) -> bool {
    path.components().all(|part| matches!(part, Component::Normal(_)))
}

/// Unpacks `file` into `<apps>/<app>` and returns the entry the app starts from. An update
/// replaces the folder in one step and carries the layout's settings files into the new copy; a
/// failure at any point leaves the installed copy as it was.
pub fn install(app: &CraftApp, file: &Path, layout: &ArchiveLayout, paths: &Paths) -> Result<PathBuf, String> {
    let apps = paths.apps();
    std::fs::create_dir_all(&apps).map_err(|e| format!("couldn't create {} ({e})", apps.display()))?;
    // Beside the destination, so the final rename never crosses a filesystem.
    let staged = apps.join(format!(".{}.new", app.id));
    let result = stage(file, &staged, layout).and_then(|top| {
        let dest = apps.join(app.id);
        carry_settings(&dest, &top, layout)?;
        swap_into_place(&top, &dest)?;
        Ok(dest.join(layout.entry))
    });
    let _ = std::fs::remove_dir_all(&staged);
    result
}

fn stage(file: &Path, staged: &Path, layout: &ArchiveLayout) -> Result<PathBuf, String> {
    let _ = std::fs::remove_dir_all(staged);
    std::fs::create_dir_all(staged).map_err(|e| format!("couldn't create {} ({e})", staged.display()))?;
    unpack(file, staged)?;
    let top = single_top_folder(staged)?;
    if top == staged {
        return Err("the download isn't a single app folder".into());
    }
    let entry = top.join(layout.entry);
    if !std::fs::symlink_metadata(&entry).is_ok_and(|meta| meta.file_type().is_file()) {
        return Err("the download doesn't contain the app".into());
    }
    // A regular file inside our own staging folder, so the chmod cannot reach anything else.
    std::fs::set_permissions(&entry, std::fs::Permissions::from_mode(0o755)).map_err(|e| format!("couldn't make the app runnable ({e})"))?; // NOSONAR
    Ok(top)
}

fn unpack(file: &Path, staged: &Path) -> Result<(), String> {
    let xz = find_in(&SYSTEM_BIN_DIRS, OsStr::new("xz")).ok_or("xz isn't installed (Ubuntu and Debian package: xz-utils)")?;
    let mut child = crate::process::command(xz)
        .args(["-dc", "--"])
        .arg(file)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| format!("couldn't start xz ({e})"))?;
    let Some(mut stdout) = child.stdout.take() else {
        let _ = child.kill();
        let _ = child.wait();
        return Err("couldn't read from xz".into());
    };
    let mut result = unpack_entries(&mut stdout, staged);
    if result.is_ok() {
        // Read to the end: a truncated or corrupt stream makes xz fail, even if tar saw a clean end.
        result = std::io::copy(&mut stdout, &mut std::io::sink()).map(|_| ()).map_err(|e| format!("couldn't read the archive ({e})"));
    }
    if result.is_err() {
        let _ = child.kill();
    }
    drop(stdout);
    let status = child.wait().map_err(|e| e.to_string())?;
    result?;
    if status.success() { Ok(()) } else { Err("the archive is damaged".into()) }
}

fn unpack_entries(stream: &mut impl Read, staged: &Path) -> Result<(), String> {
    let mut archive = tar::Archive::new(stream);
    // Keeps the execute bits and strips setuid, setgid and sticky.
    archive.set_preserve_permissions(false);
    let mut entries = 0usize;
    let mut bytes = 0u64;
    let mut directories = Vec::new();
    for entry in archive.entries().map_err(|e| format!("couldn't read archive entries ({e})"))? {
        let mut entry = entry.map_err(|e| format!("invalid tar entry ({e})"))?;
        entries += 1;
        bytes = bytes.saturating_add(entry.size());
        if entries > MAX_ENTRIES {
            return Err("the archive has too many entries".into());
        }
        if bytes > MAX_UNPACKED_BYTES {
            return Err("the archive is larger than the reviewed limit".into());
        }
        let kind = entry.header().entry_type();
        let name = entry.path().map_err(|e| e.to_string())?.into_owned();
        if !plain_relative(&name) {
            return Err("the archive contains a path outside its folder".into());
        }
        match kind {
            tar::EntryType::Regular => {}
            tar::EntryType::Directory => {
                directories.push(entry);
                continue;
            }
            tar::EntryType::Symlink => {
                // A link to a sibling name only: it can never point out of the folder.
                let target = entry.link_name().map_err(|e| e.to_string())?.ok_or("the archive contains a link without a target")?;
                if !plain_relative(&target) {
                    return Err("the archive contains a link that leaves its folder".into());
                }
            }
            _ => return Err("the archive contains an entry type the launcher doesn't accept".into()),
        }
        if !entry.unpack_in(staged).map_err(|e| format!("couldn't unpack an archive entry ({e})"))? {
            return Err("the archive contains a path outside its folder".into());
        }
    }
    // Folders last, so a read-only mode doesn't stop their children from being written.
    directories.sort_by(|a, b| b.path_bytes().cmp(&a.path_bytes()));
    for mut dir in directories {
        dir.unpack_in(staged).map_err(|e| format!("couldn't unpack an archive folder ({e})"))?;
    }
    Ok(())
}

/// Copies the user's settings files from the installed copy into the staged one.
fn carry_settings(dest: &Path, top: &Path, layout: &ArchiveLayout) -> Result<(), String> {
    for relative in layout.keep {
        let relative = Path::new(relative);
        if !plain_relative(relative) {
            return Err("the recipe names a settings file outside the app folder".into());
        }
        let old = dest.join(relative);
        if !std::fs::symlink_metadata(&old).is_ok_and(|meta| meta.file_type().is_file()) {
            continue;
        }
        let new = top.join(relative);
        let mut here = top.to_path_buf();
        for part in relative.parent().into_iter().flat_map(Path::components) {
            here.push(part);
            match std::fs::symlink_metadata(&here) {
                Ok(meta) if meta.file_type().is_symlink() || !meta.is_dir() => return Err("the download's folders don't match the recipe".into()),
                Ok(_) => {}
                Err(_) => std::fs::create_dir(&here).map_err(|e| format!("couldn't keep the settings ({e})"))?,
            }
        }
        if std::fs::symlink_metadata(&new).is_ok_and(|meta| meta.file_type().is_symlink()) {
            return Err("the download's settings file is a link".into());
        }
        std::fs::copy(&old, &new).map_err(|e| format!("couldn't keep the settings ({e})"))?;
    }
    Ok(())
}

/// Removes everything the launcher unpacked and leaves the layout's settings files, with the
/// folders that hold them: they are the user's, so a reinstall finds them where they were.
pub fn remove(app: &CraftApp, layout: &ArchiveLayout, paths: &Paths) -> Result<(), String> {
    let root = paths.apps().join(app.id);
    if !root.starts_with(paths.apps()) || !std::fs::symlink_metadata(&root).is_ok_and(|meta| meta.is_dir()) {
        return Err(format!("{} isn't installed in this folder", app.name));
    }
    let keep: BTreeSet<PathBuf> = layout.keep.iter().map(PathBuf::from).collect();
    if !keep.iter().all(|path| plain_relative(path)) {
        return Err("the recipe names a settings file outside the app folder".into());
    }
    if prune(&root, &root, &keep).map_err(|e| format!("couldn't remove {} ({e})", root.display()))? && root.exists() {
        std::fs::remove_dir(&root).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Deletes what isn't kept under `dir`; true when nothing is left in it.
fn prune(dir: &Path, root: &Path, keep: &BTreeSet<PathBuf>) -> std::io::Result<bool> {
    let mut empty = true;
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let meta = std::fs::symlink_metadata(&path)?;
        if meta.is_dir() {
            if prune(&path, root, keep)? {
                std::fs::remove_dir(&path)?;
            } else {
                empty = false;
            }
        } else if path.strip_prefix(root).is_ok_and(|relative| keep.contains(relative)) {
            empty = false;
        } else {
            std::fs::remove_file(&path)?;
        }
    }
    Ok(empty)
}

#[cfg(test)]
mod tests {
    use super::*;
    use artcraft_launcher_core::pending::UMI_OCR_LAYOUT;
    use std::io::Write;

    const LAYOUT: ArchiveLayout = ArchiveLayout { entry: "run.sh", keep: &["data/.settings"] };

    enum Item<'a> {
        File(&'a str, &'a [u8], u32),
        Dir(&'a str, u32),
        Symlink(&'a str, &'a str),
        Hardlink(&'a str, &'a str),
    }

    /// An xz-compressed tarball written with the system `xz`, with header names set verbatim so
    /// hostile paths can be built.
    fn archive(dir: &Path, name: &str, items: &[Item]) -> PathBuf {
        let mut builder = tar::Builder::new(Vec::new());
        for item in items {
            let mut header = tar::Header::new_gnu();
            let (path, link, kind, mode, data): (&str, Option<&str>, tar::EntryType, u32, &[u8]) = match item {
                Item::File(path, data, mode) => (path, None, tar::EntryType::Regular, *mode, data),
                Item::Dir(path, mode) => (path, None, tar::EntryType::Directory, *mode, &[]),
                Item::Symlink(path, target) => (path, Some(*target), tar::EntryType::Symlink, 0o777, &[]),
                Item::Hardlink(path, target) => (path, Some(*target), tar::EntryType::Link, 0o644, &[]),
            };
            header.set_entry_type(kind);
            header.set_mode(mode);
            header.set_size(data.len() as u64);
            header.as_old_mut().name[..path.len()].copy_from_slice(path.as_bytes());
            if let Some(target) = link {
                header.as_old_mut().linkname[..target.len()].copy_from_slice(target.as_bytes());
            }
            header.set_cksum();
            builder.append(&header, data).unwrap();
        }
        let tar_bytes = builder.into_inner().unwrap();
        let out = dir.join(name);
        let mut child =
            std::process::Command::new("xz").args(["-zc"]).stdin(Stdio::piped()).stdout(std::fs::File::create(&out).unwrap()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(&tar_bytes).unwrap();
        assert!(child.wait().unwrap().success());
        out
    }

    fn demo(version: &str, settings: &str) -> Vec<Item<'static>> {
        let script: &'static [u8] = Box::leak(format!("#!/bin/sh\necho {version}\n").into_bytes().into_boxed_slice());
        let settings: &'static [u8] = Box::leak(settings.as_bytes().to_vec().into_boxed_slice());
        vec![
            Item::Dir("demo/", 0o755),
            Item::File("demo/run.sh", script, 0o644),
            Item::Dir("demo/data/", 0o755),
            Item::File("demo/data/.settings", settings, 0o600),
            Item::File("demo/lib/libx.so.1", b"lib", 0o644),
            Item::Symlink("demo/lib/libx.so", "libx.so.1"),
        ]
    }

    fn setup() -> (tempfile::TempDir, Paths) {
        let root = tempfile::tempdir().unwrap();
        let paths = Paths::under(root.path());
        (root, paths)
    }

    fn app() -> CraftApp {
        artcraft_launcher_core::curated::UMI_OCR
    }

    #[test]
    fn installs_the_folder_with_a_runnable_entry_and_sibling_links() {
        let (root, paths) = setup();
        let file = archive(root.path(), "demo-1.tar.xz", &demo("1", "default"));
        let entry = install(&app(), &file, &LAYOUT, &paths).unwrap();
        assert_eq!(entry, paths.apps().join("umi-ocr/run.sh"));
        assert!(std::fs::metadata(&entry).unwrap().permissions().mode() & 0o111 != 0, "the entry must be executable");
        let link = paths.apps().join("umi-ocr/lib/libx.so");
        assert_eq!(std::fs::read_link(&link).unwrap(), Path::new("libx.so.1"));
        assert_eq!(std::fs::read(&link).unwrap(), b"lib");
        assert!(!paths.apps().join(".umi-ocr.new").exists(), "staging is cleaned up");
    }

    #[test]
    fn an_update_replaces_the_program_and_keeps_the_user_settings() {
        let (root, paths) = setup();
        install(&app(), &archive(root.path(), "v1.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        let settings = paths.apps().join("umi-ocr/data/.settings");
        std::fs::write(&settings, "synthetic user choice").unwrap();
        let entry = install(&app(), &archive(root.path(), "v2.tar.xz", &demo("2", "default")), &LAYOUT, &paths).unwrap();
        assert!(std::fs::read_to_string(entry).unwrap().contains("echo 2"));
        assert_eq!(std::fs::read_to_string(settings).unwrap(), "synthetic user choice");
    }

    #[test]
    fn a_removal_leaves_only_the_settings() {
        let (root, paths) = setup();
        install(&app(), &archive(root.path(), "v1.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        remove(&app(), &LAYOUT, &paths).unwrap();
        let left: Vec<_> = tree_files(&paths.apps().join("umi-ocr"));
        assert_eq!(left, [PathBuf::from("data/.settings")]);
        // A reinstall finds the settings where they were.
        install(&app(), &archive(root.path(), "v1b.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        assert!(paths.apps().join("umi-ocr/run.sh").is_file());
    }

    #[test]
    fn a_removal_without_settings_deletes_the_folder_and_a_second_one_says_so() {
        let (root, paths) = setup();
        install(&app(), &archive(root.path(), "v1.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        std::fs::remove_file(paths.apps().join("umi-ocr/data/.settings")).unwrap();
        remove(&app(), &LAYOUT, &paths).unwrap();
        assert!(!paths.apps().join("umi-ocr").exists());
        assert!(remove(&app(), &LAYOUT, &paths).unwrap_err().contains("isn't installed"));
    }

    fn tree_files(root: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                } else {
                    out.push(path.strip_prefix(root).unwrap().to_path_buf());
                }
            }
        }
        out.sort();
        out
    }

    #[test]
    fn hostile_archives_are_refused_and_never_disturb_the_installed_copy() {
        let (root, paths) = setup();
        install(&app(), &archive(root.path(), "good.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        let entry = paths.apps().join("umi-ocr/run.sh");
        let hostile: Vec<(&str, Vec<Item>)> = vec![
            ("parent path", vec![Item::Dir("demo/", 0o755), Item::File("demo/../escape", b"x", 0o644)]),
            ("absolute path", vec![Item::File("/tmp/escape", b"x", 0o644)]),
            ("absolute link", vec![Item::Dir("demo/", 0o755), Item::File("demo/run.sh", b"x", 0o755), Item::Symlink("demo/etc", "/etc")]),
            ("link out of the folder", vec![Item::Dir("demo/", 0o755), Item::File("demo/run.sh", b"x", 0o755), Item::Symlink("demo/up", "../..")]),
            ("hard link", vec![Item::Dir("demo/", 0o755), Item::File("demo/run.sh", b"x", 0o755), Item::Hardlink("demo/h", "demo/run.sh")]),
            ("no entry script", vec![Item::Dir("demo/", 0o755), Item::File("demo/other.sh", b"x", 0o755)]),
            ("entry is a link", vec![Item::Dir("demo/", 0o755), Item::File("demo/real", b"x", 0o755), Item::Symlink("demo/run.sh", "real")]),
            ("two top folders", vec![Item::File("a/run.sh", b"x", 0o755), Item::File("b/run.sh", b"x", 0o755)]),
            ("no folder at all", vec![Item::File("run.sh", b"x", 0o755)]),
        ];
        for (label, items) in hostile {
            let file = archive(root.path(), "bad.tar.xz", &items);
            assert!(install(&app(), &file, &LAYOUT, &paths).is_err(), "{label} must be refused");
            assert!(std::fs::read_to_string(&entry).unwrap().contains("echo 1"), "{label} disturbed the installed copy");
            assert!(!root.path().join("escape").exists() && !Path::new("/tmp/escape").exists(), "{label} wrote outside the folder");
            assert!(!paths.apps().join(".umi-ocr.new").exists(), "{label} left staging behind");
        }
    }

    #[test]
    fn a_truncated_download_fails_without_touching_the_installed_copy() {
        let (root, paths) = setup();
        install(&app(), &archive(root.path(), "good.tar.xz", &demo("1", "default")), &LAYOUT, &paths).unwrap();
        let whole = archive(root.path(), "v2.tar.xz", &demo("2", "default"));
        let bytes = std::fs::read(&whole).unwrap();
        std::fs::write(&whole, &bytes[..bytes.len() - 24]).unwrap();
        assert!(install(&app(), &whole, &LAYOUT, &paths).is_err());
        assert!(std::fs::read_to_string(paths.apps().join("umi-ocr/run.sh")).unwrap().contains("echo 1"));
        assert!(install(&app(), &root.path().join("missing.tar.xz"), &LAYOUT, &paths).is_err());
    }

    #[test]
    fn the_reviewed_umi_ocr_layout_only_names_files_inside_the_folder() {
        let (root, paths) = setup();
        let items = [
            Item::Dir("Umi-OCR/", 0o755),
            Item::File("Umi-OCR/umi-ocr.sh", b"#!/bin/sh\n", 0o755),
            Item::File("Umi-OCR/UmiOCR-data/.settings", b"default", 0o644),
        ];
        let entry = install(&app(), &archive(root.path(), "umi.tar.xz", &items), &UMI_OCR_LAYOUT, &paths).unwrap();
        assert_eq!(entry, paths.apps().join("umi-ocr/umi-ocr.sh"));
        std::fs::write(paths.apps().join("umi-ocr/UmiOCR-data/.settings"), "mine").unwrap();
        remove(&app(), &UMI_OCR_LAYOUT, &paths).unwrap();
        assert_eq!(std::fs::read_to_string(paths.apps().join("umi-ocr/UmiOCR-data/.settings")).unwrap(), "mine");
    }
}
