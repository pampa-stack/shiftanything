//! Editorial metadata only. Presence here never grants an installation capability.
use serde::Deserialize;
use std::sync::OnceLock;
#[derive(Debug, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub author: String,
    pub level: String,
    pub category: String,
    pub description: Description,
    pub alternatives: Vec<String>,
    pub license: String,
    pub repo: String,
    pub website: String,
    pub agent: Agent,
    pub release: Option<Release>,
    pub stars: Option<Stars>,
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
