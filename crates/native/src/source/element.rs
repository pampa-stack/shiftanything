use artcraft_launcher_core::release::{Release, element_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_element_index()?.ok_or("The Element macOS package index is unavailable")?;
    let html = std::str::from_utf8(&bytes).map_err(|_| "The Element macOS package index is unreadable")?;
    advertised_releases(html, latest_only)
}

fn advertised_releases(html: &str, latest_only: bool) -> Result<Vec<Release>, String> {
    let mut versions = html
        .split("<a href=\"Element-")
        .skip(1)
        .filter_map(|link| {
            let filename = link.split_once('"')?.0;
            let tag = filename.strip_suffix("-universal.dmg")?;
            let release = element_mac_release(tag)?;
            Some((release.version.clone()?, release))
        })
        .collect::<Vec<_>>();
    versions.sort_unstable_by(|(first, _), (second, _)| second.cmp(first));
    versions.dedup_by(|(first, _), (second, _)| first == second);
    let mut versions = versions.into_iter().map(|(_, release)| release);
    let latest = versions.next().ok_or("Element does not list a stable universal macOS package")?;
    Ok(std::iter::once(latest).chain(versions.filter(|release| !latest_only && !release.assets.is_empty())).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_index_sorts_stable_versions_and_limits_downloads_to_reviewed_packages() {
        let index = r#"<a href="Element-1.12.35-universal.dmg">Older</a>
<a href="Element-1.12.36-universal.dmg">Newer</a>
<a href="Element-1.12.37-rc.1-universal.dmg">Preview</a>
<a href="Element-1.12.39-x64.dmg">Intel</a>
<a href="https://example.test/Element-1.12.42-universal.dmg">External</a>
<a href="Element-1.12.43-universal.dmg.zip">Archive</a>"#;
        let releases = advertised_releases(index, false).unwrap();
        assert_eq!(releases.iter().map(|release| release.tag.as_str()).collect::<Vec<_>>(), ["1.12.36", "1.12.35"]);
        assert!(releases.iter().all(|release| release.assets.len() == 1));
        let index = format!("{index}<a href=\"Element-1.12.40-universal.dmg\">Future stable</a>");
        let latest = advertised_releases(&index, true).unwrap();
        assert_eq!(latest.len(), 1);
        assert_eq!(latest[0].tag, "1.12.40");
        assert!(latest[0].assets.is_empty());
        assert!(advertised_releases("<a href=\"Element.dmg\">Alias</a>", true).is_err());
    }
}
