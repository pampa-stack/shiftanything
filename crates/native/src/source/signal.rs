use artcraft_launcher_core::release::{Release, signal_mac_release};

use crate::http::Http;

pub(super) fn releases(http: &Http) -> Result<Vec<Release>, String> {
    let bytes = http.get_signal_mac_feed()?.ok_or("The stable Signal Desktop feed is unavailable")?;
    let manifest = std::str::from_utf8(&bytes).map_err(|_| "The stable Signal Desktop feed is unreadable")?;
    let latest = advertised_release(manifest).ok_or("Signal Desktop does not advertise a stable macOS DMG")?;
    let latest_version = latest.version.clone();
    let history = ["8.36.0", "8.35.0"].into_iter().filter_map(signal_mac_release).filter(|older| older.version < latest_version);
    Ok(std::iter::once(latest).chain(history).collect())
}

// The feed identifies the current version and DMG name. The reviewed CDN bytes and
// the notarized bundle determine the asset because the 8.36.0 feed reports a different size.
fn advertised_release(manifest: &str) -> Option<Release> {
    let mut versions = manifest.lines().filter_map(|line| line.strip_prefix("version: "));
    let tag = versions.next()?;
    if versions.next().is_some() {
        return None;
    }
    let release = signal_mac_release(tag)?;
    let name = format!("  - url: signal-desktop-mac-universal-{tag}.dmg");
    manifest.lines().any(|line| line == name).then_some(release)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_feed_names_current_dmg_and_cannot_enable_unreviewed_downloads() {
        let current = "version: 8.36.0\nfiles:\n  - url: signal-desktop-mac-universal-8.36.0.dmg\n    size: 269301286\n";
        let release = advertised_release(current).unwrap();
        assert_eq!(release.tag, "8.36.0");
        assert_eq!(release.assets[0].size, 269_303_849);

        let next = current.replace("8.36.0", "8.37.0");
        assert!(advertised_release(&next).unwrap().assets.is_empty());
        assert!(advertised_release(&current.replace("-universal-", "-arm64-")).is_none());
        assert!(advertised_release(&current.replace("8.36.0", "8.36.0-beta.1")).is_none());
        assert!(advertised_release(&format!("{current}version: 8.35.0\n")).is_none());
    }
}
