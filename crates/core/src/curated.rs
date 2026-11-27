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
                    && !crate::flathub::approved_catalog_identity(app)
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

    #[test]
    fn native_recipes_have_unique_app_ids() {
        let mut ids = std::collections::HashSet::new();
        for app in &super::NATIVE_APPS {
            assert!(ids.insert(app.id), "duplicate native recipe for {}", app.id);
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
    msi_publisher: Some("KeePassXC Team"),
    bundle: Some("org.keepassxc.keepassxc"),
    site: Some("https://keepassxc.org/"),
    mac_bundle_name: None,
    mac_team: Some("G2S7P7J672"),
};

pub const LIBREOFFICE: crate::CraftApp = crate::CraftApp {
    id: "libreoffice",
    name: "LibreOffice",
    tagline: "Documents, spreadsheets and presentations",
    repo: "LibreOffice/core",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [99, 171, 111],
    ink_light: [45, 99, 55],
    ink_dark: [157, 224, 166],
    kind: crate::catalog::AppKind::Community,
    publisher: "The Document Foundation",
    msi_publisher: None,
    bundle: Some("org.libreoffice.script"),
    site: Some("https://www.libreoffice.org/"),
    mac_bundle_name: None,
    mac_team: Some("7P5S3ZLCN7"),
};

pub const JOPLIN: crate::CraftApp = crate::CraftApp {
    id: "joplin",
    name: "Joplin",
    tagline: "Collect notes, notebooks and pages clipped from the web",
    repo: "laurent22/joplin",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [93, 145, 231],
    ink_light: [36, 71, 132],
    ink_dark: [157, 191, 251],
    kind: crate::catalog::AppKind::Community,
    publisher: "Laurent Cozic",
    msi_publisher: None,
    bundle: Some("net.cozic.joplin-desktop"),
    site: Some("https://joplinapp.org/"),
    mac_bundle_name: None,
    mac_team: Some("A9BXAFS6CT"),
};

pub const SHOTCUT: crate::CraftApp = crate::CraftApp {
    id: "shotcut",
    name: "Shotcut",
    tagline: "Multitrack video editing",
    repo: "mltframework/shotcut",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [73, 166, 184],
    ink_light: [38, 95, 110],
    ink_dark: [146, 219, 233],
    kind: crate::catalog::AppKind::Community,
    publisher: "Meltytech, LLC",
    msi_publisher: None,
    bundle: Some("com.Meltytech.Shotcut"),
    site: Some("https://shotcut.org/"),
    mac_bundle_name: None,
    mac_team: Some("Y6RX44QG2G"),
};

pub const PINTA: crate::CraftApp = crate::CraftApp {
    id: "pinta",
    name: "Pinta",
    tagline: "Drawing and image editing",
    repo: "PintaProject/Pinta",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [101, 155, 217],
    ink_light: [41, 80, 125],
    ink_dark: [169, 205, 248],
    kind: crate::catalog::AppKind::Community,
    publisher: "Pinta Project",
    msi_publisher: None,
    bundle: Some("com.ximian.pinta"),
    site: Some("https://www.pinta-project.com/"),
    mac_bundle_name: None,
    mac_team: Some("D5G6C56TBH"),
};

pub const ZED: crate::CraftApp = crate::CraftApp {
    id: "zed",
    name: "Zed",
    tagline: "Code editor",
    repo: "zed-industries/zed",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [89, 149, 232],
    ink_light: [40, 76, 124],
    ink_dark: [163, 202, 252],
    kind: crate::catalog::AppKind::Community,
    publisher: "Zed Industries, Inc.",
    msi_publisher: None,
    bundle: Some("dev.zed.Zed"),
    site: Some("https://zed.dev/"),
    mac_bundle_name: None,
    mac_team: Some("MQ55VZLNZQ"),
};

pub const DRAWIO: crate::CraftApp = crate::CraftApp {
    id: "drawio",
    name: "draw.io Desktop",
    tagline: "Offline diagram and flowchart editor",
    repo: "jgraph/drawio-desktop",
    former_repo: None,
    former_ids: &["draw.io"],
    former_names: &["draw.io"],
    former_bundles: &[],
    color: [244, 127, 30],
    ink_light: [127, 58, 4],
    ink_dark: [255, 179, 110],
    kind: crate::catalog::AppKind::Community,
    publisher: "DRAW.IO LTD",
    msi_publisher: Some("JGraph"),
    bundle: Some("com.jgraph.drawio.desktop"),
    site: Some("https://www.drawio.com/"),
    mac_bundle_name: Some("draw.io"),
    mac_team: Some("UZEUFB4N53"),
};

pub const STATS: crate::CraftApp = crate::CraftApp {
    id: "stats",
    name: "Stats",
    tagline: "System monitoring in the menu bar",
    repo: "exelban/stats",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [95, 165, 240],
    ink_light: [37, 77, 130],
    ink_dark: [169, 205, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Serhiy Mytrovtsiy",
    msi_publisher: None,
    bundle: Some("eu.exelban.Stats"),
    site: Some("https://mac-stats.com/"),
    mac_bundle_name: None,
    mac_team: Some("RP2S87B72W"),
};

pub const WHISPERING: crate::CraftApp = crate::CraftApp {
    id: "epicenter-whispering",
    name: "Whispering",
    tagline: "Local voice transcription",
    repo: "EpicenterHQ/epicenter",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [108, 171, 149],
    ink_light: [38, 87, 73],
    ink_dark: [159, 216, 196],
    kind: crate::catalog::AppKind::Community,
    publisher: "Braden Wong",
    msi_publisher: None,
    bundle: Some("com.bradenwong.whispering"),
    site: Some("https://epicenter.so/"),
    mac_bundle_name: None,
    mac_team: Some("3C7J97QA5Z"),
};

pub const ENTE_AUTH: crate::CraftApp = crate::CraftApp {
    id: "ente-auth",
    name: "Ente Auth",
    tagline: "Two-factor authentication codes",
    repo: "ente/ente",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [103, 181, 124],
    ink_light: [37, 90, 49],
    ink_dark: [158, 220, 172],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ente Technologies, Inc.",
    msi_publisher: None,
    bundle: Some("io.ente.auth.mac"),
    site: Some("https://ente.com/auth/"),
    mac_bundle_name: None,
    mac_team: Some("6Z68YJY9Q2"),
};

pub const CRYPTOMATOR: crate::CraftApp = crate::CraftApp {
    id: "cryptomator",
    name: "Cryptomator",
    tagline: "Encrypt files in local and cloud folders",
    repo: "cryptomator/cryptomator",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [86, 177, 123],
    ink_light: [40, 88, 58],
    ink_dark: [151, 218, 174],
    kind: crate::catalog::AppKind::Community,
    publisher: "Skymatic GmbH",
    msi_publisher: None,
    bundle: Some("org.cryptomator"),
    site: Some("https://cryptomator.org/"),
    mac_bundle_name: None,
    mac_team: Some("YZQJQUHA3L"),
};

pub const ACTIVITYWATCH: crate::CraftApp = crate::CraftApp {
    id: "activitywatch",
    name: "ActivityWatch",
    tagline: "Track how you spend time on your computer",
    repo: "ActivityWatch/activitywatch",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [75, 128, 201],
    ink_light: [30, 70, 126],
    ink_dark: [171, 209, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "ActivityWatch",
    msi_publisher: None,
    bundle: Some("net.activitywatch.ActivityWatch"),
    site: Some("https://activitywatch.net/"),
    mac_bundle_name: None,
    mac_team: Some("XM9GC3SUL2"),
};

pub const GODOT_RELEASES_REPO: &str = "godotengine/godot-builds";

pub const GODOT: crate::CraftApp = crate::CraftApp {
    id: "godot-engine",
    name: "Godot Engine",
    tagline: "Create 2D and 3D games",
    repo: "godotengine/godot",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [71, 140, 191],
    ink_light: [37, 84, 120],
    ink_dark: [162, 208, 238],
    kind: crate::catalog::AppKind::Community,
    publisher: "The Godot Engine Community",
    msi_publisher: None,
    bundle: Some("org.godotengine.godot"),
    site: Some("https://godotengine.org/"),
    mac_bundle_name: Some("Godot"),
    mac_team: Some("6K46PWY5DM"),
};

pub const TERMIX: crate::CraftApp = crate::CraftApp {
    id: "termix",
    name: "Termix",
    tagline: "Manage your SSH connections, tunnels and files",
    repo: "Termix-SSH/Termix",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [80, 153, 198],
    ink_light: [36, 85, 115],
    ink_dark: [154, 211, 238],
    kind: crate::catalog::AppKind::Community,
    publisher: "Luke Gustafson",
    msi_publisher: None,
    bundle: Some("com.karmaa.termix"),
    site: Some("https://termix.site/"),
    mac_bundle_name: None,
    mac_team: Some("2CPC5YZYFN"),
};

pub const NEOVIDE: crate::CraftApp = crate::CraftApp {
    id: "neovide",
    name: "Neovide",
    tagline: "Graphical client for Neovim",
    repo: "neovide/neovide",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [144, 121, 217],
    ink_light: [78, 54, 143],
    ink_dark: [190, 175, 244],
    kind: crate::catalog::AppKind::Community,
    publisher: "Neovide contributors",
    msi_publisher: None,
    bundle: Some("com.neovide.neovide"),
    site: Some("https://neovide.dev/"),
    mac_bundle_name: None,
    mac_team: Some("X8CNW77992"),
};

pub const PENCIL2D: crate::CraftApp = crate::CraftApp {
    id: "pencil2d",
    name: "Pencil2D",
    tagline: "Draw and animate frame by frame",
    repo: "pencil2d/pencil",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [130, 158, 225],
    ink_light: [49, 75, 140],
    ink_dark: [176, 199, 247],
    kind: crate::catalog::AppKind::Community,
    publisher: "The Pencil2D Team",
    msi_publisher: None,
    bundle: Some("org.pencil2d.Pencil2D"),
    site: Some("https://www.pencil2d.org/"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const OPENSHOT: crate::CraftApp = crate::CraftApp {
    id: "openshot-video-editor",
    name: "OpenShot Video Editor",
    tagline: "Edit video with tracks, titles and transitions",
    repo: "OpenShot/openshot-qt",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [113, 178, 220],
    ink_light: [47, 91, 130],
    ink_dark: [160, 211, 242],
    kind: crate::catalog::AppKind::Community,
    publisher: "OpenShot Studios, LLC",
    msi_publisher: None,
    bundle: Some("org.openshot.OpenShot"),
    site: Some("https://www.openshot.org/"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const KEYCASTR: crate::CraftApp = crate::CraftApp {
    id: "keycastr",
    name: "KeyCastr",
    tagline: "Visualize keystrokes during a presentation",
    repo: "keycastr/keycastr",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [129, 150, 218],
    ink_light: [64, 81, 135],
    ink_dark: [185, 196, 238],
    kind: crate::catalog::AppKind::Community,
    publisher: "KeyCastr contributors",
    msi_publisher: None,
    bundle: Some("io.github.keycastr"),
    site: Some("https://github.com/keycastr/keycastr"),
    mac_bundle_name: None,
    mac_team: Some("68TUCLNAPS"),
};

pub const PICARD: crate::CraftApp = crate::CraftApp {
    id: "musicbrainz-picard",
    name: "MusicBrainz Picard",
    tagline: "Tag music files using MusicBrainz",
    repo: "metabrainz/picard",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [178, 115, 220],
    ink_light: [99, 53, 142],
    ink_dark: [220, 178, 248],
    kind: crate::catalog::AppKind::Community,
    publisher: "MetaBrainz Foundation",
    msi_publisher: None,
    bundle: Some("org.musicbrainz.Picard"),
    site: Some("https://picard.musicbrainz.org/"),
    mac_bundle_name: None,
    mac_team: Some("8D89TSJ88H"),
};

pub const MUSESCORE: crate::CraftApp = crate::CraftApp {
    id: "musescore",
    name: "MuseScore Studio",
    tagline: "Write, arrange and play sheet music",
    repo: "musescore/MuseScore",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &["org.musescore.MuseScore"],
    color: [120, 183, 240],
    ink_light: [37, 82, 146],
    ink_dark: [130, 189, 248],
    kind: crate::catalog::AppKind::Community,
    publisher: "MuseScore Limited",
    msi_publisher: None,
    bundle: Some("org.musescore.MuseScore4portable"),
    site: Some("https://musescore.org/"),
    mac_bundle_name: Some("MuseScore 4"),
    mac_team: Some("6EPAF2X3PR"),
};

pub const ORCA_SLICER: crate::CraftApp = crate::CraftApp {
    id: "orca-slicer",
    name: "OrcaSlicer",
    tagline: "Prepare sliced models for 3D printing",
    repo: "OrcaSlicer/OrcaSlicer",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [120, 194, 189],
    ink_light: [34, 105, 103],
    ink_dark: [146, 206, 201],
    kind: crate::catalog::AppKind::Community,
    publisher: "OrcaSlicer contributors",
    msi_publisher: None,
    bundle: Some("com.orcaslicer.OrcaSlicer"),
    site: Some("https://www.orcaslicer.com/"),
    mac_bundle_name: None,
    mac_team: Some("XQK7C38HH5"),
};

pub const DBEAVER_CE: crate::CraftApp = crate::CraftApp {
    id: "dbeaver-ce",
    name: "DBeaver Community",
    tagline: "Explore databases and edit SQL",
    repo: "dbeaver/dbeaver",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &["org.jkiss.dbeaver.core.product"],
    color: [188, 157, 126],
    ink_light: [108, 75, 46],
    ink_dark: [229, 201, 172],
    kind: crate::catalog::AppKind::Community,
    publisher: "DBeaver",
    msi_publisher: None,
    bundle: Some("io.dbeaver.DBeaverCommunity"),
    site: Some("https://dbeaver.io/"),
    mac_bundle_name: Some("DBeaver"),
    mac_team: Some("42B6MDKMW8"),
};

pub const PRUSA_SLICER_MAC_BUNDLE: &str = "com.prusa3d.slic3r/";

pub const PRUSA_SLICER: crate::CraftApp = crate::CraftApp {
    id: "prusaslicer",
    name: "PrusaSlicer",
    tagline: "Prepare models for 3D printing",
    repo: "prusa3d/PrusaSlicer",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[PRUSA_SLICER_MAC_BUNDLE],
    color: [243, 151, 108],
    ink_light: [148, 58, 20],
    ink_dark: [255, 179, 139],
    kind: crate::catalog::AppKind::Community,
    publisher: "Prusa Research a.s.",
    msi_publisher: None,
    bundle: Some("com.prusa3d.PrusaSlicer"),
    site: Some("https://www.prusa3d.com/p/prusaslicer/"),
    mac_bundle_name: None,
    mac_team: Some("DKPB65N43Z"),
};

pub const LOSSLESSCUT: crate::CraftApp = crate::CraftApp {
    id: "losslesscut",
    name: "LosslessCut",
    tagline: "Trim and rearrange media without re-encoding",
    repo: "mifi/lossless-cut",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &["losslesscut"],
    color: [137, 191, 237],
    ink_light: [48, 86, 131],
    ink_dark: [147, 204, 245],
    kind: crate::catalog::AppKind::Community,
    publisher: "Mikael Finstad",
    msi_publisher: None,
    bundle: Some("no.mifi.losslesscut-mac"),
    site: Some("https://losslesscut.app/"),
    mac_bundle_name: Some("LosslessCut"),
    mac_team: Some("46F6T3M669"),
};

pub const ZETTLR: crate::CraftApp = crate::CraftApp {
    id: "zettlr",
    name: "Zettlr",
    tagline: "Write and publish from local Markdown files",
    repo: "Zettlr/Zettlr",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &["Zettlr"],
    color: [95, 177, 156],
    ink_light: [36, 104, 88],
    ink_dark: [143, 212, 194],
    kind: crate::catalog::AppKind::Community,
    publisher: "Hendrik Erz and contributors",
    msi_publisher: None,
    bundle: Some("com.zettlr.app"),
    site: Some("https://www.zettlr.com/"),
    mac_bundle_name: None,
    mac_team: Some("QS52BN8W68"),
};

pub const ONLYOFFICE: crate::CraftApp = crate::CraftApp {
    id: "onlyoffice",
    name: "ONLYOFFICE Desktop Editors",
    tagline: "Work on documents, spreadsheets and presentations",
    repo: "ONLYOFFICE/DesktopEditors",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [116, 173, 231],
    ink_light: [45, 90, 151],
    ink_dark: [150, 196, 244],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ascensio System SIA",
    msi_publisher: None,
    bundle: Some("onlyoffice-desktopeditors"),
    site: Some("https://www.onlyoffice.com/desktop.aspx"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const LAPCE: crate::CraftApp = crate::CraftApp {
    id: "lapce",
    name: "Lapce",
    tagline: "Edit code with a native interface",
    repo: "lapce/lapce",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [94, 148, 215],
    ink_light: [41, 81, 130],
    ink_dark: [167, 204, 244],
    kind: crate::catalog::AppKind::Community,
    publisher: "Lapce contributors",
    msi_publisher: None,
    bundle: Some("io.lapce"),
    site: Some("https://lapce.dev/"),
    mac_bundle_name: None,
    mac_team: Some("CYSGAZFR8D"),
};

pub const VORTA: crate::CraftApp = crate::CraftApp {
    id: "vorta",
    name: "Vorta",
    tagline: "Back up and restore files with Borg",
    repo: "borgbase/vorta",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [219, 140, 91],
    ink_light: [129, 62, 33],
    ink_dark: [236, 178, 143],
    kind: crate::catalog::AppKind::Community,
    publisher: "Vorta contributors",
    msi_publisher: None,
    bundle: Some("com.borgbase.client.macos"),
    site: Some("https://vorta.borgbase.com/"),
    mac_bundle_name: None,
    mac_team: Some("CNMSCAXT48"),
};

pub const SOLVESPACE: crate::CraftApp = crate::CraftApp {
    id: "solvespace",
    name: "SolveSpace",
    tagline: "Model parts with parametric 2D and 3D CAD",
    repo: "solvespace/solvespace",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [225, 145, 89],
    ink_light: [129, 67, 34],
    ink_dark: [239, 184, 146],
    kind: crate::catalog::AppKind::Community,
    publisher: "SolveSpace contributors",
    msi_publisher: None,
    bundle: Some("com.solvespace"),
    site: Some("https://solvespace.com/"),
    mac_bundle_name: None,
    mac_team: Some("8X77K9NDG3"),
};

pub const NEXTCLOUD_DESKTOP: crate::CraftApp = crate::CraftApp {
    id: "nextcloud-desktop",
    name: "Nextcloud Desktop",
    tagline: "Synchronizes files with your Nextcloud server",
    repo: "nextcloud/desktop",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [101, 161, 219],
    ink_light: [36, 73, 129],
    ink_dark: [152, 197, 240],
    kind: crate::catalog::AppKind::Community,
    publisher: "Nextcloud",
    msi_publisher: None,
    bundle: Some("com.nextcloud.desktopclient.nextcloud"),
    site: Some("https://nextcloud.com/install/#install-clients"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const NEXTCLOUD_DESKTOP_RELEASES_REPO: &str = "nextcloud-releases/desktop";

pub const VIBE: crate::CraftApp = crate::CraftApp {
    id: "vibe-transcribe",
    name: "Vibe",
    tagline: "Transcribe audio and video locally",
    repo: "thewh1teagle/vibe",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [103, 126, 230],
    ink_light: [54, 68, 157],
    ink_dark: [181, 190, 247],
    kind: crate::catalog::AppKind::Community,
    publisher: "Vibe contributors",
    msi_publisher: None,
    bundle: Some("github.com.thewh1teagle.vibe"),
    site: Some("https://thewh1teagle.github.io/vibe/"),
    mac_bundle_name: Some("vibe"),
    mac_team: Some("VSTAWC326G"),
};

pub const OPENSCREEN: crate::CraftApp = crate::CraftApp {
    id: "openscreen",
    name: "OpenScreen",
    tagline: "Record, edit and export screen demos",
    repo: "getopenscreen/openscreen",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [98, 132, 223],
    ink_light: [52, 74, 161],
    ink_dark: [172, 190, 247],
    kind: crate::catalog::AppKind::Community,
    publisher: "OpenScreen contributors",
    msi_publisher: None,
    bundle: Some("com.etiennelescot.openscreen"),
    site: Some("https://getopenscreen.com/"),
    mac_bundle_name: Some("Openscreen"),
    mac_team: Some("M4LK7C6S84"),
};

pub const DOSBOX_STAGING: crate::CraftApp = crate::CraftApp {
    id: "dosbox-staging",
    name: "DOSBox Staging",
    tagline: "Run DOS software on a modern computer",
    repo: "dosbox-staging/dosbox-staging",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [136, 148, 200],
    ink_light: [66, 75, 137],
    ink_dark: [185, 197, 236],
    kind: crate::catalog::AppKind::Community,
    publisher: "The DOSBox Staging Team",
    msi_publisher: None,
    bundle: Some("io.github.dosbox-staging"),
    site: Some("https://www.dosbox-staging.org/"),
    mac_bundle_name: None,
    mac_team: Some("944SS92GAH"),
};

pub const NOTEPAD_NEXT: crate::CraftApp = crate::CraftApp {
    id: "notepadnext",
    name: "Notepad Next",
    tagline: "Edit text, code and logs in tabs",
    repo: "dail8859/NotepadNext",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [103, 151, 77],
    ink_light: [58, 87, 43],
    ink_dark: [182, 226, 156],
    kind: crate::catalog::AppKind::Community,
    publisher: "Notepad Next contributors",
    msi_publisher: None,
    bundle: None,
    site: Some("https://github.com/dail8859/NotepadNext"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const BLOCKBENCH: crate::CraftApp = crate::CraftApp {
    id: "blockbench",
    name: "Blockbench",
    tagline: "Model, texture and animate low-poly 3D assets",
    repo: "JannisX11/blockbench",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [64, 145, 225],
    ink_light: [24, 77, 132],
    ink_dark: [143, 202, 250],
    kind: crate::catalog::AppKind::Community,
    publisher: "Jannis Petersen",
    msi_publisher: None,
    bundle: Some("blockbench"),
    site: Some("https://www.blockbench.net/"),
    mac_bundle_name: None,
    mac_team: Some("5QMG7G7462"),
};

pub const RAPIDRAW: crate::CraftApp = crate::CraftApp {
    id: "rapidraw",
    name: "RapidRAW",
    tagline: "Edit RAW photos with non-destructive controls",
    repo: "CyberTimon/RapidRAW",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [126, 105, 210],
    ink_light: [60, 45, 126],
    ink_dark: [198, 179, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "RapidRAW contributors",
    msi_publisher: None,
    bundle: None,
    site: Some("https://github.com/CyberTimon/RapidRAW"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const APPFLOWY: crate::CraftApp = crate::CraftApp {
    id: "appflowy",
    name: "AppFlowy",
    tagline: "Write and organize documents, notes and boards locally",
    repo: "AppFlowy-IO/AppFlowy",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [114, 104, 244],
    ink_light: [62, 54, 155],
    ink_dark: [192, 187, 252],
    kind: crate::catalog::AppKind::Community,
    publisher: "AppFlowy Pte. Ltd.",
    msi_publisher: None,
    bundle: Some("com.appflowy.appflowy.flutter"),
    site: Some("https://appflowy.com/"),
    mac_bundle_name: None,
    mac_team: Some("VHB67HRSZG"),
};

pub const ACTUAL_BUDGET: crate::CraftApp = crate::CraftApp {
    id: "actual",
    name: "Actual Budget",
    tagline: "Manage personal budgets, spending and goals locally",
    repo: "actualbudget/actual",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [100, 84, 202],
    ink_light: [60, 47, 145],
    ink_dark: [197, 187, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "Actual Budget",
    msi_publisher: None,
    bundle: Some("com.actualbudget.actual"),
    site: Some("https://actualbudget.org/"),
    mac_bundle_name: Some("Actual"),
    mac_team: Some("79ANZ983YF"),
};

pub const RUSTDESK: crate::CraftApp = crate::CraftApp {
    id: "rustdesk",
    name: "RustDesk",
    tagline: "Provide support and access computers remotely",
    repo: "rustdesk/rustdesk",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [86, 141, 237],
    ink_light: [35, 66, 120],
    ink_dark: [177, 209, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "RustDesk contributors",
    msi_publisher: None,
    bundle: Some("com.carriez.rustdesk"),
    site: Some("https://rustdesk.com/"),
    mac_bundle_name: None,
    mac_team: Some("HZF9JMC8YN"),
};

pub const SUPER_PRODUCTIVITY: crate::CraftApp = crate::CraftApp {
    id: "super-productivity",
    name: "Super Productivity",
    tagline: "Plan tasks, track time, and manage focus sessions",
    repo: "super-productivity/super-productivity",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Johannes Millan",
    msi_publisher: None,
    bundle: Some("com.super-productivity.app"),
    site: Some("https://super-productivity.com/"),
    mac_bundle_name: None,
    mac_team: Some("363FAFK383"),
};

pub const DB_BROWSER_SQLITE: crate::CraftApp = crate::CraftApp {
    id: "db-browser-for-sqlite",
    name: "DB Browser for SQLite",
    tagline: "Browse and edit SQLite databases locally",
    repo: "sqlitebrowser/sqlitebrowser",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [66, 133, 190],
    ink_light: [32, 83, 130],
    ink_dark: [164, 206, 246],
    kind: crate::catalog::AppKind::Community,
    publisher: "DB Browser for SQLite developers",
    msi_publisher: None,
    bundle: Some("net.sourceforge.sqlitebrowser"),
    site: Some("https://sqlitebrowser.org/"),
    mac_bundle_name: None,
    mac_team: Some("C34AV33YLK"),
};

pub const NAPS2: crate::CraftApp = crate::CraftApp {
    id: "naps2",
    name: "NAPS2",
    tagline: "Scan, rearrange and OCR documents",
    repo: "cyanfish/naps2",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [48, 132, 192],
    ink_light: [21, 82, 122],
    ink_dark: [137, 209, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ben Olden-Cooligan",
    msi_publisher: None,
    bundle: Some("com.naps2.desktop"),
    site: Some("https://www.naps2.com/"),
    mac_bundle_name: None,
    mac_team: Some("2P9Q2K3XM9"),
};

pub const OPENCLAW: crate::CraftApp = crate::CraftApp {
    id: "openclaw",
    name: "OpenClaw",
    tagline: "Connect to the OpenClaw gateway from macOS",
    repo: "openclaw/openclaw",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "OpenClaw Foundation",
    msi_publisher: None,
    bundle: Some("ai.openclaw.mac"),
    site: Some("https://openclaw.ai/"),
    mac_bundle_name: None,
    mac_team: Some("FWJYW4S8P8"),
};

pub const OPENCODE: crate::CraftApp = crate::CraftApp {
    id: "opencode",
    name: "OpenCode",
    tagline: "Work with the OpenCode coding agent from desktop",
    repo: "anomalyco/opencode",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Anomaly Innovations, Inc.",
    msi_publisher: None,
    bundle: Some("ai.opencode.desktop"),
    site: Some("https://opencode.ai/"),
    mac_bundle_name: None,
    mac_team: Some("5NZ4Q7NXJ4"),
};

pub const OLLAMA: crate::CraftApp = crate::CraftApp {
    id: "ollama",
    name: "Ollama",
    tagline: "Run and interact with language models locally",
    repo: "ollama/ollama",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Infra Technologies, Inc",
    msi_publisher: None,
    bundle: Some("com.electron.ollama"),
    site: Some("https://ollama.com/"),
    mac_bundle_name: None,
    mac_team: Some("3MU9H2V9Y9"),
};

pub const CLASH_VERGE_REV: crate::CraftApp = crate::CraftApp {
    id: "clash-verge-rev",
    name: "Clash Verge Rev",
    tagline: "Manage proxy settings from a desktop interface",
    repo: "clash-verge-rev/clash-verge-rev",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "won fen",
    msi_publisher: None,
    bundle: Some("io.github.clash-verge-rev.clash-verge-rev"),
    site: Some("https://www.clashverge.dev/"),
    mac_bundle_name: Some("Clash Verge"),
    mac_team: Some("JPH3Z7PPBB"),
};

pub const ORCA: crate::CraftApp = crate::CraftApp {
    id: "orca",
    name: "Orca",
    tagline: "Organize coding agents and worktrees",
    repo: "stablyai/orca",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Lovecast LLC",
    msi_publisher: None,
    bundle: Some("com.stablyai.orca"),
    site: Some("https://orca.dev/"),
    mac_bundle_name: None,
    mac_team: Some("6CX3WHS9HZ"),
};

pub const VOICE_STUDIO: crate::CraftApp = crate::CraftApp {
    id: "voice-studio",
    name: "VoiceStudio",
    tagline: "Edit and produce audio from the desktop",
    repo: "debpalash/VoiceStudio",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "YUPCHA SOFTWARES PRIVATE LIMITED",
    msi_publisher: None,
    bundle: Some("com.voicestudio.desktop"),
    site: Some("https://voicestudio.sh/"),
    mac_bundle_name: None,
    mac_team: Some("CR8M2NTH2K"),
};

pub const T3_CODE: crate::CraftApp = crate::CraftApp {
    id: "t3code",
    name: "T3 Code",
    tagline: "Work with coding agents in a desktop workspace",
    repo: "pingdotgg/t3code",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "T3 Tools, Inc.",
    msi_publisher: None,
    bundle: Some("com.t3tools.t3code"),
    site: Some("https://t3.codes/"),
    mac_bundle_name: Some("T3 Code (Beta)"),
    mac_team: Some("ARK85ZXQ4Z"),
};

pub const ANYTHING_LLM: crate::CraftApp = crate::CraftApp {
    id: "anythingllm",
    name: "AnythingLLM",
    tagline: "Run local AI models and chat with documents",
    repo: "Mintplex-Labs/anything-llm",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Timothy Carambat",
    msi_publisher: None,
    bundle: Some("com.anythingllm"),
    site: Some("https://anythingllm.com/"),
    mac_bundle_name: None,
    mac_team: Some("35S2NMU3G4"),
};

pub const BRUNO: crate::CraftApp = crate::CraftApp {
    id: "bruno",
    name: "Bruno",
    tagline: "Test APIs with local, Git-friendly collections",
    repo: "usebruno/bruno",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [237, 175, 118],
    ink_light: [113, 51, 23],
    ink_dark: [255, 215, 173],
    kind: crate::catalog::AppKind::Community,
    publisher: "Anoop MD",
    msi_publisher: None,
    bundle: Some("com.usebruno.app"),
    site: Some("https://www.usebruno.com/"),
    mac_bundle_name: None,
    mac_team: Some("W7LPPWA48L"),
};

pub const UPSCAYL: crate::CraftApp = crate::CraftApp {
    id: "upscayl",
    name: "Upscayl",
    tagline: "Upscale images locally with AI",
    repo: "upscayl/upscayl",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Mayank Sharma",
    msi_publisher: None,
    bundle: Some("org.upscayl.Upscayl"),
    site: Some("https://upscayl.org/"),
    mac_bundle_name: None,
    mac_team: Some("W2T4W74X87"),
};

pub const GHOST_DOWNLOADER: crate::CraftApp = crate::CraftApp {
    id: "ghost-downloader-3",
    name: "Ghost Downloader 3",
    tagline: "Manage downloads from a desktop queue",
    repo: "XiaoYouChR/Ghost-Downloader-3",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "XiaoYouChR",
    msi_publisher: None,
    bundle: Some("com.xychr.ghostdownloader"),
    site: Some("https://gd.xychr.com/"),
    mac_bundle_name: Some("Ghost Downloader"),
    mac_team: Some("GQ255NCW97"),
};

pub const JAN: crate::CraftApp = crate::CraftApp {
    id: "jan",
    name: "Jan",
    tagline: "Chat with local AI models",
    repo: "janhq/jan",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "JAN AI PTE. LTD.",
    msi_publisher: None,
    bundle: Some("jan.ai.app"),
    site: Some("https://jan.ai/"),
    mac_bundle_name: None,
    mac_team: Some("F8AH6NHVY5"),
};

pub const LOCALAI: crate::CraftApp = crate::CraftApp {
    id: "localai",
    name: "LocalAI Desktop",
    tagline: "Run AI models locally",
    repo: "mudler/LocalAI",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "LocalAI S.R.L.",
    msi_publisher: None,
    bundle: Some("io.localai.launcher"),
    site: Some("https://localai.io/"),
    mac_bundle_name: Some("local-ai-launcher"),
    mac_team: Some("34F3GGU9A6"),
};

pub const KAP: crate::CraftApp = crate::CraftApp {
    id: "kap",
    name: "Kap",
    tagline: "Record screen clips and export them to common formats",
    repo: "wulkano/Kap",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Wulkano AS",
    msi_publisher: None,
    bundle: Some("com.wulkano.kap"),
    site: Some("https://getkap.co/"),
    mac_bundle_name: None,
    mac_team: Some("2KEEHXF6R6"),
};

pub const SIMPLEX_CHAT: crate::CraftApp = crate::CraftApp {
    id: "simplex-chat",
    name: "SimpleX Chat",
    tagline: "Chat privately across mobile and desktop",
    repo: "simplex-chat/simplex-chat",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "SimpleX Chat Ltd",
    msi_publisher: None,
    bundle: Some("chat.simplex.app"),
    site: Some("https://simplex.chat/"),
    mac_bundle_name: Some("SimpleX"),
    mac_team: Some("5NN7GUYB6T"),
};

pub const SIYUAN: crate::CraftApp = crate::CraftApp {
    id: "siyuan",
    name: "SiYuan",
    tagline: "Organize notes and connect ideas",
    repo: "siyuan-note/siyuan",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Yunnan Liandi Technology Co., Ltd.",
    msi_publisher: None,
    bundle: Some("org.b3log.siyuan"),
    site: Some("https://b3log.org/siyuan/"),
    mac_bundle_name: None,
    mac_team: Some("FJT3K7XAD8"),
};

pub const MAGPIE: crate::CraftApp = crate::CraftApp {
    id: "magpie",
    name: "Magpie",
    tagline: "Connect AI tools to model providers",
    repo: "yetone/magpie",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Xipeng Guan",
    msi_publisher: None,
    bundle: Some("com.yetone.magpie"),
    site: Some("https://usemagpie.ai"),
    mac_bundle_name: Some("magpie"),
    mac_team: Some("LY7MVTUDZG"),
};

pub const TRILIUM: crate::CraftApp = crate::CraftApp {
    id: "trilium",
    name: "Trilium Notes",
    tagline: "Build a personal knowledge base",
    repo: "TriliumNext/Trilium",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Jonathon Fuller",
    msi_publisher: None,
    bundle: Some("com.electron.trilium-notes"),
    site: Some("https://triliumnotes.org/"),
    mac_bundle_name: None,
    mac_team: Some("6549K3VGX9"),
};

pub const KANDO: crate::CraftApp = crate::CraftApp {
    id: "kando",
    name: "Kando",
    tagline: "Open apps and shortcuts from a radial menu",
    repo: "kando-menu/kando",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Simon Schneegans",
    msi_publisher: None,
    bundle: Some("com.electron.kando"),
    site: Some("https://kando.menu/"),
    mac_bundle_name: None,
    mac_team: Some("ASTCNA5X2Y"),
};

pub const RECORDLY: crate::CraftApp = crate::CraftApp {
    id: "recordly",
    name: "Recordly",
    tagline: "Record and edit screen demonstrations",
    repo: "webadderallorg/Recordly",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Fido Tech",
    msi_publisher: None,
    bundle: Some("dev.recordly.app"),
    site: Some("https://recordly.dev/"),
    mac_bundle_name: None,
    mac_team: Some("54QUWA9PZA"),
};

pub const VSCODIUM: crate::CraftApp = crate::CraftApp {
    id: "vscodium",
    name: "VSCodium",
    tagline: "Edit source code with the VSCodium desktop editor",
    repo: "VSCodium/vscodium",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Baptiste Augrain",
    msi_publisher: None,
    bundle: Some("com.vscodium"),
    site: Some("https://vscodium.com/"),
    mac_bundle_name: None,
    mac_team: Some("VC39D2VNQ7"),
};

pub const ANKI: crate::CraftApp = crate::CraftApp {
    id: "anki",
    name: "Anki",
    tagline: "Create and review flashcards",
    repo: "ankitects/anki",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Anki Software LLC",
    msi_publisher: None,
    bundle: Some("net.ankiweb.anki"),
    site: Some("https://apps.ankiweb.net/"),
    mac_bundle_name: None,
    mac_team: Some("ZL66D3NMZM"),
};

pub const OPENCHAMBER: crate::CraftApp = crate::CraftApp {
    id: "openchamber",
    name: "OpenChamber",
    tagline: "OpenCode workspace for desktop",
    repo: "openchamber/openchamber",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Bohdan Tryapitsyn",
    msi_publisher: None,
    bundle: Some("dev.openchamber.desktop"),
    site: Some("https://openchamber.dev/"),
    mac_bundle_name: None,
    mac_team: Some("5J7WJGPA2Q"),
};

pub const CALIBRE: crate::CraftApp = crate::CraftApp {
    id: "calibre",
    name: "calibre",
    tagline: "Manage an ebook library and convert ebooks",
    repo: "kovidgoyal/calibre",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Kovid Goyal",
    msi_publisher: None,
    bundle: Some("net.kovidgoyal.calibre"),
    site: Some("https://calibre-ebook.com/"),
    mac_bundle_name: None,
    mac_team: Some("NTY7FVCEKP"),
};

pub const OSAURUS: crate::CraftApp = crate::CraftApp {
    id: "osaurus",
    name: "Osaurus",
    tagline: "Run local AI models on your Mac",
    repo: "osaurus-ai/osaurus",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Terence Pae",
    msi_publisher: None,
    bundle: Some("com.dinoki.osaurus"),
    site: Some("https://osaurus.ai/"),
    mac_bundle_name: None,
    mac_team: Some("4W8QF9VR2F"),
};

pub const HEADLAMP: crate::CraftApp = crate::CraftApp {
    id: "headlamp",
    name: "Headlamp",
    tagline: "Manage Kubernetes with a desktop dashboard",
    repo: "kubernetes-sigs/headlamp",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "LF Open Source, LLC",
    msi_publisher: None,
    bundle: Some("com.microsoft.Headlamp"),
    site: Some("https://headlamp.dev/"),
    mac_bundle_name: None,
    mac_team: Some("5N2JF58U87"),
};

pub const ZEN: crate::CraftApp = crate::CraftApp {
    id: "zen",
    name: "Zen",
    tagline: "Browse with vertical tabs and workspaces",
    repo: "zen-browser/desktop",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Mauro Baladés",
    msi_publisher: None,
    bundle: Some("app.zen-browser.zen"),
    site: Some("https://zen-browser.app/"),
    mac_bundle_name: None,
    mac_team: Some("9V5K9TP787"),
};

pub const DEEPTUTOR: crate::CraftApp = crate::CraftApp {
    id: "deeptutor",
    name: "DeepTutor",
    tagline: "Study documents and build learning paths",
    repo: "HKUDS/DeepTutor",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "HKUDS contributors",
    msi_publisher: None,
    bundle: Some("org.deeptutor.open-desktop"),
    site: Some("https://github.com/HKUDS/DeepTutor"),
    mac_bundle_name: Some("Open DeepTutor"),
    mac_team: Some("ZP59KG8ABY"),
};

pub const OPENWORK_DESKTOP: crate::CraftApp = crate::CraftApp {
    id: "openwork-desktop",
    name: "OpenWork",
    tagline: "Work with coding agents in a desktop workspace",
    repo: "different-ai/openwork",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Different AI inc.",
    msi_publisher: None,
    bundle: Some("com.differentai.openwork"),
    site: Some("https://openworklabs.com"),
    mac_bundle_name: None,
    mac_team: Some("F5DJWB4CCV"),
};

pub const MISTER_DOWNLOAD_MANAGER: crate::CraftApp = crate::CraftApp {
    id: "mister-download-manager",
    name: "Mister Download Manager",
    tagline: "Manage downloads and queues across browsers",
    repo: "anefzaoui/misterdownloadmanager",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [75, 128, 232],
    ink_light: [32, 82, 177],
    ink_dark: [163, 193, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ahmed Nefzaoui and Mister Download Manager contributors",
    msi_publisher: None,
    bundle: None,
    site: Some("https://misterdownloadmanager.com/"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const MOONLIGHT: crate::CraftApp = crate::CraftApp {
    id: "moonlight",
    name: "Moonlight",
    tagline: "Stream games from your own computer",
    repo: "moonlight-stream/moonlight-qt",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [55, 111, 181],
    ink_light: [28, 74, 132],
    ink_dark: [155, 199, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Cameron Gutman",
    msi_publisher: None,
    bundle: Some("com.moonlight-stream.Moonlight"),
    site: Some("https://moonlight-stream.org/"),
    mac_bundle_name: None,
    mac_team: Some("45U78722YL"),
};

pub const BLENDER: crate::CraftApp = crate::CraftApp {
    id: "blender",
    name: "Blender",
    tagline: "Model, animate and render in 3D",
    repo: "blender/blender",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [216, 112, 35],
    ink_light: [137, 60, 9],
    ink_dark: [252, 186, 122],
    kind: crate::catalog::AppKind::Community,
    publisher: "Blender Foundation",
    msi_publisher: None,
    bundle: Some("org.blenderfoundation.blender"),
    site: Some("https://www.blender.org/"),
    mac_bundle_name: None,
    mac_team: Some("68UA947AUU"),
};

pub const GIMP: crate::CraftApp = crate::CraftApp {
    id: "gimp",
    name: "GIMP",
    tagline: "Edit images and create graphics",
    repo: "GNOME/gimp",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [120, 114, 90],
    ink_light: [83, 64, 42],
    ink_dark: [225, 206, 175],
    kind: crate::catalog::AppKind::Community,
    publisher: "GIMP Team",
    msi_publisher: None,
    bundle: Some("org.gimp.gimp"),
    site: Some("https://www.gimp.org/"),
    mac_bundle_name: None,
    mac_team: Some("T25BQ8HSJF"),
};

pub const KRITA: crate::CraftApp = crate::CraftApp {
    id: "krita",
    name: "Krita",
    tagline: "Paint and animate digitally",
    repo: "KDE/krita",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [95, 144, 206],
    ink_light: [33, 76, 139],
    ink_dark: [159, 199, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Krita Foundation",
    msi_publisher: None,
    bundle: Some("org.krita"),
    site: Some("https://krita.org/"),
    mac_bundle_name: Some("krita"),
    mac_team: Some("DL93766A3G"),
};

pub const VLC: crate::CraftApp = crate::CraftApp {
    id: "vlc",
    name: "VLC",
    tagline: "Play video, music and streams",
    repo: "videolan/vlc",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [209, 104, 19],
    ink_light: [134, 62, 16],
    ink_dark: [254, 184, 116],
    kind: crate::catalog::AppKind::Community,
    publisher: "VideoLAN",
    msi_publisher: None,
    bundle: Some("org.videolan.vlc"),
    site: Some("https://www.videolan.org/vlc/"),
    mac_bundle_name: None,
    mac_team: Some("75GAHG3SZQ"),
};

pub const INKSCAPE: crate::CraftApp = crate::CraftApp {
    id: "inks",
    name: "Inkscape",
    tagline: "Draw and edit vector graphics",
    repo: "inkscape/inkscape",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [53, 54, 58],
    ink_light: [28, 29, 32],
    ink_dark: [225, 226, 229],
    kind: crate::catalog::AppKind::Community,
    publisher: "The Inkscape Community",
    msi_publisher: None,
    bundle: Some("org.inkscape.Inkscape"),
    site: Some("https://inkscape.org/"),
    mac_bundle_name: None,
    mac_team: Some("SW3D6BB6A6"),
};

pub const KDENLIVE: crate::CraftApp = crate::CraftApp {
    id: "kdenlive",
    name: "Kdenlive",
    tagline: "Edit video across multiple tracks",
    repo: "KDE/kdenlive",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [63, 143, 217],
    ink_light: [32, 78, 135],
    ink_dark: [161, 215, 252],
    kind: crate::catalog::AppKind::Community,
    publisher: "KDE",
    msi_publisher: None,
    bundle: Some("org.kde.Kdenlive"),
    site: Some("https://kdenlive.org/"),
    mac_bundle_name: Some("kdenlive"),
    mac_team: Some("5433B4KXM8"),
};

pub const NETNEWSWIRE: crate::CraftApp = crate::CraftApp {
    id: "netnewswire",
    name: "NetNewsWire",
    tagline: "Read feeds with an open source RSS reader",
    repo: "Ranchero-Software/NetNewsWire",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [94, 155, 228],
    ink_light: [23, 78, 150],
    ink_dark: [159, 200, 244],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ranchero Software",
    msi_publisher: None,
    bundle: Some("com.ranchero.NetNewsWire-Evergreen"),
    site: Some("https://netnewswire.com/"),
    mac_bundle_name: None,
    mac_team: Some("M8L2WTLA8W"),
};

pub const MACCY: crate::CraftApp = crate::CraftApp {
    id: "maccy",
    name: "Maccy",
    tagline: "Find and reuse items from your clipboard history",
    repo: "p0deje/Maccy",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [55, 111, 181],
    ink_light: [28, 74, 132],
    ink_dark: [155, 199, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Alexey Rodionov",
    msi_publisher: None,
    bundle: Some("org.p0deje.Maccy"),
    site: Some("https://maccy.app/"),
    mac_bundle_name: None,
    mac_team: Some("MN3X4648SC"),
};

pub const RECTANGLE: crate::CraftApp = crate::CraftApp {
    id: "rectangle",
    name: "Rectangle",
    tagline: "Arrange windows with shortcuts and snap areas",
    repo: "rxhanson/Rectangle",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [55, 111, 181],
    ink_light: [28, 74, 132],
    ink_dark: [155, 199, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Ryan Hanson",
    msi_publisher: None,
    bundle: Some("com.knollsoft.Rectangle"),
    site: Some("https://rectangleapp.com/"),
    mac_bundle_name: None,
    mac_team: Some("XSYZ3E4B7D"),
};

pub const CODEEDIT: crate::CraftApp = crate::CraftApp {
    id: "codeedit",
    name: "CodeEdit",
    tagline: "Edit code in a native macOS workspace",
    repo: "CodeEditApp/CodeEdit",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [55, 111, 181],
    ink_light: [28, 74, 132],
    ink_dark: [155, 199, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "Austin Condiff and CodeEdit contributors",
    msi_publisher: None,
    bundle: Some("app.codeedit.CodeEdit"),
    site: Some("https://codeedit.app/"),
    mac_bundle_name: None,
    mac_team: Some("9VS2VKC5LQ"),
};

pub const OPENTTD: crate::CraftApp = crate::CraftApp {
    id: "openttd",
    name: "OpenTTD",
    tagline: "Build and manage a transport network",
    repo: "OpenTTD/OpenTTD",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [50, 132, 71],
    ink_light: [36, 106, 59],
    ink_dark: [141, 211, 147],
    kind: crate::catalog::AppKind::Community,
    publisher: "OpenTTD Distribution Ltd",
    msi_publisher: None,
    bundle: Some("org.openttd.openttd"),
    site: Some("https://www.openttd.org/"),
    mac_bundle_name: None,
    mac_team: Some("475CDUM4UL"),
};

pub const AIRI: crate::CraftApp = crate::CraftApp {
    id: "airi",
    name: "AIRI",
    tagline: "An interactive voice companion that can run locally",
    repo: "moeru-ai/airi",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [174, 77, 123],
    ink_light: [129, 42, 89],
    ink_dark: [241, 173, 208],
    kind: crate::catalog::AppKind::Community,
    publisher: "MOERU AI LTD",
    msi_publisher: None,
    bundle: Some("ai.moeru.airi"),
    site: Some("https://airi.moeru.ai/"),
    mac_bundle_name: Some("airi"),
    mac_team: Some("433DLLA855"),
};

pub const SUNSHINE: crate::CraftApp = crate::CraftApp {
    id: "sunshine",
    name: "Sunshine",
    tagline: "Host game streams from your own computer",
    repo: "SunshineStream/Sunshine",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [211, 117, 52],
    ink_light: [144, 67, 24],
    ink_dark: [255, 177, 121],
    kind: crate::catalog::AppKind::Community,
    publisher: "LizardByte LLC",
    msi_publisher: None,
    bundle: Some("dev.lizardbyte.app.Sunshine"),
    site: Some("https://app.lizardbyte.dev/Sunshine/"),
    mac_bundle_name: None,
    mac_team: Some("F8XQ7XCN2R"),
};

pub const DUST3D: crate::CraftApp = crate::CraftApp {
    id: "dust3d",
    name: "Dust3D",
    tagline: "Model low-poly objects from profiles",
    repo: "huxingyi/dust3d",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [75, 128, 232],
    ink_light: [32, 82, 177],
    ink_dark: [163, 193, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "Jeremy Hu and contributors",
    msi_publisher: None,
    bundle: Some("dust3d"),
    site: Some("https://dust3d.org/"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const OPEN_PENCIL: crate::CraftApp = crate::CraftApp {
    id: "openpencil",
    name: "OpenPencil",
    tagline: "Design and edit Figma files locally",
    repo: "open-pencil/open-pencil",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [84, 95, 227],
    ink_light: [63, 73, 164],
    ink_dark: [173, 182, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "Danila Poyarkov and OpenPencil contributors",
    msi_publisher: None,
    bundle: Some("net.dannote.open-pencil"),
    site: Some("https://openpencil.dev/"),
    mac_bundle_name: None,
    mac_team: Some("N7D7M3Q3CD"),
};

pub const OPEN_WHISPR: crate::CraftApp = crate::CraftApp {
    id: "openwhispr",
    name: "OpenWhispr Local",
    tagline: "Dictate into desktop apps with local speech recognition",
    repo: "OpenWhispr/openwhispr",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [55, 111, 181],
    ink_light: [28, 74, 132],
    ink_dark: [155, 199, 249],
    kind: crate::catalog::AppKind::Community,
    publisher: "OpenWhispr contributors",
    msi_publisher: None,
    bundle: Some("com.gizmolabs.openwhispr"),
    site: Some("https://openwhispr.com"),
    mac_bundle_name: Some("OpenWhispr"),
    mac_team: Some("T832773L2J"),
};

pub const ULTIMAKER_CURA: crate::CraftApp = crate::CraftApp {
    id: "ultimaker-cura",
    name: "UltiMaker Cura",
    tagline: "Prepare 3D models for printing",
    repo: "Ultimaker/Cura",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "UltiMaker",
    msi_publisher: None,
    bundle: Some("com.ultimaker.cura"),
    site: Some("https://ultimaker.com/en/software/ultimaker-cura"),
    mac_bundle_name: None,
    mac_team: None,
};

pub const PASEO: crate::CraftApp = crate::CraftApp {
    id: "paseo",
    name: "Paseo",
    tagline: "Manage coding agents and project workspaces",
    repo: "getpaseo/paseo",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Mohamed Boudra Ziani",
    msi_publisher: None,
    bundle: Some("sh.paseo.desktop"),
    site: Some("https://paseo.sh/"),
    mac_bundle_name: None,
    mac_team: Some("99ZMJMKU9Y"),
};

pub const FREECAD: crate::CraftApp = crate::CraftApp {
    id: "free",
    name: "FreeCAD",
    tagline: "Create and edit parametric 3D models",
    repo: "FreeCAD/FreeCAD",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "The FreeCAD project association",
    msi_publisher: None,
    bundle: Some("org.freecad.FreeCAD"),
    site: Some("https://www.freecad.org/"),
    mac_bundle_name: None,
    mac_team: Some("289DJRF23X"),
};

pub const OPENMAUSBOT: crate::CraftApp = crate::CraftApp {
    id: "openmausbot",
    name: "OpenMausBot",
    tagline: "Chat with a team of AI assistants",
    repo: "milind-soni/OpenMausBot",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [56, 113, 193],
    ink_light: [30, 82, 148],
    ink_dark: [157, 197, 253],
    kind: crate::catalog::AppKind::Community,
    publisher: "Milind Soni",
    msi_publisher: None,
    bundle: Some("com.openmausbot.app"),
    site: Some("https://www.mausbot.com/"),
    mac_bundle_name: None,
    mac_team: Some("993D98NH4J"),
};

pub const ZOTERO: crate::CraftApp = crate::CraftApp {
    id: "zotero",
    name: "Zotero",
    tagline: "Collect, organize, annotate and cite research",
    repo: "zotero/zotero",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [199, 52, 60],
    ink_light: [155, 30, 41],
    ink_dark: [255, 169, 173],
    kind: crate::catalog::AppKind::Community,
    publisher: "Corporation for Digital Scholarship",
    msi_publisher: None,
    bundle: Some("org.zotero.zotero"),
    site: Some("https://www.zotero.org/"),
    mac_bundle_name: None,
    mac_team: Some("8LAYR367YV"),
};

pub const DESKREEN: crate::CraftApp = crate::CraftApp {
    id: "deskreen",
    name: "Deskreen Community Edition",
    tagline: "Share a screen or window with a browser on the same network",
    repo: "pavlobu/deskreen",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [53, 120, 223],
    ink_light: [34, 78, 151],
    ink_dark: [178, 208, 255],
    kind: crate::catalog::AppKind::Community,
    publisher: "Pavlo Buidenkov",
    msi_publisher: None,
    bundle: Some("com.deskreen-ce.app"),
    site: Some("https://deskreen.com/"),
    mac_bundle_name: Some("Deskreen CE"),
    mac_team: Some("3MGY23CG64"),
};

pub const SUBTITLE_EDIT: crate::CraftApp = crate::CraftApp {
    id: "subtitle-edit",
    name: "Subtitle Edit",
    tagline: "Create, synchronize and edit subtitles",
    repo: "SubtitleEdit/subtitleedit",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [66, 149, 215],
    ink_light: [30, 79, 120],
    ink_dark: [157, 216, 248],
    kind: crate::catalog::AppKind::Community,
    publisher: "Nikolaj Olsson",
    msi_publisher: None,
    bundle: Some("dk.nikse.subtitleedit"),
    site: Some("https://github.com/SubtitleEdit/subtitleedit"),
    mac_bundle_name: None,
    mac_team: Some("MFC9L2T4RB"),
};

pub const DEEPCHAT: crate::CraftApp = crate::CraftApp {
    id: "deepchat",
    name: "DeepChat",
    tagline: "Chat with local and cloud models from your desktop",
    repo: "ThinkInAIXYZ/deepchat",
    former_repo: None,
    former_ids: &[],
    former_names: &[],
    former_bundles: &[],
    color: [104, 102, 218],
    ink_light: [58, 55, 128],
    ink_dark: [198, 193, 254],
    kind: crate::catalog::AppKind::Community,
    publisher: "ThinkInAI / DeepChat contributors",
    msi_publisher: None,
    bundle: Some("com.wefonk.deepchat"),
    site: Some("https://deepchat.thinkinai.xyz/"),
    mac_bundle_name: None,
    mac_team: Some("Y7P5QLKLYG"),
};

pub(crate) static COMMUNITY_RECIPES: &[crate::CraftApp] = &[
    GHOST_DOWNLOADER,
    HANDY,
    LOCALSEND,
    OBS_STUDIO,
    KOPIA,
    TRANSMISSION,
    IINA,
    HANDBRAKE,
    MEETILY,
    RIZZO_PII,
    AUDACITY,
    KEEPASSXC,
    LIBREOFFICE,
    JOPLIN,
    SHOTCUT,
    PINTA,
    ZED,
    DRAWIO,
    STATS,
    WHISPERING,
    ENTE_AUTH,
    CRYPTOMATOR,
    ACTIVITYWATCH,
    GODOT,
    TERMIX,
    NEOVIDE,
    KEYCASTR,
    PICARD,
    LAPCE,
    VORTA,
    VIBE,
    OPENSCREEN,
    SOLVESPACE,
    ACTUAL_BUDGET,
    APPFLOWY,
    BLOCKBENCH,
    DOSBOX_STAGING,
    PENCIL2D,
    OPENSHOT,
    MUSESCORE,
    ORCA_SLICER,
    DBEAVER_CE,
    PRUSA_SLICER,
    LOSSLESSCUT,
    ZETTLR,
    ONLYOFFICE,
    NEXTCLOUD_DESKTOP,
    NOTEPAD_NEXT,
    RAPIDRAW,
    RUSTDESK,
    SUPER_PRODUCTIVITY,
    DB_BROWSER_SQLITE,
    NAPS2,
    OPENCLAW,
    OPENCODE,
    OLLAMA,
    CLASH_VERGE_REV,
    ORCA,
    VOICE_STUDIO,
    T3_CODE,
    ANYTHING_LLM,
    UPSCAYL,
    BRUNO,
    JAN,
    LOCALAI,
    KAP,
    SIMPLEX_CHAT,
    SIYUAN,
    MAGPIE,
    TRILIUM,
    KANDO,
    RECORDLY,
    VSCODIUM,
    ANKI,
    OPENCHAMBER,
    CALIBRE,
    OSAURUS,
    HEADLAMP,
    ZEN,
    DEEPTUTOR,
    OPENWORK_DESKTOP,
    PASEO,
    ULTIMAKER_CURA,
    DUST3D,
    OPEN_PENCIL,
    OPEN_WHISPR,
    FREECAD,
    OPENMAUSBOT,
    ZOTERO,
    DESKREEN,
    MISTER_DOWNLOAD_MANAGER,
    MOONLIGHT,
    SUNSHINE,
    AIRI,
    OPENTTD,
    RECTANGLE,
    CODEEDIT,
    MACCY,
    BLENDER,
    GIMP,
    NETNEWSWIRE,
    KRITA,
    VLC,
    INKSCAPE,
    KDENLIVE,
    SUBTITLE_EDIT,
    DEEPCHAT,
];

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
pub static NATIVE_APPS: [crate::CraftApp; crate::SUITE.len() + COMMUNITY_RECIPES.len()] = native_apps();

/// AppImage recipes that completed install, launch, update and removal on Linux x86_64.
/// Editorial rows and release metadata cannot add recipes to this list.
pub fn verified_linux_appimage(app: &crate::CraftApp, platform: crate::Platform) -> bool {
    if platform.os != crate::Os::Linux || platform.arch != crate::Arch::X64 {
        return false;
    }
    if [
        LOCALSEND,
        HANDY,
        PENCIL2D,
        OPENSHOT,
        MUSESCORE,
        ORCA_SLICER,
        LOSSLESSCUT,
        ZETTLR,
        ONLYOFFICE,
        AUDACITY,
        DRAWIO,
        NEXTCLOUD_DESKTOP,
        CRYPTOMATOR,
        JOPLIN,
        NOTEPAD_NEXT,
        RAPIDRAW,
        RUSTDESK,
        ACTIVITYWATCH,
        GHOST_DOWNLOADER,
    ]
    .iter()
    .any(|candidate| (app.id, app.repo) == (candidate.id, candidate.repo))
    {
        return true;
    }
    matches!(
        (app.id, app.repo),
        ("anicraft", "storytold/anicraft")
            | ("cadcraft", "storytold/cadcraft")
            | ("craft-anything", "storytold/craft-anything")
            | ("deckcraft", "storytold/deckcraft")
            | ("designcraft", "storytold/designcraft")
            | ("effectcraft", "storytold/effectcraft")
            | ("figcraft", "storytold/figcraft")
            | ("filmcraft", "storytold/filmcraft")
            | ("gridcraft", "storytold/gridcraft")
            | ("lightcraft", "storytold/lightcraft")
            | ("mailcraft", "UCSLabs/mailcraft")
            | ("pdfcraft", "storytold/pdfcraft")
            | ("photocraft", "storytold/photocraft")
            | ("solidcraft", "tangency-labs/solidcraft")
            | ("soundcraft", "storytold/soundcraft")
            | ("texturecraft", "storytold/texturecraft")
            | ("vectorcraft", "storytold/vectorcraft")
            | ("wordcraft", "storytold/wordcraft")
    )
}

#[cfg(test)]
mod recipe_tests {
    #[test]
    fn godot_recipe_accepts_only_the_official_stable_zip_with_digest() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("godot-engine").expect("compiled Godot recipe");
        assert_eq!(app.repo, "godotengine/godot");
        let repo = app.release_repository();
        assert_eq!(repo, super::GODOT_RELEASES_REPO);
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let release = release::release_from_sums(repo, "4.7.2-stable", &[]);
        let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).expect("official Mac ZIP");
        assert_eq!(kind, PackageKind::MacZip);
        assert_eq!(release.version_label(), "4.7.2");
        assert_eq!(asset.size, 170_622_178);
        for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }] {
            assert!(platform::pick_asset(&release, &[app.id], host, host.download_kinds()).is_none());
        }
        let mut wrong = asset.clone();
        wrong.sha256 = None;
        assert!(!crate::release::godot_mac_asset(&release, &wrong));
        wrong = asset.clone();
        wrong.url = wrong.url.replace(repo, app.repo);
        assert!(!crate::release::godot_mac_asset(&release, &wrong));
        wrong = asset.clone();
        wrong.name = wrong.name.replace("stable", "rc1");
        assert!(!crate::release::godot_mac_asset(&release, &wrong));
        let minor_release =
            release::release_from_sums(repo, "4.7-stable", &[(asset.sha256.clone().unwrap(), "Godot_v4.7-stable_macos.universal.zip".into())]);
        assert_eq!(minor_release.version_label(), "4.7.0");
    }

    #[test]
    fn vibe_recipe_accepts_only_the_official_apple_silicon_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("vibe-transcribe").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("github.com.thewh1teagle.vibe"), Some("VSTAWC326G")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, name, size) in [("v3.3.5", "vibe_3.3.5_aarch64.dmg", 38_715_249), ("v3.3.8", "vibe_3.3.8_aarch64.dmg", 38_640_733)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            assert_eq!((asset.name.as_str(), asset.size, kind), (name, size, PackageKind::Dmg));
            for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }, Platform { os: Os::Windows, ..mac }] {
                assert!(platform::pick_asset(&release, &[app.id], host, host.download_kinds()).is_none());
            }
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let bad_checksum = release::Asset { sha256: Some("f".repeat(64)), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            let wrong_repo = release::Asset { url: asset.url.replace("thewh1teagle", "mirror"), ..asset.clone() };
            let intel = release::Asset { name: asset.name.replace("aarch64", "x64"), url: asset.url.replace("aarch64", "x64"), ..asset.clone() };
            for different in [different_size, bad_checksum, no_checksum, wrong_repo, intel] {
                assert!(!release::vibe_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::vibe_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn openscreen_recipe_accepts_only_the_official_apple_silicon_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("openscreen").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("com.etiennelescot.openscreen"), Some("M4LK7C6S84")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, size) in [("v2.3.0", 141_157_755), ("v2.4.0", 141_308_945)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            let version = release.version.as_ref().unwrap();
            assert_eq!(asset.name, format!("Openscreen-macOS-Apple-Silicon-{version}.dmg"));
            assert_eq!((asset.size, kind), (size, PackageKind::Dmg));
            for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }, Platform { os: Os::Windows, ..mac }] {
                assert!(platform::pick_asset(&release, &[app.id], host, host.download_kinds()).is_none());
            }
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let wrong_tag = release::Asset { url: asset.url.replace(tag, "v9.0.0"), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            let wrong_repo = release::Asset { url: asset.url.replace("getopenscreen", "mirror"), ..asset.clone() };
            let intel = release::Asset {
                name: asset.name.replace("Apple-Silicon", "Intel"),
                url: asset.url.replace("Apple-Silicon", "Intel"),
                ..asset.clone()
            };
            for different in [different_size, wrong_tag, no_checksum, wrong_repo, intel] {
                assert!(!release::openscreen_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::openscreen_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn dosbox_staging_recipe_accepts_only_the_official_stable_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("dosbox-staging").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("io.github.dosbox-staging"), Some("944SS92GAH")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, size) in [("v0.83.0", 44_531_761), ("v0.83.1", 44_185_617)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            assert_eq!(asset.name, format!("dosbox-staging-macOS-{tag}.dmg"));
            assert_eq!((asset.size, kind), (size, PackageKind::Dmg));
            for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }, Platform { os: Os::Windows, ..mac }] {
                assert!(platform::pick_asset(&release, &[app.id], host, host.download_kinds()).is_none());
            }
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let wrong_tag = release::Asset { url: asset.url.replace(tag, "v0.83.2"), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            let wrong_repo = release::Asset { url: asset.url.replace("/dosbox-staging/", "/mirror/"), ..asset.clone() };
            for different in [different_size, wrong_tag, no_checksum, wrong_repo] {
                assert!(!release::dosbox_staging_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::dosbox_staging_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn blockbench_selects_only_the_official_silicon_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("blockbench").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("blockbench"), Some("5QMG7G7462")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, size) in [("v5.2.1", 128_702_946), ("v5.2.2", 128_721_695)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            assert_eq!(asset.name, format!("Blockbench_arm64_{}.dmg", &tag[1..]));
            assert_eq!((asset.size, kind), (size, PackageKind::Dmg));
            let intel = Platform { arch: Arch::X64, ..mac };
            assert!(platform::pick_asset(&release, &[app.id], intel, intel.download_kinds()).is_none());
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let wrong_tag = release::Asset { url: asset.url.replace(tag, "v5.2.3"), ..asset.clone() };
            let intel_asset = release::Asset { name: asset.name.replace("arm64", "x64"), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            for different in [different_size, wrong_tag, intel_asset, no_checksum] {
                assert!(!release::blockbench_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::blockbench_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn appflowy_selects_only_the_official_silicon_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("appflowy").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("com.appflowy.appflowy.flutter"), Some("VHB67HRSZG")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, size) in [("0.15.0", 119_452_957), ("0.15.1", 119_286_434)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            assert_eq!(asset.name, format!("AppFlowy-{tag}-macos-arm64.dmg"));
            assert_eq!((asset.size, kind), (size, PackageKind::Dmg));
            let intel = Platform { arch: Arch::X64, ..mac };
            assert!(platform::pick_asset(&release, &[app.id], intel, intel.download_kinds()).is_none());
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let wrong_tag = release::Asset { url: asset.url.replace(tag, "0.15.2"), ..asset.clone() };
            let intel_asset = release::Asset { name: asset.name.replace("arm64", "x86_64"), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            for different in [different_size, wrong_tag, intel_asset, no_checksum] {
                assert!(!release::appflowy_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::appflowy_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn actual_budget_selects_only_the_official_silicon_dmg() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id("actual").unwrap();
        assert_eq!((app.bundle, app.mac_team), (Some("com.actualbudget.actual"), Some("79ANZ983YF")));
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (tag, size) in [("v26.10.0", 143_479_378), ("v26.11.0", 145_818_730)] {
            let release = release::release_from_sums(app.repo, tag, &[]);
            let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).unwrap();
            assert_eq!(asset.name, "Actual-mac-arm64.dmg");
            assert_eq!((asset.size, kind), (size, PackageKind::Dmg));
            let intel = Platform { arch: Arch::X64, ..mac };
            assert!(platform::pick_asset(&release, &[app.id], intel, intel.download_kinds()).is_none());
            let different_size = release::Asset { size: size + 1, ..asset.clone() };
            let wrong_tag = release::Asset { url: asset.url.replace(tag, "v26.12.0"), ..asset.clone() };
            let intel_asset = release::Asset { name: asset.name.replace("arm64", "x64"), ..asset.clone() };
            let no_checksum = release::Asset { sha256: None, ..asset.clone() };
            for different in [different_size, wrong_tag, intel_asset, no_checksum] {
                assert!(!release::actual_budget_mac_asset(&release, &different));
            }
            let prerelease = release::Release { prerelease: true, ..release.clone() };
            assert!(!release::actual_budget_mac_asset(&prerelease, asset));
        }
    }

    #[test]
    fn termix_recipe_accepts_only_the_signed_channel_asset_on_apple_silicon() {
        use crate::{Arch, Os, PackageKind, Platform, platform, release};

        let app = super::TERMIX;
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let release = release::release_from_sums(app.repo, "v26.11.3", &[]);
        let (asset, kind) = platform::pick_asset(&release, &[app.id], mac, mac.download_kinds()).expect("official ARM64 DMG");
        assert_eq!(kind, PackageKind::Dmg);
        assert_eq!(release.version_label(), "26.11.3");
        assert_eq!(asset.size, 263_900_889);
        for host in [Platform { arch: Arch::X64, ..mac }, Platform { os: Os::Linux, ..mac }, Platform { os: Os::Windows, ..mac }] {
            assert!(platform::pick_asset(&release, &[app.id], host, host.download_kinds()).is_none());
        }
        assert!(release::release_from_sums(app.repo, "sdk-v2.11.0", &[]).version.is_none());
        let bad_version = release::Release { version: crate::version::parse("26.11.2"), ..release.clone() };
        assert!(!release::termix_mac_asset(&bad_version, asset));
        let bad_artifact = release::Asset { url: asset.url.replace("v26.11.3", "v26.11.2"), ..asset.clone() };
        assert!(!release::termix_mac_asset(&release, &bad_artifact));
        let missing_digest = release::Asset { sha256: None, ..asset.clone() };
        assert!(!release::termix_mac_asset(&release, &missing_digest));
    }

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
        assert!(super::verified_linux_appimage(&super::HANDY, linux));
        assert!(!super::verified_linux_appimage(&super::HANDY, arm));
        for id in ["craft-anything", "designcraft", "mailcraft", "solidcraft"] {
            let app = CraftApp::by_id(id).unwrap();
            assert!(super::verified_linux_appimage(app, linux));
            assert!(!super::verified_linux_appimage(app, arm));
        }
        assert_eq!(super::NATIVE_APPS.iter().filter(|app| super::verified_linux_appimage(app, linux)).count(), 37);
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
    fn libreoffice_checks_official_arm64_path_and_digest() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let app = CraftApp::by_id(super::LIBREOFFICE.id).unwrap();
        assert_eq!(app.bundle, Some("org.libreoffice.script"));
        assert_eq!(app.mac_team, Some("7P5S3ZLCN7"));
        let version = release::libreoffice_stable_version("26.8.3").unwrap();
        let (name, url) = release::libreoffice_arm64_dmg(&version);
        let asset = release::Asset { name, url, size: 299_927_705, sha256: Some("a".repeat(64)) };
        let verified = release::Release { tag: version.to_string(), version: Some(version), assets: vec![asset.clone()], ..Default::default() };
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let selected = |release: &release::Release, platform| {
            platform::pick_asset(release, &[app.id], platform, &[PackageKind::Dmg]).map(|(asset, kind)| (asset.clone(), kind))
        };
        assert_eq!(selected(&verified, mac), Some((asset.clone(), PackageKind::Dmg)));
        assert!(selected(&verified, Platform { arch: Arch::X64, ..mac }).is_none());
        assert!(selected(&verified, Platform { os: Os::Linux, ..mac }).is_none());
        assert!(selected(&verified, Platform { os: Os::Windows, ..mac }).is_none());
        for asset in [
            release::Asset { url: asset.url.replace("26.8.3/", "26.8.2/"), ..asset.clone() },
            release::Asset { url: asset.url.replace("https://", "http://"), ..asset.clone() },
            release::Asset { name: asset.name.replace("aarch64", "x86-64"), ..asset.clone() },
            release::Asset { sha256: None, ..asset.clone() },
            release::Asset { sha256: Some("wrong".into()), ..asset.clone() },
            release::Asset { size: 0, ..asset.clone() },
        ] {
            assert!(selected(&release::Release { assets: vec![asset], ..verified.clone() }, mac).is_none());
        }
        assert!(selected(&release::Release { prerelease: true, ..verified.clone() }, mac).is_none());
        assert!(selected(&release::Release { tag: "26.8.3.1".into(), ..verified.clone() }, mac).is_none());
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
            (super::JOPLIN, ["v3.7.22", "v3.7.24"], ["Joplin-3.7.22-arm64.DMG", "Joplin-3.7.24-arm64.DMG"], "net.cozic.joplin-desktop", "A9BXAFS6CT"),
            (
                super::SHOTCUT,
                ["v26.9.27", "v26.11.15"],
                ["shotcut-macos-26.9.27.dmg", "shotcut-macos-26.11.15.dmg"],
                "com.Meltytech.Shotcut",
                "Y6RX44QG2G",
            ),
            (
                super::DRAWIO,
                ["v32.6.10", "v32.7.4"],
                ["draw.io-arm64-32.6.10.dmg", "draw.io-arm64-32.7.4.dmg"],
                "com.jgraph.drawio.desktop",
                "UZEUFB4N53",
            ),
            (
                super::WHISPERING,
                ["v7.9.0", "v7.11.0"],
                ["Whispering_7.9.0_aarch64.dmg", "Whispering_7.11.0_aarch64.dmg"],
                "com.bradenwong.whispering",
                "3C7J97QA5Z",
            ),
            (
                super::ENTE_AUTH,
                ["auth-v4.4.26", "auth-v4.4.27"],
                ["ente-auth-v4.4.26.dmg", "ente-auth-v4.4.27.dmg"],
                "io.ente.auth.mac",
                "6Z68YJY9Q2",
            ),
            (
                super::CRYPTOMATOR,
                ["1.19.6", "1.19.7"],
                ["Cryptomator-1.19.6-arm64.dmg", "Cryptomator-1.19.7-arm64.dmg"],
                "org.cryptomator",
                "YZQJQUHA3L",
            ),
            (
                super::ACTIVITYWATCH,
                ["v0.14.2", "v0.14.3"],
                ["activitywatch-v0.14.2-macos-arm64.dmg", "activitywatch-v0.14.3-macos-arm64.dmg"],
                "net.activitywatch.ActivityWatch",
                "XM9GC3SUL2",
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
                let linux = Platform { os: Os::Linux, arch: Arch::X64 };
                assert_eq!(
                    select(&official, linux).map(|(_, kind)| kind),
                    super::verified_linux_appimage(&app, linux).then_some(PackageKind::AppImage),
                );
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
            (super::JOPLIN, "Joplin-3.7.24.DMG"),
            (super::JOPLIN, "Joplin-3.7.24-arm64.dmg.blockmap"),
            (super::JOPLIN, "Joplin-3.7.21-arm64.DMG"),
            (super::JOPLIN, "Joplin-3.7.24-beta.1-arm64.DMG"),
            (super::SHOTCUT, "shotcut-macos-unsigned-26.11.15.dmg"),
            (super::SHOTCUT, "shotcut-macos-26.11.15-x86_64.dmg"),
            (super::SHOTCUT, "shotcut-macos-26.11.15-beta.1.dmg"),
            (super::DRAWIO, "draw.io-x64-32.7.4.dmg"),
            (super::DRAWIO, "draw.io-universal-32.7.4.dmg"),
            (super::DRAWIO, "draw.io-arm64-32.7.4-beta.1.dmg"),
            (super::WHISPERING, "Whispering_7.10.0_x64.dmg"),
            (super::WHISPERING, "Whispering_7.11.0-beta.1_aarch64.dmg"),
            (super::WHISPERING, "Epicenter_7.11.0_aarch64.dmg"),
            (super::ENTE_AUTH, "ente-auth-v4.4.27-beta.1.dmg"),
            (super::ENTE_AUTH, "ente-v4.4.27.dmg"),
            (super::CRYPTOMATOR, "Cryptomator-1.19.7-x64.dmg"),
            (super::CRYPTOMATOR, "Cryptomator-1.19.7-beta.1-arm64.dmg"),
            (super::CRYPTOMATOR, "Cryptomator-1.19.7+other-arm64.dmg"),
            (super::ACTIVITYWATCH, "activitywatch-v0.14.3-macos-x86_64.dmg"),
            (super::ACTIVITYWATCH, "activitywatch-v0.14.3-beta.1-macos-arm64.dmg"),
            (super::ACTIVITYWATCH, "activitywatch-tauri-v0.14.3-macos-arm64.dmg"),
            (super::ACTIVITYWATCH, "activitywatch-research-v0.14.3-macos-arm64.dmg"),
        ] {
            assert!(platform::parse_asset_name(other_asset, &[app.id]).is_none());
        }
        assert_eq!(super::AUDACITY.mac_bundle_names().collect::<Vec<_>>(), ["Audacity 4"]);
        assert!(!super::AUDACITY.bundle_ids().any(|id| id == "org.audacityteam.audacity"));
        let cryptomator = super::CRYPTOMATOR;
        let mut wrong_tag = release::release_from_sums(cryptomator.repo, "1.19.7", &[]);
        wrong_tag.tag.insert(0, 'v');
        for asset in &mut wrong_tag.assets {
            asset.url = asset.url.replace("/1.19.7/", "/v1.19.7/");
        }
        assert!(platform::pick_asset(&wrong_tag, &[cryptomator.id], mac, mac.download_kinds()).is_none());
    }

    #[test]
    fn reviewed_mac_recipes_use_only_their_apple_silicon_release_packages() {
        use crate::release::release_from_sums;
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform::pick_asset};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (id, tags, kind) in [
            ("keycastr", ["v0.11.0", "v0.11.1"], PackageKind::MacZip),
            ("musicbrainz-picard", ["release-3.0.4", "release-3.0.5"], PackageKind::Dmg),
            ("vorta", ["v0.12.1", "v0.12.2"], PackageKind::Dmg),
        ] {
            let app = CraftApp::by_id(id).unwrap();
            assert!(app.installs(kind));
            for tag in tags {
                let release = release_from_sums(app.repo, tag, &[]);
                let valid = pick_asset(&release, &[id], mac, &[kind]).unwrap();
                assert_eq!(valid.1, kind);
                assert!(pick_asset(&release, &[id], Platform { arch: Arch::X64, ..mac }, &[kind]).is_none());
                assert!(pick_asset(&release, &[id], mac, &[PackageKind::AppImage]).is_none());
                let mut invalid = release.clone();
                invalid.prerelease = true;
                assert!(pick_asset(&invalid, &[id], mac, &[kind]).is_none());
                let mut invalid = release.clone();
                invalid.assets[0].sha256 = Some("0".repeat(64));
                assert!(pick_asset(&invalid, &[id], mac, &[kind]).is_none());
                let mut invalid = release.clone();
                invalid.assets[0].url = invalid.assets[0].url.replace(app.repo, "third-party/mirror");
                assert!(pick_asset(&invalid, &[id], mac, &[kind]).is_none());
                let mut invalid = release;
                invalid.tag = tags.into_iter().find(|other| *other != tag).unwrap().into();
                assert!(pick_asset(&invalid, &[id], mac, &[kind]).is_none());
            }
        }
    }

    #[test]
    fn unversioned_arm64_assets_are_bound_to_their_official_stable_tags() {
        use crate::{Arch, CraftApp, Os, PackageKind, Platform, platform, release};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (id, tags, filename, bundle, team, wrong_arch_filename) in [
            ("pinta", ["3.1.1", "3.1.2"], "Pinta-macos-arm64.dmg", "com.ximian.pinta", "D5G6C56TBH", "Pinta-macos-x86_64.dmg"),
            ("zed", ["v1.29.3", "v1.30.2"], "Zed-aarch64.dmg", "dev.zed.Zed", "MQ55VZLNZQ", "Zed-x86_64.dmg"),
            ("stats", ["v3.1.0", "v3.1.1"], "Stats.dmg", "eu.exelban.Stats", "RP2S87B72W", "Stats-intel.dmg"),
            ("lapce", ["v0.4.5", "v0.4.6"], "Lapce-macos.dmg", "io.lapce", "CYSGAZFR8D", "Lapce-macos-x86_64.dmg"),
            ("solvespace", ["v3.2", "v3.3"], "SolveSpace.dmg", "com.solvespace", "8X77K9NDG3", "SolveSpace-x86_64.dmg"),
            (
                "neovide",
                ["0.17.3", "0.17.4"],
                "Neovide-aarch64-apple-darwin.dmg",
                "com.neovide.neovide",
                "X8CNW77992",
                "Neovide-x86_64-apple-darwin.dmg",
            ),
        ] {
            let app = CraftApp::by_id(id).expect("compiled recipe");
            let installable =
                |release: &release::Release, host: Platform| platform::pick_asset(release, &[app.id], host, host.download_kinds()).is_some();
            assert_eq!(app.bundle, Some(bundle));
            assert_eq!(app.mac_team, Some(team));
            for tag in tags {
                let official = release::release_from_sums(app.repo, tag, &[]);
                let (asset, kind) = platform::pick_asset(&official, &[app.id], mac, mac.download_kinds()).expect("reviewed DMG");
                assert_eq!((asset.name.as_str(), kind), (filename, PackageKind::Dmg));
                assert!(asset.sha256.as_deref().is_some_and(release::is_hex64));
                assert!(asset.size > 0);
                for host in [
                    Platform { os: Os::MacOs, arch: Arch::X64 },
                    Platform { os: Os::Linux, arch: Arch::X64 },
                    Platform { os: Os::Windows, arch: Arch::Arm64 },
                ] {
                    assert!(!installable(&official, host));
                }
                let mut wrong = official.clone();
                wrong.assets[0].url = wrong.assets[0].url.replace(tag, "v1.0.0");
                assert!(!installable(&wrong, mac));
                let mut wrong = official.clone();
                wrong.assets[0].name = wrong_arch_filename.into();
                wrong.assets[0].url = wrong.assets[0].url.replace(filename, wrong_arch_filename);
                assert!(!installable(&wrong, mac));
                let mut wrong = official.clone();
                wrong.assets[0].url = wrong.assets[0].url.replace(app.repo, "mirror/community");
                assert!(!installable(&wrong, mac));
                let mut wrong = official.clone();
                wrong.assets[0].sha256 = None;
                assert!(!installable(&wrong, mac));
                let mut wrong = official.clone();
                wrong.prerelease = true;
                assert!(!installable(&wrong, mac));
                if [super::STATS.id, super::ZED.id, super::NEOVIDE.id].contains(&id) {
                    let mut wrong = official.clone();
                    wrong.tag = match tag.strip_prefix('v') {
                        Some(bare) => bare.into(),
                        None => format!("v{tag}"),
                    };
                    wrong.assets[0].url = wrong.assets[0].url.replace(tag, &wrong.tag);
                    assert!(!installable(&wrong, mac));
                }
                let mut wrong = official;
                wrong.version = crate::version::parse("8.0.0");
                assert!(!installable(&wrong, mac));
            }
        }
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
        let assets = [
            "Handy_0.10.2_aarch64.dmg",
            "Handy_0.10.2_x64.dmg",
            "Handy_0.10.2_x64_en-US.msi",
            "Handy_0.10.2_amd64.AppImage",
            "Handy_0.10.2_aarch64.AppImage",
        ];
        let json = serde_json::json!({"tag_name":"v0.10.2", "assets":assets.map(|name| serde_json::json!({
            "name":name, "browser_download_url":format!("https://github.com/cjpais/Handy/releases/download/v0.10.2/{name}"),
            "size":match name { "Handy_0.10.2_amd64.AppImage" => 124_029_432, "Handy_0.10.2_aarch64.dmg" => 14_979_688, _ => 0 }
        }))})
        .to_string();
        let release = crate::release::parse_github_release(&json, "cjpais/Handy").unwrap();
        for os in [crate::Os::MacOs, crate::Os::Windows, crate::Os::Linux] {
            assert!(super::HANDY.runs_on(os));
            for arch in [crate::Arch::X64, crate::Arch::Arm64] {
                let platform = crate::Platform { os, arch };
                let chosen = crate::platform::pick_asset(&release, &["handy"], platform, platform.download_kinds());
                if os == crate::Os::MacOs && arch == crate::Arch::Arm64 {
                    let (asset, kind) = chosen.expect("native Apple Silicon disk image");
                    assert_eq!(asset.name, "Handy_0.10.2_aarch64.dmg");
                    assert_eq!(kind, crate::PackageKind::Dmg);
                    let mut intel_only = release.clone();
                    intel_only.assets.retain(|asset| asset.name == "Handy_0.10.2_x64.dmg");
                    assert_eq!(intel_only.assets.len(), 1);
                    assert!(crate::platform::pick_asset(&intel_only, &["handy"], platform, platform.download_kinds()).is_none());
                } else if os == crate::Os::Linux && arch == crate::Arch::X64 {
                    let (asset, kind) = chosen.expect("official Linux x86_64 disk image");
                    assert_eq!(asset.name, "Handy_0.10.2_amd64.AppImage");
                    assert_eq!(kind, crate::PackageKind::AppImage);
                    assert_eq!(asset.size, 124_029_432);
                    assert!(asset.sha256.is_some());
                    let mut wrong_checksum = release.clone();
                    wrong_checksum.assets.iter_mut().find(|asset| asset.name == "Handy_0.10.2_amd64.AppImage").unwrap().sha256 = Some("0".repeat(64));
                    assert!(crate::platform::pick_asset(&wrong_checksum, &["handy"], platform, platform.download_kinds()).is_none());
                } else {
                    assert!(chosen.is_none());
                }
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
