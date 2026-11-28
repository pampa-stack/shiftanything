use artcraft_launcher_core::release::{ITSYCAL_APPCAST_DOWNLOADS, Release, itsycal_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let version = crate::install::current_macos_major_minor().ok_or("Could not determine the macOS version for Itsycal")?;
    let bytes = http.get_itsycal_appcast()?.ok_or("The Itsycal appcast is unavailable")?;
    let xml = std::str::from_utf8(&bytes).map_err(|_| "The Itsycal appcast is unreadable")?;
    advertised_releases(xml, version, latest_only)
}

fn advertised_releases(xml: &str, version: (u16, u16), latest_only: bool) -> Result<Vec<Release>, String> {
    let latest = xml
        .split("<item>")
        .skip(1)
        .filter_map(|entry| compatible_release(entry, version))
        .max_by(|left, right| left.version.cmp(&right.version))
        .ok_or("Itsycal does not advertise a stable release for this macOS version")?;

    let latest_version = latest.version.clone();
    let history = ["0.15.15", "0.15.14"].into_iter().filter_map(itsycal_mac_release).filter(|older| !latest_only && older.version < latest_version);
    let releases = std::iter::once(latest).chain(history).collect();
    Ok(releases)
}

fn compatible_release(entry: &str, version: (u16, u16)) -> Option<Release> {
    let (entry, _) = entry.split_once("</item>")?;
    let (_, minimum) = entry.split_once("<sparkle:minimumSystemVersion>")?;
    let (minimum, _) = minimum.split_once("</sparkle:minimumSystemVersion>")?;
    let (major, minor) = minimum.trim().split_once('.')?;
    let minimum = (major.parse::<u16>().ok()?, minor.parse::<u16>().ok()?);
    if minimum < (11, 0) || minimum > version {
        return None;
    }

    let (_, enclosure) = entry.split_once("<enclosure")?;
    if !enclosure.starts_with(char::is_whitespace) {
        return None;
    }
    let (attributes, _) = enclosure.split_once('>')?;
    let tag = super::appcast_attribute(attributes, "sparkle:shortVersionString")?;
    let mut release = itsycal_mac_release(tag)?;
    let url = super::appcast_attribute(attributes, "url")?;
    let length = super::appcast_attribute(attributes, "length")?.parse::<u64>().ok()?;
    if length == 0 || url.strip_prefix(ITSYCAL_APPCAST_DOWNLOADS) != Some(format!("Itsycal-{tag}.zip").as_str()) {
        return None;
    }
    release.assets.retain(|asset| asset.size == length && minimum == (11, 0));
    Some(release)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_current_version_for_the_os_and_only_offers_reviewed_packages() {
        let item = |tag: &str, minimum: &str, size: u64| {
            format!(
                r#"<item><sparkle:minimumSystemVersion>{minimum}</sparkle:minimumSystemVersion>
<enclosure url="{ITSYCAL_APPCAST_DOWNLOADS}Itsycal-{tag}.zip" sparkle:shortVersionString="{tag}" length="{size}" /></item>"#
            )
        };
        let previous_macos = item("0.15.15", "11.0", 1_074_708);
        let future_macos = item("0.29.0", "27.0", 1_905_120);
        let appcast = format!("{future_macos}{previous_macos}");
        let legacy = advertised_releases(&appcast, (26, 4), false).unwrap();
        assert_eq!(legacy.iter().map(|release| release.tag.as_str()).collect::<Vec<_>>(), ["0.15.15", "0.15.14"]);
        assert!(legacy.iter().all(|release| release.assets.len() == 1));
        let current = advertised_releases(&appcast, (27, 0), true).unwrap();
        assert_eq!(current[0].tag, "0.29.0");
        assert!(current[0].assets.is_empty());
        assert!(advertised_releases(&appcast, (10, 15), true).is_err());
        for rejected in [
            previous_macos.replace("s3.amazonaws.com", "untrusted.test"),
            previous_macos.replace("Itsycal-0.15.15.zip", "Itsycal-0.15.14.zip"),
            item("0.15.16-rc.1", "11.0", 1_074_708),
        ] {
            assert!(advertised_releases(&rejected, (26, 4), true).is_err());
        }
        for unreviewed in [item("0.15.15", "11.0", 1), item("0.15.15", "12.0", 1_074_708)] {
            assert!(advertised_releases(&unreviewed, (26, 4), true).unwrap()[0].assets.is_empty());
        }
    }
}
