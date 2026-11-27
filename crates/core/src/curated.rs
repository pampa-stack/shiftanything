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
    #[serde(default)]
    pub review: Option<Review>,
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
#[derive(Debug, Deserialize)]
pub struct Review {
    pub source: Option<String>,
    #[serde(default)]
    pub sources: Vec<String>,
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

/// Native OBS Studio identity from the notarized Apple Silicon DMG.
pub const OBS_STUDIO: crate::CraftApp = crate::CraftApp {
    id: "obs",
    name: "OBS Studio",
    tagline: "Record and stream video",
    repo: "obsproject/obs-studio",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [113, 122, 138],
    ink_light: [52, 63, 83],
    ink_dark: [197, 206, 224],
    kind: crate::catalog::AppKind::Community,
    publisher: "OBS Project",
    msi_publisher: None,
    bundle: Some("com.obsproject.obs-studio"),
    site: Some("https://obsproject.com/"),
    mac_bundle_name: Some("OBS"),
    mac_team: Some("2MMRE5MTB8"),
};

/// Native identity from the notarized KopiaUI DMGs for Apple Silicon.
pub const KOPIA: crate::CraftApp = crate::CraftApp {
    id: "kopia",
    name: "Kopia",
    tagline: "Encrypt and restore file backups",
    repo: "kopia/kopia",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [102, 143, 210],
    ink_light: [36, 73, 137],
    ink_dark: [171, 200, 246],
    kind: crate::catalog::AppKind::Community,
    publisher: "Kopia LLC",
    msi_publisher: None,
    bundle: Some("io.kopia.ui"),
    site: Some("https://kopia.io/"),
    mac_bundle_name: Some("KopiaUI"),
    mac_team: Some("7299Z9ZVTK"),
};

/// Native identities from the notarized releases tested on Apple Silicon.
pub const TRANSMISSION: crate::CraftApp = crate::CraftApp {
    id: "transmission",
    name: "Transmission",
    tagline: "Download and share files over BitTorrent",
    repo: "transmission/transmission",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [212, 115, 110],
    ink_light: [126, 48, 44],
    ink_dark: [250, 184, 180],
    kind: crate::catalog::AppKind::Community,
    publisher: "Transmission Project",
    msi_publisher: None,
    bundle: Some("org.m0k.transmission"),
    site: Some("https://transmissionbt.com/"),
    mac_bundle_name: Some("Transmission"),
    mac_team: Some("5DPYRBHEAR"),
};

pub const IINA: crate::CraftApp = crate::CraftApp {
    id: "iina",
    name: "IINA",
    tagline: "Play local videos and audio on macOS",
    repo: "iina/iina",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [103, 148, 216],
    ink_light: [47, 81, 137],
    ink_dark: [168, 206, 251],
    kind: crate::catalog::AppKind::Community,
    publisher: "IINA Developers",
    msi_publisher: None,
    bundle: Some("com.colliderli.iina"),
    site: Some("https://iina.io/"),
    mac_bundle_name: Some("IINA"),
    mac_team: Some("67CQ77V27R"),
};

pub const HANDBRAKE: crate::CraftApp = crate::CraftApp {
    id: "handbrake",
    name: "HandBrake",
    tagline: "Convert and compress videos",
    repo: "HandBrake/HandBrake",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [188, 154, 78],
    ink_light: [104, 76, 29],
    ink_dark: [236, 207, 139],
    kind: crate::catalog::AppKind::Community,
    publisher: "HandBrake Team",
    msi_publisher: None,
    bundle: Some("fr.handbrake.HandBrake"),
    site: Some("https://handbrake.fr/"),
    mac_bundle_name: Some("HandBrake"),
    mac_team: Some("5X9DE89KYV"),
};

/// The open-source Community edition distributed by Zackriya Solutions.
pub const MEETILY: crate::CraftApp = crate::CraftApp {
    id: "meetily",
    name: "Meetily",
    tagline: "Transcribe and summarize meetings locally",
    repo: "Zackriya-Solutions/meetily",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [97, 157, 181],
    ink_light: [47, 87, 106],
    ink_dark: [161, 221, 238],
    kind: crate::catalog::AppKind::Community,
    publisher: "Zackriya Solutions",
    msi_publisher: None,
    bundle: Some("com.meetily.ai"),
    site: Some("https://meetily.ai/"),
    mac_bundle_name: Some("meetily"),
    mac_team: Some("554AZZ38TB"),
};

pub const RIZZO_PII: crate::CraftApp = crate::CraftApp {
    id: "rizzo-pii",
    name: "Rizzo PII",
    tagline: "Redact personal information in local documents",
    repo: "Rizzo-AI-Academy/rizzo-pii",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [109, 174, 140],
    ink_light: [50, 96, 72],
    ink_dark: [166, 226, 195],
    kind: crate::catalog::AppKind::Community,
    publisher: "Rizzo AI Academy",
    msi_publisher: None,
    bundle: Some("com.rizzoai.pii-anonymizer"),
    site: Some("https://rizzo-ai-academy.github.io/rizzo-pii/"),
    mac_bundle_name: Some("Rizzo PII"),
    mac_team: Some("5J76G3383Y"),
};

pub const AUDACITY: crate::CraftApp = crate::CraftApp {
    id: "audacity",
    name: "Audacity 4",
    tagline: "Audio editing and recording",
    repo: "audacity/audacity",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [102, 152, 230],
    ink_light: [37, 65, 125],
    ink_dark: [155, 187, 247],
    kind: crate::catalog::AppKind::Community,
    publisher: "Audacity Team",
    msi_publisher: None,
    bundle: Some("org.audacityteam.audacity4"),
    site: Some("https://www.audacityteam.org/"),
    mac_bundle_name: None,
    mac_team: Some("6EPAF2X3PR"),
};

pub const KEEPASSXC: crate::CraftApp = crate::CraftApp {
    id: "keepassxc",
    name: "KeePassXC",
    tagline: "Local password manager",
    repo: "keepassxreboot/keepassxc",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [100, 172, 125],
    ink_light: [46, 97, 64],
    ink_dark: [157, 225, 179],
    kind: crate::catalog::AppKind::Community,
    publisher: "KeePassXC Team",
    msi_publisher: None,
    bundle: Some("org.keepassxc.keepassxc"),
    site: Some("https://keepassxc.org/"),
    mac_bundle_name: None,
    mac_team: Some("G2S7P7J672"),
};

pub(crate) const COMMUNITY_RECIPES: [crate::CraftApp; 11] =
    [HANDY, LOCALSEND, OBS_STUDIO, KOPIA, TRANSMISSION, IINA, HANDBRAKE, MEETILY, RIZZO_PII, AUDACITY, KEEPASSXC];

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
    fn obs_recipe_selects_only_the_apple_silicon_dmg_for_the_exact_release() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("obs").expect("compiled OBS recipe");
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let latest = release::release_from_sums(app.repo, "33.0.3", &[]);
        let (asset, kind) = platform::pick_asset(&latest, &[app.id], mac, mac.download_kinds()).expect("reviewed DMG");
        assert_eq!((asset.name.as_str(), kind), ("OBS-Studio-33.0.3-macOS-Apple.dmg", PackageKind::Dmg));
        assert_eq!(asset.sha256.as_deref(), Some("92212873b6f84c54285203cac3f846590ac00dfad76b312cf266100ff5edf0c8"));
        let intel = Platform { os: Os::MacOs, arch: Arch::X64 };
        let windows = Platform { os: Os::Windows, arch: Arch::Arm64 };
        for host in [intel, windows] {
            assert!(platform::pick_asset(&latest, &[app.id], host, host.download_kinds()).is_none());
        }
        for name in ["OBS-Studio-33.0.2-macOS-Intel.dmg", "OBS-Studio-33.0.3-macOS-Apple.dmg", "obs-33.0.2-macos-aarch64.dmg"] {
            let different = release::release_from_sums(app.repo, "33.0.2", &[("a".repeat(64), name.to_owned())]);
            assert!(platform::pick_asset(&different, &[app.id], mac, mac.download_kinds()).is_none());
        }
    }

    #[test]
    fn kopia_recipe_uses_only_its_verified_mac_format_and_official_checksum_source() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("kopia").expect("compiled Kopia recipe");
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let select = |release: &release::Release, host: Platform| {
            platform::pick_asset(release, &[app.id], host, host.download_kinds()).map(|(asset, kind)| (asset.name.clone(), kind))
        };
        for (tag, dmg) in [("v0.23.0", "KopiaUI-0.23.0-arm64.dmg"), ("v0.23.1", "KopiaUI-0.23.1-arm64.dmg")] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            assert_eq!(select(&release, mac), Some((dmg.to_owned(), PackageKind::Dmg)));
            assert!(release.assets.iter().find(|asset| asset.name == dmg).unwrap().sha256.is_some());
            for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }, Platform { os: Os::Windows, ..mac }] {
                assert!(select(&release, host).is_none());
            }
        }

        // A new stable version can use the upstream manifest when the API omits per-file hashes.
        let names = ["KopiaUI-0.24.0-arm64.dmg", "checksums.txt"];
        let json = |repo| {
            serde_json::json!({"tag_name":"v0.24.0", "assets": names.map(|name| serde_json::json!({
                "name":name, "browser_download_url":format!("https://github.com/{repo}/releases/download/v0.24.0/{name}")
            }))})
            .to_string()
        };
        let upstream = release::parse_github_release(&json(app.repo), app.repo).unwrap();
        assert_eq!(select(&upstream, mac), Some((names[0].to_owned(), PackageKind::Dmg)));
        assert_eq!(upstream.sums_asset().map(|asset| asset.name.as_str()), Some(names[1]));

        let mut no_manifest = upstream.clone();
        no_manifest.assets.retain(|asset| asset.name != names[1]);
        assert!(select(&no_manifest, mac).is_none());
        let other = release::parse_github_release(&json("mirror/kopia"), "mirror/kopia").unwrap();
        assert!(other.sums_asset().is_none());
        assert!(select(&other, mac).is_none());

        for wrong in ["KopiaUI-0.24.0.dmg", "KopiaUI-0.24.0-x64.dmg", "KopiaUI-0.24.0.AppImage", "kopia-0.24.0-macos-arm64.dmg"] {
            assert!(platform::parse_asset_name(wrong, &[app.id]).is_none());
        }
        let mismatched = release::release_from_sums(app.repo, "v0.24.1", &[("a".repeat(64), names[0].to_owned())]);
        assert!(select(&mismatched, mac).is_none());
    }

    #[test]
    fn community_dmg_recipes_accept_documented_apple_silicon_packages() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (app, tags, names, bundle, team) in [
            (super::TRANSMISSION, ["4.1.3", "4.1.4"], ["Transmission-4.1.3.dmg", "Transmission-4.1.4.dmg"], "org.m0k.transmission", "5DPYRBHEAR"),
            (super::IINA, ["v1.5.0", "v1.5.1"], ["IINA.v1.5.0.dmg", "IINA.v1.5.1.dmg"], "com.colliderli.iina", "67CQ77V27R"),
            (super::HANDBRAKE, ["1.11.1", "1.11.2"], ["HandBrake-1.11.1.dmg", "HandBrake-1.11.2.dmg"], "fr.handbrake.HandBrake", "5X9DE89KYV"),
            (super::MEETILY, ["v0.4.1", "v0.5.0"], ["meetily_0.4.1_aarch64.dmg", "meetily_0.5.0_aarch64.dmg"], "com.meetily.ai", "554AZZ38TB"),
            (
                super::RIZZO_PII,
                ["v2.1.0", "v2.2.0"],
                ["Rizzo-PII-2.1.0-macOS-arm64.dmg", "Rizzo-PII-2.2.0-macOS-arm64.dmg"],
                "com.rizzoai.pii-anonymizer",
                "5J76G3383Y",
            ),
            (
                super::AUDACITY,
                ["Audacity-4.0.1", "Audacity-4.0.2"],
                ["audacity-macOS-4.0.1-arm64.dmg", "audacity-macOS-4.0.2-arm64.dmg"],
                "org.audacityteam.audacity4",
                "6EPAF2X3PR",
            ),
            (
                super::KEEPASSXC,
                ["2.7.12", "2.8.0"],
                ["KeePassXC-2.7.12-arm64.dmg", "KeePassXC-2.8.0-1-arm64.dmg"],
                "org.keepassxc.keepassxc",
                "G2S7P7J672",
            ),
        ] {
            let native = CraftApp::by_id(app.id).expect("compiled recipe");
            assert_eq!(native.bundle, Some(bundle));
            assert_eq!(native.mac_team, Some(team));
            let select = |release: &release::Release, host: Platform| {
                platform::pick_asset(release, &[app.id], host, host.download_kinds()).map(|(asset, kind)| (asset.name.clone(), kind))
            };
            for (tag, filename) in tags.into_iter().zip(names) {
                let official = release::release_from_sums(app.repo, tag, &[]);
                assert_eq!(select(&official, mac), Some((filename.to_owned(), PackageKind::Dmg)));
                assert_eq!(official.assets[0].sha256.as_ref().map(String::len), Some(64));
                assert!(select(&official, Platform { os: Os::MacOs, arch: Arch::X64 }).is_none());
                assert!(select(&official, Platform { os: Os::Linux, arch: Arch::X64 }).is_none());
                assert!(select(&official, Platform { os: Os::Windows, arch: Arch::Arm64 }).is_none());
                let mut missing_hash = official.clone();
                missing_hash.assets.retain(|asset| asset.name == filename);
                missing_hash.assets[0].sha256 = None;
                assert!(select(&missing_hash, mac).is_none());
                let mut preview = official.clone();
                preview.prerelease = true;
                assert!(select(&preview, mac).is_none());
                let foreign = release::release_from_sums("mirror/community", tag, &[("a".repeat(64), filename.to_owned())]);
                assert!(select(&foreign, mac).is_none());
            }
            let mut stale = release::release_from_sums(app.repo, tags[1], &[("a".repeat(64), names[0].to_owned())]);
            stale.assets.retain(|asset| asset.name == names[0]);
            assert!(select(&stale, mac).is_none());
        }
        for (app, other_asset) in [
            (super::TRANSMISSION, "Transmission-4.1.4-intel.dmg"),
            (super::IINA, "IINA.v1.5.1-nightly.dmg"),
            (super::HANDBRAKE, "HandBrakeCLI-1.11.2.dmg"),
            (super::MEETILY, "meetily-pro_0.5.0_aarch64.dmg"),
            (super::MEETILY, "meetily_0.5.0_amd64.AppImage"),
            (super::RIZZO_PII, "Rizzo-PII-2.2.0-macOS-arm64-offline.dmg"),
            (super::AUDACITY, "audacity-macOS-3.7.9-arm64.dmg"),
            (super::AUDACITY, "audacity-macOS-4.0.2-universal.dmg"),
            (super::KEEPASSXC, "KeePassXC-2.7.11-arm64.dmg"),
            (super::KEEPASSXC, "KeePassXC-2.8.0-arm64.dmg"),
            (super::KEEPASSXC, "KeePassXC-2.8.0-2-arm64.dmg"),
            (super::KEEPASSXC, "KeePassXC-2.8.0-1-x86_64.dmg"),
        ] {
            assert!(platform::parse_asset_name(other_asset, &[app.id]).is_none());
        }
        assert_eq!(super::AUDACITY.mac_bundle_names().collect::<Vec<_>>(), ["Audacity 4"]);
        assert!(!super::AUDACITY.bundle_ids().any(|id| id == "org.audacityteam.audacity"));
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
