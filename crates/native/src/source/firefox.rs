use artcraft_launcher_core::release::{FIREFOX_REVIEWED_TAGS, Release, firefox_mac_release};
use serde::Deserialize;

use crate::http::Http;

#[derive(Deserialize)]
struct ProductDetails {
    #[serde(rename = "LATEST_FIREFOX_VERSION")]
    latest: String,
}

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_firefox_versions()?.ok_or("The Mozilla product details are unavailable")?;
    let version = serde_json::from_slice::<ProductDetails>(&bytes).map_err(|_| "The Mozilla product details are unreadable")?;
    let latest = firefox_mac_release(&version.latest).ok_or("Mozilla does not advertise a stable Firefox version")?;
    let mut releases = vec![latest];
    if !latest_only {
        let older = FIREFOX_REVIEWED_TAGS
            .iter()
            .filter_map(|tag| firefox_mac_release(tag))
            .filter(|release| release.version < releases[0].version)
            .collect::<Vec<_>>();
        releases.extend(older);
    }
    for release in &mut releases {
        if !release.assets.is_empty() {
            let sums = http.get_firefox_sums(&release.tag)?.ok_or("The Mozilla checksum manifest is unavailable")?;
            let manifest = std::str::from_utf8(&sums).map_err(|_| "The Mozilla checksum manifest is unreadable")?;
            let digest = official_digest(&release.tag, manifest);
            release.assets.retain(|asset| asset.sha256.is_some() && asset.sha256.as_deref() == digest);
        }
    }
    Ok(releases)
}

fn official_digest<'a>(tag: &str, manifest: &'a str) -> Option<&'a str> {
    let path = format!("mac/en-US/Firefox {tag}.dmg");
    let mut matches = manifest.lines().filter_map(|line| line.split_once("  ")).filter(|(_, name)| *name == path).map(|(digest, _)| digest);
    let digest = matches.next()?;
    matches.next().is_none().then_some(digest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_firefox_resolves_only_a_unique_english_macos_digest() {
        let tag = "161.0";
        let release = firefox_mac_release(tag).unwrap();
        let digest = release.assets[0].sha256.as_deref().unwrap();
        let wrong_locale = format!("{digest}  mac/it/Firefox {tag}.dmg");
        let correct = format!("{digest}  mac/en-US/Firefox {tag}.dmg");
        assert_eq!(official_digest(tag, &format!("{wrong_locale}\n{correct}")), Some(digest));
        assert_eq!(official_digest(tag, &wrong_locale), None);
        assert_eq!(official_digest(tag, &format!("{correct}\n{correct}")), None);
        assert!(firefox_mac_release("162.0b3").is_none());
        assert!(firefox_mac_release("153.8.0esr").is_none());
        assert!(firefox_mac_release("162.0").unwrap().assets.is_empty());
    }
}
