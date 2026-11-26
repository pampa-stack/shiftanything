//! Editorial metadata only. Presence here never grants an installation capability.
use serde::Deserialize;
use std::sync::OnceLock;
#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub author: String,
    pub level: String,
    pub delivery: String,
    pub category: String,
    pub description: Description,
    pub alternatives: Vec<String>,
    pub license: String,
    pub repo: String,
    pub website: String,
    pub logo: String,
    pub screenshots: Vec<String>,
    pub license_url: String,
    pub media_source: serde_json::Value,
    pub agent: Agent,
    pub release: Option<Release>,
    pub stars: Option<Stars>,
    pub popularity: Option<Popularity>,
    #[serde(default)]
    pub platform_support: std::collections::BTreeMap<String, String>,
    /// Only a complete, documented platform scope can establish incompatibility.
    #[serde(default)]
    pub platform_scope_complete: bool,
    #[serde(default)]
    pub distribution: Vec<Distribution>,
}
/// Documented channels only; these never grant a native installation capability.
#[derive(Debug, Deserialize)]
pub struct Distribution {
    pub platform: String,
    pub manager: String,
    pub id: String,
    pub source: String,
    pub download_size: Option<u64>,
    pub download_size_source: Option<String>,
    pub download_size_checked: Option<String>,
}
pub struct DownloadSize<'a> {
    pub platform: &'a str,
    pub minimum: u64,
    pub maximum: Option<u64>,
    pub manager: Option<&'a str>,
    pub source: &'a str,
}
impl App {
    pub fn explicitly_unsupported_on(&self, os: crate::Os) -> bool {
        let key = match os {
            crate::Os::MacOs => "macos",
            crate::Os::Windows => "windows",
            crate::Os::Linux => "linux",
            crate::Os::FreeBsd => "freebsd",
        };
        self.platform_scope_complete && self.platform_source(key).is_none()
    }
    pub fn platform_source(&self, platform: &str) -> Option<&str> {
        self.release
            .as_ref()
            .filter(|release| release.packages.iter().any(|package| package.platform == platform))
            .map(|release| release.source.as_str())
            .or_else(|| self.distribution.iter().find(|channel| channel.platform == platform).map(|channel| channel.source.as_str()))
            .or_else(|| self.platform_support.get(platform).map(String::as_str))
    }
    /// Unknown platform uses the first documented size and explicitly retains its OS/channel.
    pub fn download_size(&self, platform: Option<&str>) -> Option<DownloadSize<'_>> {
        if let Some(release) = &self.release
            && let Some(package) =
                release.packages.iter().find(|package| platform.is_none_or(|os| os == package.platform) && package.size_min.is_some())
        {
            return Some(DownloadSize {
                platform: &package.platform,
                minimum: package.size_min?,
                maximum: package.size_max,
                manager: None,
                source: &release.source,
            });
        }
        let channel = self.distribution.iter().find(|channel| platform.is_none_or(|os| os == channel.platform) && channel.download_size.is_some())?;
        Some(DownloadSize {
            platform: &channel.platform,
            minimum: channel.download_size?,
            maximum: channel.download_size,
            manager: Some(&channel.manager),
            source: channel.download_size_source.as_deref().unwrap_or(&channel.source),
        })
    }
}
#[derive(Debug, Deserialize)]
pub struct Description {
    pub en: String,
    pub it: String,
}
#[derive(Debug, Deserialize)]
pub struct Agent {
    pub kind: String,
    pub source: Option<String>,
}
#[derive(Debug, Deserialize)]
pub struct Release {
    pub version: String,
    pub date: String,
    pub source: String,
    #[serde(default)]
    pub packages: Vec<Package>,
}
#[derive(Debug, Deserialize)]
pub struct Package {
    pub platform: String,
    pub size_min: Option<u64>,
    pub size_max: Option<u64>,
}
#[derive(Debug, Deserialize)]
pub struct Stars {
    pub count: u64,
    pub date: String,
    pub source: String,
}
#[derive(Debug, Deserialize)]
pub struct Popularity {
    pub kind: String,
    pub count: u64,
    pub date: String,
    pub source: String,
    pub page: String,
}
impl Popularity {
    pub fn label(&self) -> String {
        let count = self.count as f64;
        if self.count >= 1_000_000 {
            format!("{:.1}M Flathub", count / 1_000_000.0)
        } else if self.count >= 1_000 {
            format!("{:.1}K Flathub", count / 1_000.0)
        } else {
            format!("{} Flathub", self.count)
        }
    }
}
pub fn apps() -> &'static [App] {
    static APPS: OnceLock<Vec<App>> = OnceLock::new();
    APPS.get_or_init(|| serde_json::from_str(include_str!("../curated.json")).unwrap_or_default())
}
#[cfg(test)]
mod tests {
    #[test]
    fn only_explicit_complete_platform_evidence_can_disable_install_as_incompatible() {
        let apps = super::apps();
        let timeshift = apps.iter().find(|app| app.id == "timeshift").unwrap();
        assert!(timeshift.explicitly_unsupported_on(crate::Os::MacOs));
        assert!(timeshift.explicitly_unsupported_on(crate::Os::Windows));
        assert!(!timeshift.explicitly_unsupported_on(crate::Os::Linux));
        for id in ["handy", "restic", "kopia", "plane"] {
            let app = apps.iter().find(|app| app.id == id).unwrap();
            for os in [crate::Os::MacOs, crate::Os::Windows, crate::Os::Linux] {
                assert!(!app.explicitly_unsupported_on(os), "missing recipes or evidence are not incompatibility: {id}");
            }
        }
    }

    #[test]
    fn distribution_metadata_supplies_platforms_and_sizes_without_granting_a_recipe() {
        let app = super::apps()
            .iter()
            .find(|app| {
                app.release.as_ref().is_none_or(|release| release.packages.is_empty())
                    && app.distribution.iter().any(|channel| channel.download_size.is_some())
                    && crate::CraftApp::by_id(&app.id).is_none()
            })
            .unwrap();
        let channel = app.distribution.iter().find(|channel| channel.download_size.is_some()).unwrap();
        assert_eq!(app.platform_source(&channel.platform), Some(channel.source.as_str()));
        let size = app.download_size(Some(&channel.platform)).unwrap();
        assert_eq!(Some(size.minimum), channel.download_size);
        assert_eq!(size.manager, Some(channel.manager.as_str()));
        assert!(app.download_size(None).is_some());
        assert!(app.download_size(Some("unsupported-os")).is_none());
        assert!(app.platform_source("unsupported-os").is_none());
        let request =
            crate::automation::Request { id: "metadata-only".into(), operation: crate::automation::Operation::Install, apps: vec![app.id.clone()] };
        assert!(request.validate().is_err());
    }

    #[test]
    fn curated_catalog_is_valid_and_excludes_deferred_products() {
        let apps = serde_json::from_str::<Vec<super::App>>(include_str!("../curated.json")).expect("valid editorial data");
        assert!(!apps.is_empty());
        let mut ids = std::collections::HashSet::new();
        for app in apps {
            assert!(ids.insert(app.id.clone()), "duplicate catalog identity");
            assert!(!["prompticraft", "eyecraft", "oneshotanything"].contains(&app.id.as_str()));
        }
    }
}

/// Native recipe identity. Asset selection, integrity and signature verification remain native.
pub const HANDY: crate::CraftApp = crate::CraftApp {
    id: "handy",
    name: "Handy",
    tagline: "Local speech to text",
    repo: "cjpais/Handy",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [230, 157, 183],
    ink_light: [120, 40, 70],
    ink_dark: [240, 180, 205],
    kind: crate::catalog::AppKind::Community,
    publisher: "CJ Pais",
    msi_publisher: None,
    bundle: Some("com.pais.handy"),
    site: Some("https://handy.computer"),
    mac_bundle_name: Some("Handy"),
    mac_team: Some("UWFLB4GC25"),
};

const fn native_apps() -> [crate::CraftApp; crate::SUITE.len() + 1] {
    let mut result = [HANDY; crate::SUITE.len() + 1];
    let mut i = 0;
    while i < crate::SUITE.len() {
        result[i] = crate::SUITE[i];
        i += 1;
    }
    result
}
pub const NATIVE_APPS: [crate::CraftApp; crate::SUITE.len() + 1] = native_apps();

#[cfg(test)]
mod recipe_tests {
    #[test]
    fn upstream_handy_targets_stay_supported_without_enabling_unverified_recipes() {
        let assets = ["Handy_0.10.2_aarch64.dmg", "Handy_0.10.2_x64.dmg", "Handy_0.10.2_x64_en-US.msi"];
        let json = serde_json::json!({"tag_name":"v0.10.2", "assets":assets.map(|name| serde_json::json!({
            "name":name, "browser_download_url":format!("https://github.com/cjpais/Handy/releases/download/v0.10.2/{name}")
        }))})
        .to_string();
        let release = crate::release::parse_github_release(&json, "cjpais/Handy").unwrap();
        for os in [crate::Os::MacOs, crate::Os::Windows, crate::Os::Linux] {
            assert!(super::HANDY.runs_on(os));
            for arch in [crate::Arch::X64, crate::Arch::Arm64] {
                let platform = crate::Platform { os, arch };
                let chosen = crate::platform::pick_asset(&release, &["handy"], platform, platform.download_kinds());
                assert_eq!(chosen.is_some(), os == crate::Os::MacOs && arch == crate::Arch::Arm64);
            }
        }
    }

    #[test]
    fn handy_recipe_pins_exact_official_asset_and_only_admits_verified_host() {
        let json = serde_json::json!({"tag_name":"v0.10.2", "assets":[{"name":"Handy_0.10.2_aarch64.dmg", "browser_download_url":"https://github.com/cjpais/Handy/releases/download/v0.10.2/Handy_0.10.2_aarch64.dmg"}]}).to_string();
        let release = crate::release::parse_github_release(&json, "cjpais/Handy").expect("official package");
        assert!(release.assets[0].sha256.is_some());
        let arm = crate::Platform { os: crate::Os::MacOs, arch: crate::Arch::Arm64 };
        assert!(crate::platform::pick_asset(&release, &["handy"], arm, &[crate::PackageKind::Dmg]).is_some());
        let intel = crate::Platform { os: crate::Os::MacOs, arch: crate::Arch::X64 };
        assert!(crate::platform::pick_asset(&release, &["handy"], intel, &[crate::PackageKind::Dmg]).is_none());
        let other = crate::release::parse_github_release(&json, "untrusted/Handy").expect("valid response with rejected asset");
        assert!(other.assets.is_empty());
        assert!(super::HANDY.runs_on(crate::Os::Windows));
        assert!(super::HANDY.runs_on(crate::Os::Linux));
        assert_eq!(crate::CraftApp::by_id("handy").map(|app| app.repo), Some("cjpais/Handy"));
    }
}

/// Refuse image allocations above the same budget used for native screenshots.
pub fn media_within_budget(bytes: &[u8]) -> bool {
    if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
        let Some(header) = bytes.get(8..24) else { return false };
        if header[..8] != [0, 0, 0, 13, b'I', b'H', b'D', b'R'] {
            return false;
        }
        let width = u32::from_be_bytes([header[8], header[9], header[10], header[11]]);
        let height = u32::from_be_bytes([header[12], header[13], header[14], header[15]]);
        width > 0 && height > 0 && u64::from(width) * u64::from(height) <= crate::details::MAX_SCREENSHOT_PIXELS
    } else {
        crate::details::screenshot_within_budget(bytes)
    }
}

#[cfg(test)]
mod media_tests {
    #[test]
    fn png_dimension_budget_precedes_decoding() {
        let png = |w: u32, h: u32| [b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR".as_slice(), &w.to_be_bytes(), &h.to_be_bytes()].concat();
        assert!(super::media_within_budget(&png(1200, 750)));
        assert!(!super::media_within_budget(&png(u32::MAX, u32::MAX)));
        assert!(!super::media_within_budget(&png(0, 750)));
        assert!(!super::media_within_budget(b"\x89PNG\r\n\x1a\n"));
        assert!(!super::media_within_budget(b"<svg>"));
    }
}
