use std::collections::BTreeSet;

use artcraft_launcher_core::release::{Release, blender_release};
use artcraft_launcher_core::version::Version;

use crate::http::Http;

const SERIES_LIMIT: usize = 2;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let base = index(http, None)?;
    let mut releases = Vec::new();
    for series in stable_series(&base).into_iter().rev().take(SERIES_LIMIT) {
        let directory = index(http, Some(series))?;
        for version in arm64_versions(&directory, series).into_iter().rev() {
            let release = blender_release(version);
            if latest_only {
                return Ok(vec![release]);
            }
            if releases.is_empty() || !release.assets.is_empty() {
                releases.push(release);
            }
        }
    }
    if releases.is_empty() {
        return Err("The Blender archive contains no recognized Apple Silicon releases".into());
    }
    Ok(releases)
}

fn index(http: &Http, series: Option<(u64, u64)>) -> Result<String, String> {
    let bytes = http.get_blender_index(series)?.ok_or("The Blender release archive is unavailable")?;
    String::from_utf8(bytes).map_err(|_| "The Blender release archive is unreadable".into())
}

fn targets(index: &str) -> impl Iterator<Item = &str> {
    index.split("<a href=\"").skip(1).filter_map(|link| link.split_once('"').map(|(target, _)| target))
}

fn stable_series(index: &str) -> BTreeSet<(u64, u64)> {
    targets(index)
        .filter_map(|target| {
            let series = target.strip_prefix("Blender")?.strip_suffix('/')?;
            let (major, minor) = series.split_once('.')?;
            let (major, minor) = (major.parse::<u64>().ok()?, minor.parse::<u64>().ok()?);
            (format!("{major}.{minor}") == series).then_some((major, minor))
        })
        .collect()
}

fn arm64_versions(index: &str, series: (u64, u64)) -> BTreeSet<Version> {
    targets(index)
        .filter_map(|target| {
            let raw = target.strip_prefix("blender-")?.strip_suffix("-macos-arm64.dmg")?;
            let version = Version::parse(raw).ok()?;
            (version.pre.is_empty() && version.build.is_empty() && version.to_string() == raw && (version.major, version.minor) == series)
                .then_some(version)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn archive_series_are_canonical_and_sorted_as_numbers() {
        let html = r#"<a href="Blender5.2/">5.2</a><a href="Blender5.10/">5.10</a><a href="Blender5.2/">duplicate</a>
<a href="Blender05.3/">leading zero</a><a href="../Blender9.2/">relative path</a><a href="Blender5.11-rc/">prerelease</a>"#;
        assert_eq!(stable_series(html).into_iter().rev().collect::<Vec<_>>(), [(5, 10), (5, 2)]);
    }

    #[test]
    fn archive_versions_require_the_stable_arm64_name_in_the_listed_series() {
        let html = r#"<a href="blender-5.3.0-macos-arm64.dmg">package</a><a href="blender-5.3.10-macos-arm64.dmg">package</a>
<a href="blender-5.3.11-rc.1-macos-arm64.dmg">prerelease</a><a href="blender-5.4.0-macos-arm64.dmg">different series</a>
<a href="blender-5.3.11-macos-x64.dmg">Intel</a><a href="../blender-5.3.11-macos-arm64.dmg">relative path</a>"#;
        assert_eq!(arm64_versions(html, (5, 3)).into_iter().rev().map(|version| version.to_string()).collect::<Vec<_>>(), ["5.3.10", "5.3.0"]);
    }
}
