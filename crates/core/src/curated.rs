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
        assert!(!super::HANDY.runs_on(crate::Os::Windows));
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
