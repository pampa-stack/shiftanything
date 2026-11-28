use artcraft_launcher_core::release::{Release, kdenlive_mac_release};

use crate::http::Http;

const LINK_PREFIX: &str = "<link>https://apps.kde.org/kdenlive/#";

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_kdenlive_releases()?.ok_or("The KDE release feed is unavailable")?;
    let xml = std::str::from_utf8(&bytes).map_err(|_| "The KDE release feed is unreadable")?;
    parse_releases(xml, latest_only)
}

fn parse_releases(xml: &str, latest_only: bool) -> Result<Vec<Release>, String> {
    let mut releases = xml
        .split("<item>")
        .skip(1)
        .filter_map(|item| {
            let (body, _) = item.split_once("</item>")?;
            let (_, link) = body.split_once(LINK_PREFIX)?;
            let (tag, _) = link.split_once("</link>")?;
            kdenlive_mac_release(tag)
        })
        .collect::<Vec<_>>();
    releases.sort_by(|left, right| right.version.cmp(&left.version));
    releases.dedup_by(|left, right| left.tag == right.tag);
    let mut releases = releases.into_iter();
    // Display the current release even before its Mac asset is qualified.
    let latest = releases.next().ok_or("KDE does not advertise a stable Kdenlive release")?;
    let history = releases.filter(|release| !latest_only && !release.assets.is_empty());
    Ok(std::iter::once(latest).chain(history).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feed_accepts_only_kde_stable_calendar_versions_and_known_assets() {
        let xml = r#"<item><link>https://apps.kde.org/kdenlive/#26.08.2</link></item>
<item><link>https://other.test/kdenlive/#26.12.9</link></item>
<item><link>https://apps.kde.org/kdenlive/#26.09.0-beta.1</link></item>
<item><link>https://apps.kde.org/kdenlive/#26.08.4</link></item>
<item><link>https://apps.kde.org/kdenlive/#26.08.3</link></item>"#;
        let releases = parse_releases(xml, false).unwrap();
        assert_eq!(releases.iter().map(|rel| rel.tag.as_str()).collect::<Vec<_>>(), ["26.08.4", "26.08.3", "26.08.2"]);
        assert!(releases[0].assets.is_empty());
        assert_eq!(releases[1].version.as_ref().unwrap().to_string(), "26.8.3");
        assert_eq!(releases[1].assets.len(), 1);
        assert_eq!(parse_releases(xml, true).unwrap().len(), 1);
        assert!(parse_releases(&xml.replace("apps.kde.org", "other.test"), false).is_err());
    }
}
