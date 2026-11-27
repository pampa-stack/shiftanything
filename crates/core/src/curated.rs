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
        for id in ["handy", "backrest", "kopia", "plane"] {
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

pub const LOCALSEND: crate::CraftApp = crate::CraftApp {
    id: "localsend",
    name: "LocalSend",
    tagline: "Share files with nearby devices",
    repo: "localsend/localsend",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [109, 200, 174],
    ink_light: [22, 109, 87],
    ink_dark: [126, 221, 191],
    kind: crate::catalog::AppKind::Community,
    publisher: "Tien Do Nam",
    msi_publisher: None,
    bundle: Some("org.localsend.localsendApp"),
    site: Some("https://localsend.org/"),
    mac_bundle_name: Some("LocalSend"),
    mac_team: Some("3W7H4PYMCV"),
};

pub(crate) const COMMUNITY_RECIPES: [crate::CraftApp; 2] = [HANDY, LOCALSEND];

const fn native_apps() -> [crate::CraftApp; crate::SUITE.len() + COMMUNITY_RECIPES.len()] {
    let mut result = [HANDY; crate::SUITE.len() + COMMUNITY_RECIPES.len()];
    let mut i = 0;
    while i < crate::SUITE.len() {
        result[i] = crate::SUITE[i];
        i += 1;
    }
    let mut recipe = 0;
    while recipe < COMMUNITY_RECIPES.len() {
        result[i + recipe] = COMMUNITY_RECIPES[recipe];
        recipe += 1;
    }
    result
}
pub const NATIVE_APPS: [crate::CraftApp; crate::SUITE.len() + COMMUNITY_RECIPES.len()] = native_apps();

/// AppImage recipes that completed install, launch, update and removal on Linux x86_64.
/// Editorial rows and release metadata cannot add recipes to this list.
pub fn verified_linux_appimage(app: &crate::CraftApp, platform: crate::Platform) -> bool {
    if platform.os != crate::Os::Linux || platform.arch != crate::Arch::X64 {
        return false;
    }
    if app.id == LOCALSEND.id {
        return app.repo == LOCALSEND.repo;
    }
    matches!(
        (app.id, app.repo),
        ("anicraft", "storytold/anicraft")
            | ("cadcraft", "storytold/cadcraft")
            | ("deckcraft", "storytold/deckcraft")
            | ("effectcraft", "storytold/effectcraft")
            | ("figcraft", "storytold/figcraft")
            | ("filmcraft", "storytold/filmcraft")
            | ("gridcraft", "storytold/gridcraft")
            | ("lightcraft", "storytold/lightcraft")
            | ("pdfcraft", "storytold/pdfcraft")
            | ("photocraft", "storytold/photocraft")
            | ("soundcraft", "storytold/soundcraft")
            | ("texturecraft", "storytold/texturecraft")
            | ("vectorcraft", "storytold/vectorcraft")
            | ("wordcraft", "storytold/wordcraft")
    )
}

#[cfg(test)]
mod recipe_tests {
    #[test]
    fn verified_linux_appimages_require_the_tested_host_and_publisher() {
        use crate::{Arch, CraftApp, Os, Platform};

        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        let arm = Platform { arch: Arch::Arm64, ..linux };
        let photo = *CraftApp::by_id("photocraft").unwrap();
        let impostor = CraftApp { repo: "elsewhere/photocraft", ..photo };
        assert!(super::verified_linux_appimage(&photo, linux));
        assert!(!super::verified_linux_appimage(&photo, arm));
        assert!(!super::verified_linux_appimage(&impostor, linux));
        assert!(!super::verified_linux_appimage(&super::HANDY, linux));
        assert_eq!(super::NATIVE_APPS.iter().filter(|app| super::verified_linux_appimage(app, linux)).count(), 15);
    }

    #[test]
    fn localsend_dmg_recipe_accepts_the_reviewed_mac_asset_for_apple_silicon() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("localsend").expect("compiled LocalSend recipe");
        let tag = "v1.19.0";
        let filename = "LocalSend-1.19.0.dmg";
        let latest = release::release_from_sums(app.repo, tag, &[]);
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let (asset, kind) = platform::pick_asset(&latest, &[app.id], mac, mac.download_kinds()).expect("reviewed DMG");
        assert_eq!((asset.name.as_str(), kind), (filename, PackageKind::Dmg));
        assert_eq!(asset.sha256.as_deref(), Some("5645789cc976298b893267c025f0945d8673baed3bb302a201eb8b96991e43d4"));
        for host in [
            Platform { os: Os::MacOs, arch: Arch::X64 },
            Platform { os: Os::Linux, arch: Arch::Arm64 },
            Platform { os: Os::Windows, arch: Arch::Arm64 },
        ] {
            assert!(platform::pick_asset(&latest, &[app.id], host, host.download_kinds()).is_none());
        }
        let mismatched = release::release_from_sums(app.repo, "v1.18.2", &[("a".repeat(64), filename.to_owned())]);
        assert!(platform::pick_asset(&mismatched, &[app.id], mac, mac.download_kinds()).is_none());
        let alternate = release::release_from_sums(app.repo, "v1.18.2", &[("a".repeat(64), "LocalSend-1.18.2-no-proxy.dmg".to_owned())]);
        assert!(platform::pick_asset(&alternate, &[app.id], mac, mac.download_kinds()).is_none());
        let unapproved = release::release_from_sums("other/localsend", tag, &[]);
        assert!(platform::pick_asset(&unapproved, &[app.id], mac, mac.download_kinds()).is_none());
    }

    #[test]
    fn localsend_recipe_accepts_only_the_official_linux_x64_appimage() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("localsend").expect("compiled LocalSend recipe");
        assert_eq!(app.repo, "localsend/localsend");
        assert_eq!(app.linux_desktop_id(), "org.localsend.localsend_app");
        let names = [
            "LocalSend-1.19.0-linux-x86-64.AppImage",
            "LocalSend-1.19.0-linux-arm-64.AppImage",
            "LocalSend-1.19.0-linux-x86_64.AppImage",
            "LocalSend-1.19.0-linux-x86-64.deb",
        ];
        let sha256 = "e72ce88383a32beec9701583c22e298037f7683f9590cea7b3a281f58c2ec2b3";
        let json = serde_json::json!({"tag_name":"v1.19.0", "assets": names.map(|name| serde_json::json!({
            "name":name, "browser_download_url":format!("https://github.com/localsend/localsend/releases/download/v1.19.0/{name}"),
            "size":61_752_512, "digest":format!("sha256:{sha256}")
        }))})
        .to_string();
        let release = release::parse_github_release(&json, app.repo).expect("official release");
        let linux_x64 = Platform { os: Os::Linux, arch: Arch::X64 };
        let (asset, kind) = platform::pick_asset(&release, &[app.id], linux_x64, linux_x64.download_kinds()).expect("reviewed AppImage");
        assert_eq!(kind, PackageKind::AppImage);
        assert_eq!(asset.name, names[0]);
        assert_eq!(asset.size, 61_752_512);
        assert_eq!(asset.sha256.as_deref(), Some(sha256));
        assert!(names[1..].iter().all(|name| platform::parse_asset_name(name, &[app.id]).is_none()));
        for os in [Os::Linux, Os::MacOs, Os::Windows, Os::FreeBsd] {
            for arch in [Arch::Arm64, Arch::X64] {
                let host = Platform { os, arch };
                assert_eq!(
                    platform::recipe_platform_verified(&[app.id], host),
                    host == linux_x64 || host == (Platform { os: Os::MacOs, arch: Arch::Arm64 }),
                );
            }
        }
        let other = release::parse_github_release(&json, "untrusted/localsend").expect("valid response with rejected assets");
        assert!(other.assets.is_empty());
    }

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
