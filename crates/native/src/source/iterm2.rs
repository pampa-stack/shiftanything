use artcraft_launcher_core::release::{ITERM2_ZIP_URL, Release, iterm2_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http) -> Result<Vec<Release>, String> {
    let bytes = http.get_iterm2_appcast()?.ok_or("The stable iTerm2 appcast is unavailable")?;
    let xml = std::str::from_utf8(&bytes).map_err(|_| "The stable iTerm2 appcast is unreadable")?;
    advertised_release(xml).map(|latest| vec![latest])
}

fn advertised_release(xml: &str) -> Result<Release, String> {
    xml.split("<enclosure")
        .skip(1)
        .filter_map(|enclosure| {
            if !enclosure.starts_with(char::is_whitespace) {
                return None;
            }
            let (attributes, _) = enclosure.split_once('>')?;
            let value = |name| super::appcast_attribute(attributes, name);
            let tag = value("sparkle:version")?;
            let mut release = iterm2_mac_release(tag)?;
            let url = value("url")?;
            let size: u64 = value("length")?.parse().ok()?;
            let expected_name = format!("iTerm2-{}.zip", tag.replace('.', "_"));
            if size == 0 || url.strip_prefix(ITERM2_ZIP_URL) != Some(expected_name.as_str()) {
                return None;
            }
            release.assets.retain(|asset| asset.size == size);
            Some(release)
        })
        .max_by(|left, right| left.version.cmp(&right.version))
        .ok_or_else(|| "iTerm2 does not advertise a supported stable package".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modern_feed_uses_only_canonical_versions_and_reviewed_bytes() {
        let enclosure = |tag: &str, size: u64| {
            format!(r#"<enclosure url="{ITERM2_ZIP_URL}iTerm2-{}.zip" sparkle:version="{tag}" length="{size}"/>"#, tag.replace('.', "_"))
        };
        let current = enclosure("3.7.9", 78_118_306);
        assert_eq!(advertised_release(&current).unwrap().assets.len(), 1);
        assert!(advertised_release(&enclosure("3.7.8", 78_120_806)).is_err());
        assert!(advertised_release(&enclosure("3.7.9", 1)).unwrap().assets.is_empty());
        assert!(advertised_release(&current.replace("iterm2.com/", "other.test/")).is_err());
        assert!(advertised_release(&current.replace("iTerm2-3_7_9", "iTerm2-3_7_8")).is_err());
        let newer = format!("{current}{}", enclosure("3.7.10", 78_118_307));
        let latest = advertised_release(&newer).unwrap();
        assert_eq!(latest.tag, "3.7.10");
        assert!(latest.assets.is_empty());
    }
}
