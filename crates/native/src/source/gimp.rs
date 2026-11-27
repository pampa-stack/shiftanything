use std::collections::BTreeMap;

use artcraft_launcher_core::release::{Release, gimp_release};
use artcraft_launcher_core::version::Version;
use serde::Deserialize;

use crate::http::Http;

#[derive(Deserialize)]
struct Index {
    #[serde(rename = "STABLE")]
    stable: Vec<Stable>,
}

#[derive(Deserialize)]
struct Stable {
    version: String,
    #[serde(default)]
    macos: Vec<Package>,
}

#[derive(Deserialize)]
struct Package {
    filename: String,
    sha256: Option<String>,
}

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let bytes = http.get_gimp_index()?.ok_or("The GIMP stable version index is unavailable")?;
    stable_arm64(&bytes, latest_only)
}

fn stable_arm64(bytes: &[u8], latest_only: bool) -> Result<Vec<Release>, String> {
    let index = serde_json::from_slice::<Index>(bytes).map_err(|_| "The GIMP stable version index is unreadable")?;
    let mut versions = BTreeMap::new();
    for stable in index.stable {
        let Some(version) = Version::parse(&stable.version)
            .ok()
            .filter(|version| version.pre.is_empty() && version.build.is_empty() && version.to_string() == stable.version)
        else {
            continue;
        };
        let name = format!("gimp-{version}-arm64.dmg");
        let mut packages = stable.macos.iter().filter(|asset| asset.filename == name);
        let Some(package) = packages.next() else { continue };
        if packages.next().is_some() {
            return Err(format!("Multiple GIMP downloads are published for Apple Silicon {version}"));
        }
        let mut release = gimp_release(version.clone());
        release.assets.retain(|asset| asset.sha256 == package.sha256);
        if versions.insert(version, release).is_some() {
            return Err("The GIMP stable index lists an Apple Silicon version twice".into());
        }
    }
    let mut tags = versions.into_values().rev();
    let latest = tags.next().ok_or("The GIMP stable index contains no recognized Apple Silicon releases")?;
    let mut releases = vec![latest];
    if !latest_only {
        releases.extend(tags.filter(|release| !release.assets.is_empty()));
    }
    Ok(releases)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_index_limits_assets_to_reviewed_digests_and_exact_arm64_names() {
        let index = serde_json::json!({
            "STABLE": [
                {"version": "3.2.6", "macos": [{"filename": "gimp-3.2.6-arm64.dmg", "sha256": "854573aec2be6a185aa021109c3c0ca94f210379204b704231bdd191b4060294"}]},
                {"version": "3.2.8", "macos": [{"filename": "gimp-3.2.8-arm64.dmg", "sha256": "unexpected"}]},
                {"version": "3.2.9", "macos": [{"filename": "gimp-3.2.9-x86_64.dmg"}]},
                {"version": "3.3.0-rc.1", "macos": [{"filename": "gimp-3.3.0-rc.1-arm64.dmg"}]}
            ],
            "DEVELOPMENT": [{"version": "3.3.1", "macos": [{"filename": "gimp-3.3.1-arm64.dmg"}]}]
        });
        let versions = stable_arm64(&serde_json::to_vec(&index).unwrap(), false).unwrap();
        assert_eq!(versions.iter().map(|release| release.tag.as_str()).collect::<Vec<_>>(), ["3.2.8", "3.2.6"]);
        assert!(versions[0].assets.is_empty());
        assert_eq!(versions[1].assets.len(), 1);
    }
}
