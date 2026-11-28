//! Platforms, package formats, and choosing the right release asset.
//!
//! Every Craft App release names its files `<app>-<version>-<os>-<arch>[-portable].<ext>`, for
//! example `photocraft-0.5.0-windows-x64.msi` or `photocraft-0.5.0-linux-aarch64.AppImage`.
//!
//! Labels are English. The UI translates [`PackageKind::label`]; OS and architecture names are
//! shown as they are.

use serde::{Deserialize, Serialize};

use crate::release::{Asset, Release};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Os {
    Windows,
    MacOs,
    Linux,
    FreeBsd,
}

impl Os {
    pub fn label(self) -> &'static str {
        match self {
            Self::Windows => "Windows",
            Self::MacOs => "macOS",
            Self::Linux => "Linux",
            Self::FreeBsd => "FreeBSD",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Arch {
    X64,
    X86,
    Arm64,
    Riscv64,
}

impl Arch {
    pub fn label(self) -> &'static str {
        match self {
            Self::X64 => "x64",
            Self::X86 => "x86",
            Self::Arm64 => "ARM64",
            Self::Riscv64 => "RISC-V",
        }
    }
}

/// A package format a release ships.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackageKind {
    /// Windows Installer package (per user by default; an existing per-machine copy stays per-machine).
    Msi,
    /// Windows zip with the exe at the top level.
    PortableZip,
    /// macOS disk image with the `.app` bundle.
    Dmg,
    /// A signed macOS .app bundle in a ZIP from an approved release source.
    MacZip,
    /// An approved macOS PKG from which the launcher extracts one verified application.
    MacPkg,
    AppImage,
    Deb,
    Rpm,
    /// Arch Linux package (`.pkg.tar.zst`).
    ArchPkg,
    Flatpak,
    /// Linux or FreeBSD tarball (`bin/<app>`, `share/…`).
    TarGz,
}

impl PackageKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Msi => "Installer (.msi)",
            Self::PortableZip => "Portable (.zip)",
            Self::Dmg => "Disk image (.dmg)",
            Self::MacZip => "App archive (.zip)",
            Self::MacPkg => "Application package (.pkg)",
            Self::AppImage => "AppImage",
            Self::Deb => "Debian package (.deb)",
            Self::Rpm => "RPM package (.rpm)",
            Self::ArchPkg => "Arch package (.pkg.tar.zst)",
            Self::Flatpak => "Flatpak bundle",
            Self::TarGz => "Tarball (.tar.gz)",
        }
    }

    fn suffix(self) -> &'static str {
        match self {
            Self::Msi => ".msi",
            Self::PortableZip => "-portable.zip",
            Self::Dmg => ".dmg",
            Self::MacZip => ".zip",
            Self::MacPkg => ".pkg",
            Self::AppImage => ".AppImage",
            Self::Deb => ".deb",
            Self::Rpm => ".rpm",
            Self::ArchPkg => ".pkg.tar.zst",
            Self::Flatpak => ".flatpak",
            Self::TarGz => ".tar.gz",
        }
    }
}

/// Where the launcher is running (or, on the web, the visitor's machine).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Platform {
    pub os: Os,
    pub arch: Arch,
}

impl Platform {
    /// The platform this binary was built for, or `None` for wasm and unsupported targets.
    pub fn current() -> Option<Self> {
        let os = if cfg!(target_os = "windows") {
            Os::Windows
        } else if cfg!(target_os = "macos") {
            Os::MacOs
        } else if cfg!(target_os = "linux") {
            Os::Linux
        } else if cfg!(target_os = "freebsd") {
            Os::FreeBsd
        } else {
            return None;
        };
        let arch = if cfg!(target_arch = "x86_64") {
            Arch::X64
        } else if cfg!(target_arch = "x86") {
            Arch::X86
        } else if cfg!(target_arch = "aarch64") {
            Arch::Arm64
        } else if cfg!(target_arch = "riscv64") {
            Arch::Riscv64
        } else {
            return None;
        };
        Some(Self { os, arch })
    }

    pub fn label(self) -> String {
        match self.os {
            Os::MacOs => "macOS".into(),
            os => format!("{} {}", os.label(), self.arch.label()),
        }
    }

    /// Asset platform tokens to try, best first. Windows can fall back to emulated builds.
    /// macOS prefers universal builds and only accepts packages with code for the current CPU.
    fn tokens(self) -> &'static [&'static str] {
        match (self.os, self.arch) {
            (Os::Windows, Arch::X64) => &["windows-x64", "windows-x86"],
            (Os::Windows, Arch::X86) => &["windows-x86"],
            (Os::Windows, Arch::Arm64) => &["windows-arm64", "windows-x64", "windows-x86"],
            (Os::MacOs, Arch::X64) => &["macos-universal", "macos-x86_64"],
            (Os::MacOs, Arch::Arm64) => &["macos-universal", "macos-aarch64", "macos-arm64"],
            (Os::Linux, Arch::X64) => &["linux-x86_64"],
            (Os::Linux, Arch::Arm64) => &["linux-aarch64"],
            (Os::Linux, Arch::Riscv64) => &["linux-riscv64"],
            (Os::FreeBsd, Arch::X64) => &["freebsd-x86_64"],
            (Os::FreeBsd, Arch::Arm64) => &["freebsd-aarch64"],
            (Os::Windows | Os::MacOs | Os::FreeBsd, _) | (Os::Linux, Arch::X86) => &[],
        }
    }

    /// Package formats the launcher installs on this OS, in order of preference.
    pub fn default_kinds(self) -> &'static [PackageKind] {
        match self.os {
            Os::Windows => &[PackageKind::Msi, PackageKind::PortableZip],
            Os::MacOs => &[PackageKind::Dmg, PackageKind::MacZip, PackageKind::MacPkg],
            Os::Linux => &[PackageKind::AppImage, PackageKind::TarGz],
            Os::FreeBsd => &[PackageKind::TarGz],
        }
    }

    /// Every format offered as a manual download on this OS (the web build's download menu).
    pub fn download_kinds(self) -> &'static [PackageKind] {
        match self.os {
            Os::Windows => &[PackageKind::Msi, PackageKind::PortableZip],
            Os::MacOs => &[PackageKind::Dmg, PackageKind::MacZip, PackageKind::MacPkg],
            Os::Linux => &[PackageKind::AppImage, PackageKind::Deb, PackageKind::Rpm, PackageKind::ArchPkg, PackageKind::Flatpak, PackageKind::TarGz],
            Os::FreeBsd => &[PackageKind::TarGz],
        }
    }
}

/// What an asset name says about itself.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetName {
    pub stem: String,
    pub version: String,
    pub token: String,
    pub kind: PackageKind,
}

impl AssetName {
    /// The architecture the asset is built for: the last part of its token (`windows-x86` → x86).
    /// `None` for a token that names none (`macos-universal`).
    pub fn arch(&self) -> Option<Arch> {
        match self.token.rsplit('-').next()? {
            "x64" | "x86_64" => Some(Arch::X64),
            "x86" => Some(Arch::X86),
            "arm64" | "aarch64" => Some(Arch::Arm64),
            "riscv64" => Some(Arch::Riscv64),
            _ => None,
        }
    }
}

/// Strips `<stem><sep>` (ASCII case-insensitively) from the start of `base`, returning the rest.
fn strip_stem_prefix<'a>(base: &'a str, stem: &str, sep: char) -> Option<&'a str> {
    if base.get(..stem.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(stem)) && base.as_bytes().get(stem.len()) == Some(&(sep as u8)) {
        Some(&base[stem.len() + 1..])
    } else {
        None
    }
}

fn community_dmg_format(stem: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match stem {
        id if id == crate::curated::LOCALSEND.id => Some(("LocalSend-", "", "macos-universal")),
        id if id == crate::curated::OBS_STUDIO.id => Some(("OBS-Studio-", "-macOS-Apple", "macos-aarch64")),
        id if id == crate::curated::KOPIA.id => Some(("KopiaUI-", "-arm64", "macos-aarch64")),
        id if id == crate::curated::TRANSMISSION.id => Some(("Transmission-", "", "macos-universal")),
        id if id == crate::curated::IINA.id => Some(("IINA.v", "", "macos-universal")),
        id if id == crate::curated::HANDBRAKE.id => Some(("HandBrake-", "", "macos-universal")),
        id if id == crate::curated::MEETILY.id => Some(("meetily_", "_aarch64", "macos-aarch64")),
        id if id == crate::curated::RIZZO_PII.id => Some(("Rizzo-PII-", "-macOS-arm64", "macos-aarch64")),
        id if id == crate::curated::AUDACITY.id => Some(("audacity-macOS-", "-arm64", "macos-aarch64")),
        id if id == crate::curated::KEEPASSXC.id => Some(("KeePassXC-", "-arm64", "macos-aarch64")),
        id if id == crate::curated::LIBREOFFICE.id => Some(("LibreOffice_", "_MacOS_aarch64", "macos-aarch64")),
        id if id == crate::curated::JOPLIN.id => Some(("Joplin-", "-arm64", "macos-aarch64")),
        id if id == crate::curated::SHOTCUT.id => Some(("shotcut-macos-", "", "macos-universal")),
        id if id == crate::curated::DRAWIO.id => Some(("draw.io-arm64-", "", "macos-aarch64")),
        id if id == crate::curated::WHISPERING.id => Some(("Whispering_", "_aarch64", "macos-aarch64")),
        id if id == crate::curated::ENTE_AUTH.id => Some(("ente-auth-v", "", "macos-universal")),
        id if id == crate::curated::CRYPTOMATOR.id => Some(("Cryptomator-", "-arm64", "macos-aarch64")),
        id if id == crate::curated::ACTIVITYWATCH.id => Some(("activitywatch-v", "-macos-arm64", "macos-aarch64")),
        _ => None,
    }
}

fn valid_community_dmg_version(stem: &str, version: &str) -> bool {
    use crate::curated::{ACTIVITYWATCH, AUDACITY, CRYPTOMATOR, JOPLIN, KEEPASSXC};

    crate::version::parse(version).is_some_and(|parsed| {
        parsed.to_string() == version
            && parsed.pre.is_empty()
            && match stem {
                id if id == AUDACITY.id => parsed.major == 4,
                id if [ACTIVITYWATCH.id, CRYPTOMATOR.id].contains(&id) => parsed.build.is_empty(),
                id if id == KEEPASSXC.id => (parsed.major, parsed.minor, parsed.patch) >= (2, 7, 12),
                id if id == JOPLIN.id => parsed.build.is_empty() && (parsed.major, parsed.minor, parsed.patch) >= (3, 7, 22),
                _ => true,
            }
    })
}

/// Reads `<stem>-<version>-<os>-<arch>[-portable].<ext>` or Studio's `<Stem>_<version>_<arch>[_<locale>].<ext>`;
/// `None` for checksums, zsync files, CLI and web bundles, or anything else.
pub fn parse_asset_name(name: &str, stems: &[&str]) -> Option<AssetName> {
    const KINDS: [PackageKind; 10] = [
        PackageKind::PortableZip,
        PackageKind::MacZip,
        PackageKind::TarGz,
        PackageKind::Msi,
        PackageKind::Dmg,
        PackageKind::AppImage,
        PackageKind::Deb,
        PackageKind::Rpm,
        PackageKind::ArchPkg,
        PackageKind::Flatpak,
    ];
    // Joplin publishes its signed macOS images with a capitalized extension.
    let joplin_dmg = name
        .strip_suffix(".DMG")
        .filter(|base| base.starts_with("Joplin-") && stems.contains(&crate::curated::JOPLIN.id))
        .map(|base| (PackageKind::Dmg, base));
    let (kind, base) = KINDS.iter().find_map(|k| name.strip_suffix(k.suffix()).map(|b| (*k, b))).or(joplin_dmg)?;
    if kind == PackageKind::MacZip {
        let version = base.strip_prefix("Godot_v")?.strip_suffix("-stable_macos.universal")?;
        let stem = crate::curated::GODOT.id;
        return (stems.contains(&stem) && crate::release::godot_stable_version(&format!("{version}-stable")).is_some()).then(|| AssetName {
            stem: stem.into(),
            version: version.into(),
            token: "macos-universal".into(),
            kind,
        });
    }
    for stem in stems {
        if *stem == crate::curated::GHOST_DOWNLOADER.id
            && kind == PackageKind::AppImage
            && let Some(version) = base.strip_prefix("Ghost-Downloader-v").and_then(|rest| rest.strip_suffix("-Linux-x86_64"))
            && crate::version::parse(version).is_some_and(|parsed| parsed.to_string() == version && parsed.pre.is_empty() && parsed.build.is_empty())
        {
            return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token: "linux-x86_64".into(), kind });
        }
        if *stem == crate::curated::LOCALSEND.id {
            // LocalSend publishes a hyphenated architecture label. Recognize only this recipe's reviewed format.
            if kind == PackageKind::AppImage
                && let Some(version) = strip_stem_prefix(base, stem, '-').and_then(|rest| rest.strip_suffix("-linux-x86-64"))
                && version.starts_with(|c: char| c.is_ascii_digit())
                && crate::version::parse(version).is_some()
            {
                return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token: "linux-x86_64".into(), kind });
            }
        }
        if let Some((prefix, suffix, token)) = community_dmg_format(stem) {
            // The publisher replaced its original 2.8.0 DMG with the -1 build under the same tag.
            let suffix = if *stem == crate::curated::KEEPASSXC.id && base.starts_with("KeePassXC-2.8.0-") { "-1-arm64" } else { suffix };
            if kind == PackageKind::Dmg
                && let Some(version) = base.strip_prefix(prefix).and_then(|rest| rest.strip_suffix(suffix))
                && valid_community_dmg_version(stem, version)
            {
                return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token: token.into(), kind });
            }
            continue;
        }
        if let Some(rest) = strip_stem_prefix(base, stem, '-') {
            // rest = <version>-<os>-<arch>; os and arch never contain '-', versions may.
            let mut parts = rest.rsplitn(3, '-');
            let (Some(arch), Some(os), Some(version)) = (parts.next(), parts.next(), parts.next()) else { continue };
            if crate::version::parse(version).is_none() || !version.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token: format!("{os}-{arch}"), kind });
        }
        if let Some(rest) = strip_stem_prefix(base, stem, '_') {
            // Studio / Tauri format: <version>_<arch>[_<locale>] where <arch> may be `x86_64`.
            let Some((version, suffix)) = rest.split_once('_') else { continue };
            if crate::version::parse(version).is_none() || !version.starts_with(|c: char| c.is_ascii_digit()) {
                continue;
            }
            let arch_token = if suffix == "x86_64" || suffix.starts_with("x86_64_") { "x86_64" } else { suffix.split('_').next().unwrap_or("") };
            let token = match (kind, arch_token) {
                (PackageKind::Dmg, "universal") => Some("macos-universal".to_string()),
                (PackageKind::Dmg, "x86_64" | "x64") => Some("macos-x86_64".to_string()),
                (PackageKind::Dmg, "aarch64" | "arm64") => Some("macos-aarch64".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "x86_64" | "x64") => Some("windows-x64".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "x86") => Some("windows-x86".to_string()),
                (PackageKind::Msi | PackageKind::PortableZip, "arm64" | "aarch64") => Some("windows-arm64".to_string()),
                _ => None,
            };
            if let Some(token) = token {
                return Some(AssetName { stem: (*stem).to_string(), version: version.to_string(), token, kind });
            }
        }
    }
    None
}

type AssetMatcher = fn(&Release, &Asset) -> bool;

fn reviewed_mac_archive(stems: &[&str]) -> Option<(PackageKind, AssetMatcher)> {
    use crate::{curated, release};

    let recipes: [(&str, PackageKind, AssetMatcher); 14] = [
        (curated::NAPS2.id, PackageKind::MacPkg, release::naps2_mac_asset),
        (curated::KEYCASTR.id, PackageKind::MacZip, release::keycastr_mac_asset),
        (curated::MACCY.id, PackageKind::MacZip, release::maccy_mac_asset),
        (curated::NETNEWSWIRE.id, PackageKind::MacZip, release::netnewswire_mac_asset),
        (curated::ALT_TAB.id, PackageKind::MacZip, release::alt_tab_mac_asset),
        (curated::AMETHYST.id, PackageKind::MacZip, release::amethyst_mac_asset),
        (curated::HIDDEN_BAR.id, PackageKind::MacZip, release::hidden_bar_mac_asset),
        (curated::SWIFTBAR.id, PackageKind::MacZip, release::swiftbar_mac_asset),
        (curated::GITHUB_DESKTOP.id, PackageKind::MacZip, release::github_desktop_mac_asset),
        (curated::ITERM2.id, PackageKind::MacZip, release::iterm2_mac_asset),
        (curated::LOOP.id, PackageKind::MacZip, release::loop_mac_asset),
        (curated::DYAD.id, PackageKind::MacZip, release::dyad_mac_asset),
        (curated::HAMMERSPOON.id, PackageKind::MacZip, release::hammerspoon_mac_asset),
        (curated::ICE.id, PackageKind::MacZip, release::ice_mac_asset),
    ];
    recipes.into_iter().find(|(id, _, _)| stems.contains(id)).map(|(_, kind, matches_asset)| (kind, matches_asset))
}

/// The best asset of `release` for `platform`, trying `kinds` in order and, within a kind, the
/// platform's tokens in order (native before emulated).
pub fn pick_asset<'a>(release: &'a Release, stems: &[&str], platform: Platform, kinds: &[PackageKind]) -> Option<(&'a Asset, PackageKind)> {
    use crate::curated::{
        ACTIVITYWATCH, AUDACITY, CRYPTOMATOR, DRAWIO, ENTE_AUTH, HANDBRAKE, IINA, JOPLIN, KEEPASSXC, KOPIA, LOCALSEND, MEETILY, NEOVIDE, OBS_STUDIO,
        PINTA, RIZZO_PII, SHOTCUT, STATS, TERMIX, TRANSMISSION, WHISPERING, ZED,
    };

    use crate::curated::NEXTCLOUD_DESKTOP;

    if !recipe_platform_verified(stems, platform) {
        return None;
    }
    if platform.os == Os::Windows
        && let Some(repo) = stems.iter().find_map(|id| crate::release::reviewed_windows_msi_repo(id))
    {
        let kind = *kinds.iter().find(|&&kind| kind == PackageKind::Msi)?;
        return release.assets.iter().find(|asset| crate::release::reviewed_windows_msi_asset(repo, release, asset)).map(|asset| (asset, kind));
    }
    if platform.os == Os::Linux
        && let Some(app) = crate::release::qualified_linux_appimage_recipe(stems)
            .or_else(|| [crate::curated::GHOST_DOWNLOADER, crate::curated::HANDY].iter().find(|app| stems.contains(&app.id)))
    {
        kinds.iter().find(|kind| **kind == PackageKind::AppImage)?;
        return release
            .assets
            .iter()
            .find(|asset| match app.id {
                "ghost-downloader" => crate::release::ghost_downloader_linux_asset(release, asset),
                "handy" => crate::release::handy_linux_asset(release, asset),
                _ => crate::release::qualified_linux_appimage_asset(app, release, asset),
            })
            .map(|asset| (asset, PackageKind::AppImage));
    }
    if platform.os == Os::MacOs
        && let Some(app) = crate::release::qualified_mac_dmg_recipe(stems)
    {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release
            .assets
            .iter()
            .find(|asset| crate::release::qualified_mac_dmg_asset(app, release, asset))
            .map(|asset| (asset, PackageKind::Dmg));
    }
    if let Some((kind, matches_asset)) = reviewed_mac_archive(stems) {
        if !kinds.contains(&kind) {
            return None;
        }
        return release.assets.iter().find(|asset| matches_asset(release, asset)).map(|asset| (asset, kind));
    }
    if stems.contains(&crate::curated::LAPCE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::lapce_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::SUPER_PRODUCTIVITY.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release
            .assets
            .iter()
            .find(|asset| crate::release::super_productivity_mac_asset(release, asset))
            .map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::DB_BROWSER_SQLITE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release
            .assets
            .iter()
            .find(|asset| crate::release::db_browser_sqlite_mac_asset(release, asset))
            .map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::BLOCKBENCH.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::blockbench_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::APPFLOWY.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::appflowy_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::ACTUAL_BUDGET.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::actual_budget_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::OPENSCREEN.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::openscreen_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::VIBE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::vibe_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::DOSBOX_STAGING.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::dosbox_staging_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::SOLVESPACE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::solvespace_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::VORTA.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::vorta_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&crate::curated::PICARD.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::picard_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    if stems.contains(&NEOVIDE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::neovide_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    use crate::curated::NOTEPAD_NEXT;
    use crate::curated::RAPIDRAW;
    use crate::curated::RUSTDESK;
    if platform.os == Os::Linux
        && ["drawio", "audacity", "nextcloud-desktop", "cryptomator", "joplin", "notepadnext", "rapidraw", "rustdesk", "activitywatch"]
            .iter()
            .any(|id| stems.contains(id))
    {
        let app = [AUDACITY, DRAWIO, NEXTCLOUD_DESKTOP, CRYPTOMATOR, JOPLIN, NOTEPAD_NEXT, RAPIDRAW, RUSTDESK, ACTIVITYWATCH]
            .into_iter()
            .find(|app| stems.contains(&app.id))?;
        if release.prerelease || !kinds.contains(&PackageKind::AppImage) {
            return None;
        }
        let version = release.version.as_ref().filter(|version| version.pre.is_empty() && version.build.is_empty())?;
        let (tag_prefix, expected_name) = match app.id {
            "audacity" => ("Audacity-", format!("audacity-linux-{version}-x86_64.AppImage")),
            "drawio" => ("v", format!("drawio-x86_64-{version}.AppImage")),
            "nextcloud-desktop" => ("v", format!("Nextcloud-{version}-x86_64.AppImage")),
            "activitywatch" => ("v", "activitywatch-linux-x86_64.AppImage".to_owned()),
            "rustdesk" => ("", format!("rustdesk-{version}-x86_64.AppImage")),
            "rapidraw" => ("v", format!("03_RapidRAW_v{version}_ubuntu-22.04_amd64.AppImage")),
            "notepadnext" if version.patch == 0 => ("v", format!("NotepadNext-v{}.{}-x86_64.AppImage", version.major, version.minor)),
            "cryptomator" => ("", format!("cryptomator-{version}-x86_64.AppImage")),
            "joplin" => ("v", format!("Joplin-{version}.AppImage")),
            _ => return None,
        };
        let tag_version = if app.id == "notepadnext" { format!("{}.{}", version.major, version.minor) } else { version.to_string() };
        if release.tag != format!("{tag_prefix}{tag_version}") {
            return None;
        }
        let expected_url = format!("{}{}/{expected_name}", crate::release::download_prefix(app.release_repository()), release.tag);
        return release
            .assets
            .iter()
            .find(|asset| {
                asset.name == expected_name
                    && asset.url == expected_url
                    && asset.size > 0
                    && asset.sha256.as_deref().is_some_and(crate::release::is_hex64)
            })
            .map(|asset| (asset, PackageKind::AppImage));
    }
    use crate::curated::{LOSSLESSCUT, MUSESCORE, ONLYOFFICE, OPENSHOT, ORCA_SLICER, PENCIL2D, ZETTLR};
    if let Some(app) = [PENCIL2D, OPENSHOT, MUSESCORE, ORCA_SLICER, LOSSLESSCUT, ZETTLR, ONLYOFFICE].into_iter().find(|app| stems.contains(&app.id)) {
        if release.prerelease || !kinds.contains(&PackageKind::AppImage) {
            return None;
        }
        let version = release.version.as_ref().filter(|version| version.pre.is_empty() && version.build.is_empty())?;
        if release.tag.strip_prefix('v') != Some(version.to_string().as_str()) {
            return None;
        }
        let exact_name = match app.id {
            "pencil2d" => Some(format!("pencil2d-linux-amd64-v{version}.AppImage")),
            "openshot-video-editor" => Some(format!("OpenShot-v{version}-x86_64.AppImage")),
            "orca-slicer" => Some(format!("OrcaSlicer_Linux_AppImage_Ubuntu2404_V{version}.AppImage")),
            "losslesscut" => Some("LosslessCut-linux-x86_64.AppImage".into()),
            "zettlr" => Some(format!("Zettlr-{version}-x86_64.AppImage")),
            "onlyoffice" => Some("DesktopEditors-x86_64.AppImage".into()),
            _ => None,
        };
        let versioned_musescore = format!("MuseScore-Studio-{version}.");
        let url_prefix = format!("{}{}/", crate::release::download_prefix(app.repo), release.tag);
        return release
            .assets
            .iter()
            .find(|asset| {
                let expected_name = exact_name.as_ref().map_or_else(
                    || {
                        asset
                            .name
                            .strip_prefix(&versioned_musescore)
                            .and_then(|rest| rest.strip_suffix("-x86_64.AppImage"))
                            .is_some_and(|build| build.len() == 9 && build.bytes().all(|byte| byte.is_ascii_digit()))
                    },
                    |name| asset.name == *name,
                );
                expected_name
                    && asset.url.strip_prefix(&url_prefix) == Some(asset.name.as_str())
                    && asset.size > 0
                    && asset.sha256.as_deref().is_some_and(crate::release::is_hex64)
            })
            .map(|asset| (asset, PackageKind::AppImage));
    }
    if stems.contains(&TERMIX.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::termix_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
    }
    // These Apple Silicon-compatible images keep the same filename across releases. The stable tag in
    // the canonical URL binds the package; installation checks the signed bundle.
    if let Some((app, filename)) =
        [(PINTA, "Pinta-macos-arm64.dmg"), (ZED, "Zed-aarch64.dmg"), (STATS, "Stats.dmg")].into_iter().find(|(app, _)| stems.contains(&app.id))
    {
        if !kinds.contains(&PackageKind::Dmg)
            || release.prerelease
            || !release.version.as_ref().is_some_and(|version| {
                version.pre.is_empty()
                    && version.build.is_empty()
                    && crate::version::parse(&release.tag).as_ref() == Some(version)
                    && (![STATS.id, ZED.id].contains(&app.id) || release.tag.strip_prefix('v') == Some(version.to_string().as_str()))
            })
        {
            return None;
        }
        let expected = format!("{}{}/{filename}", crate::release::download_prefix(app.repo), release.tag);
        return release
            .assets
            .iter()
            .find(|asset| {
                asset.name == filename && asset.url == expected && asset.size > 0 && asset.sha256.as_deref().is_some_and(crate::release::is_hex64)
            })
            .map(|asset| (asset, PackageKind::Dmg));
    }
    let parsed: Vec<(&Asset, AssetName)> = release
        .assets
        .iter()
        .filter_map(|asset| parse_asset_name(&asset.name, stems).map(|name| (asset, name)))
        .filter(|(asset, name)| {
            if name.kind == PackageKind::MacZip {
                return crate::release::godot_mac_asset(release, asset);
            }
            if name.stem == crate::curated::LIBREOFFICE.id {
                return crate::release::libreoffice_arm64_asset(release, asset);
            }
            if name.stem == ENTE_AUTH.id {
                return !release.prerelease
                    && crate::release::ente_auth_tag_from_download(&asset.url).as_deref() == Some(release.tag.as_str())
                    && release.tag.strip_prefix("auth-v") == Some(name.version.as_str())
                    && release.version.as_ref().is_some_and(|version| version.to_string() == name.version)
                    && asset.size > 0
                    && asset.sha256.as_deref().is_some_and(crate::release::is_hex64);
            }
            let official = [
                KOPIA,
                TRANSMISSION,
                IINA,
                HANDBRAKE,
                MEETILY,
                RIZZO_PII,
                AUDACITY,
                KEEPASSXC,
                JOPLIN,
                SHOTCUT,
                DRAWIO,
                WHISPERING,
                CRYPTOMATOR,
                ACTIVITYWATCH,
            ]
            .iter()
            .find(|app| app.id == name.stem)
            .map(|app| app.repo);
            if !official.is_none_or(|repo| {
                !release.prerelease && asset.url == format!("{}{}/{}", crate::release::download_prefix(repo), release.tag, asset.name)
            }) {
                return false;
            }
            let tag_prefix = match name.stem.as_str() {
                id if id == CRYPTOMATOR.id => Some(""),
                id if [ACTIVITYWATCH.id, JOPLIN.id, SHOTCUT.id, DRAWIO.id, WHISPERING.id].contains(&id) => Some("v"),
                _ => None,
            };
            if tag_prefix.is_some_and(|prefix| {
                release.tag.strip_prefix(prefix) != Some(name.version.as_str())
                    || !asset.sha256.as_deref().is_some_and(crate::release::is_hex64)
                    || asset.size == 0
            }) {
                return false;
            }
            let needs_checksum = official.is_some() || [LOCALSEND.id, OBS_STUDIO.id].contains(&name.stem.as_str());
            !needs_checksum
                || (release.version.as_ref().is_some_and(|version| name.version == version.to_string())
                    && (asset.sha256.is_some() || release.sums_asset().is_some()))
        })
        .collect();
    first_compatible_asset(&parsed, platform, kinds)
}

fn first_compatible_asset<'a>(parsed: &[(&'a Asset, AssetName)], platform: Platform, kinds: &[PackageKind]) -> Option<(&'a Asset, PackageKind)> {
    for kind in kinds {
        let compatible = platform.tokens().iter().find_map(|token| parsed.iter().find(|(_, name)| name.kind == *kind && name.token == *token));
        if let Some((asset, _)) = compatible {
            return Some((asset, *kind));
        }
    }
    None
}

/// Recipe validation is separate from the publisher's list of supported platforms.
pub fn recipe_platform_verified(stems: &[&str], platform: Platform) -> bool {
    use crate::curated::{
        ACTIVITYWATCH, AUDACITY, CRYPTOMATOR, DRAWIO, ENTE_AUTH, GODOT, HANDBRAKE, HANDY, IINA, JOPLIN, KEEPASSXC, KOPIA, LIBREOFFICE, LOCALSEND,
        MEETILY, NEOVIDE, OBS_STUDIO, PINTA, RIZZO_PII, SHOTCUT, STATS, TERMIX, TRANSMISSION, WHISPERING, ZED,
    };

    let mac_arm64 = Platform { os: Os::MacOs, arch: Arch::Arm64 };
    let linux_x64 = Platform { os: Os::Linux, arch: Arch::X64 };
    if platform.os == Os::Windows && stems.iter().any(|id| crate::release::reviewed_windows_msi_repo(id).is_some()) {
        return platform.arch == Arch::X64;
    }
    if ["drawio", "audacity", "nextcloud-desktop", "cryptomator", "joplin", "notepadnext", "rapidraw", "rustdesk", "activitywatch"]
        .iter()
        .any(|id| stems.contains(id))
        && platform.os == Os::Linux
    {
        return platform == linux_x64;
    }
    if crate::release::qualified_linux_appimage_recipe(stems).is_some() {
        return platform == linux_x64
            || (platform == mac_arm64 && (stems.contains(&crate::curated::DYAD.id) || crate::release::qualified_mac_dmg_recipe(stems).is_some()));
    }
    if [
        crate::curated::DBEAVER_CE.id,
        crate::curated::GHOST_DOWNLOADER.id,
        crate::curated::LOSSLESSCUT.id,
        crate::curated::MUSESCORE.id,
        crate::curated::ORCA_SLICER.id,
        crate::curated::PRUSA_SLICER.id,
        crate::curated::ZETTLR.id,
    ]
    .iter()
    .any(|id| stems.contains(id))
    {
        return platform == mac_arm64 || platform == linux_x64;
    }
    if crate::release::qualified_mac_dmg_recipe(stems).is_some() {
        return platform == mac_arm64;
    }
    if ["nextcloud-desktop", "notepadnext", "rapidraw", "rustdesk"].iter().any(|id| stems.contains(id)) {
        return false;
    }
    if [
        OBS_STUDIO.id,
        KOPIA.id,
        TRANSMISSION.id,
        IINA.id,
        HANDBRAKE.id,
        MEETILY.id,
        RIZZO_PII.id,
        AUDACITY.id,
        KEEPASSXC.id,
        LIBREOFFICE.id,
        JOPLIN.id,
        SHOTCUT.id,
        PINTA.id,
        ZED.id,
        DRAWIO.id,
        STATS.id,
        WHISPERING.id,
        ENTE_AUTH.id,
        CRYPTOMATOR.id,
        ACTIVITYWATCH.id,
        GODOT.id,
        TERMIX.id,
        NEOVIDE.id,
        crate::curated::KEYCASTR.id,
        crate::curated::MACCY.id,
        crate::curated::NETNEWSWIRE.id,
        crate::curated::ALT_TAB.id,
        crate::curated::AMETHYST.id,
        crate::curated::HIDDEN_BAR.id,
        crate::curated::SWIFTBAR.id,
        crate::curated::GITHUB_DESKTOP.id,
        crate::curated::ITERM2.id,
        crate::curated::LOOP.id,
        crate::curated::DYAD.id,
        crate::curated::HAMMERSPOON.id,
        crate::curated::ICE.id,
        crate::curated::PICARD.id,
        crate::curated::LAPCE.id,
        crate::curated::VORTA.id,
        crate::curated::SOLVESPACE.id,
        crate::curated::DOSBOX_STAGING.id,
        crate::curated::VIBE.id,
        crate::curated::OPENSCREEN.id,
        crate::curated::ACTUAL_BUDGET.id,
        crate::curated::APPFLOWY.id,
        crate::curated::BLOCKBENCH.id,
        crate::curated::DB_BROWSER_SQLITE.id,
        crate::curated::NAPS2.id,
        crate::curated::SUPER_PRODUCTIVITY.id,
    ]
    .iter()
    .any(|id| stems.contains(id))
    {
        return platform == mac_arm64;
    }
    use crate::curated::{MUSESCORE, ONLYOFFICE, OPENSHOT, ORCA_SLICER, PENCIL2D};
    if [PENCIL2D, OPENSHOT, MUSESCORE, ORCA_SLICER, ONLYOFFICE].iter().any(|app| stems.contains(&app.id)) {
        return platform == linux_x64;
    }
    if stems.contains(&LOCALSEND.id) || stems.contains(&HANDY.id) {
        return platform == mac_arm64 || platform == linux_x64;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_x64_selects_only_reviewed_msi_receipts() {
        use crate::release::release_from_sums;

        let windows = Platform { os: Os::Windows, arch: Arch::X64 };
        for (id, releases) in [
            ("photocraft", [("v0.23.0", "photocraft-0.23.0-windows-x64.msi"), ("v0.23.1", "photocraft-0.23.1-windows-x64.msi")]),
            ("soundcraft", [("v0.20.0", "soundcraft-0.20.0-windows-x64.msi"), ("v0.20.1", "soundcraft-0.20.1-windows-x64.msi")]),
            ("texturecraft", [("v0.6.0", "texturecraft-0.6.0-windows-x64.msi"), ("v0.6.1", "texturecraft-0.6.1-windows-x64.msi")]),
            ("wordcraft", [("v0.18.0", "wordcraft-0.18.0-windows-x64.msi"), ("v0.19.1", "wordcraft-0.19.1-windows-x64.msi")]),
            ("keepassxc", [("2.7.12", "KeePassXC-2.7.12-Win64.msi"), ("2.8.0", "KeePassXC-2.8.0-x64.msi")]),
            ("drawio", [("v31.5.2", "draw.io-31.5.2.msi"), ("v32.7.4", "draw.io-32.7.4.msi")]),
        ] {
            let repo = crate::release::reviewed_windows_msi_repo(id).unwrap();
            let select = |release: &Release, platform, kinds: &[PackageKind]| {
                pick_asset(release, &[id], platform, kinds).map(|(asset, kind)| (asset.clone(), kind))
            };
            for (tag, name) in releases {
                let mut release = release_from_sums(repo, tag, &[]);
                let (verified, kind) = select(&release, windows, &[PackageKind::PortableZip, PackageKind::Msi]).unwrap();
                assert_eq!(kind, PackageKind::Msi);
                assert_eq!(verified.name, name);
                assert!(verified.sha256.as_deref().is_some_and(crate::release::is_hex64));
                for arch in [Arch::X86, Arch::Arm64] {
                    assert!(select(&release, Platform { arch, ..windows }, &[PackageKind::Msi]).is_none());
                }
                assert!(select(&release, windows, &[PackageKind::PortableZip]).is_none());
                for substituted in [
                    Asset { sha256: None, ..verified.clone() },
                    Asset { sha256: Some("a".repeat(64)), ..verified.clone() },
                    Asset { size: verified.size + 1, ..verified.clone() },
                    Asset { url: verified.url.replace(repo, "other/installer"), ..verified.clone() },
                    Asset { name: verified.name.replace(".msi", "-x86.msi"), ..verified.clone() },
                    Asset { name: verified.name.replace(".msi", "-portable.zip"), ..verified.clone() },
                ] {
                    release.assets = vec![substituted];
                    assert!(select(&release, windows, &[PackageKind::Msi, PackageKind::PortableZip]).is_none());
                }
                release.assets = vec![verified];
                release.prerelease = true;
                assert!(select(&release, windows, &[PackageKind::Msi]).is_none());
                release.prerelease = false;
                release.version = crate::version::parse("99.0.0");
                assert!(select(&release, windows, &[PackageKind::Msi]).is_none());
            }
            let future = release_from_sums(repo, "v99.0.0", &[("a".repeat(64), format!("{id}-99.0.0-windows-x64.msi"))]);
            assert!(select(&future, windows, &[PackageKind::Msi]).is_none());
        }
    }

    #[test]
    fn libreoffice_windows_x64_uses_only_pinned_vendor_msis() {
        use crate::{curated::LIBREOFFICE, release};

        let windows = Platform { os: Os::Windows, arch: Arch::X64 };
        let select = |release: &Release, platform| pick_asset(release, &[LIBREOFFICE.id], platform, &[PackageKind::Msi]).is_some();
        for tag in ["26.8.2", "26.8.3"] {
            let version = release::libreoffice_stable_version(tag).unwrap();
            let exact = release::libreoffice_windows_msi(&version).unwrap();
            let mut release = Release { version: Some(version), tag: tag.to_string(), assets: vec![exact.clone()], ..Release::default() };
            assert!(select(&release, windows));
            assert!(!select(&release, Platform { arch: Arch::Arm64, ..windows }));
            assert!(!select(&release, Platform { arch: Arch::X86, ..windows }));
            for substituted in [
                Asset { name: exact.name.replace("x86-64", "x86"), ..exact.clone() },
                Asset { url: exact.url.replace("download.documentfoundation.org", "other.example"), ..exact.clone() },
                Asset { sha256: None, ..exact.clone() },
                Asset { sha256: Some("a".repeat(64)), ..exact.clone() },
                Asset { size: exact.size + 1, ..exact.clone() },
            ] {
                release.assets = vec![substituted];
                assert!(!select(&release, windows));
            }
            release.assets = vec![exact];
            release.prerelease = true;
            assert!(!select(&release, windows));
            release.prerelease = false;
            release.tag = format!("v{tag}");
            assert!(!select(&release, windows));
        }
        assert!(release::libreoffice_windows_msi(&crate::version::Version::new(26, 8, 4)).is_none());
    }

    #[test]
    fn super_productivity_macos_requires_stable_tag_and_official_arm64_asset() {
        use crate::curated::SUPER_PRODUCTIVITY;
        use crate::release::release_from_sums;

        let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let select = |release: &Release, target| pick_asset(release, &[SUPER_PRODUCTIVITY.id], target, &[PackageKind::Dmg]).is_some();
        for tag in ["v20.0.0", "v20.1.0"] {
            let mut release = release_from_sums(SUPER_PRODUCTIVITY.repo, tag, &[]);
            assert!(select(&release, platform));
            assert!(!select(&release, Platform { arch: Arch::X64, ..platform }));
            release.prerelease = true;
            assert!(!select(&release, platform));
            release.prerelease = false;
            release.assets[0].sha256 = None;
            assert!(!select(&release, platform));
        }
        let mut release = release_from_sums(SUPER_PRODUCTIVITY.repo, "v20.1.0", &[]);
        let wrong_tag = release.assets[0].url.replace("/v20.1.0/", "/v20.0.0/");
        let canonical_url = std::mem::replace(&mut release.assets[0].url, wrong_tag);
        assert!(!select(&release, platform));
        release.assets[0].url = canonical_url.replace("superProductivity-arm64.dmg", "superProductivity-x64.dmg");
        release.assets[0].name = "superProductivity-x64.dmg".into();
        assert!(!select(&release, platform));
    }

    #[test]
    fn db_browser_sqlite_selects_only_version_bound_verified_macos_assets() {
        use crate::curated::DB_BROWSER_SQLITE;
        use crate::release::release_from_sums;

        let platform = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let select = |release: &Release, target| pick_asset(release, &[DB_BROWSER_SQLITE.id], target, &[PackageKind::Dmg]).is_some();
        let mut previous = release_from_sums(DB_BROWSER_SQLITE.repo, "v3.12.2", &[]);
        assert!(select(&previous, platform));
        assert!(!select(&previous, Platform { arch: Arch::X64, ..platform }));
        previous.assets[0].name = "DB.Browser.for.SQLite-3.12.2.dmg".into();
        assert!(!select(&previous, platform));

        let mut latest = release_from_sums(DB_BROWSER_SQLITE.repo, "v3.13.1", &[]);
        assert!(select(&latest, platform));
        latest.assets[0].size += 1;
        assert!(!select(&latest, platform));
        latest.assets[0].size -= 1;
        latest.assets[0].sha256 = Some("a".repeat(64));
        assert!(!select(&latest, platform));
        latest.assets[0].name = "DB.Browser.for.SQLite-v3.13.0.dmg".into();
        latest.assets[0].url = format!("{}v3.13.0/{}", crate::release::download_prefix(DB_BROWSER_SQLITE.repo), latest.assets[0].name);
        latest.version = crate::version::parse("3.13.0");
        latest.tag = "v3.13.0".into();
        assert!(!select(&latest, platform));
    }

    #[test]
    fn qualified_mac_dmg_matches_only_the_exact_reviewed_apple_silicon_artifact() {
        let host = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (app, tags) in [
            (crate::curated::OPENCLAW, ["v2026.11.9", "v2026.11.10"]),
            (crate::curated::RUSTDESK, ["1.5.1", "1.5.3"]),
            (crate::curated::DESKREEN, ["v3.2.15", "v3.2.16"]),
            (crate::curated::MOONLIGHT, ["v6.1.0", "v6.2.0"]),
            (crate::curated::SUNSHINE, ["v2026.906.222525", "v2026.914.233613"]),
            (crate::curated::OPENTTD, ["15.2", "15.3"]),
            (crate::curated::RECTANGLE, ["v2.0.4", "v2.0.5"]),
            (crate::curated::CODEEDIT, ["v0.3.5", "v0.3.6"]),
            (crate::curated::BLENDER, ["5.2.5", "5.3.0"]),
            (crate::curated::GIMP, ["3.2.6", "3.2.8"]),
            (crate::curated::KRITA, ["5.3.4.1", "5.3.5"]),
            (crate::curated::VLC, ["3.0.23", "3.0.24"]),
            (crate::curated::INKSCAPE, ["1.4.3", "1.4.4"]),
            (crate::curated::KDENLIVE, ["26.08.2", "26.08.3"]),
            (crate::curated::SUBTITLE_EDIT, ["v5.3.0", "v5.4.0"]),
            (crate::curated::DEEPCHAT, ["v1.1.3", "v1.1.4"]),
            (crate::curated::EASYDICT, ["2.25.0", "2.26.0"]),
            (crate::curated::UTM, ["v5.0.8", "v5.0.9-2"]),
            (crate::curated::OPENMTP, ["v3.2.25", "v3.3.0"]),
            (crate::curated::INVOKE_COMMUNITY, ["v1.8.4", "v1.8.5"]),
            (crate::curated::MIST, ["v0.30", "v0.40"]),
            (crate::curated::COTEDITOR, ["7.1.3", "7.2.0"]),
            (crate::curated::MACSHOT, ["v4.3.0", "v4.4.0"]),
            (crate::curated::FREEPLANE, ["release-1.13.3", "release-1.13.5"]),
            (crate::curated::DOCKDOOR, ["1.42.4", "1.42.5"]),
            (crate::curated::MACVIM, ["release-183", "release-184"]),
            (crate::curated::OPEN_PENCIL, ["v0.16.0", "v0.17.0"]),
            (crate::curated::OPEN_WHISPR, ["v1.12.1", "v1.12.2"]),
            (crate::curated::OPENCODE, ["v2.4.1", "v2.4.2"]),
            (crate::curated::LINEAR_MOUSE, ["v0.12.0", "v0.13.0"]),
            (crate::curated::FINICKY, ["v4.2.1", "v4.2.2"]),
            (crate::curated::VIRTUALBUDDY, ["2.2", "2.2.1"]),
            (crate::curated::OLLAMA, ["v0.40.17", "v0.40.18"]),
            (crate::curated::SIYUAN, ["v3.8.11", "v3.8.12"]),
            (crate::curated::MAGPIE, ["v0.1.4969", "v0.1.4971"]),
            (crate::curated::GHOST_DOWNLOADER, ["v4.5.11", "v4.6.1"]),
            (crate::curated::KANDO, ["v3.0.0", "v3.0.1"]),
            (crate::curated::TRILIUM, ["v0.106.1", "v0.107.0"]),
            (crate::curated::RECORDLY, ["v1.3.3", "v1.4.0"]),
            (crate::curated::VSCODIUM, ["1.143.17183", "1.143.17232"]),
            (crate::curated::OPENCHAMBER, ["v2.8.0", "v2.8.1"]),
            (crate::curated::OSAURUS, ["1.0.12", "1.0.13"]),
            (crate::curated::ZEN, ["1.24.2b", "1.24.3b"]),
            (crate::curated::HEADLAMP, ["v0.45.0", "v0.46.0"]),
            (crate::curated::KAP, ["v3.5.5", "v3.6.0"]),
            (crate::curated::SIMPLEX_CHAT, ["v7.1.3", "v7.1.4"]),
            (crate::curated::LOCALAI, ["v5.0.0", "v5.0.1"]),
            (crate::curated::UPSCAYL, ["v2.11.5", "v2.15.0"]),
            (crate::curated::LOSSLESSCUT, ["v3.70.1", "v3.70.2"]),
            (crate::curated::BRUNO, ["v4.3.0", "v4.4.0"]),
            (crate::curated::ANYTHING_LLM, ["v1.19.0", "v1.20.0"]),
            (crate::curated::T3_CODE, ["v0.1.12", "v0.1.13"]),
            (crate::curated::VOICE_STUDIO, ["v0.5.7", "v0.5.8"]),
            (crate::curated::CLASH_VERGE_REV, ["v2.5.9", "v2.5.10"]),
            (crate::curated::ORCA, ["v1.4.279", "v1.4.280"]),
        ] {
            let selects = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::Dmg]).is_some();
            for tag in tags {
                let mut release = crate::release::release_from_sums(app.repo, tag, &[]);
                release.assets.retain(|asset| asset.name.ends_with(".dmg"));
                assert!(selects(&release, host), "{} {tag}", app.id);
                assert!(!selects(&release, Platform { arch: Arch::X64, ..host }));
                assert!(!selects(&release, Platform { os: Os::Linux, ..host }));

                let mut untrusted = release.clone();
                untrusted.prerelease = true;
                assert!(!selects(&untrusted, host));
                untrusted = release.clone();
                untrusted.assets[0].url = untrusted.assets[0].url.replace(tag, "v0.0.1");
                assert!(!selects(&untrusted, host));
                untrusted = release.clone();
                untrusted.assets[0].sha256 = None;
                assert!(!selects(&untrusted, host));
                untrusted = release.clone();
                untrusted.assets[0].size += 1;
                assert!(!selects(&untrusted, host));
                untrusted = release;
                untrusted.assets[0].name = "foreign.dmg".into();
                assert!(!selects(&untrusted, host));
            }
        }
    }

    #[test]
    fn freeplane_cannot_select_the_unnotarized_intermediate_release() {
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let app = crate::curated::FREEPLANE;
        let mut release = crate::release::release_from_sums(app.repo, "release-1.13.4", &[]);
        let name = "Freeplane-1.13.4-apple.dmg";
        release.assets.push(Asset {
            name: name.into(),
            size: 147_305_756,
            url: format!("{}{}/{name}", crate::release::download_prefix(app.repo), release.tag),
            sha256: Some("109885a01fb229bb7789469d79f04227561a4d91b3700dd7b958261449623981".into()),
        });
        assert!(pick_asset(&release, &[app.id], mac, &[PackageKind::Dmg]).is_none());
    }

    #[test]
    fn window_utilities_use_only_reviewed_stable_apple_silicon_archives() {
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (app, tags) in [
            (crate::curated::MACCY, ["2.7.1", "2.8.0"]),
            (crate::curated::ALT_TAB, ["v11.10.1", "v11.10.2"]),
            (crate::curated::AMETHYST, ["v0.25.0", "v0.25.1"]),
            (crate::curated::HIDDEN_BAR, ["v1.13.3", "v1.13.4"]),
            (crate::curated::SWIFTBAR, ["v2.1.0", "v2.1.1"]),
            (crate::curated::LOOP, ["1.4.1", "1.4.2"]),
            (crate::curated::DYAD, ["v1.23.0", "v1.24.0"]),
            (crate::curated::HAMMERSPOON, ["1.1.0", "1.1.1"]),
            (crate::curated::ICE, ["0.11.11", "0.11.12"]),
        ] {
            let accepts = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::MacZip]).is_some();
            for tag in tags {
                let mut release = crate::release::release_from_sums(app.repo, tag, &[]);
                release.assets.retain(|asset| asset.name.ends_with(".zip"));
                assert_eq!(release.assets.len(), 1);
                assert!(accepts(&release, mac), "{} {tag}", app.id);
                assert!(!accepts(&release, Platform { arch: Arch::X64, ..mac }));
                assert!(!accepts(&release, Platform { os: Os::Linux, ..mac }));
                assert!(pick_asset(&release, &[app.id], mac, &[PackageKind::Dmg]).is_none());
                let mut untrusted = release.clone();
                untrusted.prerelease = true;
                assert!(!accepts(&untrusted, mac));
                untrusted = release.clone();
                untrusted.assets[0].size += 1;
                assert!(!accepts(&untrusted, mac));
                untrusted = release.clone();
                untrusted.assets[0].url = untrusted.assets[0].url.replace(app.repo, "example/replacement");
                assert!(!accepts(&untrusted, mac));
                untrusted = release;
                untrusted.assets[0].sha256 = None;
                assert!(!accepts(&untrusted, mac));
            }
        }
    }

    #[test]
    fn github_desktop_accepts_only_the_reviewed_current_apple_silicon_zip() {
        let app = crate::curated::GITHUB_DESKTOP;
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let selected = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::MacZip]).is_some();
        let mut release = crate::release::release_from_sums(app.repo, "release-3.7.1", &[]);
        assert_eq!(pick_asset(&release, &[app.id], mac, &[PackageKind::MacZip]).unwrap().0.size, 556_131_016);
        assert!(!selected(&release, Platform { arch: Arch::X64, ..mac }));
        assert!(!selected(&release, Platform { os: Os::Linux, ..mac }));
        release.assets[0].name = "GitHub.Desktop-x64.zip".into();
        assert!(!selected(&release, mac));
        release.assets[0].name = "GitHub.Desktop-arm64.zip".into();
        release.assets[0].size += 1;
        assert!(!selected(&release, mac));
        let future = crate::release::release_from_sums(app.repo, "release-3.7.2", &[]);
        assert!(!selected(&future, mac));
        let mut previous = crate::release::release_from_sums(app.repo, "release-3.7.0", &[]);
        let name = "GitHub.Desktop-arm64.zip";
        previous.assets.push(Asset {
            name: name.into(),
            size: 538_188_819,
            url: format!("{}{}/{name}", crate::release::download_prefix(app.repo), previous.tag),
            sha256: Some("cc41c3cfdae2eaa332ef6547f82c6e672b5a11b0bf637aada2bf2796921eb1a5".into()),
        });
        assert!(!selected(&previous, mac));
    }

    #[test]
    fn iterm2_offers_only_the_reviewed_modern_stable_zip_on_apple_silicon() {
        let app = crate::curated::ITERM2;
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let mut release = crate::release::iterm2_mac_release("3.7.9").unwrap();
        let selected = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::MacZip]).is_some();
        assert!(selected(&release, mac));
        assert!(!selected(&release, Platform { arch: Arch::X64, ..mac }));
        assert!(!selected(&release, Platform { os: Os::Linux, ..mac }));
        assert!(!selected(&crate::release::iterm2_mac_release("3.7.10").unwrap(), mac));
        release.assets[0].size += 1;
        assert!(!selected(&release, mac));
        assert!(crate::release::iterm2_mac_release("3.7.8").is_none());
    }

    #[test]
    fn jan_macos_accepts_only_the_patched_apple_silicon_release() {
        let app = crate::curated::JAN;
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let accepted = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::Dmg]).is_some();
        let release = crate::release::release_from_sums(app.repo, "v0.8.8", &[]);
        assert!(accepted(&release, mac));
        assert!(!accepted(&release, Platform { arch: Arch::X64, ..mac }));
        assert!(!accepted(&release, Platform { os: Os::Linux, ..mac }));

        let mut previous = crate::release::release_from_sums(app.repo, "v0.8.7", &[]);
        previous.assets = vec![crate::release::Asset {
            name: "Jan_0.8.7_universal.dmg".into(),
            url: "https://github.com/janhq/jan/releases/download/v0.8.7/Jan_0.8.7_universal.dmg".into(),
            size: 105_287_843,
            sha256: Some("bda3da88f0c799e4bbe08b95d039937e6f0d504d62b1bb79423b34aaae3aa1da".into()),
        }];
        assert!(!accepted(&previous, mac));
    }

    #[test]
    fn naps2_macos_accepts_only_reviewed_arm64_packages() {
        use crate::curated::NAPS2;

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let select = |release: &Release, target| pick_asset(release, &[NAPS2.id], target, target.default_kinds()).is_some();
        for tag in ["v8.6.0", "v8.6.1"] {
            let mut release = crate::release::release_from_sums(NAPS2.repo, tag, &[]);
            assert!(select(&release, mac));
            assert!(!select(&release, Platform { arch: Arch::X64, ..mac }));
            assert!(pick_asset(&release, &["handy"], mac, mac.default_kinds()).is_none());
            release.prerelease = true;
            assert!(!select(&release, mac));
            release.prerelease = false;
            let asset = &mut release.assets[0];
            let original = asset.sha256.replace("a".repeat(64));
            assert!(!select(&release, mac));
            release.assets[0].sha256 = original;
            release.assets[0].url = release.assets[0].url.replace("arm64", "x64");
            release.assets[0].name = release.assets[0].name.replace("arm64", "x64");
            assert!(!select(&release, mac));
        }
        let mut future = crate::release::release_from_sums(NAPS2.repo, "v8.6.1", &[]);
        future.version = crate::version::parse("v8.6.2");
        future.tag = "v8.6.2".into();
        assert!(!select(&future, mac));
    }

    #[test]
    fn reviewed_linux_appimages_require_verified_stable_asset_metadata() {
        use crate::curated::{LOSSLESSCUT, MUSESCORE, ONLYOFFICE, OPENSHOT, ORCA_SLICER, PENCIL2D, ZETTLR};

        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        for (app, version, filename) in [
            (crate::curated::OPENCODE, "2.4.2", "opencode-desktop-linux-x86_64.AppImage"),
            (crate::curated::ORCA, "1.4.280", "orca-linux.AppImage"),
            (PENCIL2D, "0.7.2", "pencil2d-linux-amd64-v0.7.2.AppImage"),
            (OPENSHOT, "4.1.1", "OpenShot-v4.1.1-x86_64.AppImage"),
            (MUSESCORE, "4.7.7", "MuseScore-Studio-4.7.7.261109195-x86_64.AppImage"),
            (ORCA_SLICER, "2.5.2", "OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.2.AppImage"),
            (LOSSLESSCUT, "3.70.2", "LosslessCut-linux-x86_64.AppImage"),
            (ZETTLR, "4.11.0", "Zettlr-4.11.0-x86_64.AppImage"),
            (ONLYOFFICE, "10.0.0", "DesktopEditors-x86_64.AppImage"),
        ] {
            let tag = format!("v{version}");
            let mut release = Release {
                assets: vec![Asset {
                    url: format!("{}{tag}/{filename}", crate::release::download_prefix(app.repo)),
                    name: filename.into(),
                    size: 42,
                    sha256: Some("a".repeat(64)),
                }],
                version: crate::version::parse(version),
                tag,
                ..Release::default()
            };
            let select = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::AppImage]).is_some();
            assert!(select(&release, linux));
            assert!(!select(&release, Platform { arch: Arch::Arm64, ..linux }));
            assert!(!select(&release, Platform { os: Os::MacOs, ..linux }));
            release.prerelease = true;
            assert!(!select(&release, linux));
            release.prerelease = false;
            let expected_url = std::mem::replace(&mut release.assets[0].url, format!("https://example.com/{filename}"));
            assert!(!select(&release, linux));
            release.assets[0].url = expected_url;
            release.assets[0].sha256 = None;
            assert!(!select(&release, linux));
        }
    }

    #[test]
    fn qualified_appimages_match_reviewed_bytes_and_platform() {
        use crate::curated::{
            DUST3D, INVOKE_COMMUNITY, MISTER_DOWNLOAD_MANAGER, OPEN_PENCIL, OPENWORK_DESKTOP, PASEO, RAWTHERAPEE, T3_CODE, ULTIMAKER_CURA,
            VOICE_STUDIO,
        };

        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        for (app, tags) in [
            (MISTER_DOWNLOAD_MANAGER, ["v0.5.0", "v0.5.2"]),
            (DUST3D, ["1.1.5", "1.1.6"]),
            (INVOKE_COMMUNITY, ["v1.8.4", "v1.8.5"]),
            (RAWTHERAPEE, ["5.12", "5.13"]),
            (OPEN_PENCIL, ["v0.16.0", "v0.17.0"]),
            (OPENWORK_DESKTOP, ["v0.18.72", "v0.18.73"]),
            (PASEO, ["v0.18.0", "v0.18.1"]),
            (T3_CODE, ["v0.1.12", "v0.1.13"]),
            (ULTIMAKER_CURA, ["5.13.0", "5.14.0"]),
            (VOICE_STUDIO, ["v0.5.7", "v0.5.8"]),
        ] {
            let select = |release: &Release, host| pick_asset(release, &[app.id], host, &[PackageKind::AppImage]).is_some();
            assert!(recipe_platform_verified(&[app.id], linux));
            assert_eq!(
                recipe_platform_verified(&[app.id], Platform { os: Os::MacOs, arch: Arch::Arm64 }),
                crate::release::qualified_mac_dmg_recipe(&[app.id]).is_some(),
            );
            for tag in tags {
                let mut release = crate::release::release_from_sums(app.repo, tag, &[]);
                assert!(select(&release, linux), "{} {tag}", app.id);
                assert!(!select(&release, Platform { arch: Arch::Arm64, ..linux }));
                assert!(!select(&release, Platform { os: Os::Windows, ..linux }));
                release.prerelease = true;
                assert!(!select(&release, linux));
                release.prerelease = false;
                let asset = release.assets.iter_mut().find(|asset| asset.name.ends_with(".AppImage")).unwrap();
                asset.sha256 = Some("f".repeat(64));
                assert!(!select(&release, linux));
            }
            assert!(!select(&crate::release::release_from_sums(app.repo, "v99.0.0", &[]), linux));
        }
    }

    #[test]
    fn tested_linux_appimages_are_available_without_github_api_quota() {
        let platform = Platform { os: Os::Linux, arch: Arch::X64 };
        for (repo, tag, filename, size) in [
            ("anomalyco/opencode", "v2.4.1", "opencode-desktop-linux-x86_64.AppImage", 325_645_884),
            ("anomalyco/opencode", "v2.4.2", "opencode-desktop-linux-x86_64.AppImage", 325_566_590),
            ("stablyai/orca", "v1.4.279", "orca-linux.AppImage", 287_088_086),
            ("stablyai/orca", "v1.4.280", "orca-linux.AppImage", 285_631_432),
            ("pencil2d/pencil", "v0.7.1", "pencil2d-linux-amd64-v0.7.1.AppImage", 60_471_800),
            ("pencil2d/pencil", "v0.7.2", "pencil2d-linux-amd64-v0.7.2.AppImage", 60_471_800),
            ("OpenShot/openshot-qt", "v4.1.0", "OpenShot-v4.1.0-x86_64.AppImage", 258_708_672),
            ("OpenShot/openshot-qt", "v4.1.1", "OpenShot-v4.1.1-x86_64.AppImage", 258_921_664),
            ("musescore/MuseScore", "v4.7.6", "MuseScore-Studio-4.7.6.261022093-x86_64.AppImage", 192_616_952),
            ("musescore/MuseScore", "v4.7.7", "MuseScore-Studio-4.7.7.261109195-x86_64.AppImage", 192_743_928),
            ("mifi/lossless-cut", "v3.70.1", "LosslessCut-linux-x86_64.AppImage", 221_418_564),
            ("mifi/lossless-cut", "v3.70.2", "LosslessCut-linux-x86_64.AppImage", 221_357_823),
            ("OrcaSlicer/OrcaSlicer", "v2.5.1", "OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.1.AppImage", 201_116_152),
            ("OrcaSlicer/OrcaSlicer", "v2.5.2", "OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.2.AppImage", 201_034_232),
            ("audacity/audacity", "Audacity-4.0.1", "audacity-linux-4.0.1-x86_64.AppImage", 73_755_128),
            ("audacity/audacity", "Audacity-4.0.2", "audacity-linux-4.0.2-x86_64.AppImage", 73_726_456),
            ("jgraph/drawio-desktop", "v32.6.10", "drawio-x86_64-32.6.10.AppImage", 152_706_651),
            ("jgraph/drawio-desktop", "v32.7.4", "drawio-x86_64-32.7.4.AppImage", 154_092_007),
            ("nextcloud-releases/desktop", "v35.0.2", "Nextcloud-35.0.2-x86_64.AppImage", 63_248_888),
            ("nextcloud-releases/desktop", "v35.0.3", "Nextcloud-35.0.3-x86_64.AppImage", 63_248_888),
            ("cryptomator/cryptomator", "1.19.6", "cryptomator-1.19.6-x86_64.AppImage", 59_075_064),
            ("cryptomator/cryptomator", "1.19.7", "cryptomator-1.19.7-x86_64.AppImage", 59_075_064),
            ("laurent22/joplin", "v3.7.22", "Joplin-3.7.22.AppImage", 207_329_204),
            ("laurent22/joplin", "v3.7.24", "Joplin-3.7.24.AppImage", 207_407_075),
            ("dail8859/NotepadNext", "v0.14", "NotepadNext-v0.14-x86_64.AppImage", 34_466_296),
            ("dail8859/NotepadNext", "v0.15", "NotepadNext-v0.15-x86_64.AppImage", 36_092_408),
            ("CyberTimon/RapidRAW", "v1.7.0", "03_RapidRAW_v1.7.0_ubuntu-22.04_amd64.AppImage", 114_608_632),
            ("CyberTimon/RapidRAW", "v1.7.1", "03_RapidRAW_v1.7.1_ubuntu-22.04_amd64.AppImage", 117_078_520),
            ("rustdesk/rustdesk", "1.5.1", "rustdesk-1.5.1-x86_64.AppImage", 88_472_056),
            ("rustdesk/rustdesk", "1.5.3", "rustdesk-1.5.3-x86_64.AppImage", 88_492_536),
            ("ActivityWatch/activitywatch", "v0.14.2", "activitywatch-linux-x86_64.AppImage", 109_992_440),
            ("ActivityWatch/activitywatch", "v0.14.3", "activitywatch-linux-x86_64.AppImage", 110_492_152),
            ("Zettlr/Zettlr", "v4.10.0", "Zettlr-4.10.0-x86_64.AppImage", 172_628_308),
            ("Zettlr/Zettlr", "v4.11.0", "Zettlr-4.11.0-x86_64.AppImage", 172_657_896),
            ("ONLYOFFICE/DesktopEditors", "v9.4.0", "DesktopEditors-x86_64.AppImage", 625_961_464),
            ("ONLYOFFICE/DesktopEditors", "v10.0.0", "DesktopEditors-x86_64.AppImage", 425_810_424),
        ] {
            let app = crate::hosted_catalog::apps().iter().find(|app| app.release_repository() == repo).unwrap_or_else(|| panic!("missing {repo}"));
            let release = crate::release::release_from_sums(repo, tag, &[]);
            let (asset, _) = pick_asset(&release, &[app.id], platform, &[PackageKind::AppImage]).expect("a reviewed fallback asset");
            assert_eq!(asset.name, filename);
            assert_eq!(asset.size, size);
            assert!(asset.sha256.as_ref().is_some_and(|sha| sha.len() == 64));
        }
    }

    fn release(names: &[&str]) -> Release {
        Release {
            assets: names.iter().map(|n| Asset { name: (*n).into(), size: 1, url: format!("https://x/{n}"), sha256: None }).collect(),
            ..Release::default()
        }
    }

    const PHOTO: &[&str] = &[
        "photocraft-0.5.0-freebsd-x86_64.tar.gz",
        "photocraft-0.5.0-linux-aarch64.AppImage",
        "photocraft-0.5.0-linux-aarch64.AppImage.zsync",
        "photocraft-0.5.0-linux-aarch64.deb",
        "photocraft-0.5.0-linux-riscv64.tar.gz",
        "photocraft-0.5.0-linux-x86_64.AppImage",
        "photocraft-0.5.0-linux-x86_64.tar.gz",
        "photocraft-0.5.0-macos-universal.dmg",
        "photocraft-0.5.0-windows-x64-portable.zip",
        "photocraft-0.5.0-windows-x64.msi",
        "photocraft-0.5.0-windows-x86.msi",
        "photocraft-cli-0.5.0-macos-universal.zip",
        "photocraft-web-0.5.0.zip",
        "SHA256SUMS.txt",
    ];

    #[test]
    fn macos_prefers_the_published_universal_build() {
        // PhotoCraft v0.9.0's GitHub metadata, trimmed to its desktop DMG.
        let mut r = crate::release::parse_github_release(include_str!("../tests/fixtures/photocraft-release.json"), "storytold/photocraft").unwrap();
        // Split builds must not outrank the published universal DMG, even when listed first.
        r.assets.splice(0..0, release(&["photocraft-0.9.0-macos-arm64.dmg", "photocraft-0.9.0-macos-x86_64.dmg"]).assets);
        for arch in [Arch::X64, Arch::Arm64] {
            let (asset, kind) = pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch }, &[PackageKind::Dmg]).unwrap();
            assert_eq!(asset.name, "photocraft-0.9.0-macos-universal.dmg");
            assert_eq!(kind, PackageKind::Dmg);
        }
    }

    #[test]
    fn texturecraft_selects_published_desktop_assets() {
        let release =
            crate::release::parse_github_release(include_str!("../tests/fixtures/texturecraft-release.json"), "storytold/texturecraft").unwrap();
        let windows = Platform { os: Os::Windows, arch: Arch::X64 };
        assert!(pick_asset(&release, &["texturecraft"], windows, &[PackageKind::Msi]).is_none());
        for (os, kind, expected) in [
            (Os::MacOs, PackageKind::Dmg, "texturecraft-0.1.0-macos-universal.dmg"),
            (Os::Linux, PackageKind::Flatpak, "texturecraft-0.1.0-linux-x86_64.flatpak"),
        ] {
            let platform = Platform { os, arch: Arch::X64 };
            let (asset, selected) = pick_asset(&release, &["texturecraft"], platform, &[kind]).unwrap();
            assert_eq!(asset.name, expected);
            assert_eq!(selected, kind);
        }
    }

    /// ArtCraft Studio's release fixture resolves the right asset across macOS, Windows, and Linux.
    #[test]
    fn artcraft_studio_selects_published_desktop_assets() {
        let release = crate::release::parse_github_release(include_str!("../tests/fixtures/artcraft-release.json"), "storytold/artcraft").unwrap();
        assert_eq!(release.version, Some(crate::version::Version::new(0, 41, 1)));
        for (os, arch, kind, expected) in [
            (Os::Windows, Arch::X64, PackageKind::Msi, "ArtCraft_0.41.1_x64_en-US.msi"),
            (Os::MacOs, Arch::Arm64, PackageKind::Dmg, "ArtCraft_0.41.1_universal.dmg"),
            (Os::MacOs, Arch::X64, PackageKind::Dmg, "ArtCraft_0.41.1_universal.dmg"),
            (Os::Linux, Arch::X64, PackageKind::AppImage, "artcraft-0.41.1-linux-x86_64.AppImage"),
            (Os::Linux, Arch::X64, PackageKind::Flatpak, "artcraft-0.41.1-linux-x86_64.flatpak"),
            (Os::Linux, Arch::X64, PackageKind::TarGz, "artcraft-0.41.1-linux-x86_64.tar.gz"),
            (Os::Linux, Arch::Arm64, PackageKind::TarGz, "artcraft-0.41.1-linux-aarch64.tar.gz"),
        ] {
            let platform = Platform { os, arch };
            let (asset, selected) = pick_asset(&release, &["artcraft"], platform, &[kind]).unwrap();
            assert_eq!(asset.name, expected);
            assert_eq!(selected, kind);
        }
    }

    #[test]
    fn freebsd_picks_the_tarball_for_the_processor() {
        // SoundCraft v0.10.0's GitHub metadata, trimmed to its FreeBSD tarballs.
        let r = crate::release::parse_github_release(include_str!("../tests/fixtures/soundcraft-release.json"), "storytold/soundcraft").unwrap();
        let pick = |arch| {
            let platform = Platform { os: Os::FreeBsd, arch };
            pick_asset(&r, &["soundcraft"], platform, platform.default_kinds()).map(|(a, k)| (a.name.as_str(), k))
        };
        assert_eq!(pick(Arch::X64), Some(("soundcraft-0.10.0-freebsd-x86_64.tar.gz", PackageKind::TarGz)));
        assert_eq!(pick(Arch::Arm64), Some(("soundcraft-0.10.0-freebsd-aarch64.tar.gz", PackageKind::TarGz)));
        for arch in [Arch::X86, Arch::Riscv64] {
            assert_eq!(pick(arch), None);
        }
        // An x86_64 machine never takes the ARM tarball, nor an ARM machine the x86_64 one.
        let only_arm = release(&["soundcraft-0.10.0-freebsd-aarch64.tar.gz"]);
        assert_eq!(pick_asset(&only_arm, &["soundcraft"], Platform { os: Os::FreeBsd, arch: Arch::X64 }, &[PackageKind::TarGz]), None);
        let only_x64 = release(&["soundcraft-0.10.0-freebsd-x86_64.tar.gz"]);
        assert_eq!(pick_asset(&only_x64, &["soundcraft"], Platform { os: Os::FreeBsd, arch: Arch::Arm64 }, &[PackageKind::TarGz]), None);
    }

    #[test]
    fn macos_split_builds_match_the_processor() {
        let r = release(&["photocraft-0.5.0-macos-arm64.dmg", "photocraft-0.5.0-macos-x86_64.dmg"]);
        let pick = |arch| pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch }, &[PackageKind::Dmg]).map(|(a, _)| a.name.as_str());
        assert_eq!(pick(Arch::X64), Some("photocraft-0.5.0-macos-x86_64.dmg"));
        assert_eq!(pick(Arch::Arm64), Some("photocraft-0.5.0-macos-arm64.dmg"));
    }

    #[test]
    fn intel_macos_never_selects_an_arm_only_build() {
        for token in ["macos-aarch64", "macos-arm64"] {
            let name = format!("photocraft-0.5.0-{token}.dmg");
            let r = release(&[&name]);
            assert_eq!(pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch: Arch::X64 }, &[PackageKind::Dmg]), None);
            assert_eq!(
                pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch: Arch::Arm64 }, &[PackageKind::Dmg]).map(|(a, _)| a.name.as_str()),
                Some(name.as_str())
            );
        }
    }

    #[test]
    fn apple_silicon_rejects_an_intel_only_release() {
        let r = release(&["photocraft-0.5.0-macos-x86_64.dmg"]);
        for arch in [Arch::Arm64, Arch::X86, Arch::Riscv64] {
            assert_eq!(pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch }, &[PackageKind::Dmg]), None);
        }
        assert!(pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch: Arch::X64 }, &[PackageKind::Dmg]).is_some());
    }

    #[test]
    fn asset_names_carry_their_architecture() {
        let arch = |name: &str| parse_asset_name(name, &["photocraft"]).unwrap().arch();
        assert_eq!(arch("photocraft-0.5.0-windows-x64.msi"), Some(Arch::X64));
        assert_eq!(arch("photocraft-0.5.0-windows-x86.msi"), Some(Arch::X86));
        assert_eq!(arch("photocraft-0.5.0-linux-aarch64.tar.gz"), Some(Arch::Arm64));
        assert_eq!(arch("photocraft-0.5.0-macos-universal.dmg"), None);
    }

    #[test]
    fn reads_asset_names() {
        let n = parse_asset_name("photocraft-0.2.0-rc.1-windows-arm64-portable.zip", &["photocraft"]).unwrap();
        assert_eq!((n.version.as_str(), n.token.as_str(), n.kind), ("0.2.0-rc.1", "windows-arm64", PackageKind::PortableZip));
        let n = parse_asset_name("printcraft-0.2.1-linux-x86_64.tar.gz", &["pdfcraft", "printcraft"]).unwrap();
        assert_eq!((n.stem.as_str(), n.kind), ("printcraft", PackageKind::TarGz));
        for other in
            ["photocraft-cli-0.5.0-macos-universal.zip", "photocraft-web-0.5.0.zip", "SHA256SUMS.txt", "photocraft-0.5.0-linux-x86_64.AppImage.zsync"]
        {
            assert_eq!(parse_asset_name(other, &["photocraft"]), None, "{other}");
        }
        // FigCraft also ships its collaboration server; that tarball is not the app.
        assert_eq!(parse_asset_name("figcraft-server-0.5.1-linux-x86_64.tar.gz", &["figcraft"]), None);
        assert!(parse_asset_name("figcraft-0.5.1-linux-x86_64.tar.gz", &["figcraft"]).is_some());
    }

    /// Studio's underscore-form MSI and DMG names parse cleanly, including `x86_64` and non-ASCII prefixes.
    #[test]
    fn reads_artcraft_studio_asset_names() {
        let dmg = parse_asset_name("ArtCraft_0.41.1_universal.dmg", &["artcraft"]).unwrap();
        assert_eq!((dmg.version.as_str(), dmg.token.as_str(), dmg.kind), ("0.41.1", "macos-universal", PackageKind::Dmg));

        let x86_64_dmg = parse_asset_name("ArtCraft_0.41.1_x86_64.dmg", &["artcraft"]).unwrap();
        assert_eq!((x86_64_dmg.version.as_str(), x86_64_dmg.token.as_str(), x86_64_dmg.kind), ("0.41.1", "macos-x86_64", PackageKind::Dmg));

        let msi = parse_asset_name("ArtCraft_0.41.1_x64_en-US.msi", &["artcraft"]).unwrap();
        assert_eq!((msi.version.as_str(), msi.token.as_str(), msi.kind), ("0.41.1", "windows-x64", PackageKind::Msi));

        let x86_64_msi = parse_asset_name("ArtCraft_0.41.1_x86_64.msi", &["artcraft"]).unwrap();
        assert_eq!((x86_64_msi.version.as_str(), x86_64_msi.token.as_str(), x86_64_msi.kind), ("0.41.1", "windows-x64", PackageKind::Msi));

        assert_eq!(parse_asset_name("artcrafé_0.41.1_x64.msi", &["artcraft"]), None);
    }

    #[test]
    fn picks_the_native_build_first_and_falls_back_to_emulation() {
        let r = release(PHOTO);
        let pick = |os, arch, kinds: &[PackageKind]| pick_asset(&r, &["photocraft"], Platform { os, arch }, kinds).map(|(a, _)| a.name.clone());
        assert_eq!(pick(Os::MacOs, Arch::Arm64, &[PackageKind::Dmg]).as_deref(), Some("photocraft-0.5.0-macos-universal.dmg"));
        assert_eq!(pick(Os::Linux, Arch::Arm64, &[PackageKind::AppImage]).as_deref(), Some("photocraft-0.5.0-linux-aarch64.AppImage"));
        assert_eq!(pick(Os::Linux, Arch::Arm64, &[PackageKind::TarGz]), None);
        assert_eq!(pick(Os::Linux, Arch::Riscv64, &[PackageKind::TarGz]).as_deref(), Some("photocraft-0.5.0-linux-riscv64.tar.gz"));
        assert_eq!(pick(Os::FreeBsd, Arch::X64, &[PackageKind::TarGz]).as_deref(), Some("photocraft-0.5.0-freebsd-x86_64.tar.gz"));
        let windows = release(&["generic-0.5.0-windows-x64.msi", "generic-0.5.0-windows-x86.msi", "generic-0.5.0-windows-x64-portable.zip"]);
        let pick_windows = |arch, kinds: &[PackageKind]| {
            pick_asset(&windows, &["generic"], Platform { os: Os::Windows, arch }, kinds).map(|(asset, _)| asset.name.as_str())
        };
        assert_eq!(pick_windows(Arch::X64, &[PackageKind::Msi]), Some("generic-0.5.0-windows-x64.msi"));
        // No arm64 MSI in this release: Windows on ARM runs the x64 build.
        assert_eq!(pick_windows(Arch::Arm64, &[PackageKind::Msi]), Some("generic-0.5.0-windows-x64.msi"));
        // Preference order across kinds.
        assert_eq!(pick_windows(Arch::X64, &[PackageKind::PortableZip, PackageKind::Msi]), Some("generic-0.5.0-windows-x64-portable.zip"));
        assert_eq!(pick_windows(Arch::X86, &[PackageKind::PortableZip, PackageKind::Msi]), Some("generic-0.5.0-windows-x86.msi"));

        let p_riscv = Platform { os: Os::Linux, arch: Arch::Riscv64 };
        assert_eq!(
            pick_asset(&r, &["photocraft"], p_riscv, p_riscv.default_kinds()).map(|(a, _)| a.name.clone()).as_deref(),
            Some("photocraft-0.5.0-linux-riscv64.tar.gz")
        );
        assert_eq!(
            pick_asset(&r, &["photocraft"], p_riscv, p_riscv.download_kinds()).map(|(a, _)| a.name.clone()).as_deref(),
            Some("photocraft-0.5.0-linux-riscv64.tar.gz")
        );
        let r_riscv_deb =
            release(&["photocraft-0.5.0-linux-riscv64.tar.gz", "photocraft-0.5.0-linux-riscv64.deb", "photocraft-0.5.0-linux-riscv64.rpm"]);
        assert_eq!(
            pick_asset(&r_riscv_deb, &["photocraft"], p_riscv, p_riscv.download_kinds()).map(|(a, _)| a.name.clone()).as_deref(),
            Some("photocraft-0.5.0-linux-riscv64.deb")
        );
    }

    #[test]
    fn strictly_matches_architecture_for_all_linux_package_formats() {
        let multi_arch_release = release(&[
            "photocraft-0.5.0-linux-aarch64.AppImage",
            "photocraft-0.5.0-linux-x86_64.AppImage",
            "photocraft-0.5.0-linux-aarch64.deb",
            "photocraft-0.5.0-linux-x86_64.deb",
            "photocraft-0.5.0-linux-aarch64.rpm",
            "photocraft-0.5.0-linux-x86_64.rpm",
            "photocraft-0.5.0-linux-aarch64.pkg.tar.zst",
            "photocraft-0.5.0-linux-x86_64.pkg.tar.zst",
            "photocraft-0.5.0-linux-aarch64.flatpak",
            "photocraft-0.5.0-linux-x86_64.flatpak",
            "photocraft-0.5.0-linux-aarch64.tar.gz",
            "photocraft-0.5.0-linux-x86_64.tar.gz",
        ]);

        let p_x64 = Platform { os: Os::Linux, arch: Arch::X64 };
        let p_arm64 = Platform { os: Os::Linux, arch: Arch::Arm64 };

        for kind in [PackageKind::AppImage, PackageKind::Deb, PackageKind::Rpm, PackageKind::ArchPkg, PackageKind::Flatpak, PackageKind::TarGz] {
            let pick_x64 = pick_asset(&multi_arch_release, &["photocraft"], p_x64, &[kind]).map(|(a, _)| a.name.clone());
            assert!(pick_x64.as_deref().unwrap().contains("linux-x86_64"), "Expected x86_64 asset for {kind:?}, got {pick_x64:?}");

            let pick_arm64 = pick_asset(&multi_arch_release, &["photocraft"], p_arm64, &[kind]).map(|(a, _)| a.name.clone());
            assert!(pick_arm64.as_deref().unwrap().contains("linux-aarch64"), "Expected aarch64 asset for {kind:?}, got {pick_arm64:?}");
        }

        // On x86_64 Linux, an aarch64-only asset must NEVER match or fall back
        let arm_only_release = release(&[
            "photocraft-0.5.0-linux-aarch64.AppImage",
            "photocraft-0.5.0-linux-aarch64.deb",
            "photocraft-0.5.0-linux-aarch64.rpm",
            "photocraft-0.5.0-linux-aarch64.pkg.tar.zst",
            "photocraft-0.5.0-linux-aarch64.flatpak",
            "photocraft-0.5.0-linux-aarch64.tar.gz",
        ]);
        for kind in [PackageKind::AppImage, PackageKind::Deb, PackageKind::Rpm, PackageKind::ArchPkg, PackageKind::Flatpak, PackageKind::TarGz] {
            assert_eq!(pick_asset(&arm_only_release, &["photocraft"], p_x64, &[kind]), None, "x86_64 must never match an aarch64 asset for {kind:?}");
        }
    }
}
