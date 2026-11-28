use artcraft_launcher_core::release::{Release, krita_mac_release, krita_mac_release_from_download};

use crate::http::Http;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_krita_downloads()?.ok_or("The Krita download page is unavailable")?;
    let html = std::str::from_utf8(&bytes).map_err(|_| "The Krita download page is unreadable")?;
    let latest = advertised_mac_release(html)?;
    let previous = ["5.3.5", "5.3.4.1"]
        .into_iter()
        .filter_map(krita_mac_release)
        .filter(|release| !latest_only && release.version < latest.version && !release.assets.is_empty())
        .collect::<Vec<_>>();
    Ok(std::iter::once(latest).chain(previous).collect())
}

fn advertised_mac_release(html: &str) -> Result<Release, String> {
    let mut releases = html.split(" href=").skip(1).filter_map(|link| {
        let address = match link.chars().next()? {
            quote @ ('"' | '\'') => link[1..].split_once(quote)?.0,
            _ => link.split(|character: char| character.is_ascii_whitespace() || character == '>').next()?,
        };
        krita_mac_release_from_download(address)
    });
    let current = releases.next().ok_or("Krita does not advertise a stable macOS installer")?;
    if releases.any(|release| release.tag != current.tag) {
        return Err("Krita advertises multiple stable macOS installers".into());
    }
    Ok(current)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_page_accepts_same_mac_link_twice_and_rejects_ambiguous_sources() {
        let html = r#"<a href="https://download.kde.org/stable/krita/5.3.5/krita-5.3.5-signed.dmg">Mac</a>
<a href=https://download.kde.org/stable/krita/5.3.5/krita-5.3.5-signed.dmg>Mac again</a>
<a href="https://download.kde.org/stable/krita/5.3.5/krita-5.3.5-x86_64.AppImage">Linux</a>
<a href="https://example.org/stable/krita/9.0.0/krita-9.0.0-signed.dmg">Other site</a>"#;
        let current = advertised_mac_release(html).unwrap();
        assert_eq!(current.tag, "5.3.5");
        assert_eq!(current.assets.len(), 1);
        let next = html.replace("5.3.5", "5.3.6");
        assert!(advertised_mac_release(&next).unwrap().assets.is_empty());
        assert!(advertised_mac_release(&format!("{html}{next}")).is_err());
        assert!(advertised_mac_release(&html.replace("https:", "http:")).is_err());
    }
}
