use artcraft_launcher_core::release::{INKSCAPE_DOWNLOADS_URL, Release, inkscape_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let latest = http
        .redirect_target(INKSCAPE_DOWNLOADS_URL)?
        .as_deref()
        .and_then(advertised_mac_release)
        .ok_or("Inkscape does not advertise a stable macOS release")?;
    let previous = ["1.4.4", "1.4.3"]
        .into_iter()
        .filter_map(inkscape_mac_release)
        .filter(|release| !latest_only && release.version < latest.version && !release.assets.is_empty());
    let revisions = previous.collect::<Vec<_>>();
    Ok(std::iter::once(latest).chain(revisions).collect())
}

fn advertised_mac_release(location: &str) -> Option<Release> {
    let path = location.strip_prefix("https://inkscape.org").unwrap_or(location);
    inkscape_mac_release(path.strip_prefix("/release/inkscape-")?.strip_suffix('/')?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_redirect_reads_only_canonical_same_origin_release_links() {
        let current = advertised_mac_release("/release/inkscape-1.4.4/").unwrap();
        assert_eq!(current.tag, "1.4.4");
        assert_eq!(current.assets.len(), 1);
        assert!(advertised_mac_release("https://inkscape.org/release/inkscape-1.4.5/").unwrap().assets.is_empty());
        assert!(advertised_mac_release("https://other.example/release/inkscape-1.4.4/").is_none());
        assert!(advertised_mac_release("/release/inkscape-../1.4.4/").is_none());
        assert!(advertised_mac_release("/release/inkscape-1.4.4-beta.1/").is_none());
    }
}
