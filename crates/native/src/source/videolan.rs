use artcraft_launcher_core::release::{Release, vlc_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_vlc_stable_index()?.ok_or("The VideoLAN stable macOS directory is unavailable")?;
    let html = std::str::from_utf8(&bytes).map_err(|_| "The VideoLAN stable macOS directory is unreadable")?;
    let latest = advertised_mac_release(html)?;
    let previous = ["3.0.24", "3.0.23"]
        .into_iter()
        .filter_map(vlc_mac_release)
        .filter(|release| !latest_only && release.version < latest.version && !release.assets.is_empty())
        .collect::<Vec<_>>();
    Ok(std::iter::once(latest).chain(previous).collect())
}

fn advertised_mac_release(html: &str) -> Result<Release, String> {
    let mut releases = html.split("<a href=\"").skip(1).filter_map(|link| {
        let filename = link.split_once('"')?.0;
        vlc_mac_release(filename.strip_prefix("vlc-")?.strip_suffix("-arm64.dmg")?)
    });
    let current = releases.next().ok_or("VideoLAN does not list a stable Apple Silicon installer")?;
    if releases.any(|release| release.tag != current.tag) {
        return Err("VideoLAN lists multiple releases as the current macOS installer".into());
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_directory_requires_an_exact_arm64_dmg() {
        let html = r#"<a href="vlc-3.0.24-arm64.dmg">Mac</a>
<a href="vlc-3.0.24-arm64.dmg.sha256">Checksum</a>
<a href="vlc-3.0.25-intel64.dmg">Intel</a>
<a href="vlc-3.0.25-beta.1-arm64.dmg">Beta</a>"#;
        let current = advertised_mac_release(html).unwrap();
        assert_eq!(current.tag, "3.0.24");
        assert_eq!(current.assets.len(), 1);
        let next = html.replace("3.0.24", "3.0.25");
        assert!(advertised_mac_release(&next).unwrap().assets.is_empty());
        assert!(advertised_mac_release(&format!("{html}{next}")).is_err());
        assert!(advertised_mac_release(&html.replace("vlc-3.0.24", "../vlc-3.0.24")).is_err());
    }
}
