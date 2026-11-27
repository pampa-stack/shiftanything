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

    /// Asset platform tokens to try, best first. Windows on ARM and 64-bit Windows can fall back
    /// to emulated builds; macOS prefers universal, then native builds. Apple silicon can fall
    /// back to Intel builds, but Intel Macs cannot run ARM builds.
    fn tokens(self) -> &'static [&'static str] {
        match (self.os, self.arch) {
            (Os::Windows, Arch::X64) => &["windows-x64", "windows-x86"],
            (Os::Windows, Arch::X86) => &["windows-x86"],
            (Os::Windows, Arch::Arm64) => &["windows-arm64", "windows-x64", "windows-x86"],
            (Os::MacOs, Arch::X64) => &["macos-universal", "macos-x86_64"],
            (Os::MacOs, Arch::Arm64) => &["macos-universal", "macos-aarch64", "macos-arm64", "macos-x86_64"],
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
            Os::MacOs => &[PackageKind::Dmg, PackageKind::MacZip],
            Os::Linux => &[PackageKind::AppImage, PackageKind::TarGz],
            Os::FreeBsd => &[PackageKind::TarGz],
        }
    }

    /// Every format offered as a manual download on this OS (the web build's download menu).
    pub fn download_kinds(self) -> &'static [PackageKind] {
        match self.os {
            Os::Windows => &[PackageKind::Msi, PackageKind::PortableZip],
            Os::MacOs => &[PackageKind::Dmg, PackageKind::MacZip],
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
    if stems.contains(&crate::curated::KEYCASTR.id) {
        if !kinds.contains(&PackageKind::MacZip) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::keycastr_mac_asset(release, asset)).map(|asset| (asset, PackageKind::MacZip));
    }
    if stems.contains(&crate::curated::LAPCE.id) {
        if !kinds.contains(&PackageKind::Dmg) {
            return None;
        }
        return release.assets.iter().find(|asset| crate::release::lapce_mac_asset(release, asset)).map(|asset| (asset, PackageKind::Dmg));
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
    if platform.os == Os::Linux && ["drawio", "audacity", "nextcloud-desktop", "cryptomator", "joplin"].iter().any(|id| stems.contains(id)) {
        let app = [AUDACITY, DRAWIO, NEXTCLOUD_DESKTOP, CRYPTOMATOR, JOPLIN].into_iter().find(|app| stems.contains(&app.id))?;
        if release.prerelease || !kinds.contains(&PackageKind::AppImage) {
            return None;
        }
        let version = release.version.as_ref().filter(|version| version.pre.is_empty() && version.build.is_empty())?;
        let (tag_prefix, expected_name) = match app.id {
            "audacity" => ("Audacity-", format!("audacity-linux-{version}-x86_64.AppImage")),
            "drawio" => ("v", format!("drawio-x86_64-{version}.AppImage")),
            "nextcloud-desktop" => ("v", format!("Nextcloud-{version}-x86_64.AppImage")),
            "cryptomator" => ("", format!("cryptomator-{version}-x86_64.AppImage")),
            "joplin" => ("v", format!("Joplin-{version}.AppImage")),
            _ => return None,
        };
        if release.tag != format!("{tag_prefix}{version}") {
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
    for kind in kinds {
        for token in platform.tokens() {
            if let Some((asset, _)) = parsed.iter().find(|(_, n)| n.kind == *kind && n.token == *token) {
                return Some((asset, *kind));
            }
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
    if ["drawio", "audacity", "nextcloud-desktop", "cryptomator", "joplin"].iter().any(|id| stems.contains(id)) && platform.os == Os::Linux {
        return platform == linux_x64;
    }
    if ["nextcloud-desktop"].iter().any(|id| stems.contains(id)) {
        return false;
    }
    if [
        HANDY.id,
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
        crate::curated::PICARD.id,
        crate::curated::LAPCE.id,
        crate::curated::VORTA.id,
        crate::curated::SOLVESPACE.id,
        crate::curated::DOSBOX_STAGING.id,
        crate::curated::VIBE.id,
        crate::curated::OPENSCREEN.id,
    ]
    .iter()
    .any(|id| stems.contains(id))
    {
        return platform == mac_arm64;
    }
    use crate::curated::{LOSSLESSCUT, MUSESCORE, ONLYOFFICE, OPENSHOT, ORCA_SLICER, PENCIL2D, ZETTLR};
    if [PENCIL2D, OPENSHOT, MUSESCORE, ORCA_SLICER, LOSSLESSCUT, ZETTLR, ONLYOFFICE].iter().any(|app| stems.contains(&app.id)) {
        return platform == linux_x64;
    }
    if stems.contains(&LOCALSEND.id) {
        return platform == mac_arm64 || platform == linux_x64;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reviewed_linux_appimages_require_verified_stable_asset_metadata() {
        use crate::curated::{LOSSLESSCUT, MUSESCORE, ONLYOFFICE, OPENSHOT, ORCA_SLICER, PENCIL2D, ZETTLR};

        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        for (app, version, filename) in [
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
    fn tested_linux_appimages_are_available_without_github_api_quota() {
        let platform = Platform { os: Os::Linux, arch: Arch::X64 };
        for (repo, tag, filename, size) in [
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
        for (os, kind, expected) in [
            (Os::Windows, PackageKind::Msi, "texturecraft-0.1.0-windows-x64.msi"),
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
    fn apple_silicon_keeps_the_intel_fallback() {
        let r = release(&["photocraft-0.5.0-macos-x86_64.dmg"]);
        assert_eq!(
            pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch: Arch::Arm64 }, &[PackageKind::Dmg]).map(|(a, _)| a.name.as_str()),
            Some("photocraft-0.5.0-macos-x86_64.dmg")
        );
        for arch in [Arch::X86, Arch::Riscv64] {
            assert_eq!(pick_asset(&r, &["photocraft"], Platform { os: Os::MacOs, arch }, &[PackageKind::Dmg]), None);
        }
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
        assert_eq!(pick(Os::Windows, Arch::X64, &[PackageKind::Msi]).as_deref(), Some("photocraft-0.5.0-windows-x64.msi"));
        // No arm64 MSI in this release: Windows on ARM runs the x64 build.
        assert_eq!(pick(Os::Windows, Arch::Arm64, &[PackageKind::Msi]).as_deref(), Some("photocraft-0.5.0-windows-x64.msi"));
        assert_eq!(pick(Os::Linux, Arch::Arm64, &[PackageKind::AppImage]).as_deref(), Some("photocraft-0.5.0-linux-aarch64.AppImage"));
        assert_eq!(pick(Os::Linux, Arch::Arm64, &[PackageKind::TarGz]), None);
        assert_eq!(pick(Os::Linux, Arch::Riscv64, &[PackageKind::TarGz]).as_deref(), Some("photocraft-0.5.0-linux-riscv64.tar.gz"));
        assert_eq!(pick(Os::FreeBsd, Arch::X64, &[PackageKind::TarGz]).as_deref(), Some("photocraft-0.5.0-freebsd-x86_64.tar.gz"));
        // Preference order across kinds.
        assert_eq!(
            pick(Os::Windows, Arch::X64, &[PackageKind::PortableZip, PackageKind::Msi]).as_deref(),
            Some("photocraft-0.5.0-windows-x64-portable.zip")
        );
        assert_eq!(pick(Os::Windows, Arch::X86, &[PackageKind::PortableZip, PackageKind::Msi]).as_deref(), Some("photocraft-0.5.0-windows-x86.msi"));

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
