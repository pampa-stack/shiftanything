//! Versions: release tags (`v0.5.0`), installed versions from many sources, and ordering.

use std::cmp::Ordering;

pub use semver::Version;

const DBEAVER_HOTFIX: &str = "dbeaver-hotfix.";
const KARING_REVISION: &str = "karing-revision.";

fn publisher_revision(metadata: &str) -> Option<&str> {
    metadata.strip_prefix(DBEAVER_HOTFIX).or_else(|| metadata.strip_prefix(KARING_REVISION))
}

/// Reads a version from a tag or an installed-version string.
///
/// Accepts `v0.5.0`, `0.5.0`, `0.2.0-rc.1`, Debian's `0.2.0~rc.1-1`, and the two-part `0.5`
/// some platforms report. Build metadata is kept but never affects ordering.
pub fn parse(text: &str) -> Option<Version> {
    let t = text.trim();
    let t = t.strip_prefix("artcraft-v").unwrap_or(t);
    let t = t.strip_prefix(['v', 'V']).unwrap_or(t);
    if t.is_empty() || t.len() > 64 {
        return None;
    }
    // Debian/RPM spell pre-releases with `~` and append a package revision after `-`.
    let debianish = t.contains('~');
    let t = if debianish { t.split_once('-').map_or(t, |(v, _)| v).replace('~', "-") } else { t.to_string() };
    if let Ok(v) = Version::parse(&t) {
        return (publisher_revision(v.build.as_str()).is_none()).then_some(v);
    }
    // `0.5` → `0.5.0`
    let (core, rest) = match t.find(['-', '+']) {
        Some(i) => t.split_at(i),
        None => (t.as_str(), ""),
    };
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() == 2 && parts.iter().all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit())) {
        return Version::parse(&format!("{core}.0{rest}")).ok().filter(|v| publisher_revision(v.build.as_str()).is_none());
    }
    None
}

/// DBeaver's stable tags and macOS bundle versions can have a fourth numeric component.
pub fn parse_dbeaver(text: &str) -> Option<Version> {
    parse_publisher_revision(text, DBEAVER_HOTFIX)
}

/// Karing uses an explicit build number in the stable tag and macOS bundle.
pub fn parse_karing(text: &str) -> Option<Version> {
    let version = parse_publisher_revision(text, KARING_REVISION)?;
    (!version.build.is_empty()).then_some(version)
}

fn parse_publisher_revision(text: &str, marker: &str) -> Option<Version> {
    let (core, hotfix) = match text.rsplit_once('.') {
        Some((core, hotfix)) if core.matches('.').count() == 2 => (core, Some(hotfix)),
        _ => (text, None),
    };
    let mut version = Version::parse(core).ok()?;
    if !version.pre.is_empty() || !version.build.is_empty() {
        return None;
    }
    if let Some(hotfix) = hotfix {
        let revision = hotfix.parse::<u64>().ok()?;
        if revision == 0 || revision.to_string() != hotfix {
            return None;
        }
        version.build = semver::BuildMetadata::new(&format!("{marker}{revision}")).ok()?;
    }
    Some(version)
}

fn revision(version: &Version) -> u64 {
    publisher_revision(version.build.as_str()).and_then(|n| n.parse().ok()).unwrap_or(0)
}

/// Display the original version, including the publisher's fourth numeric component.
pub fn label(version: &Version) -> String {
    let revision = revision(version);
    if revision != 0 { format!("{}.{}.{}.{revision}", version.major, version.minor, version.patch) } else { version.to_string() }
}

/// Reads a version as a system package manager reports it, dropping what the packaging adds:
/// an epoch (`1:`), the package revision (`-1`, `-2.fc40`), Debian's `~` pre-release spelling, and
/// a VCS snapshot suffix (`0.3.0.r47.g47f9306` from an AUR `-git` package, read as `0.3.0`).
pub fn parse_package(text: &str) -> Option<Version> {
    let t = text.trim();
    // Epoch.
    let t = match t.split_once(':') {
        Some((epoch, rest)) if !epoch.is_empty() && epoch.bytes().all(|b| b.is_ascii_digit()) => rest,
        _ => t,
    };
    // Package revision: whatever follows the last '-' when the part before it is a version.
    let t = match t.rsplit_once('-') {
        Some((upstream, rev)) if !rev.is_empty() && rev.starts_with(|c: char| c.is_ascii_digit()) && upstream.contains('.') => upstream,
        _ => t,
    };
    let t = t.replace('~', "-");
    if let Some(v) = parse(&t) {
        return Some(v);
    }
    // VCS snapshots and other suffixes: the leading X.Y.Z.
    let core: String = t.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    let parts: Vec<&str> = core.split('.').filter(|p| !p.is_empty()).take(3).collect();
    match parts.as_slice() {
        [a, b, c] => Version::parse(&format!("{a}.{b}.{c}")).ok(),
        [a, b] => Version::parse(&format!("{a}.{b}.0")).ok(),
        // Text around the version, as in `--version` output (`PhotoCraft 0.5.0 (abc, 2026)`):
        // the first `X.Y.Z` run anywhere.
        _ => first_xyz(&t),
    }
}

/// The first `major.minor.patch` run anywhere in `text`.
fn first_xyz(text: &str) -> Option<Version> {
    let bytes = text.as_bytes();
    for start in 0..bytes.len() {
        if !bytes.get(start).is_some_and(u8::is_ascii_digit) || (start > 0 && bytes.get(start - 1).is_some_and(u8::is_ascii_digit)) {
            continue;
        }
        let Some((major, i)) = digits(text, start) else { continue };
        if bytes.get(i) != Some(&b'.') {
            continue;
        }
        let Some((minor, j)) = digits(text, i + 1) else { continue };
        if bytes.get(j) != Some(&b'.') {
            continue;
        }
        let Some((patch, _)) = digits(text, j + 1) else { continue };
        return Some(Version::new(major, minor, patch));
    }
    None
}

/// A run of ASCII digits from `from`, as a number and the index just past it.
fn digits(text: &str, from: usize) -> Option<(u64, usize)> {
    let bytes = text.as_bytes();
    let mut end = from;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end += 1;
    }
    if end == from {
        return None;
    }
    text.get(from..end)?.parse().ok().map(|n| (n, end))
}

/// A version that's a plain `X.Y.Z` with no pre-release suffix.
pub fn is_stable(v: &Version) -> bool {
    v.pre.is_empty()
}

/// Orders by precedence, including the numeric revision of known publishers.
pub fn compare(a: &Version, b: &Version) -> Ordering {
    a.cmp_precedence(b).then_with(|| revision(a).cmp(&revision(b)))
}

/// True when `latest` is newer than `installed`.
pub fn is_newer(latest: &Version, installed: &Version) -> bool {
    compare(latest, installed) == Ordering::Greater
}

/// Platforms that only record `X.Y.Z` (the MSI `ProductVersion`, `CFBundleShortVersionString`)
/// drop a pre-release suffix. A numeric-only installed version therefore counts as current when it
/// matches a pre-release's numbers, rather than nagging forever.
pub fn is_update(latest: &Version, installed: &Version) -> bool {
    if installed.pre.is_empty()
        && !latest.pre.is_empty()
        && (latest.major, latest.minor, latest.patch) == (installed.major, installed.minor, installed.patch)
    {
        return false;
    }
    is_newer(latest, installed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        parse(s).unwrap()
    }

    #[test]
    fn reads_tags_and_platform_spellings() {
        assert_eq!(v("v0.5.0"), Version::new(0, 5, 0));
        assert_eq!(v("artcraft-v0.42.0"), Version::new(0, 42, 0));
        assert_eq!(v(" 1.2.3\n"), Version::new(1, 2, 3));
        assert_eq!(v("0.5"), Version::new(0, 5, 0));
        assert_eq!(v("0.2.0~rc.1-1").to_string(), "0.2.0-rc.1");
        assert_eq!(v("0.2.0-rc.1").to_string(), "0.2.0-rc.1");
        for bad in ["", "v", "latest", "1", "1.x.0", "0.5.", &"9".repeat(80)] {
            assert_eq!(parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn reads_package_manager_versions() {
        let p = |s: &str| parse_package(s).map(|v| v.to_string());
        // dpkg and pacman add a package revision; it is not a pre-release.
        assert_eq!(p("0.5.0-1").as_deref(), Some("0.5.0"));
        assert_eq!(p("0.5.0-12").as_deref(), Some("0.5.0"));
        assert_eq!(p("1:0.5.0-1").as_deref(), Some("0.5.0"));
        assert_eq!(p("0.2.0~rc.1-1").as_deref(), Some("0.2.0-rc.1"));
        assert_eq!(p("0.5.0-2.fc40").as_deref(), Some("0.5.0"));
        // AUR -git packages: the release the snapshot follows.
        assert_eq!(p("0.3.0.r47.g47f9306-1").as_deref(), Some("0.3.0"));
        // Plain versions (rpm %{VERSION}, Flatpak) pass through.
        assert_eq!(p("0.4.0").as_deref(), Some("0.4.0"));
        assert_eq!(p("0.5").as_deref(), Some("0.5.0"));
        // A version inside text, as `--version` prints it.
        assert_eq!(p("PhotoCraft 0.5.0 (abc, 2026)").as_deref(), Some("0.5.0"));
        assert_eq!(p("junk"), None);
        assert_eq!(p("garbage"), None);
        // The bug this guards: 0.5.0-1 used to read as a pre-release, so an installed .deb of the
        // latest release always looked out of date.
        assert!(!is_update(&v("0.5.0"), &parse_package("0.5.0-1").unwrap()));
    }

    #[test]
    fn dbeaver_hotfixes_keep_numeric_precedence_and_original_label() {
        let base = parse_dbeaver("26.2.5").unwrap();
        let first = parse_dbeaver("26.2.5.2").unwrap();
        let second = parse_dbeaver("26.2.5.10").unwrap();
        assert!(is_update(&first, &base));
        assert!(is_update(&second, &first));
        assert!(is_update(&v("26.2.6"), &second));
        assert!(!is_update(&first, &first));
        assert_eq!(label(&first), "26.2.5.2");
        assert_eq!(label(&v("26.2.5+other")), "26.2.5+other");
        for invalid in ["26.2.5.0", "26.2.5.02", "26.2.5.2.1", "v26.2.5.2", "26.2.5-rc", "26.2.5.+2"] {
            assert!(parse_dbeaver(invalid).is_none(), "{invalid}");
        }
        assert!(parse("26.2.5.2").is_none());
        assert!(parse("26.2.5+dbeaver-hotfix.2").is_none());
        assert!(parse("26.2+dbeaver-hotfix.2").is_none());
    }

    #[test]
    fn orders_versions_and_spots_updates() {
        assert!(is_newer(&v("0.10.0"), &v("0.9.9")));
        assert!(is_newer(&v("0.2.0"), &v("0.2.0-rc.5")));
        assert!(!is_newer(&v("0.2.0+abc"), &v("0.2.0")));
        assert!(is_update(&v("0.5.0"), &v("0.4.1")));
        assert!(!is_update(&v("0.5.0"), &v("0.5.0")));
        assert!(!is_update(&v("0.5.0"), &v("0.6.0")));
        // An MSI that recorded 0.3.0 for 0.3.0-rc.2 is not out of date against 0.3.0-rc.2.
        assert!(!is_update(&v("0.3.0-rc.2"), &v("0.3.0")));
        assert!(is_stable(&v("1.0.0")) && !is_stable(&v("1.0.0-beta.1")));
    }
}
