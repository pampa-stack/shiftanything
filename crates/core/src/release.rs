//! GitHub releases: reading the API's answer and the publisher's checksum manifest.
//!
//! Everything here is untrusted input. Sizes are bounded by the caller, download links must point
//! at the app's own release downloads, and checksums must look like checksums.

use serde::{Deserialize, Serialize};

use crate::version::{self, Version};

/// One downloadable file of a release.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asset {
    pub name: String,
    /// Bytes; 0 when unknown.
    pub size: u64,
    /// The publisher's pinned release download URL.
    pub url: String,
    /// Lowercase hex SHA-256, when GitHub or the publisher's checksum manifest reported it.
    pub sha256: Option<String>,
}

/// A published release of one app.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Release {
    pub tag: String,
    pub version: Option<Version>,
    /// `PhotoCraft v0.5.0`
    pub name: String,
    /// Unix seconds; `None` when only the tag is known.
    pub published_at: Option<i64>,
    pub html_url: String,
    /// Release notes (Markdown), capped.
    pub notes: String,
    pub prerelease: bool,
    pub assets: Vec<Asset>,
}

impl Release {
    pub fn version_label(&self) -> String {
        self.version.as_ref().map_or_else(|| self.tag.clone(), version::label)
    }

    /// Fills in checksums from a publisher manifest for assets that don't have one.
    pub fn apply_sums(&mut self, sums: &[(String, String)]) {
        for asset in &mut self.assets {
            if asset.sha256.is_none() {
                asset.sha256 = sums.iter().find(|(_, n)| *n == asset.name).map(|(h, _)| h.clone());
            }
        }
    }

    pub fn sums_asset(&self) -> Option<&Asset> {
        self.assets.iter().find(|asset| {
            asset.name == "SHA256SUMS.txt"
                || [crate::curated::KOPIA.repo, crate::curated::TERMIX.repo, crate::curated::OPENCODE.repo, crate::curated::OPEN_PENCIL.repo]
                    .iter()
                    .any(|repo| {
                        let manifest = checksum_manifest_name(repo);
                        asset.name == manifest && asset.url == format!("{}{}/{manifest}", download_prefix(repo), self.tag)
                    })
        })
    }
}

/// The checksum manifest published under a release's download path.
pub fn checksum_manifest_name(repo: &str) -> &'static str {
    if repo == crate::curated::KOPIA.repo {
        "checksums.txt"
    } else if [crate::curated::TERMIX.repo, crate::curated::OPENCODE.repo, crate::curated::OPEN_PENCIL.repo].contains(&repo) {
        "SHA256SUMS"
    } else {
        "SHA256SUMS.txt"
    }
}

const MAX_NOTES: usize = 64 * 1024;

pub(crate) fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

pub const OPENTTD_DMG_URL: &str = "https://cdn.openttd.org/openttd-releases/";
pub const CALIBRE_DMG_URL: &str = "https://download.calibre-ebook.com/";
pub const LIBREOFFICE_STABLE_URL: &str = "https://download.documentfoundation.org/libreoffice/stable/";
pub const LIBREOFFICE_DMG_MAX_SIZE: u64 = 1024 * 1024 * 1024;
pub const ZOTERO_STABLE_URL: &str = "https://www.zotero.org/download/client/dl?channel=release&platform=mac";
pub const ZOTERO_DMG_URL: &str = "https://download.zotero.org/client/release/";
pub const ZOTERO_CHANGELOG: &str = "https://www.zotero.org/support/changelog";
pub const BLENDER_DMG_URL: &str = "https://download.blender.org/release/";
pub const BLENDER_RELEASES_URL: &str = "https://www.blender.org/releases/";
pub const GIMP_DMG_URL: &str = "https://download.gimp.org/gimp/";
pub const GIMP_DOWNLOADS_URL: &str = "https://www.gimp.org/downloads/";
pub const GIMP_INDEX_URL: &str = "https://www.gimp.org/gimp_versions.json";
pub const VLC_DMG_URL: &str = "https://download.videolan.org/pub/videolan/vlc/";
pub const VLC_DOWNLOADS_URL: &str = "https://www.videolan.org/vlc/download-macosx.html";
pub const INKSCAPE_DMG_URL: &str = "https://media.inkscape.org/dl/resources/file/";
pub const INKSCAPE_DOWNLOADS_URL: &str = "https://inkscape.org/release/";
pub const KDENLIVE_DMG_URL: &str = "https://cdn.download.kde.org/stable/kdenlive/";
pub const KDENLIVE_DOWNLOADS_URL: &str = "https://kdenlive.org/en/download/";
pub const KDENLIVE_RELEASES_URL: &str = "https://apps.kde.org/kdenlive/index.xml";
pub const KRITA_DOWNLOADS_URL: &str = "https://krita.org/en/download/";
pub const KRITA_DMG_URL: &str = "https://download.kde.org/stable/krita/";
pub const NETNEWSWIRE_STABLE_MAC_URL: &str = "https://netnewswire.com/NetNewsWire.zip";
type DigestReceipt = (&'static str, u64, &'static str);
const ZOTERO_DMG_RECEIPTS: &[(&str, DigestReceipt)] = &[
    ("10.0.9", ("Zotero-10.0.9.dmg", 192_346_587, "fa42a04c33028ee020ccf47e64b3e135ce2fc57ba9a099e9f467f280ea56a387")),
    ("10.0.8", ("Zotero-10.0.8.dmg", 192_345_844, "bf6192a0745ae67efbb7ff4e7f1953d8b78f5eed7083f4eefa12b762ecddcabe")),
];

pub fn zotero_stable_version(location: &str) -> Option<Version> {
    let (tag, name) = location.strip_prefix(ZOTERO_DMG_URL)?.split_once('/')?;
    let version = version::parse(tag)?;
    (version.to_string() == tag && version.pre.is_empty() && version.build.is_empty() && name == format!("Zotero-{tag}.dmg")).then_some(version)
}

pub fn zotero_release(version: Version) -> Release {
    let tag = version.to_string();
    Release {
        assets: reviewed_assets(crate::curated::ZOTERO.repo, &tag).collect(),
        name: format!("Zotero {tag}"),
        html_url: ZOTERO_CHANGELOG.to_string(),
        version: Some(version),
        tag,
        ..Release::default()
    }
}

/// The official latest version, followed by previously qualified versions still in the archive.
pub fn zotero_release_history(latest: Release) -> Vec<Release> {
    let latest_version = latest.version.clone();
    let older = ZOTERO_DMG_RECEIPTS
        .iter()
        .filter_map(|&(tag, _)| version::parse(tag))
        .filter(|version| Some(version) < latest_version.as_ref())
        .map(zotero_release);
    std::iter::once(latest).chain(older).collect()
}

pub fn blender_release(version: Version) -> Release {
    let tag = version.to_string();
    Release {
        assets: reviewed_assets(crate::curated::BLENDER.repo, &tag).collect(),
        name: format!("Blender {tag}"),
        html_url: BLENDER_RELEASES_URL.to_string(),
        version: Some(version),
        tag,
        ..Release::default()
    }
}

pub fn gimp_release(version: Version) -> Release {
    let tag = version.to_string();
    Release {
        assets: reviewed_assets(crate::curated::GIMP.repo, &tag).collect(),
        name: format!("GIMP {tag}"),
        html_url: format!("https://www.gimp.org/release/{tag}/"),
        version: Some(version),
        tag,
        ..Release::default()
    }
}

pub fn vlc_mac_release(tag: &str) -> Option<Release> {
    let version = Version::parse(tag).ok()?;
    if !version.pre.is_empty() || !version.build.is_empty() || version.to_string() != tag {
        return None;
    }
    Some(Release {
        assets: reviewed_assets(crate::curated::VLC.repo, tag).collect(),
        name: format!("VLC {tag}"),
        html_url: VLC_DOWNLOADS_URL.to_string(),
        version: Some(version),
        tag: tag.to_string(),
        ..Release::default()
    })
}

pub fn inkscape_mac_release(tag: &str) -> Option<Release> {
    let version = Version::parse(tag).ok()?;
    if !version.pre.is_empty() || !version.build.is_empty() || version.to_string() != tag {
        return None;
    }
    Some(Release {
        assets: reviewed_assets(crate::curated::INKSCAPE.repo, tag).collect(),
        name: format!("Inkscape {tag}"),
        html_url: INKSCAPE_DOWNLOADS_URL.to_string(),
        version: Some(version),
        tag: tag.to_string(),
        ..Release::default()
    })
}

/// KDE’s macOS bundle and release feed use YY.MM.PATCH with a zero-padded month.
pub fn kdenlive_mac_version(tag: &str) -> Option<Version> {
    let (year, remainder) = tag.split_once('.')?;
    let (month, patch) = remainder.split_once('.')?;
    let version = Version::new(year.parse().ok()?, month.parse().ok()?, patch.parse().ok()?);
    ((1..=12).contains(&version.minor) && format!("{}.{:02}.{}", version.major, version.minor, version.patch) == tag).then_some(version)
}

pub fn kdenlive_mac_release(tag: &str) -> Option<Release> {
    Some(Release {
        assets: reviewed_assets(crate::curated::KDENLIVE.repo, tag).collect(),
        name: format!("Kdenlive {tag}"),
        html_url: KDENLIVE_DOWNLOADS_URL.to_string(),
        version: Some(kdenlive_mac_version(tag)?),
        tag: tag.to_string(),
        ..Release::default()
    })
}

fn krita_mac_version(tag: &str) -> Option<Version> {
    // KDE labels the hotfix 5.3.4.1, but the signed Mac bundle identifies itself as 5.3.4.
    let label = if tag == "5.3.4.1" { "5.3.4" } else { tag };
    let version = Version::parse(label).ok()?;
    (version.to_string() == label && version.pre.is_empty() && version.build.is_empty()).then_some(version)
}

pub fn krita_mac_release(tag: &str) -> Option<Release> {
    Some(Release {
        assets: reviewed_assets(crate::curated::KRITA.repo, tag).collect(),
        name: format!("Krita {tag}"),
        html_url: KRITA_DOWNLOADS_URL.to_string(),
        version: Some(krita_mac_version(tag)?),
        tag: tag.to_string(),
        ..Release::default()
    })
}

pub fn krita_mac_release_from_download(address: &str) -> Option<Release> {
    let (tag, name) = address.strip_prefix(KRITA_DMG_URL)?.split_once('/')?;
    (name == format!("krita-{tag}-signed.dmg")).then(|| krita_mac_release(tag))?
}

fn netnewswire_mac_version(tag: &str) -> Option<Version> {
    let raw = tag.strip_prefix("mac-")?;
    let version = version::parse(raw)?;
    (version.to_string() == raw || (version.patch == 0 && raw == format!("{}.{}", version.major, version.minor)))
        .then_some(version)
        .filter(|version| version.pre.is_empty() && version.build.is_empty())
}

fn netnewswire_release(tag: &str, version: Version) -> Release {
    let repo = crate::curated::NETNEWSWIRE.repo;
    Release {
        assets: reviewed_assets(repo, tag).collect(),
        name: format!("NetNewsWire {}", tag.trim_start_matches("mac-")),
        html_url: format!("https://github.com/{repo}/releases/tag/{tag}"),
        version: Some(version),
        tag: tag.to_string(),
        ..Release::default()
    }
}

/// The publisher's Mac download link is separate from the shared GitHub latest release.
pub fn netnewswire_mac_release_from_download(location: &str) -> Option<Release> {
    let prefix = download_prefix(crate::curated::NETNEWSWIRE.repo);
    let (tag, file) = location.strip_prefix(&prefix)?.split_once('/')?;
    let version = netnewswire_mac_version(tag)?;
    (file == format!("NetNewsWire{}.zip", tag.strip_prefix("mac-")?)).then(|| netnewswire_release(tag, version))
}

pub fn netnewswire_mac_release_history(latest: Release) -> Vec<Release> {
    let latest_version = latest.version.clone();
    let older = ["mac-7.3.1", "mac-7.3"]
        .into_iter()
        .filter_map(|tag| Some((tag, netnewswire_mac_version(tag)?)))
        .filter(|(_, version)| Some(version) < latest_version.as_ref())
        .map(|(tag, version)| netnewswire_release(tag, version));
    std::iter::once(latest).chain(older).collect()
}

pub fn libreoffice_stable_version(tag: &str) -> Option<Version> {
    let version = version::parse(tag)?;
    (version.pre.is_empty() && version.build.is_empty() && version.to_string() == tag).then_some(version)
}

pub fn libreoffice_arm64_dmg(version: &Version) -> (String, String) {
    let name = format!("LibreOffice_{version}_MacOS_aarch64.dmg");
    let url = format!("{LIBREOFFICE_STABLE_URL}{version}/mac/aarch64/{name}");
    (name, url)
}

pub fn libreoffice_arm64_asset(release: &Release, asset: &Asset) -> bool {
    let Some(version) = libreoffice_stable_version(&release.tag) else { return false };
    let (name, url) = libreoffice_arm64_dmg(&version);
    !release.prerelease
        && release.version.as_ref() == Some(&version)
        && asset.name == name
        && asset.url == url
        && (1..=LIBREOFFICE_DMG_MAX_SIZE).contains(&asset.size)
        && asset.sha256.as_deref().is_some_and(is_hex64)
}

/// Windows Installer packages reviewed on the official TDF stable channel, limited to x64.
pub fn libreoffice_windows_msi(version: &Version) -> Option<Asset> {
    let (size, sha256) = match version.to_string().as_str() {
        "26.8.2" => (375_934_976, "ff4b911403b67ee9e6249f820f9782c44bb32c94fdaa45b0da53a8e419e5307a"),
        "26.8.3" => (376_311_808, "0e6918c273eabbdbbd800ee0dacbbf130a14c13c2e4cec861b06dd59e015bf94"),
        _ => return None,
    };
    let name = format!("LibreOffice_{version}_Win_x86-64.msi");
    let url = format!("{LIBREOFFICE_STABLE_URL}{version}/win/x86_64/{name}");
    Some(Asset { name, url, size, sha256: Some(sha256.to_string()) })
}

/// Whether `url` is safe to hand to the system browser from untrusted text (release notes):
/// `https` only, a host, no user-info, no spaces or control characters, and no shell metacharacters.
///
/// The system opener is not always a direct `open`: the pinned `webbrowser` crate's WSL fallback
/// passes the URL to `cmd.exe /c start` while escaping only `^` and `&`, so a character like `|`
/// in an otherwise valid URL becomes a command separator (CWE-78). Because a legitimate
/// `https://` web link never needs these characters unescaped, we refuse any URL that contains a
/// shell/cmd metacharacter rather than rely on the opener to neutralise it.
pub fn safe_web_link(url: &str) -> bool {
    let Some(rest) = url.strip_prefix("https://") else {
        return false;
    };
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    url.len() <= 2000
        && !authority.is_empty()
        && !authority.contains(['@', '\\'])
        && !url.chars().any(|c| c.is_control() || c.is_whitespace())
        && !url.chars().any(is_shell_meta)
}

/// Characters a command interpreter (POSIX shells or `cmd.exe`) may treat specially. A real
/// `https` URL never needs any of them unescaped in a browser-opened link. `%` matters because
/// `cmd.exe` expands `%NAME%` references, and an expanded value could itself carry metacharacters.
pub fn is_shell_meta(c: char) -> bool {
    matches!(c, '|' | '&' | ';' | '<' | '>' | '(' | ')' | '$' | '`' | '"' | '\'' | '^' | '!' | '*' | '%' | '{' | '}' | '[' | ']')
}

/// Whether `url` is a valid release page URL strictly for `repo` on GitHub:
/// starts with `https://github.com/<repo>/releases`, contains no dot-segments (`.` or `..`),
/// no percent-encoded path separators (`%2f`, `%5c`), no query/fragment/backslash, and
/// satisfies [`safe_web_link`] (no shell metacharacters or control bytes, CWE-78).
pub fn safe_release_html_url(repo: &str, url: &str) -> Option<String> {
    if !safe_web_link(url) {
        return None;
    }
    let releases = format!("https://github.com/{repo}/releases");
    if url == releases {
        return Some(url.to_string());
    }
    let rest = url.strip_prefix(&releases)?;
    let path = rest.strip_prefix('/')?;
    if path.is_empty() || path.contains(['?', '#', '\\']) {
        return None;
    }
    for seg in path.split('/') {
        if seg.is_empty() || seg == "." || seg == ".." || seg.contains("%2f") || seg.contains("%2F") || seg.contains("%5c") || seg.contains("%5C") {
            return None;
        }
    }
    Some(url.to_string())
}

/// The prefix every download of `repo` must start with.
pub fn download_prefix(repo: &str) -> String {
    format!("https://github.com/{repo}/releases/download/")
}

/// Whether `url` is a download that truly belongs to `repo`.
///
/// A raw `starts_with(prefix)` check is not enough: a value such as
/// `https://github.com/storytold/craft-launcher/releases/download/../../../ATTACKER/REPO/releases/download/v1/x`
/// shares the prefix textually, yet its interpreted path escapes the repository. The link is
/// accepted only when the text after the prefix is exactly `<tag>/<name>` — two non-empty path
/// segments with no traversal (`.`/`..`), no additional slashes, no query or fragment, and no
/// backslash or percent-encoded separators that a client could later re-interpret.
pub fn download_url_under(url: &str, prefix: &str) -> bool {
    let Some(rest) = url.strip_prefix(prefix) else {
        return false;
    };
    download_suffix_ok(rest)
}

/// Whether `url` sits under a developer mirror `base` (ending in `/`) without escaping it.
pub fn download_url_under_mirror(url: &str, base: &str) -> bool {
    let Some(rest) = url.strip_prefix(base) else {
        return false;
    };
    // A mirror serves `<tag>/<name>` just like GitHub, but we only require that it can't climb out
    // of its own folder; its layout is otherwise the developer's business.
    !rest.is_empty() && !url.contains(['?', '#', '\\']) && !contains_traversal(rest) && !contains_encoded_separator(rest)
}

/// `<tag>/<name>`: exactly two non-empty segments, no traversal, no re-interpretable separators.
fn download_suffix_ok(rest: &str) -> bool {
    if rest.contains(['?', '#', '\\']) || contains_encoded_separator(rest) {
        return false;
    }
    let mut parts = rest.split('/');
    let (Some(tag), Some(name), None) = (parts.next(), parts.next(), parts.next()) else {
        return false;
    };
    valid_tag(tag) && valid_file_name(name)
}

/// Any `.` or `..` path segment that could climb out of the repository.
fn contains_traversal(rest: &str) -> bool {
    rest.split('/').any(|seg| seg == "." || seg == "..")
}

/// Percent-encoded slash or backslash (`%2f`, `%5c`): a client could decode these back into
/// separators after the check, so a confined URL must not contain them.
fn contains_encoded_separator(rest: &str) -> bool {
    let bytes = rest.as_bytes();
    bytes.windows(3).any(|w| w[0] == b'%' && matches!(&w[1..].to_ascii_lowercase()[..], b"2f" | b"5c"))
}

/// Reads `GET /repos/<repo>/releases/latest` (or one element of `/releases`).
///
/// # Errors
///
/// Returns an error if the JSON is malformed or does not describe a usable release.
pub fn parse_github_release(json: &str, repo: &str) -> Result<Release, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("unreadable answer from GitHub ({e})"))?;
    release_from_value(&v, repo)
}

/// Published releases offered for manual selection, in GitHub's order.
/// Drafts and unparseable tags cannot be installed.
///
/// # Errors
///
/// Returns an error if the JSON is malformed or its top level is not an array.
pub fn parse_github_versions(json: &str, repo: &str) -> Result<Vec<Release>, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("unreadable answer from GitHub ({e})"))?;
    let list = v.as_array().ok_or("unexpected answer from GitHub")?;
    Ok(list
        .iter()
        .filter(|item| !item["draft"].as_bool().unwrap_or(false))
        .filter_map(|item| release_from_value(item, repo).ok())
        .filter(|r| r.version.is_some())
        .take(100)
        .collect())
}

/// A published, explicitly marked nightly build, not merely a beta or release candidate.
pub fn is_nightly(release: &Release) -> bool {
    release.prerelease && release.version.as_ref().is_some_and(|v| v.pre.as_str().split('.').any(|part| part.eq_ignore_ascii_case("nightly")))
}

/// Canonical parser-level Nightly eligibility, not descriptor or target validation.
///
/// Only a strict SemVer tag, optionally prefixed with lowercase `v`, qualifies. The parsed
/// version must agree with the release, including build metadata, and its first prerelease
/// component must be exactly `nightly`. Short canonical tags remain candidates for the native
/// layer to validate against their descriptors.
pub fn canonical_nightly_version(release: &Release) -> Option<Version> {
    if !release.prerelease {
        return None;
    }
    let tag = release.tag.strip_prefix('v').unwrap_or(&release.tag);
    let parsed = Version::parse(tag).ok()?;
    if parsed.to_string() != tag || release.version.as_ref() != Some(&parsed) || parsed.pre.as_str().split('.').next() != Some("nightly") {
        return None;
    }
    Some(parsed)
}

/// Whether a release has a published checksum or checksum-list asset.
pub fn has_published_checksum(r: &Release) -> bool {
    r.sums_asset().is_some() || r.assets.iter().any(|a| a.sha256.is_some())
}

/// Selects a published release. A nightly must be canonical and have a published checksum;
/// a pin never falls back. Nightlies use SemVer precedence, ignoring build metadata. Equal
/// precedence prefers the lexically smallest tag, never publication time or a separate run ordinal.
pub fn choose_version(releases: &[Release], track: &crate::protocol::VersionTrack) -> Option<Release> {
    choose_version_with_policy(releases, track, NightlySelection::Canonical)
}

/// The separate channel has canonical build ordering; ordinary legacy Nightly tracks retain
/// main's publication-time ordering. Eligibility such as checksums/targets remains caller-owned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NightlySelection {
    Canonical,
    Published,
}

/// Best candidates first, without cloning release bodies or fetching evidence.
/// Stable sorting then reversing also preserves the old last-input winner on exact ties.
/// Native discovery must reject malformed candidate identities before calling this helper and
/// must not silently prefilter missing checksum evidence; ordinary selection requires checksums.
pub fn ranked_nightly_candidates<'a>(releases: impl IntoIterator<Item = &'a Release>, policy: NightlySelection) -> std::vec::IntoIter<&'a Release> {
    let mut ordered = match policy {
        NightlySelection::Canonical => {
            let mut candidates: Vec<_> =
                releases.into_iter().filter_map(|release| canonical_nightly_version(release).map(|version| (version, release))).collect();
            candidates.sort_by(|(av, a), (bv, b)| version::compare(av, bv).then_with(|| b.tag.cmp(&a.tag)));
            candidates.into_iter().map(|(_, release)| release).collect::<Vec<_>>()
        }
        NightlySelection::Published => {
            let mut candidates: Vec<_> = releases.into_iter().filter(|release| is_nightly(release)).collect();
            candidates.sort_by_key(|release| release.published_at);
            candidates
        }
    };
    ordered.reverse();
    ordered.into_iter()
}

/// What to ask GitHub about one app's releases. `repo` may follow the feed's repo renames, so
/// whether the app's nightlies have their own channel is decided by the app, never by `repo`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseQuery {
    pub repo: String,
    pub separate_nightly: bool,
    /// The app's [`former_repo`](crate::CraftApp::former_repo), asked when `repo` has no release.
    pub former_repo: Option<String>,
    /// Who is waiting for the answer, when the GitHub budget runs short.
    pub priority: Priority,
}

/// Who a GitHub request is for, most urgent first. When the GitHub budget runs short, the less
/// urgent ones wait so something the user just asked for still gets an answer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Something the user just did: Choose version, an install, a check they pressed.
    #[default]
    User,
    /// A background check of an app that is installed.
    Installed,
    /// A background check of an app that isn't installed.
    NotInstalled,
    /// A background check of an app following the nightly.
    NightlyTrack,
    /// Finding new builds for the Developer Builds page.
    Discovery,
}

impl ReleaseQuery {
    pub fn for_app(app: &crate::CraftApp, repo: impl Into<String>) -> Self {
        Self {
            repo: repo.into(),
            separate_nightly: app.has_separate_nightly_channel(),
            former_repo: app.former_repo.map(str::to_string),
            priority: Priority::User,
        }
    }

    /// The same question, asked on behalf of `priority`.
    pub fn with_priority(self, priority: Priority) -> Self {
        Self { priority, ..self }
    }

    /// The same question about the former repo, when there is one. Its answer's downloads must be
    /// under the former repo, as for any repo asked.
    pub fn former(&self) -> Option<Self> {
        self.former_repo.clone().map(|repo| Self { repo, separate_nightly: self.separate_nightly, former_repo: None, priority: self.priority })
    }
}

/// A version whose prerelease names a Nightly build, whatever the release's prerelease flag says.
fn nightly_version(version: &Version) -> bool {
    version.pre.as_str().split('.').any(|part| part.eq_ignore_ascii_case("nightly"))
}

/// A Nightly of an app with its own channel: by its flag or by its tag alone, so a build published
/// without the prerelease flag still stays out of the ordinary channel.
pub fn separate_channel_nightly(release: &Release) -> bool {
    is_nightly(release) || release.version.as_ref().is_some_and(nightly_version)
}

/// A Nightly-follow choice or a parsed exact Nightly pin, including main-supported legacy tags.
pub fn is_nightly_track(track: &crate::protocol::VersionTrack) -> bool {
    use crate::protocol::VersionTrack;
    match track {
        VersionTrack::Nightly => true,
        VersionTrack::Pinned(tag) => version::parse(tag).is_some_and(|v| nightly_version(&v)),
        VersionTrack::Stable => false,
    }
}

/// Ordinary settings for separately channelled apps cannot redirect their stable updater.
pub fn ordinary_track_allowed(app: &crate::CraftApp, track: &crate::protocol::VersionTrack) -> bool {
    !app.has_separate_nightly_channel() || !is_nightly_track(track)
}

/// The most release pages an ordinary lookup reads past one that holds no ordinary release, so a
/// page full of Nightlies cannot hide the versions behind it. Shared by every client that makes the
/// same promise: the desktop's API and version-list lookups, and the browser's pre-release lookup.
pub const ORDINARY_PAGES: usize = 5;

/// Ordinary release and cache paths never offer a Nightly, for any app: a pre-release means a
/// beta or release candidate. Nightlies have their own channel, or the explicit Nightly track.
pub fn ordinary_release_allowed(release: &Release) -> bool {
    !separate_channel_nightly(release)
}

/// Ordinary source callers keep main's Nightly selection for apps without their own channel.
pub fn choose_version_for(releases: &[Release], track: &crate::protocol::VersionTrack, separate_nightly: bool) -> Option<Release> {
    let policy = if separate_nightly { NightlySelection::Canonical } else { NightlySelection::Published };
    choose_version_with_policy(releases, track, policy)
}

fn choose_version_with_policy(releases: &[Release], track: &crate::protocol::VersionTrack, policy: NightlySelection) -> Option<Release> {
    use crate::protocol::VersionTrack;
    match track {
        VersionTrack::Stable => releases
            .iter()
            .filter(|r| !r.prerelease && r.version.as_ref().is_some_and(version::is_stable))
            .max_by(|a, b| a.version.cmp(&b.version))
            .cloned(),
        VersionTrack::Nightly => ranked_nightly_candidates(releases.iter().filter(|release| has_published_checksum(release)), policy).next().cloned(),
        VersionTrack::Pinned(tag) => releases.iter().find(|r| r.tag == *tag).cloned(),
    }
}

/// Reads `GET /repos/<repo>/releases` and returns the newest stable (or any, with
/// `allow_prerelease`) published release. Nightlies are never part of it.
///
/// # Errors
///
/// Returns an error if the JSON is malformed or its top level is not an array.
pub fn parse_github_release_list(json: &str, repo: &str, allow_prerelease: bool) -> Result<Option<Release>, String> {
    let v: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("unreadable answer from GitHub ({e})"))?;
    let list = v.as_array().ok_or("unexpected answer from GitHub")?;
    let mut best: Option<Release> = None;
    for item in list {
        if item["draft"].as_bool().unwrap_or(false) {
            continue;
        }
        let Ok(r) = release_from_value(item, repo) else {
            continue;
        };
        let Some(ver) = r.version.as_ref() else {
            continue;
        };
        if !ordinary_release_allowed(&r) || (!allow_prerelease && (r.prerelease || !version::is_stable(ver))) {
            continue;
        }
        let newer = best.as_ref().and_then(|b| b.version.as_ref()).is_none_or(|b| version::is_newer(ver, b));
        if newer {
            best = Some(r);
        }
    }
    Ok(best)
}

fn release_from_value(v: &serde_json::Value, repo: &str) -> Result<Release, String> {
    if let Some(msg) = v["message"].as_str() {
        return Err(if msg == "Not Found" { "no releases yet".into() } else { format!("GitHub says: {}", truncate(msg, 200)) });
    }
    let tag = v["tag_name"].as_str().filter(|t| valid_tag(t)).ok_or("no release found")?.to_string();
    let releases = format!("https://github.com/{repo}/releases");
    let html_url = v["html_url"].as_str().and_then(|u| safe_release_html_url(repo, u)).unwrap_or_else(|| releases.clone());
    let prefix = download_prefix(repo);
    let mut assets: Vec<Asset> = v["assets"]
        .as_array()
        .map(|list| {
            list.iter()
                .filter_map(|a| {
                    let name = a["name"].as_str().filter(|n| valid_file_name(n))?.to_string();
                    let url = a["browser_download_url"]
                        .as_str()
                        .filter(|u| download_url_under(u, &prefix) && *u == format!("{prefix}{tag}/{name}"))?
                        .to_string();
                    let sha256 = a["digest"].as_str().and_then(|d| d.strip_prefix("sha256:")).filter(|h| is_hex64(h)).map(str::to_ascii_lowercase);
                    // Reviewed package bytes, tied to this exact official repository/tag/name.
                    let sha256 = reviewed_assets(repo, &tag).find(|asset| asset.name == name).and_then(|asset| asset.sha256).or(sha256);
                    Some(Asset { name, size: a["size"].as_u64().unwrap_or(0), url, sha256 })
                })
                .collect()
        })
        .unwrap_or_default();
    // These publishers keep the reviewed macOS images in their own release archives.
    if [crate::curated::CALIBRE.repo, crate::curated::OPENTTD.repo].contains(&repo) {
        for asset in reviewed_assets(repo, &tag) {
            assets.retain(|other| other.name != asset.name);
            assets.push(asset);
        }
    }
    Ok(Release {
        version: release_tag_version(repo, &tag),
        name: v["name"].as_str().map(|n| truncate(n, 120)).filter(|n| !n.is_empty()).unwrap_or_else(|| tag.clone()),
        tag,
        published_at: v["published_at"].as_str().and_then(crate::time::parse_rfc3339),
        html_url,
        notes: v["body"].as_str().map(|b| truncate(b, MAX_NOTES)).unwrap_or_default(),
        prerelease: v["prerelease"].as_bool().unwrap_or(false),
        assets,
    })
}

/// Reads a release tag using only the version rules of its exact publisher repository.
pub fn release_tag_version(repo: &str, tag: &str) -> Option<Version> {
    if repo == crate::curated::DBEAVER_CE.repo {
        version::parse_dbeaver(tag)
    } else if repo == crate::curated::PRUSA_SLICER.repo {
        prusa_slicer_version(tag.strip_prefix("version_")?)
    } else if repo == crate::curated::KDENLIVE.repo {
        kdenlive_mac_version(tag)
    } else if repo == crate::curated::AUDACITY.repo {
        version::parse(tag.strip_prefix("Audacity-")?)
    } else if repo == crate::curated::FREEPLANE.repo {
        version::parse(tag.strip_prefix("release-")?)
    } else if repo == crate::curated::ENTE_AUTH.repo {
        ente_auth_version(tag)
    } else if repo == crate::curated::TERMIX.repo {
        termix_desktop_version(tag)
    } else if repo == crate::curated::OPENCODE.repo {
        opencode_desktop_version(tag)
    } else if repo == crate::curated::KRITA.repo {
        krita_mac_version(tag)
    } else if repo == crate::curated::NETNEWSWIRE.repo {
        netnewswire_mac_version(tag)
    } else if repo == crate::curated::PICARD.repo {
        picard_version(tag)
    } else if repo == crate::curated::ANKI.repo {
        anki_date_version(tag).or_else(|| version::parse(tag))
    } else if repo == crate::curated::UTM.repo && tag == "v5.0.9-2" {
        // The stable retag contains UTM 5.0.9 build 132, not a prerelease.
        version::parse("5.0.9")
    } else if repo == crate::curated::ZEN.repo {
        zen_release_version(tag)
    } else if repo == crate::curated::GODOT_RELEASES_REPO {
        godot_stable_version(tag)
    } else {
        version::parse(tag)
    }
}

/// PrusaSlicer's stable release and bundle metadata contain the same numeric version.
pub fn prusa_slicer_version(label: &str) -> Option<Version> {
    let version = Version::parse(label).ok()?;
    (version.pre.is_empty() && version.build.is_empty() && version.to_string() == label).then_some(version)
}

/// Zen's stable channel appends b to both the release tag and the bundle version.
pub fn zen_release_version(tag: &str) -> Option<Version> {
    let version = version::parse(tag.strip_suffix('b')?)?;
    let canonical = tag == format!("{version}b") || (version.patch == 0 && tag == format!("{}.{}b", version.major, version.minor));
    (version.pre.is_empty() && version.build.is_empty() && canonical).then_some(version)
}

// Anki's calendar tags pad the month while the macOS bundle uses ordinary SemVer.
fn anki_date_version(tag: &str) -> Option<Version> {
    let (year, rest) = tag.split_once('.')?;
    let (month, patch) = rest.split_once('.')?;
    let version = Version::new(year.parse().ok()?, month.parse().ok()?, patch.parse().ok()?);
    ((1..=12).contains(&version.minor) && tag == format!("{}.{:02}.{}", version.major, version.minor, version.patch)).then_some(version)
}

/// GitHub publishes OpenCode 2 alongside the legacy desktop updater's 1.x releases.
pub fn opencode_desktop_version(tag: &str) -> Option<Version> {
    let label = tag.strip_prefix('v')?;
    let version = Version::parse(label).ok()?;
    (version.major == 2 && version.to_string() == label).then_some(version)
}

pub const OPENCODE_DESKTOP_FEED: &str = "https://opencode.ai/update/api/latest/desktop/opencode/";

/// The publisher's current stable desktop release, mapped to the same bytes on GitHub.
pub fn opencode_desktop_release(bytes: &[u8]) -> Option<Release> {
    let value: serde_json::Value = serde_json::from_slice(bytes).ok()?;
    if value["channel"].as_str()? != "latest"
        || value["distribution"].as_str()? != "opencode"
        || value["name"].as_str()? != "desktop"
        || !value["active"].as_bool()?
    {
        return None;
    }
    let version_label = value["version"].as_str()?;
    let tag = format!("v{version_label}");
    let version = opencode_desktop_version(&tag)?;
    if !version.pre.is_empty() || !version.build.is_empty() {
        return None;
    }
    let repo = crate::curated::OPENCODE.repo;
    let asset = |name: &str| {
        let file = &value["metadata"]["files"][name];
        let size = file["size"].as_u64().filter(|size| *size > 0)?;
        let sha256 = file["sha256"].as_str().filter(|hash| is_hex64(hash))?.to_ascii_lowercase();
        if file["url"].as_str()? != format!("https://opencode.ai/files/bin/{version_label}/{name}") {
            return None;
        }
        let url = format!("{}{tag}/{name}", download_prefix(repo));
        Some(Asset { name: name.into(), url, size, sha256: Some(sha256) })
    };
    let mut assets = vec![asset("opencode-desktop-mac-arm64.dmg")?];
    assets.extend(asset("opencode-desktop-linux-x86_64.AppImage"));
    Some(Release {
        version: Some(version),
        name: format!("OpenCode {version_label}"),
        html_url: format!("https://github.com/{repo}/releases/tag/{tag}"),
        tag,
        assets,
        ..Release::default()
    })
}

/// Desktop recipes that require an exact upstream x86_64 AppImage.
pub fn qualified_linux_appimage_recipe(stems: &[&str]) -> Option<&'static crate::CraftApp> {
    use crate::curated::{
        DESKREEN, DUST3D, DYAD, MISTER_DOWNLOAD_MANAGER, OPEN_PENCIL, OPENCHAMBER, OPENCODE, ORCA, PASEO, RAWTHERAPEE, RECORDLY, SNIFFNET, T3_CODE,
        ULTIMAKER_CURA, VOICE_STUDIO,
    };

    const RECIPES: &[crate::CraftApp] = &[
        DESKREEN,
        DUST3D,
        DYAD,
        MISTER_DOWNLOAD_MANAGER,
        OPEN_PENCIL,
        OPENCHAMBER,
        OPENCODE,
        ORCA,
        PASEO,
        RAWTHERAPEE,
        RECORDLY,
        SNIFFNET,
        T3_CODE,
        ULTIMAKER_CURA,
        VOICE_STUDIO,
    ];
    RECIPES.iter().find(|app| stems.contains(&app.id))
}

pub fn qualified_linux_appimage_asset(app: &crate::CraftApp, release: &Release, asset: &Asset) -> bool {
    use crate::curated::{OPENCODE, ORCA};

    let Some(version) = release.version.as_ref() else {
        return false;
    };
    if qualified_linux_appimage_recipe(&[app.id]).is_none_or(|recipe| recipe.repo != app.repo)
        || release.prerelease
        || !version.pre.is_empty()
        || !version.build.is_empty()
        || release_tag_version(app.repo, &release.tag).as_ref() != Some(version)
    {
        return false;
    }
    let name = match app.id {
        id if id == OPENCODE.id => "opencode-desktop-linux-x86_64.AppImage",
        id if id == ORCA.id => "orca-linux.AppImage",
        _ => {
            return reviewed_assets(app.repo, &release.tag).any(|reviewed| reviewed.name.ends_with(".AppImage") && &reviewed == asset);
        }
    };
    release.tag == format!("v{version}")
        && asset.name == name
        && asset.url == format!("{}{}/{name}", download_prefix(app.repo), release.tag)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
}

pub(crate) fn godot_stable_version(tag: &str) -> Option<Version> {
    let label = tag.strip_suffix("-stable")?;
    let version = version::parse(label)?;
    let canonical = version.to_string() == label || (version.patch == 0 && format!("{}.{}", version.major, version.minor) == label);
    (version.pre.is_empty() && version.build.is_empty() && canonical).then_some(version)
}

/// Godot publishes binaries from a dedicated repository. Pin the stable, standard, universal editor variant.
pub fn godot_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && godot_stable_version(&release.tag).as_ref().is_some_and(|version| Some(version) == release.version.as_ref())
        && asset.name == format!("Godot_v{}_macos.universal.zip", release.tag)
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::GODOT_RELEASES_REPO), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
}

const TERMIX_MAC_DMG: &str = "termix_macos_arm64_dmg.dmg";

fn termix_desktop_version(tag: &str) -> Option<Version> {
    let label = tag.strip_prefix('v')?;
    let version = Version::parse(label).ok()?;
    (version.pre.is_empty() && version.build.is_empty() && version.to_string() == label).then_some(version)
}

pub fn termix_mac_package_url(tag: &str) -> Option<String> {
    termix_desktop_version(tag)?;
    Some(format!("{}{tag}/{TERMIX_MAC_DMG}", download_prefix(crate::curated::TERMIX.repo)))
}

/// Bind Termix's unversioned filename to the stable desktop tag and standard ARM disk image.
pub fn termix_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && termix_desktop_version(&release.tag).as_ref().is_some_and(|version| Some(version) == release.version.as_ref())
        && asset.name == TERMIX_MAC_DMG
        && termix_mac_package_url(&release.tag).as_deref() == Some(asset.url.as_str())
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
}

const NEOVIDE_MAC_DMG: &str = "Neovide-aarch64-apple-darwin.dmg";

/// The filename does not encode a version. Bind the exact Apple Silicon package to the official stable tag.
pub fn neovide_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| version.pre.is_empty() && version.build.is_empty() && version.to_string() == release.tag)
        && asset.name == NEOVIDE_MAC_DMG
        && asset.url == format!("{}{}/{NEOVIDE_MAC_DMG}", download_prefix(crate::curated::NEOVIDE.repo), release.tag)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::NEOVIDE.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Match only the Apple Silicon image of the official stable release.
pub fn vorta_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| version.pre.is_empty() && version.build.is_empty() && format!("v{version}") == release.tag)
        && asset.name == format!("Vorta-{}-arm.dmg", release.tag)
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::VORTA.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::VORTA.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Match only the official stable Apple Silicon disk image, not the Intel build or updater ZIP.
pub fn openscreen_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("Openscreen-macOS-Apple-Silicon-{version}.dmg")
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::OPENSCREEN.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::OPENSCREEN.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Match the stable Apple Silicon installer and its publisher's digest.
pub fn vibe_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("vibe_{version}_aarch64.dmg")
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::VIBE.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::VIBE.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Stable releases ship a signed universal DMG. Development builds have a different filename.
pub fn dosbox_staging_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("dosbox-staging-macOS-{}.dmg", release.tag)
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::DOSBOX_STAGING.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::DOSBOX_STAGING.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Selects the official stable Apple Silicon package, never the Intel build or an updater archive.
pub fn blockbench_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("Blockbench_arm64_{version}.dmg")
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::BLOCKBENCH.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::BLOCKBENCH.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Selects the official stable Apple Silicon package, never the Intel build or an updater archive.
pub fn appflowy_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == version.to_string()
                && asset.name == format!("AppFlowy-{version}-macos-arm64.dmg")
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::APPFLOWY.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::APPFLOWY.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Selects the official stable Apple Silicon package, never the Intel build or an updater archive.
pub fn actual_budget_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty() && version.build.is_empty() && release.tag == format!("v{version}") && asset.name == "Actual-mac-arm64.dmg"
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::ACTUAL_BUDGET.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::ACTUAL_BUDGET.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

const SUPER_PRODUCTIVITY_MAC_DMG: &str = "superProductivity-arm64.dmg";

/// Selects the stable Apple Silicon DMG. The identical filename across releases is
/// bound to the canonical tag and a publisher digest.
pub fn super_productivity_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release
            .version
            .as_ref()
            .is_some_and(|version| version.major >= 20 && version.pre.is_empty() && version.build.is_empty() && release.tag == format!("v{version}"))
        && asset.name == SUPER_PRODUCTIVITY_MAC_DMG
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::SUPER_PRODUCTIVITY.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::SUPER_PRODUCTIVITY.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

const DB_BROWSER_SQLITE_ARM64_3_12_2: &str = "DB.Browser.for.SQLite-arm64-3.12.2.dmg";

/// Older releases separate Apple Silicon from Intel. The original v3.13.0 universal
/// DMG identifies itself as 3.13.99, so it is deliberately excluded.
pub fn db_browser_sqlite_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            let package_matches = if release.tag == "v3.12.2" {
                asset.name == DB_BROWSER_SQLITE_ARM64_3_12_2
            } else {
                (version.major, version.minor, version.patch) >= (3, 13, 1) && asset.name == format!("DB.Browser.for.SQLite-v{version}.dmg")
            };
            version.pre.is_empty() && version.build.is_empty() && release.tag == format!("v{version}") && package_matches
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::DB_BROWSER_SQLITE.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::DB_BROWSER_SQLITE.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Desktop bundle recipes restricted to exact packages qualified on Apple Silicon.
pub fn qualified_mac_dmg_recipe(stems: &[&str]) -> Option<&'static crate::CraftApp> {
    const RECIPES: &[crate::CraftApp] = &[
        crate::curated::OPENCLAW,
        crate::curated::RUSTDESK,
        crate::curated::OPENCODE,
        crate::curated::OLLAMA,
        crate::curated::CLASH_VERGE_REV,
        crate::curated::ORCA,
        crate::curated::VOICE_STUDIO,
        crate::curated::T3_CODE,
        crate::curated::ANYTHING_LLM,
        crate::curated::UPSCAYL,
        crate::curated::LOSSLESSCUT,
        crate::curated::SIYUAN,
        crate::curated::MAGPIE,
        crate::curated::GHOST_DOWNLOADER,
        crate::curated::KANDO,
        crate::curated::TRILIUM,
        crate::curated::RECORDLY,
        crate::curated::ZETTLR,
        crate::curated::VSCODIUM,
        crate::curated::ANKI,
        crate::curated::CALIBRE,
        crate::curated::OPENCHAMBER,
        crate::curated::OSAURUS,
        crate::curated::ZEN,
        crate::curated::HEADLAMP,
        crate::curated::KAP,
        crate::curated::SIMPLEX_CHAT,
        crate::curated::LOCALAI,
        crate::curated::BRUNO,
        crate::curated::JAN,
        crate::curated::DEEPTUTOR,
        crate::curated::OPENWORK_DESKTOP,
        crate::curated::PASEO,
        crate::curated::FREECAD,
        crate::curated::OPENMAUSBOT,
        crate::curated::MUSESCORE,
        crate::curated::ORCA_SLICER,
        crate::curated::DBEAVER_CE,
        crate::curated::PRUSA_SLICER,
        crate::curated::ZOTERO,
        crate::curated::DESKREEN,
        crate::curated::MOONLIGHT,
        crate::curated::OPEN_PENCIL,
        crate::curated::OPEN_WHISPR,
        crate::curated::SUNSHINE,
        crate::curated::OPENTTD,
        crate::curated::RECTANGLE,
        crate::curated::CODEEDIT,
        crate::curated::LINEAR_MOUSE,
        crate::curated::FINICKY,
        crate::curated::VIRTUALBUDDY,
        crate::curated::BLENDER,
        crate::curated::GIMP,
        crate::curated::KRITA,
        crate::curated::VLC,
        crate::curated::INKSCAPE,
        crate::curated::KDENLIVE,
        crate::curated::SUBTITLE_EDIT,
        crate::curated::DEEPCHAT,
        crate::curated::EASYDICT,
        crate::curated::UTM,
        crate::curated::OPENMTP,
        crate::curated::FREEPLANE,
    ];
    RECIPES.iter().find(|app| stems.contains(&app.id))
}

pub fn qualified_mac_dmg_asset(app: &crate::CraftApp, release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            if app.repo == crate::curated::DBEAVER_CE.repo && app.id == crate::curated::DBEAVER_CE.id {
                return version::parse_dbeaver(&release.tag).as_ref() == Some(version);
            }
            let canonical_tag = if app.uses_krita_releases() {
                krita_mac_version(&release.tag).as_ref() == Some(version)
            } else if app.uses_kdenlive_releases() {
                kdenlive_mac_version(&release.tag).as_ref() == Some(version)
            } else if (app.id, app.repo) == (crate::curated::PRUSA_SLICER.id, crate::curated::PRUSA_SLICER.repo) {
                release.tag == format!("version_{version}")
            } else if (app.id, app.repo) == (crate::curated::FREEPLANE.id, crate::curated::FREEPLANE.repo) {
                release.tag == format!("release-{version}")
            } else if (app.id, app.repo) == (crate::curated::OPENTTD.id, crate::curated::OPENTTD.repo) {
                version.patch == 0 && release.tag == format!("{}.{}", version.major, version.minor)
            } else if (app.id, app.repo) == (crate::curated::VIRTUALBUDDY.id, crate::curated::VIRTUALBUDDY.repo) {
                release.tag == version.to_string() || (version.patch == 0 && release.tag == format!("{}.{}", version.major, version.minor))
            } else if (app.id, app.repo) == (crate::curated::UTM.id, crate::curated::UTM.repo) && release.tag == "v5.0.9-2" {
                release_tag_version(app.repo, &release.tag).as_ref() == Some(version)
            } else if app.id == crate::curated::ANKI.id {
                anki_date_version(&release.tag).as_ref() == Some(version)
            } else if (app.id, app.repo) == (crate::curated::ZEN.id, crate::curated::ZEN.repo) {
                zen_release_version(&release.tag).as_ref() == Some(version)
            } else if [crate::curated::RUSTDESK.id, crate::curated::VSCODIUM.id].contains(&app.id)
                || (app.id, app.repo) == (crate::curated::OSAURUS.id, crate::curated::OSAURUS.repo)
                || (app.id, app.repo) == (crate::curated::FREECAD.id, crate::curated::FREECAD.repo)
                || (app.id, app.repo) == (crate::curated::EASYDICT.id, crate::curated::EASYDICT.repo)
                || app.uses_zotero_releases()
                || app.uses_blender_releases()
                || app.uses_gimp_releases()
                || app.uses_vlc_releases()
                || app.uses_inkscape_releases()
            {
                release.tag == version.to_string()
            } else {
                release.tag == format!("v{version}")
            };
            version.pre.is_empty() && version.build.is_empty() && canonical_tag
        })
        && reviewed_assets(app.repo, &release.tag).any(|reviewed| {
            reviewed.name.ends_with(".dmg")
                && reviewed.name == asset.name
                && reviewed.url == asset.url
                && reviewed.size == asset.size
                && reviewed.sha256 == asset.sha256
        })
}

/// A PKG is extracted only after qualification of that exact release and layout.
pub fn naps2_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == format!("v{version}"))
        && reviewed_assets(crate::curated::NAPS2.repo, &release.tag).any(|reviewed| {
            reviewed.name == asset.name && reviewed.url == asset.url && reviewed.size == asset.size && reviewed.sha256 == asset.sha256
        })
}

const SOLVESPACE_MAC_DMG: &str = "SolveSpace.dmg";

/// SolveSpace publishes the same DMG filename under stable tags such as v3.3.
pub fn solvespace_mac_asset(release: &Release, asset: &Asset) -> bool {
    let Some(version) = &release.version else { return false };
    let canonical_tag =
        release.tag == format!("v{version}") || (version.patch == 0 && release.tag == format!("v{}.{}", version.major, version.minor));
    !release.prerelease
        && version.pre.is_empty()
        && version.build.is_empty()
        && canonical_tag
        && asset.name == SOLVESPACE_MAC_DMG
        && asset.url == format!("{}{}/{SOLVESPACE_MAC_DMG}", download_prefix(crate::curated::SOLVESPACE.repo), release.tag)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::SOLVESPACE.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

const LAPCE_MAC_DMG: &str = "Lapce-macos.dmg";

/// The universal package is unversioned; bind it to the publisher's stable tag and digest.
pub fn lapce_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| version.pre.is_empty() && version.build.is_empty() && format!("v{version}") == release.tag)
        && asset.name == LAPCE_MAC_DMG
        && asset.url == format!("{}{}/{LAPCE_MAC_DMG}", download_prefix(crate::curated::LAPCE.repo), release.tag)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::LAPCE.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

const KEYCASTR_MAC_ZIP: &str = "KeyCastr.app.zip";

/// KeyCastr uses one unversioned ZIP name. The official stable tag and checksum identify its bytes.
pub fn keycastr_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| version.pre.is_empty() && version.build.is_empty() && format!("v{version}") == release.tag)
        && asset.name == KEYCASTR_MAC_ZIP
        && asset.url == format!("{}{}/{KEYCASTR_MAC_ZIP}", download_prefix(crate::curated::KEYCASTR.repo), release.tag)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::KEYCASTR.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// The official Maccy ZIP has one name across versions, so qualify each exact archive.
pub fn maccy_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| version.pre.is_empty() && version.build.is_empty() && release.tag == version.to_string())
        && reviewed_assets(crate::curated::MACCY.repo, &release.tag).any(|reviewed| {
            reviewed.name == asset.name && reviewed.url == asset.url && reviewed.size == asset.size && reviewed.sha256 == asset.sha256
        })
}

pub fn alt_tab_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == format!("v{version}"))
        && reviewed_assets(crate::curated::ALT_TAB.repo, &release.tag).any(|reviewed| reviewed == *asset)
}

pub fn amethyst_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == format!("v{version}"))
        && reviewed_assets(crate::curated::AMETHYST.repo, &release.tag).any(|reviewed| &reviewed == asset)
}

pub fn loop_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == version.to_string())
        && reviewed_assets(crate::curated::LOOP.repo, &release.tag).any(|reviewed| &reviewed == asset)
}

pub fn dyad_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == format!("v{version}"))
        && asset.name.ends_with(".zip")
        && reviewed_assets(crate::curated::DYAD.repo, &release.tag).any(|reviewed| &reviewed == asset)
}

pub fn hammerspoon_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == version.to_string())
        && reviewed_assets(crate::curated::HAMMERSPOON.repo, &release.tag).any(|reviewed| &reviewed == asset)
}

pub fn ice_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag == version.to_string())
        && reviewed_assets(crate::curated::ICE.repo, &release.tag).any(|reviewed| &reviewed == asset)
}

pub fn netnewswire_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && netnewswire_mac_version(&release.tag).as_ref() == release.version.as_ref()
        && reviewed_assets(crate::curated::NETNEWSWIRE.repo, &release.tag).any(|reviewed| reviewed == *asset)
}

fn picard_version(tag: &str) -> Option<Version> {
    let label = tag.strip_prefix("release-")?;
    let version = Version::parse(label).ok()?;
    (version.to_string() == label && version.pre.is_empty() && version.build.is_empty() && version.major >= 3).then_some(version)
}

/// Use the stable Apple Silicon disk image from the official Picard release.
pub fn picard_mac_asset(release: &Release, asset: &Asset) -> bool {
    !release.prerelease
        && picard_version(&release.tag).as_ref().is_some_and(|version| {
            Some(version) == release.version.as_ref() && asset.name == format!("MusicBrainz-Picard-{version}-macOS-13.0-arm64.dmg")
        })
        && asset.url == format!("{}{}/{}", download_prefix(crate::curated::PICARD.repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(crate::curated::PICARD.repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Only the official stable Handy AppImage for Linux x86_64 is managed.
pub fn handy_linux_asset(release: &Release, asset: &Asset) -> bool {
    let repo = crate::curated::HANDY.repo;
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("Handy_{version}_amd64.AppImage")
        })
        && asset.url == format!("{}{}/{}", download_prefix(repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

/// Only the official stable Ghost Downloader 3 AppImage for Linux x86_64 is managed.
pub fn ghost_downloader_linux_asset(release: &Release, asset: &Asset) -> bool {
    let repo = crate::curated::GHOST_DOWNLOADER.repo;
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| {
            version.pre.is_empty()
                && version.build.is_empty()
                && release.tag == format!("v{version}")
                && asset.name == format!("Ghost-Downloader-v{version}-Linux-x86_64.AppImage")
        })
        && asset.url == format!("{}{}/{}", download_prefix(repo), release.tag, asset.name)
        && asset.size > 0
        && asset.sha256.as_deref().is_some_and(is_hex64)
        && reviewed_assets(repo, &release.tag)
            .find(|reviewed| reviewed.name == asset.name)
            .is_none_or(|reviewed| reviewed.size == asset.size && reviewed.sha256 == asset.sha256)
}

fn ente_auth_version(tag: &str) -> Option<Version> {
    let label = tag.strip_prefix("auth-v")?;
    let version = version::parse(label)?;
    (version.pre.is_empty()
        && version.build.is_empty()
        && version.to_string() == label
        && (version.major, version.minor, version.patch) >= (4, 4, 26))
        .then_some(version)
}

/// Only the Ente Auth desktop DMG on the official GitHub release can name the stable channel.
pub fn ente_auth_tag_from_download(location: &str) -> Option<String> {
    let prefix = download_prefix(crate::curated::ENTE_AUTH.repo);
    let (tag, name) = location.strip_prefix(&prefix)?.split_once('/')?;
    let _ = ente_auth_version(tag)?;
    (name == format!("ente-{tag}.dmg")).then(|| tag.to_string())
}

/// Repositories whose Windows installer recipe requires a reviewed x64 MSI.
pub fn reviewed_windows_msi_repo(id: &str) -> Option<&'static str> {
    match id {
        "photocraft" => Some("storytold/photocraft"),
        "wordcraft" => Some("storytold/wordcraft"),
        "keepassxc" => Some(crate::curated::KEEPASSXC.repo),
        "drawio" => Some(crate::curated::DRAWIO.repo),
        "libreoffice" => Some(crate::curated::LIBREOFFICE.repo),
        _ => None,
    }
}

/// Admit only the exact official x64 MSI whose signed package was reviewed.
pub fn reviewed_windows_msi_asset(repo: &str, release: &Release, asset: &Asset) -> bool {
    if repo == crate::curated::LIBREOFFICE.repo {
        return !release.prerelease
            && libreoffice_stable_version(&release.tag).as_ref() == release.version.as_ref()
            && release.version.as_ref().and_then(libreoffice_windows_msi).as_ref() == Some(asset);
    }
    !release.prerelease
        && release.version.as_ref().is_some_and(|version| release.tag.strip_prefix('v').unwrap_or(&release.tag) == version.to_string())
        && asset.name.ends_with(".msi")
        && reviewed_assets(repo, &release.tag).any(|reviewed| &reviewed == asset)
}

// Official package receipts apply only to the original repository, tag and exact asset.
fn reviewed_assets<'a>(repo: &'a str, tag: &'a str) -> impl Iterator<Item = Asset> + 'a {
    let receipts: &[(&str, u64, &str)] = match (repo, tag) {
        ("XiaoYouChR/Ghost-Downloader-3", "v4.5.11") => &[
            ("Ghost-Downloader-v4.5.11-Linux-x86_64.AppImage", 94_968_312, "39177d2cde51617182be5bcd836a1722012acb0147f4f93791587effe2794956"),
            ("Ghost-Downloader-v4.5.11-macOS-arm64.dmg", 67_408_491, "36d531576819349920abb0008a6105046f5e48c3bcf7d8e478c61b03edb87ca9"),
        ],
        ("XiaoYouChR/Ghost-Downloader-3", "v4.6.1") => &[
            ("Ghost-Downloader-v4.6.1-Linux-x86_64.AppImage", 86_133_240, "68e6615f66d10f26c127e94d6db6926064831488fb6f5d93f1189ed4d21519eb"),
            ("Ghost-Downloader-v4.6.1-macOS-arm64.dmg", 59_261_074, "85f87ae7d2100b2296bf09097cc3e56011b1dc96e05fab7e1c7af6bf29caf38c"),
        ],
        ("utmapp/UTM", "v5.0.8") => &[("UTM.dmg", 304_304_581, "7b2d7b2e2a02b3de1757ef19a8f36573e6d9cceae8c8cca0abcbcaa4a852019a")],
        ("utmapp/UTM", "v5.0.9-2") => &[("UTM.dmg", 304_699_821, "1a76ddcf61ab424a5ae1d1b68a16a9b2e84b748716afce905ee77740d8e86127")],
        ("tisfeng/Easydict", "2.25.0") => &[("Easydict.dmg", 35_766_921, "a8b6893cd3366c864eaac22fd8fadd52968c5119dd40fe5d61947cad3d2bfd16")],
        ("tisfeng/Easydict", "2.26.0") => &[("Easydict.dmg", 35_761_660, "b88e29e8ffb62ecad2159964e2b2c06150c522c1de34f41c62faadd750d5715e")],
        ("ganeshrvel/openmtp", "v3.2.25") => {
            &[("openmtp-3.2.25-mac-arm64.dmg", 123_800_559, "801babe2b3629d12f59431189cf45f3c5d5006bc75e8a6871a7d0c97f17ec47f")]
        }
        ("ganeshrvel/openmtp", "v3.3.0") => {
            &[("openmtp-3.3.0-mac-arm64.dmg", 126_208_067, "bf6c2cbbea94c001cb781e1ed6ab2259ebcad01d8fce58120e5dcc253c228425")]
        }
        ("freeplane/freeplane", "release-1.13.3") => {
            &[("Freeplane-1.13.3-apple.dmg", 144_017_558, "f6c8639cbe91ca431c84b44eb64ab0b3f984454ec0203e1132e49586265a2347")]
        }
        ("freeplane/freeplane", "release-1.13.5") => {
            &[("Freeplane-1.13.5-apple.dmg", 147_343_258, "14e0badf46ac98aa3919503700772c66c02b59c0bba4711c370d149838d6acfb")]
        }
        ("cjpais/Handy", "v0.10.1") => {
            &[("Handy_0.10.1_amd64.AppImage", 123_988_472, "f89e267b89983d069a9cbfa74b5e93aa3fa04455702b84fb55a5e6792048395f")]
        }
        ("cjpais/Handy", "v0.10.2") => &[
            ("Handy_0.10.2_aarch64.dmg", 14_979_688, "b8b893fbf9e8262abb0e363cb0ac6715cc69eac2fd6ffd8c0785278083a0e3ca"),
            ("Handy_0.10.2_amd64.AppImage", 124_029_432, "17c5f1a4a33f9c4c5c234a084cc0aaf766f3a5b62662e02d70e51161800ace17"),
        ],
        ("localsend/localsend", "v1.19.0") => &[
            ("LocalSend-1.19.0.dmg", 52_878_051, "5645789cc976298b893267c025f0945d8673baed3bb302a201eb8b96991e43d4"),
            ("LocalSend-1.19.0-linux-x86-64.AppImage", 61_752_512, "e72ce88383a32beec9701583c22e298037f7683f9590cea7b3a281f58c2ec2b3"),
        ],
        ("obsproject/obs-studio", "33.0.3") => {
            &[("OBS-Studio-33.0.3-macOS-Apple.dmg", 209_590_030, "92212873b6f84c54285203cac3f846590ac00dfad76b312cf266100ff5edf0c8")]
        }
        ("kopia/kopia", "v0.23.0") => {
            &[("KopiaUI-0.23.0-arm64.dmg", 147_215_056, "387ca1530829fc95667b23e88b644f9f0cf9f49a2abfeffeed22e3fc261f6e29")]
        }
        ("kopia/kopia", "v0.23.1") => {
            &[("KopiaUI-0.23.1-arm64.dmg", 149_169_871, "f2adb1869c603c66aaeabd846affd21df426c614cc8285230f5465f9389a003c")]
        }
        ("transmission/transmission", "4.1.3") => {
            &[("Transmission-4.1.3.dmg", 11_186_296, "d622736c19990262f2ab12183e1acaa5b0dad5777ddc7db0a187c06e035a7e1a")]
        }
        ("transmission/transmission", "4.1.4") => {
            &[("Transmission-4.1.4.dmg", 11_545_854, "495c5d21d515c8deb859b0db6692fd5606644a1613c2cac87c1d3e7672755a59")]
        }
        ("iina/iina", "v1.5.0") => &[("IINA.v1.5.0.dmg", 113_048_935, "8fad50479b10e09645053457cec46bd53d054ced2242cfe68f74586735a20dc7")],
        ("iina/iina", "v1.5.1") => &[("IINA.v1.5.1.dmg", 113_092_405, "f414507ec60bcfeda3a9f539f74470253e31b619d031e7a9e9a71519da1598cd")],
        ("HandBrake/HandBrake", "1.11.1") => {
            &[("HandBrake-1.11.1.dmg", 47_719_592, "0973dcc4ccf53388498fbb35dbf7118b28276f2fe4ecbf08eca8596985f2c684")]
        }
        ("HandBrake/HandBrake", "1.11.2") => {
            &[("HandBrake-1.11.2.dmg", 47_978_117, "4afe27aaa77a7bbb0dcda1a335d96a5f53649f2649fb84de6982ffecc35de9d7")]
        }
        ("Zackriya-Solutions/meetily", "v0.4.1") => {
            &[("meetily_0.4.1_aarch64.dmg", 47_951_342, "16f84b1769619ba3da93bc43e9c1fa3320ce2b00cc9e99225a3d21302770eaf5")]
        }
        ("Zackriya-Solutions/meetily", "v0.5.0") => {
            &[("meetily_0.5.0_aarch64.dmg", 48_053_942, "c7ac63bf80c4085d95388e6c569c95f9a7bcb1b539f3fe964f4522c63eac2ef3")]
        }
        ("Rizzo-AI-Academy/rizzo-pii", "v2.1.0") => {
            &[("Rizzo-PII-2.1.0-macOS-arm64.dmg", 132_561_673, "f94fd21db84e9e355b9bd873c50686ead334e38bebed39ae6daf7f54c336dfe7")]
        }
        ("Rizzo-AI-Academy/rizzo-pii", "v2.2.0") => {
            &[("Rizzo-PII-2.2.0-macOS-arm64.dmg", 197_183_674, "eb732c316dc5a60c84afe5a921162c7abb52575b3fb6e0aaf0c7902642c0e808")]
        }
        ("audacity/audacity", "Audacity-4.0.1") => &[
            ("audacity-linux-4.0.1-x86_64.AppImage", 73_755_128, "ca2f04f172124d1f18ac608749854c5d31f04ab266a758c348190c05d9b2087c"),
            ("audacity-macOS-4.0.1-arm64.dmg", 50_034_126, "278c8647b78c77af7f07dbd5e7d9bfc950bc14168047b65738716b61d12055ec"),
        ],
        ("audacity/audacity", "Audacity-4.0.2") => &[
            ("audacity-linux-4.0.2-x86_64.AppImage", 73_726_456, "a5f8787aa13cdb500a60fc013403b986776876b960cca962bc6e7fff066172f3"),
            ("audacity-macOS-4.0.2-arm64.dmg", 50_030_766, "fd462da6204cbf9758b44c7ce1c29d5fe5cab4f31ba308d33c125f1b170401c3"),
        ],
        ("keepassxreboot/keepassxc", "2.7.12") => &[
            ("KeePassXC-2.7.12-arm64.dmg", 34_360_675, "65f4f63607180c0a15794b4a4068f85e99ed5391c87c1fb9312648f1b36fed40"),
            ("KeePassXC-2.7.12-Win64.msi", 35_901_440, "feee096c1d5d0d7bb7b36b18174818f7bac889be7668c550ff0e5ef9206ea9a5"),
        ],
        ("keepassxreboot/keepassxc", "2.8.0") => &[
            ("KeePassXC-2.8.0-1-arm64.dmg", 47_786_914, "85f0e75e258ef513f0fc7f5bf470cad0943b7d7e8191b2171f9898ad32e62b9c"),
            ("KeePassXC-2.8.0-x64.msi", 69_419_008, "4f105af5ba780428718c8be5614cf95deceed5cb3b474a9bd624c5df81cec6e5"),
        ],
        ("laurent22/joplin", "v3.7.22") => &[
            ("Joplin-3.7.22.AppImage", 207_329_204, "2b0b1ee82999f00883d21d950c2fb5dff28ccc3d2f570248b3abd40e5100dfdc"),
            ("Joplin-3.7.22-arm64.DMG", 211_992_036, "185395eba8d9dac2d8656884cbb0e5b96bcb40b41d80c67126ce5526cb5186c8"),
        ],
        ("laurent22/joplin", "v3.7.24") => &[
            ("Joplin-3.7.24.AppImage", 207_407_075, "64d544b8c865b53d975d1cc9438300465be860dfd9b6c7d4bb6f66a9f3777584"),
            ("Joplin-3.7.24-arm64.DMG", 212_067_439, "d95c2376035c7630dfb94f6eecca2f6edb3a27531626553a344f2b2308a5830e"),
        ],
        ("mltframework/shotcut", "v26.9.27") => {
            &[("shotcut-macos-26.9.27.dmg", 210_131_979, "629b056bfee1670c1669dd73f19f76fe48e0cdf90e184dc1aa1bdd6d09aa2548")]
        }
        ("mltframework/shotcut", "v26.11.15") => {
            &[("shotcut-macos-26.11.15.dmg", 211_234_714, "664c6ea4195d58dbf3fd43f3d74ae46d59b6310f9ac628d5e70a8e2786a4f80b")]
        }
        ("PintaProject/Pinta", "3.1.1") => {
            &[("Pinta-macos-arm64.dmg", 67_815_026, "84912d381d835a912a11c092e74fef7ad5c13e8563d94dedb1f5e9ad7e9b2b3a")]
        }
        ("PintaProject/Pinta", "3.1.2") => {
            &[("Pinta-macos-arm64.dmg", 68_171_678, "daad4b3550e7d5f27928ccb9ca99392aa46a5ef71cfd54ce72c0b7a810f4f993")]
        }
        ("zed-industries/zed", "v1.29.3") => &[("Zed-aarch64.dmg", 88_779_557, "b692c9effdfcec874c47f56c3b27df069a63c6b3d23ecd070ffec9238a708bb8")],
        ("zed-industries/zed", "v1.30.2") => &[("Zed-aarch64.dmg", 88_057_394, "36ab60379627730fc3ac6ab79bc57dba7a1cf6d6df8117b8e22190d9b639fff5")],
        ("jgraph/drawio-desktop", "v31.5.2") => {
            &[("draw.io-31.5.2.msi", 151_302_144, "08629728da5caee6c5975e9ae6142ce2f64a5608cf785da2ba993c70aabf1911")]
        }
        ("jgraph/drawio-desktop", "v32.6.10") => &[
            ("drawio-x86_64-32.6.10.AppImage", 152_706_651, "94158126a43016b4e6f24bbc2c840ab8b5a51c6bf2796e1b3dadc64390b2d256"),
            ("draw.io-arm64-32.6.10.dmg", 157_367_951, "83e4027837f89bfba9dafbcc5c0e4befa76e2ac7f4e37f7a7b40c817141a6727"),
        ],
        ("jgraph/drawio-desktop", "v32.7.4") => &[
            ("drawio-x86_64-32.7.4.AppImage", 154_092_007, "7d78c781d4ec33d86dc48251c3cadc9c18dcea6bc5d581322c0335ccb33b222c"),
            ("draw.io-arm64-32.7.4.dmg", 158_930_055, "b0ba01224cdc4817ebf218d73b2a45ff3569f8c9135ad422c5157655062900b3"),
            ("draw.io-32.7.4.msi", 150_347_776, "e2ab1ca54a249b29dd4935017c4994ab2b7732a99fdf7b5b90c50799c75c3bc9"),
        ],
        ("exelban/stats", "v3.1.0") => &[("Stats.dmg", 8_257_600, "a5f16b6606c8bd1fc847439a3b4a507153837167cb333514954882c2bc31e0e7")],
        ("exelban/stats", "v3.1.1") => &[("Stats.dmg", 8_386_915, "2fbe64690251e5c92102a2091052c76f440561397fcb7e0246821c331386bdd9")],
        ("EpicenterHQ/epicenter", "v7.9.0") => {
            &[("Whispering_7.9.0_aarch64.dmg", 15_206_641, "6a0e45d47ecdb2ada6afa8309914dd73c03db896fd153272e0891976867851c7")]
        }
        ("EpicenterHQ/epicenter", "v7.11.0") => {
            &[("Whispering_7.11.0_aarch64.dmg", 15_606_745, "7c15883a5562e594633e2636db5abbc8197b8ae93c4714711bf802b531eaf2d0")]
        }
        ("ente/ente", "auth-v4.4.26") => &[("ente-auth-v4.4.26.dmg", 46_193_389, "f063d87dc247feed7dba0ecdae208ffc22f9b5b2de0c58506394a2cae0610ada")],
        ("ente/ente", "auth-v4.4.27") => &[("ente-auth-v4.4.27.dmg", 58_891_715, "9c05d588b612d697f50d5b54cce1677cfac3c38a9aa32910279f68cd81f6c718")],
        ("cryptomator/cryptomator", "1.19.6") => &[
            ("cryptomator-1.19.6-x86_64.AppImage", 59_075_064, "a6167f6ab3c5f161b31726b94bcf0abcb7f78c857b16e485adeb8e5096b74acd"),
            ("Cryptomator-1.19.6-arm64.dmg", 58_714_109, "fdfc8ef50b10baca1078b7a9edf404e330363c8a3a40fda29f86dd5d3a0a0b02"),
        ],
        ("cryptomator/cryptomator", "1.19.7") => &[
            ("cryptomator-1.19.7-x86_64.AppImage", 59_075_064, "7f0367d8b83fcda665016e1a907683a9037e61589f2493c4206ea850d55fef4e"),
            ("Cryptomator-1.19.7-arm64.dmg", 58_716_219, "c4818d38cb3f9919bfe276c812714596cc50da4d01a82ac2f22bf2e64b441677"),
        ],
        ("ActivityWatch/activitywatch", "v0.14.2") => &[
            ("activitywatch-v0.14.2-macos-arm64.dmg", 65_835_856, "8341d56b5b43a60ca04ea2a3cc5ffbfb5e661cdb6430145d9ceb0f49dca63d65"),
            ("activitywatch-linux-x86_64.AppImage", 109_992_440, "216575f31d47488d8865c70352f948c52711ca348e74fc2412da82e6273b3621"),
        ],
        ("ActivityWatch/activitywatch", "v0.14.3") => &[
            ("activitywatch-v0.14.3-macos-arm64.dmg", 67_250_584, "5462f3b90f9e1c0352061888d5f041029abb2ccfbe05e42c6dc63d91ff26bd4a"),
            ("activitywatch-linux-x86_64.AppImage", 110_492_152, "875aa539e81006e26695675dc55a14496d5a14fa7730495bda6a2e08ea92e59d"),
        ],
        ("borgbase/vorta", "v0.12.1") => &[("Vorta-v0.12.1-arm.dmg", 45_686_573, "b65da5b0fc3dcb1792897eaf2dad62e0b2cbc1db4a577ff39668b28c7ae2600b")],
        ("borgbase/vorta", "v0.12.2") => &[("Vorta-v0.12.2-arm.dmg", 45_694_460, "2a377fa5e04068cdc4f8c9316e34046cc1bae11acca4e8daf20e4ca7fdb8723d")],
        ("getopenscreen/openscreen", "v2.3.0") => {
            &[("Openscreen-macOS-Apple-Silicon-2.3.0.dmg", 141_157_755, "db2f75451ac364e19c4d56e45a85b8572056312c54461979f13c7b1053ea2d82")]
        }
        ("getopenscreen/openscreen", "v2.4.0") => {
            &[("Openscreen-macOS-Apple-Silicon-2.4.0.dmg", 141_308_945, "177374fd4b1b9290661db535010032247e0cea42f9bdf281c918c5e418876a51")]
        }
        ("thewh1teagle/vibe", "v3.3.5") => {
            &[("vibe_3.3.5_aarch64.dmg", 38_715_249, "efaf965175cf9d6b7c6782e38eb42b50800a4645c06771584fcbd46c74d75807")]
        }
        ("thewh1teagle/vibe", "v3.3.8") => {
            &[("vibe_3.3.8_aarch64.dmg", 38_640_733, "48dd4ffd5b4fc6a9466fc2b68ee04d398e794404758577e46f27163614138a39")]
        }
        ("dosbox-staging/dosbox-staging", "v0.83.0") => {
            &[("dosbox-staging-macOS-v0.83.0.dmg", 44_531_761, "d8a771adfb8010fa6b5f7fb5351abfba659273ad01c89f03675a92bdbdae8167")]
        }
        ("dosbox-staging/dosbox-staging", "v0.83.1") => {
            &[("dosbox-staging-macOS-v0.83.1.dmg", 44_185_617, "a016f04c25b0c27cc3dfa4165cd7242feb36d647fc9b3e4990ca83257d1012ad")]
        }
        ("JannisX11/blockbench", "v5.2.1") => {
            &[("Blockbench_arm64_5.2.1.dmg", 128_702_946, "20fd51a18c7a757d9cb03bb0b89c99874d2d5bdbae733817a7877b15043aa78c")]
        }
        ("JannisX11/blockbench", "v5.2.2") => {
            &[("Blockbench_arm64_5.2.2.dmg", 128_721_695, "0d072bf863ee3e0bcaecbd0de67a5ffd29dfbf6ee2e8fe5eeb872a14331c5c03")]
        }
        ("AppFlowy-IO/AppFlowy", "0.15.0") => {
            &[("AppFlowy-0.15.0-macos-arm64.dmg", 119_452_957, "f431bf8fb995cb9b8fb2dd7701ace55707793ef8ae9a8024e820802ad5fe8ef0")]
        }
        ("AppFlowy-IO/AppFlowy", "0.15.1") => {
            &[("AppFlowy-0.15.1-macos-arm64.dmg", 119_286_434, "132641eb6c28c0399137b058e93cdc748f4416a7b1e012f6a3073850a8786070")]
        }
        ("actualbudget/actual", "v26.10.0") => {
            &[("Actual-mac-arm64.dmg", 143_479_378, "db3df92ab6febc0c4c682df7dd2c8b74dd4f47cc7035ca154d98ff883b318cfe")]
        }
        ("actualbudget/actual", "v26.11.0") => {
            &[("Actual-mac-arm64.dmg", 145_818_730, "734a96923e461f5c2741c1a531b353ca249bb2f2ef9f7cb6e9d2b23ddeaf3ba8")]
        }
        ("super-productivity/super-productivity", "v20.0.0") => {
            &[(SUPER_PRODUCTIVITY_MAC_DMG, 148_678_260, "63cd4b4dcb18420b21c61e0f68d1d792653511bccb1de26b5043844ba20be1c0")]
        }
        ("super-productivity/super-productivity", "v20.1.0") => {
            &[(SUPER_PRODUCTIVITY_MAC_DMG, 148_973_846, "fdfb1625603537f0935209b609ba18fba587057ab01139dfe7098968c9a9748c")]
        }
        // These older release assets do not carry an API digest: https://download.sqlitebrowser.org/SHA256SUMS.txt
        ("sqlitebrowser/sqlitebrowser", "v3.12.2") => {
            &[(DB_BROWSER_SQLITE_ARM64_3_12_2, 26_445_763, "0c2076e4479cb9db5c85123cfe9750641f92566694ff9f6c99906321a2c424e8")]
        }
        ("sqlitebrowser/sqlitebrowser", "v3.13.1") => {
            &[("DB.Browser.for.SQLite-v3.13.1.dmg", 45_618_256, "a641cfbfcc2ce609f07de44a35134dab53485ecc18e6d9afa297b514d74bd75e")]
        }
        ("storytold/photocraft", "v0.23.0") => &[
            ("photocraft-0.23.0-macos-universal.dmg", 274_134_041, "8ca761f3e9af4d932cc6efdd6ecbfa22128153ba30cce955547ca6256e61516d"),
            ("photocraft-0.23.0-windows-x64.msi", 118_390_784, "eac24e0e5a675c3cb50d25752138d63c405f0fb9482163968f0553d04653c789"),
        ],
        ("storytold/photocraft", "v0.23.1") => &[
            ("photocraft-0.23.1-macos-universal.dmg", 274_188_048, "606a52917188c65dc71709781180a0e3244e0f3b7f40b2f5b80170e87364420d"),
            ("photocraft-0.23.1-windows-x64.msi", 118_427_648, "de757802d8a12a41718ba24248c281d83f3bbecd64c5230f46f10c7f62070671"),
        ],
        ("storytold/wordcraft", "v0.18.0") => {
            &[("wordcraft-0.18.0-windows-x64.msi", 95_031_296, "8a0bed4ddd40a96f6d7439a3bc7d9bebefe2507066b96ced67f4ef954174012c")]
        }
        ("storytold/wordcraft", "v0.19.1") => {
            &[("wordcraft-0.19.1-windows-x64.msi", 103_383_040, "526abab4619d19ee432a834673044dc19f6ee2eac8f0f0bec6e9f6bf06cb7661")]
        }
        ("openclaw/openclaw", "v2026.11.9") => {
            &[("OpenClaw-2026.11.9-arm64.dmg", 256_294_306, "b8583ed29e820ff9482839a73744d433c0530ba54019440c7faa6ff733f4f56f")]
        }
        ("openclaw/openclaw", "v2026.11.10") => {
            &[("OpenClaw-2026.11.10-arm64.dmg", 248_813_181, "e4d6ceaae36a9198a1410f985682541dfa719229ec30c9ed1a791f824db12f1a")]
        }
        ("anomalyco/opencode", "v2.4.1") => &[
            ("opencode-desktop-mac-arm64.dmg", 345_340_005, "0d69386bec754af9a4d933148a47bc21beea85c1d414295a5b295f2e34640cea"),
            ("opencode-desktop-linux-x86_64.AppImage", 325_645_884, "0b68c8f5a6d1ad992815c026bb90a56e43d9e3db37c7fba5ea3f6094080b9a33"),
        ],
        ("anomalyco/opencode", "v2.4.2") => &[
            ("opencode-desktop-mac-arm64.dmg", 345_253_101, "e7bd4ab50d5c870e5eec5f0d57effee94ae54e0f76ac86502f20ed41ff3e39a7"),
            ("opencode-desktop-linux-x86_64.AppImage", 325_566_590, "b42c238487bdd08ee551d08053e6a5a765bd7bec65f7ca899fec66464bd0fb2b"),
        ],
        ("ollama/ollama", "v0.40.17") => &[("Ollama.dmg", 231_575_873, "cc5eb8fa68f2f15ea6dbc79a705092a1b1e666102c54ed535c91bbbeccab8c78")],
        ("ollama/ollama", "v0.40.18") => &[("Ollama.dmg", 232_758_599, "78880366c6ecc18891bf32eb6dd7d132938aaf55fa8bc9007503701ad6d7d1ed")],
        ("clash-verge-rev/clash-verge-rev", "v2.5.9") => {
            &[("Clash.Verge_2.5.9_aarch64.dmg", 80_746_554, "3c487df30546bcbee58363edda2df0c22f9900a7c04807fb8c3945a8d77d1436")]
        }
        ("clash-verge-rev/clash-verge-rev", "v2.5.10") => {
            &[("Clash.Verge_2.5.10_aarch64.dmg", 80_769_307, "40d4a7da4543d75f573a5cfcc31543b0b4bf7e02caa122a9ca50654859585839")]
        }
        ("stablyai/orca", "v1.4.279") => &[
            ("orca-macos-arm64.dmg", 305_433_992, "fc22bd68e1da6c93df0848671dc35ed00aeb9716c28c4834cf47437fa7755bbb"),
            ("orca-linux.AppImage", 287_088_086, "72a0cd5f5cb09c69b0d998ca255dfe35aa4bf7f22145f2d218b92393cb70cd5a"),
        ],
        ("stablyai/orca", "v1.4.280") => &[
            ("orca-macos-arm64.dmg", 306_833_918, "3e2dd7fe4c6dc6db227142b16d8f69bdfb5a66ce86746945b12c7c13ef4f9528"),
            ("orca-linux.AppImage", 285_631_432, "7fa4d975bdd151787bb12537908464f87695ecc3f1c26cf553220ee86d3c06c2"),
        ],
        ("debpalash/VoiceStudio", "v0.5.7") => &[
            ("VoiceStudio-0.5.7-mac-arm64.dmg", 280_523_075, "8db7a16f35af1e4eaca25c389d376acc86b3885f30d1982f5fd6497953f888a0"),
            ("VoiceStudio-0.5.7-linux-x64.AppImage", 267_623_923, "39a3e0aae23f2a95cb2aabf4a1ca4a1b9c17931b19571179ace017981e364700"),
        ],
        ("debpalash/VoiceStudio", "v0.5.8") => &[
            ("VoiceStudio-0.5.8-mac-arm64.dmg", 284_723_682, "378db9d7bf71a69a227b0f442d40acfe957a09d1a8ce17566059b235b8ed209e"),
            ("VoiceStudio-0.5.8-linux-x64.AppImage", 289_429_504, "90bf90e0dd057632c76e5b286a2e068f9fb9cb1e1363ebaa215a17cb2e3d430e"),
        ],
        ("pingdotgg/t3code", "v0.1.12") => &[
            ("T3-Code-0.1.12-arm64.dmg", 237_227_036, "4613582f18d0c8ba60164e5d5d779bcebe4bb6cf7824b75fe94ecfe00a21f9bf"),
            ("T3-Code-x86_64.AppImage", 255_860_306, "66b91551571df6d29b55cb590295edf411cff96c95a8feb3f41c3ca595d722e2"),
        ],
        ("pingdotgg/t3code", "v0.1.13") => &[
            ("T3-Code-0.1.13-arm64.dmg", 237_415_130, "dfdc4e87a7e56ac11fb5c7ec3cf608db854b3e9c4f755acc75b13f266a9478fa"),
            ("T3-Code-x86_64.AppImage", 256_003_692, "dac1bbe9e93c1a5fb0942332befa998597f6a086232a87766ef5d8235a44f56d"),
        ],
        ("Mintplex-Labs/anything-llm", "v1.19.0") => {
            &[("AnythingLLMDesktop-Silicon.dmg", 584_275_091, "0498b5d929aa7776a30f453d2a3a16fe172a9d63e149e67d224d2630c88e7dd5")]
        }
        ("Mintplex-Labs/anything-llm", "v1.20.0") => {
            &[("AnythingLLMDesktop-Silicon.dmg", 604_481_437, "eb5d0702d86ba7452660efcb6482141e8da863e397e802498a3884f21404aade")]
        }
        ("usebruno/bruno", "v4.3.0") => {
            &[("bruno_4.3.0_arm64_mac.dmg", 167_922_780, "bb4c4d8de817a6f529880627606137fd16346ddece547c6a0862e84bd1cc69bf")]
        }
        ("usebruno/bruno", "v4.4.0") => {
            &[("bruno_4.4.0_arm64_mac.dmg", 183_254_552, "b9e7a4d8a371578116c67c20ae2892e840324d58be5a0407b2623701dd40f2a3")]
        }
        ("upscayl/upscayl", "v2.11.5") => {
            &[("upscayl-2.11.5-mac.dmg", 351_772_230, "abf7b625641560cfa180dd1b2b0864138559d29f2a3d0f9951e5034496ce8dac")]
        }
        ("upscayl/upscayl", "v2.15.0") => {
            &[("upscayl-2.15.0-mac.dmg", 385_264_901, "0e53c9ee8c1800cb3e2ce0f574e4e1a35a51945e19ff2b93f33928bbd7fd4c5a")]
        }
        ("janhq/jan", "v0.8.8") => &[("Jan_0.8.8_universal.dmg", 115_908_917, "7467b2c06ab1d2e0812b36ca73aebcbe8377c2d89207edaba266a2a0e39f404b")],
        ("wulkano/Kap", "v3.5.5") => &[("Kap-3.5.5-arm64.dmg", 124_649_861, "b9a36a9fb882eee232a89077c9bbd2776517f5f858b714c14ca745b564e60420")],
        ("wulkano/Kap", "v3.6.0") => &[("Kap-3.6.0-arm64.dmg", 124_651_995, "0f4b69d5fd4ec59da7b6e153722314c93dc263db2b81c0d0191e256360473ce3")],
        ("simplex-chat/simplex-chat", "v7.1.3") => {
            &[("simplex-desktop-macos-aarch64.dmg", 281_471_349, "9642c6fc18ba0ea07b83afab92f77a76fb069fd85695313ce3c35496fa0d0644")]
        }
        ("simplex-chat/simplex-chat", "v7.1.4") => {
            &[("simplex-desktop-macos-aarch64.dmg", 281_579_270, "d6aae0906b8adf9c18299cb795d7bd5ef79dd007d40ace37912cfb61d3617f7e")]
        }
        ("mudler/LocalAI", "v5.0.0") => &[("LocalAI.dmg", 120_721_974, "f7df5384af6523406ff303153d4c8058215356547ce7721de724ef4664298c62")],
        ("mudler/LocalAI", "v5.0.1") => &[("LocalAI.dmg", 120_740_780, "bb6099e29a45fe3d9dd6c74403f8020d9fb7cd03cc1d54145d4d82e6985f8c85")],
        ("HKUDS/DeepTutor", "v1.7.3") => {
            &[("Open-DeepTutor-MacOS-Apple-Silicon-v1.7.3.dmg", 504_015_269, "cc48aa2dd80a78856233cfbf23d8934f2aebea9d1e69012a13923d59c5030699")]
        }
        ("HKUDS/DeepTutor", "v1.7.4") => {
            &[("Open-DeepTutor-MacOS-Apple-Silicon-v1.7.4.dmg", 505_892_868, "afe5bb50e9b29af77dbf1a03ab1e04e4db6112dcacb61d11ef7f822b9321dd79")]
        }
        ("different-ai/openwork", "v0.18.72") => {
            &[("openwork-mac-arm64-0.18.72.dmg", 382_734_040, "13f1b2e4597df2d7938848629bbe9325a5c9ce6bc9399d4144048fa942e15af5")]
        }
        ("different-ai/openwork", "v0.18.73") => {
            &[("openwork-mac-arm64-0.18.73.dmg", 383_492_503, "6462fbe564667f37eeeabb7a7d5fc49c03d54f862364efc7f1dffa6473c5ec7b")]
        }
        ("FreeCAD/FreeCAD", "1.1.3") => {
            &[("FreeCAD_1.1.3-macOS-arm64-py311.dmg", 649_856_006, "f5c0ece7cd7c932466d6effadc0fc6e179b0538a9d9a6a77a6769eae3af2667c")]
        }
        ("FreeCAD/FreeCAD", "1.1.4") => {
            &[("FreeCAD_1.1.4-macOS-arm64-py311.dmg", 649_760_538, "071343b4abb70492b75c973f41eaf1d2528f9b9c7ea018d22a4f46ae14d27ac0")]
        }
        ("milind-soni/OpenMausBot", "v0.1.182") => {
            &[("OpenMausBot-0.1.182-arm64.dmg", 329_009_773, "6bd04ed658c9e56d840106947ba5dea4af60f538ec60a9661f236c52ffce46cd")]
        }
        ("milind-soni/OpenMausBot", "v0.1.184") => {
            &[("OpenMausBot-0.1.184-arm64.dmg", 329_203_649, "6382b1cb17860aae80fd71a2da1643ecff13266dad406d1aecbf61e950271a44")]
        }
        ("huxingyi/dust3d", "1.1.5") => &[("dust3d-1.1.5.AppImage", 36_648_128, "8575ac93f7403717cf92893471b8ea8525a1321c57cf0f5edaf134e6ff3e5844")],
        ("huxingyi/dust3d", "1.1.6") => &[("dust3d-1.1.6.AppImage", 36_680_896, "efbc1fafe9aa6cbc2679494e45a43bae6df50c7feec58e4a850e94189904fc5a")],
        ("RawTherapee/RawTherapee", "5.12") => {
            &[("RawTherapee_5.12_release.AppImage", 129_234_112, "95d0db7b9c6af85209af66b10b31ac47270e39d937763ac919f826586ea49f08")]
        }
        ("RawTherapee/RawTherapee", "5.13") => {
            &[("RawTherapee_5.13_x86_64_release.AppImage", 130_180_288, "746a3509a234804c3a5f66649006ac29ac67f193a8559d2297357949a890b097")]
        }
        ("anefzaoui/misterdownloadmanager", "v0.5.0") => {
            &[("MisterDownloadManager-0.5.0-linux-x86_64.AppImage", 20_625_912, "d2c7c568240a938f1010610b49b5fe591a7f37db7070a593735e11ba125a3878")]
        }
        ("anefzaoui/misterdownloadmanager", "v0.5.2") => {
            &[("MisterDownloadManager-0.5.2-linux-x86_64.AppImage", 20_777_464, "70bcad9677b754dfffd118cea1e9f0bcb4b7f28f8f70bd3128fbc1b3ad6a74f4")]
        }
        ("OpenTTD/OpenTTD", "15.2") => {
            &[("openttd-15.2-macos-universal.dmg", 14_464_742, "fbc8048714aae1199a9e444a469a0a75f7b5dcb5c43aaedfc91df12859cabe2a")]
        }
        ("OpenTTD/OpenTTD", "15.3") => {
            &[("openttd-15.3-macos-universal.dmg", 14_459_846, "7ca8c0dddd7dc4f26d66a5170af4bf094ed36440cf5641db564356c807308614")]
        }
        ("rxhanson/Rectangle", "v2.0.4") => &[("Rectangle2.0.4.dmg", 4_960_820, "9f820a2e56bd2c27cdda4fb4556de3cbcfcbac052d9eeb2179d49e2dae277abc")],
        ("rxhanson/Rectangle", "v2.0.5") => &[("Rectangle2.0.5.dmg", 4_940_143, "4dec773a2f550d64147d88e07d987ce8119bdff045f1c6993a9a5eeb9f68496c")],
        ("CodeEditApp/CodeEdit", "v0.3.5") => &[("CodeEdit.dmg", 35_095_199, "bd143db6a4d3e85b6e9cd0126dbfa8e3186f57904e893d144acc3cf2b3c065f2")],
        ("CodeEditApp/CodeEdit", "v0.3.6") => &[("CodeEdit.dmg", 35_962_501, "fa5478f80d591c15f08f5e7a93662b5baf33012718768bf0ae800b44ac4eeac9")],
        ("p0deje/Maccy", "2.7.1") => &[("Maccy.app.zip", 4_709_233, "f388aee34de09a0c7531631303785d9938bef9a92130e21ce1049c8f56aad077")],
        ("p0deje/Maccy", "2.8.0") => &[("Maccy.app.zip", 5_288_775, "e684e1ce0c60d7cffd3a2b182dc37bd7e88c29967a64a431a2c71be65b1d27e0")],
        ("lwouis/alt-tab-macos", "v11.10.1") => {
            &[("AltTab-11.10.1.zip", 4_831_056, "aa353197537ccfd378dd4a7ff43030561ce7c62267dd0b31b385691b0e7f594c")]
        }
        ("lwouis/alt-tab-macos", "v11.10.2") => {
            &[("AltTab-11.10.2.zip", 4_885_500, "5a2a821a83be1092df0f63d2403359ef2f7d52ad39cec36d930e4c7554cc045b")]
        }
        ("ianyh/Amethyst", "v0.25.0") => &[("Amethyst.zip", 7_480_074, "cbdcab38c14e5b1cb359537faf0f274b7d77aeb2d3cfef1982866e2084880437")],
        ("ianyh/Amethyst", "v0.25.1") => &[("Amethyst.zip", 7_597_173, "ab1a290a40bb4d31bf589f74246683660789a2c988110aadc6d7e2c24a7f7835")],
        ("linearmouse/linearmouse", "v0.12.0") => {
            &[("LinearMouse.dmg", 5_001_442, "aff50f3c818ced5a4e22a6174c872e3223797d43263fe647d7f955f9be0fcf3c")]
        }
        ("linearmouse/linearmouse", "v0.13.0") => {
            &[("LinearMouse.dmg", 6_184_959, "609276d5db79874034c9469a4eb56ccde6ed6fd01c52404f41fd4b493042789b")]
        }
        ("mrkai77/Loop", "1.4.1") => &[("Loop.zip", 33_816_818, "ed617a58b1c204257fd76eff1fd405db8305a9c2159396a052613a86544ef6e8")],
        ("mrkai77/Loop", "1.4.2") => &[("Loop.zip", 24_046_630, "514e97faab3843cdfb8b1859b91333704639a1a2ce580e4f684d75efe007d386")],
        ("dyad-sh/dyad", "v1.23.0") => &[
            ("dyad-darwin-arm64-1.23.0.zip", 130_606_146, "50667b4acc859d936222703bddfc51b4729a311ea281ab95f274b216b6a2fe15"),
            ("dyad_1.23.0_x86_64.AppImage", 144_538_104, "9dbea7aa51943a9ad6e9073053b260eb98b107bf02f8ad561e0ed386fe908f0a"),
        ],
        ("dyad-sh/dyad", "v1.24.0") => &[
            ("dyad-darwin-arm64-1.24.0.zip", 130_496_464, "edd4dd227fd00d33350f5a25b8c1ca7ff7d67c2b44f637d03e8eef1e8785dbcb"),
            ("dyad_1.24.0_x86_64.AppImage", 144_435_704, "541e17691fe0bd342ac4ecf099b1d10fd0c7c9b7ebf0f26fc951882401e4c2f2"),
        ],
        ("GyulyVGC/sniffnet", "v1.5.1") => {
            &[("Sniffnet_LinuxAppImage_amd64.AppImage", 21_002_744, "04f209d5e45157c57a6fdf34acf82359df88be36046af247fa55693beda7722f")]
        }
        ("GyulyVGC/sniffnet", "v1.6.0") => {
            &[("Sniffnet_LinuxAppImage_amd64.AppImage", 18_721_272, "0d03ede587bd60f8d1aa84b007476aa8c88e63dd65c9e437fd70ab6ebc494d65")]
        }
        ("johnste/finicky", "v4.2.1") => &[("Finicky.dmg", 28_569_559, "4b6008735a2885d0ae6135ba0eaefa3e0fc4e8d84d0b6166bd79cc732e6091cc")],
        ("johnste/finicky", "v4.2.2") => &[("Finicky.dmg", 28_569_676, "5f0020782c9075d27411d6083d7be8663891e3146c86eafad118c44acc12208f")],
        ("Hammerspoon/hammerspoon", "1.1.0") => {
            &[("Hammerspoon-1.1.0.zip", 9_879_298, "39ef907b7984f6cd38778d5bee3772abac8be6b48aa46a1ff5d7adccd41e73b5")]
        }
        ("Hammerspoon/hammerspoon", "1.1.1") => {
            &[("Hammerspoon-1.1.1.zip", 9_704_557, "11bb1c90faf5427f37c7bd4fe7eab9774ae43e1d5cb020c5b3088dac32849efa")]
        }
        ("jordanbaird/Ice", "0.11.11") => &[("Ice.zip", 3_478_127, "21ea5ffa614def29bb699bb7c6725526840d0d999216fb75e504c6757b6f49cb")],
        ("jordanbaird/Ice", "0.11.12") => &[("Ice.zip", 3_497_507, "d770e81597566dd2d2363feb350f808c7a92e363df95c51e48140eb30e452cc9")],
        ("insidegui/VirtualBuddy", "2.2") => {
            &[("VirtualBuddy_v2.2-428.dmg", 22_042_242, "7e672ebca6aec1e7de3adecb6cbbdf978f68dabe5406e48f22187f5dedc98339")]
        }
        ("insidegui/VirtualBuddy", "2.2.1") => {
            &[("VirtualBuddy_v2.2.1-430.dmg", 22_041_237, "16fac6dddbb012548911c0a6807a882e8df5b5b4897a218923b108aaa333fc63")]
        }
        ("blender/blender", "5.2.5") => {
            &[("blender-5.2.5-macos-arm64.dmg", 346_321_744, "3703b832f5cb04c404a96bcb3da192b20e26a78d619abc46a6ac4578e76fbdf4")]
        }
        ("blender/blender", "5.3.0") => {
            &[("blender-5.3.0-macos-arm64.dmg", 359_893_873, "2d802b54766741e612cead50b03ebbc8eb64dadc71eda932573d256b8a784720")]
        }
        ("GNOME/gimp", "3.2.6") => &[("gimp-3.2.6-arm64.dmg", 185_680_887, "854573aec2be6a185aa021109c3c0ca94f210379204b704231bdd191b4060294")],
        ("GNOME/gimp", "3.2.8") => &[("gimp-3.2.8-arm64.dmg", 185_827_254, "06404a375106b7613ba3a14c7d8d9a7cbe18b9b9e2b2050bb72f3162cbcfc395")],
        ("videolan/vlc", "3.0.23") => &[("vlc-3.0.23-arm64.dmg", 51_273_389, "fc6fac08d87f538517d44aca0c5e7a244b67c8c4cb589bf478363a7315fd5e0d")],
        ("videolan/vlc", "3.0.24") => &[("vlc-3.0.24-arm64.dmg", 56_960_605, "64a89d93cdd30b0e97131743e246373db82d6826ea882d265d46d74b136da2b7")],
        ("inkscape/inkscape", "1.4.3") => {
            &[("Inkscape-1.4.3_arm64.dmg", 156_533_257, "f4aa9c9b2e06db432c6f81a494ad9aa59b87cb7d9539cd7e6f6000002cb9edff")]
        }
        ("inkscape/inkscape", "1.4.4") => {
            &[("Inkscape-1.4.4_arm64.dmg", 156_920_591, "eacca94ab01e59467cdd5452a2867c3734450f9a4cfb3346956cc9b07f34f3f1")]
        }
        ("KDE/kdenlive", "26.08.2") => {
            &[("kdenlive-26.08.2-arm64.dmg", 144_286_250, "ffcbff6d6e6651fe7a7cd2b00cb7304c8ef049da8287c389dbda146d5e8928ac")]
        }
        ("KDE/kdenlive", "26.08.3") => {
            &[("kdenlive-26.08.3-arm64.dmg", 141_459_245, "22b385f1e2e5bf45c5bb6954bd58a97db34cead219a9a9ae6faa499f0096d71e")]
        }
        ("KDE/krita", "5.3.4.1") => &[("krita-5.3.4.1-signed.dmg", 392_023_759, "531e261da1fd8d5c468cfcd4eebd16e9edf6ab320e89406d2875f2e43c1148de")],
        ("KDE/krita", "5.3.5") => &[("krita-5.3.5-signed.dmg", 392_026_525, "273f61d6b6a0bfacb21084336cc6522b66c1d74b9f92a59e641b69cd66988175")],
        ("Ranchero-Software/NetNewsWire", "mac-7.3") => {
            &[("NetNewsWire7.3.zip", 10_478_881, "9031210d1b7772f6171c6acfbe87f46e522804f48e9c3b9a598e77b2de508bc1")]
        }
        ("Ranchero-Software/NetNewsWire", "mac-7.3.1") => {
            &[("NetNewsWire7.3.1.zip", 10_473_901, "280ffd15e2df86a39146410d362f2bafca2c559f33cab78cd538669e96245ab9")]
        }
        ("SunshineStream/Sunshine", "v2026.906.222525") => {
            &[("Sunshine-macOS-arm64.dmg", 40_287_527, "b630d35a184d8eaff39c5104f3c6a0c40e91ddc447ccf7a0c5b24706465fab6a")]
        }
        ("SunshineStream/Sunshine", "v2026.914.233613") => {
            &[("Sunshine-macOS-arm64.dmg", 40_375_035, "795f03a389c5726f9a7b15f501dc1ddf204b442cae565859ac83aae027a9e07c")]
        }
        ("ThinkInAIXYZ/deepchat", "v1.1.3") => {
            &[("DeepChat-1.1.3-mac-arm64.dmg", 425_447_652, "1a65b868af90992b5af0268115253c25bbee0dc689607a229db9648db686056f")]
        }
        ("ThinkInAIXYZ/deepchat", "v1.1.4") => {
            &[("DeepChat-1.1.4-mac-arm64.dmg", 421_016_003, "e63b0d18e7cf0a26e9d80084cc2e3b50df1aa87d82cff3494e28e001ad766055")]
        }
        ("SubtitleEdit/subtitleedit", "v5.3.0") => {
            &[("SubtitleEdit-macOS-ARM64.dmg", 135_167_783, "8cf1535b1984f10557076324245f716d6abd858f71c59ef2ca2f474079fc404b")]
        }
        ("SubtitleEdit/subtitleedit", "v5.4.0") => {
            &[("SubtitleEdit-macOS-ARM64.dmg", 138_297_144, "23ff6eb5afd79c4448cced84979b3c428bbab6f2de0de9c01a5f3370f91042e6")]
        }
        ("moonlight-stream/moonlight-qt", "v6.1.0") => {
            &[("Moonlight-6.1.0.dmg", 59_806_624, "d494740eead8ad4e620cdc8feedb56083bc29cabbbeef34cb82585fd87725fa2")]
        }
        ("moonlight-stream/moonlight-qt", "v6.2.0") => {
            &[("Moonlight-6.2.0.dmg", 78_299_984, "43617e5210d53968fcfb4a46526d1625a9056c6fe97e731673c31193dae776fc")]
        }
        ("open-pencil/open-pencil", "v0.16.0") => &[
            ("OpenPencil_0.16.0_amd64.AppImage", 96_438_776, "a7d893ac35367c8ea000b00890136ce5861a13bf2574b44760f77849cfdf77a2"),
            ("OpenPencil_0.16.0_aarch64.dmg", 16_213_804, "000976fa54233d324f38842203d87976331733f5de68480c66d7fbcf0a877d09"),
        ],
        ("open-pencil/open-pencil", "v0.17.0") => &[
            ("OpenPencil_0.17.0_amd64.AppImage", 99_834_360, "43da3dbd7a223a103692dd2d9193a86f8ddcb3db7577bd373949aae8455e94fc"),
            ("OpenPencil_0.17.0_aarch64.dmg", 19_490_106, "72c1332fe713ed97b52cc5f45811f55dc350ec4bd02dbdd8e8db4d54196018ba"),
        ],
        ("OpenWhispr/openwhispr", "v1.12.1") => {
            &[("OpenWhispr-1.12.1-arm64.dmg", 362_841_873, "e2484ac2823707732d011fb0a5eee1c592bb1d455b744283676846024c4d2c13")]
        }
        ("OpenWhispr/openwhispr", "v1.12.2") => {
            &[("OpenWhispr-1.12.2-arm64.dmg", 362_952_454, "324f3b071dc21325701a6b25f0672b29c5829f191e850d2eb09516d8bb1179ea")]
        }
        ("Ultimaker/Cura", "5.13.0") => {
            &[("UltiMaker-Cura-5.13.0-linux-X64.AppImage", 334_154_232, "100f068127b2598167f00ba4db0e0699ded45adf97bbab2d22ff171a1e1ecc40")]
        }
        ("Ultimaker/Cura", "5.14.0") => {
            &[("UltiMaker-Cura-5.14.0-linux-X64.AppImage", 370_960_888, "988a2a67cce7ee7b9b9a843aad65ec4754d2ceb0a72e70eeaf884b995b75990f")]
        }
        ("getpaseo/paseo", "v0.17.6") => {
            &[("Paseo-0.17.6-arm64.dmg", 190_151_170, "3807556e33e6680a982138c534033b1e2101a6fc5a1c84ceb53d3614be6dfce5")]
        }
        ("getpaseo/paseo", "v0.18.0") => &[
            ("Paseo-0.18.0-arm64.dmg", 190_247_198, "2fa0fe84825e3e4df60d6d8625adcd23995868f85a378206fe0e4cf8a69c1f91"),
            ("Paseo-x86_64.AppImage", 171_962_101, "3498024f3a3a7c55c7421a5dd6d85824745e638ddfb612c1909ec1058b03c24e"),
        ],
        ("getpaseo/paseo", "v0.18.1") => {
            &[("Paseo-x86_64.AppImage", 171_958_150, "a17d26d6930da647c0eacad3ebcae779bfc4bb47a824236ff790a60c199d85b6")]
        }
        ("siyuan-note/siyuan", "v3.8.11") => {
            &[("siyuan-3.8.11-mac-arm64.dmg", 285_951_329, "30c3be91cac9e2b4644766f20b788725922097f18524f861cff065311b9ba4c0")]
        }
        ("siyuan-note/siyuan", "v3.8.12") => {
            &[("siyuan-3.8.12-mac-arm64.dmg", 288_887_820, "ad8f00c1df985d3079c6e8425c34b6eb3fff7a41f5c36fc900bc429586ea90af")]
        }
        ("yetone/magpie", "v0.1.4969") => {
            &[("magpie-darwin-arm64.dmg", 49_832_017, "cb894a95cc7ad5ff4e2af32b8af1964c1c3e1e3ea83a3ba5ba8fb6fae52856a0")]
        }
        ("yetone/magpie", "v0.1.4971") => {
            &[("magpie-darwin-arm64.dmg", 49_832_333, "bdab8e88f4e3c6b288b1144483b1fb3f633ff3f6cf39b6e8e028061fe9573c8c")]
        }
        ("TriliumNext/Trilium", "v0.106.1") => {
            &[("TriliumNotes-v0.106.1-macos-arm64.dmg", 197_580_559, "8d25e356d690152a2288d0317712cb198e270b1dca5b66df70496593f951f9a3")]
        }
        ("TriliumNext/Trilium", "v0.107.0") => {
            &[("TriliumNotes-v0.107.0-macos-arm64.dmg", 196_222_346, "e3bda6547213396bd497bd680f33b0d0b12ed9dff410a10b3998cdaf72d9a231")]
        }
        ("kando-menu/kando", "v3.0.0") => {
            &[("Kando-3.0.0-arm64.dmg", 154_824_121, "4b6542e0e6191b742603f3577c599f137b127520b465289c522b23d1d31f71c0")]
        }
        ("kando-menu/kando", "v3.0.1") => {
            &[("Kando-3.0.1-arm64.dmg", 154_825_532, "da33fde36a63766e1d646c5ebfaaf62865dcb764b835c0c38873d18c45d642c2")]
        }
        ("webadderallorg/Recordly", "v1.3.3") => &[
            ("Recordly-arm64.dmg", 219_898_815, "7fa8f4116e870d40fd78bb36d2ad20af364c945023b7b5ec3e72b568b6bbdee5"),
            ("Recordly-linux-x64.AppImage", 237_379_502, "45828b49bfddf6e2f746cd4a7d1f2e118195f1a45af7c0d922c2ac6bbf598f7a"),
        ],
        ("webadderallorg/Recordly", "v1.4.0") => &[
            ("Recordly-arm64.dmg", 187_987_169, "520ee1d5570f2342e126306eda84b95e442c844b6f0e84c6e81c208445be8cc4"),
            ("Recordly-linux-x64.AppImage", 204_317_716, "37b16c416ee9970e0117a9839abe0b7e024f3f02db767f1a6122528b84704a41"),
        ],
        ("VSCodium/vscodium", "1.143.17183") => {
            &[("VSCodium.arm64.1.143.17183.dmg", 206_695_167, "b6cb6359484dfdbc1989528ab4e7429c68bf555f3853173801e9293ce282e902")]
        }
        ("VSCodium/vscodium", "1.143.17232") => {
            &[("VSCodium.arm64.1.143.17232.dmg", 206_610_884, "da2d91385ae73594b13208bc9df20cf4d905a94f84796a33bb3e09162481e475")]
        }
        ("kovidgoyal/calibre", "v9.18.0") => {
            &[("calibre-9.18.0.dmg", 406_181_327, "b4b8000433b6e4d0fdfb3bd0283cd76b7525c65daf5af027ec4dc1e629ef3d0c")]
        }
        ("kovidgoyal/calibre", "v9.19.0") => {
            &[("calibre-9.19.0.dmg", 408_549_188, "207c80d80ace59cb21a4c52fa1c40843a8e860de100304e590da664db3abfb3f")]
        }
        ("ankitects/anki", "26.09.3") => {
            &[("anki-26.09.3-mac-apple.dmg", 226_530_103, "b36fe8f6c015a602feaf3ef38f5967c27c8964ee30a5c74ca2532225cf602345")]
        }
        ("ankitects/anki", "26.09.4") => {
            &[("anki-26.09.4-mac-apple.dmg", 228_182_839, "527b968b88d54cb0308d1de7a1ee193109863546e1cb62dcd016bd93bd816684")]
        }
        ("openchamber/openchamber", "v2.8.0") => &[
            ("OpenChamber-2.8.0-mac-arm64.dmg", 172_726_811, "336ca3514797568cd54a37343777e2d2a7ffdfb4abd18569afcd36fb45fc3ae9"),
            ("OpenChamber-2.8.0-linux-x86_64.AppImage", 180_823_904, "215ded1fdf87d610889b27e4f076bc6878f607346bda58cee5e29f548ff5a55c"),
        ],
        ("openchamber/openchamber", "v2.8.1") => &[
            ("OpenChamber-2.8.1-mac-arm64.dmg", 173_393_080, "9cfa5aebfb8a5e643e7c4856f9685b784cd430d2d20e376aaccb79b6130a3217"),
            ("OpenChamber-2.8.1-linux-x86_64.AppImage", 181_496_363, "4d6de636a9611e673ce6eea6cc9cf5e2dce2f2e20f0304e3eb46de8b4f62955e"),
        ],
        ("osaurus-ai/osaurus", "1.0.12") => {
            &[("Osaurus-1.0.12.dmg", 126_257_063, "181b5f527f30bcde1ffa73f08d6cda4d720c3e26284d3197ca2d7449af10dd4d")]
        }
        ("osaurus-ai/osaurus", "1.0.13") => {
            &[("Osaurus-1.0.13.dmg", 126_326_179, "fdb747c6cdbfdbfea97f449bfeedd39de14c00234e64238088f109a2de3923a0")]
        }
        ("kubernetes-sigs/headlamp", "v0.45.0") => {
            &[("Headlamp-0.45.0-mac-arm64.dmg", 143_705_681, "4f3e7de1c048f8dbe84bfd6c18ac61d0aafd4d909278ca44648d4562c80995f3")]
        }
        ("kubernetes-sigs/headlamp", "v0.46.0") => {
            &[("Headlamp-0.46.0-mac-arm64.dmg", 145_502_945, "20294a2f1e92aa98c2a90aa0717b6b8e730ed55a0300c03db18c2f779ed7b564")]
        }
        ("zen-browser/desktop", "1.24.2b") => {
            &[("zen.macos-universal.dmg", 240_772_424, "04ee33c2e906b52f90f73384aa50e80a92530ae4b3414fb26a0a982e257f172c")]
        }
        ("zen-browser/desktop", "1.24.3b") => {
            &[("zen.macos-universal.dmg", 240_816_809, "86f6a01e786095d7661c68357396a00cd3e169d6ae5e176245077bb701af8b65")]
        }
        ("cyanfish/naps2", "v8.6.0") => {
            &[("naps2-8.6.0-mac-arm64.pkg", 27_484_564, "57aeb20a9d1c0601aa5ec50f5171ac2e3ea6d88b9fafc9e13af217a8912503bc")]
        }
        ("cyanfish/naps2", "v8.6.1") => {
            &[("naps2-8.6.1-mac-arm64.pkg", 27_550_717, "94c53d010c674dd3bfbfecfcb8af8a2cf6b6206196954dcd5181d47446f0e601")]
        }
        ("solvespace/solvespace", "v3.2") => &[(SOLVESPACE_MAC_DMG, 13_333_565, "714ca23cc6989c1fd842b353412ef9ba24a23eb2f17446c1bc91740d79869b51")],
        ("solvespace/solvespace", "v3.3") => &[(SOLVESPACE_MAC_DMG, 13_136_637, "9a83620fb2a34dd26961e35a8054a2ee68f36e3dfeaaefb443d3308c9cfc1f11")],
        ("lapce/lapce", "v0.4.5") => &[(LAPCE_MAC_DMG, 46_084_234, "354e7e5b89e9f9eb1c08d32fa08906b9230b825c0dcc70624585c4e026fd13b8")],
        ("lapce/lapce", "v0.4.6") => &[(LAPCE_MAC_DMG, 45_483_653, "d4ba42218148446c265c6a823107a969b3b0dca5e14df834563f2859927228f5")],
        ("keycastr/keycastr", "v0.11.0") => &[(KEYCASTR_MAC_ZIP, 2_628_593, "00cc656a786012bcbdb5b203b1a56604727ad2ced5e4f5c1d189296f62881033")],
        ("keycastr/keycastr", "v0.11.1") => &[(KEYCASTR_MAC_ZIP, 2_714_588, "71fb7dd90a9bb96de2990499da280ce03e13cbd559a77477515191864977a65c")],
        ("metabrainz/picard", "release-3.0.4") => {
            &[("MusicBrainz-Picard-3.0.4-macOS-13.0-arm64.dmg", 59_831_923, "dc4bd3ab3da5df45baa7e82e0a6b49930f6f8dd72e76b74fde0d533f41850310")]
        }
        ("metabrainz/picard", "release-3.0.5") => {
            &[("MusicBrainz-Picard-3.0.5-macOS-13.0-arm64.dmg", 59_600_174, "53b1f8c3a971c538421ddc89a14a7322bd5bc90521ade1f5aded51d10a64ddfe")]
        }
        ("neovide/neovide", "0.17.3") => &[(NEOVIDE_MAC_DMG, 12_337_549, "024f11f8e5a69b58cd9f9c01cf5d6bbe5c4813474f0c5cc1150cddb70e7cd2cc")],
        ("neovide/neovide", "0.17.4") => &[(NEOVIDE_MAC_DMG, 12_338_261, "5a4647cce0703552906552ae1d0d955e221f3153af832fa6739ffc0592774e2a")],
        ("Termix-SSH/Termix", "v26.11.2") => &[(TERMIX_MAC_DMG, 254_640_116, "c9b8eda86bf88f85ab2f2aa295e76fe1409805f65b3ab3c69a0cd7186c7cac5e")],
        ("Termix-SSH/Termix", "v26.11.3") => &[(TERMIX_MAC_DMG, 263_900_889, "a509ba2eebd74b7562f38d5adca342c137e431fab1744af34bdbd07eb9a96f3c")],
        (crate::curated::GODOT_RELEASES_REPO, "4.7.1-stable") => {
            &[("Godot_v4.7.1-stable_macos.universal.zip", 166_939_858, "897cb7f9799796c717ae75f31446aed883dc92b1d6c3b33d893cc7843fff2fa9")]
        }
        (crate::curated::GODOT_RELEASES_REPO, "4.7.2-stable") => {
            &[("Godot_v4.7.2-stable_macos.universal.zip", 170_622_178, "c58a24e31d720be9d62f60cb5627c4e695fb72f21b0cfe1bc9ccaa9a3b3ba63e")]
        }
        ("pencil2d/pencil", "v0.7.1") => {
            &[("pencil2d-linux-amd64-v0.7.1.AppImage", 60_471_800, "02416a6777f3b5aca826285b85d66476c1352d0d8252b55714cc7bc55ba463f6")]
        }
        ("pencil2d/pencil", "v0.7.2") => {
            &[("pencil2d-linux-amd64-v0.7.2.AppImage", 60_471_800, "4034b036258b6a13d4540911ebce085f31ee18a9dacf438845ac71998d42ff91")]
        }
        ("OpenShot/openshot-qt", "v4.1.0") => {
            &[("OpenShot-v4.1.0-x86_64.AppImage", 258_708_672, "43af897622628b6ee092d5cfc5b555d50d6253dd32c5562060c9759b041d7e4c")]
        }
        ("OpenShot/openshot-qt", "v4.1.1") => {
            &[("OpenShot-v4.1.1-x86_64.AppImage", 258_921_664, "662d0c3cec67ddc90df6a8b0c723c0aac0d3bddf0c3c467eff562e818bb8c72f")]
        }
        ("musescore/MuseScore", "v4.7.6") => &[
            ("MuseScore-Studio-4.7.6.261022093.dmg", 207_171_137, "dd78357e652a4df6ec074b9bd63a79e09eedb7c1c9dd6262eb7d9304508dec9c"),
            ("MuseScore-Studio-4.7.6.261022093-x86_64.AppImage", 192_616_952, "5fcbd1f292270eca5c3867d56b767c31a173a155216a415f5caee22553f8e7a6"),
        ],
        ("musescore/MuseScore", "v4.7.7") => &[
            ("MuseScore-Studio-4.7.7.261109195.dmg", 206_969_738, "16a6b33bb12f4a54cd97dfe8e7bc2f21477b9897e4c503e26fa3e332e4ddf8ad"),
            ("MuseScore-Studio-4.7.7.261109195-x86_64.AppImage", 192_743_928, "8597105814359676483807ce4c4051eaf439bd723ebccaa9160632f56c05a02d"),
        ],
        ("mifi/lossless-cut", "v3.70.1") => &[
            ("LosslessCut-mac-arm64.dmg", 165_847_881, "82964e13ca65beeee508fbdb27fe823121701748b8b4023800bdcf701183d2ea"),
            ("LosslessCut-linux-x86_64.AppImage", 221_418_564, "04c87c97ff3980b8ebb6a62f0394204232e73a7dff89ea86ef49be3c372b0842"),
        ],
        ("mifi/lossless-cut", "v3.70.2") => &[
            ("LosslessCut-mac-arm64.dmg", 165_792_537, "3c0e2b3ec25fec66cf0cef343450d9a333e4bba5d64456c34cd18e891a3f5947"),
            ("LosslessCut-linux-x86_64.AppImage", 221_357_823, "e9ea00c9cf6cdda10db9d9cd13fc9ca10d61cdc046a60f5ee340db6643164a4a"),
        ],
        ("dbeaver/dbeaver", "26.2.5") => {
            &[("dbeaver-ce-26.2.5-macos-aarch64.dmg", 124_466_382, "2d739a95bbc3bb2f9fada4863a9f11904b94ebb34673999915809f788e849d73")]
        }
        ("dbeaver/dbeaver", "26.2.5.2") => {
            &[("dbeaver-ce-26.2.5.2-macos-aarch64.dmg", 124_461_569, "84aa5edce7f78aa7c9bac8204956968cbdc6e194208a5654134ccbc6e29af60e")]
        }
        ("prusa3d/PrusaSlicer", "version_2.9.6") => {
            &[("PrusaSlicer-2.9.6.dmg", 142_444_072, "94fd7b8a9f87c9631e1c71739b15b184fc5f4c0ceabd69072f1c78f229a4fe40")]
        }
        ("prusa3d/PrusaSlicer", "version_2.9.7") => {
            &[("PrusaSlicer-2.9.7.dmg", 142_446_004, "f735f9d527823d159d9ac1e8f84ce1b74ea571e705e70e723bee546d070db60b")]
        }
        ("OrcaSlicer/OrcaSlicer", "v2.5.1") => &[
            ("OrcaSlicer_Mac_universal_V2.5.1.dmg", 364_654_009, "ddf48a7f34785a4d5b4313e4dc4d7ad01769d48bc4c8d825f809da96593e6f46"),
            ("OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.1.AppImage", 201_116_152, "073b8105c928356141db0ddbce88b81d52c7a9e2442f9baae0a9089c89ff04eb"),
        ],
        ("OrcaSlicer/OrcaSlicer", "v2.5.2") => &[
            ("OrcaSlicer_Mac_universal_V2.5.2.dmg", 359_667_480, "6ea1afcdffb7e9ab011e2d6f6db665cb18463d1c4774591cd5db7ee0225479d9"),
            ("OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.2.AppImage", 201_034_232, "22fa88884dec3fdbd29e90cba9912fcff6fb57a60b1bdc1a6c189c61a6b0a870"),
        ],
        ("Zettlr/Zettlr", "v4.10.0") => {
            &[("Zettlr-4.10.0-x86_64.AppImage", 172_628_308, "bf6f7458c2377818d5d4ed008a721c295bec2c25d570cf8f54d053f8819508fc")]
        }
        ("Zettlr/Zettlr", "v4.11.0") => &[
            ("Zettlr-4.11.0-x86_64.AppImage", 172_657_896, "e9b5406d62e2c9e5fef3adb092877669d849d16a217375c4e1736cf9c7aa9bd7"),
            ("Zettlr-4.11.0-arm64.dmg", 202_782_993, "68e57c3dcb8275b175ac1b67f7d94945c84e0a7c4da6e1ab6b6e8fcd64458793"),
        ],
        ("ONLYOFFICE/DesktopEditors", "v9.4.0") => {
            &[("DesktopEditors-x86_64.AppImage", 625_961_464, "f5faf24552665262fe94486a510b997d2e5a24bde14df94c802c79bf17f9254c")]
        }
        ("ONLYOFFICE/DesktopEditors", "v10.0.0") => {
            &[("DesktopEditors-x86_64.AppImage", 425_810_424, "3e4668d44f6881d1d8873b38ea9c9fd81b9c77132facdb735bfebfe960edb745")]
        }
        ("nextcloud-releases/desktop", "v35.0.2") => {
            &[("Nextcloud-35.0.2-x86_64.AppImage", 63_248_888, "cd62e276ecc8172b3075043abeec789024355bba3d0cd9ad8b3c850e9ad0d7d8")]
        }
        ("nextcloud-releases/desktop", "v35.0.3") => {
            &[("Nextcloud-35.0.3-x86_64.AppImage", 63_248_888, "334e83b145c991ae80ec9db9483334066d2562335e2191362824ccd10aa16e54")]
        }
        ("dail8859/NotepadNext", "v0.14") => {
            &[("NotepadNext-v0.14-x86_64.AppImage", 34_466_296, "a5a60f66cda70852998fd929d8ab6cb99dc77638c7903f82aac9e0e468816bd7")]
        }
        ("dail8859/NotepadNext", "v0.15") => {
            &[("NotepadNext-v0.15-x86_64.AppImage", 36_092_408, "33e092fb52fb4f8bdbe57c63ac14104a5e092ce5c614040a1dd322eebc44aa87")]
        }
        ("CyberTimon/RapidRAW", "v1.7.0") => {
            &[("03_RapidRAW_v1.7.0_ubuntu-22.04_amd64.AppImage", 114_608_632, "1d5c359f76786f3e2a5e2ae466bf99a6cc0e5e35d63159562f79db50cee4981a")]
        }
        ("CyberTimon/RapidRAW", "v1.7.1") => {
            &[("03_RapidRAW_v1.7.1_ubuntu-22.04_amd64.AppImage", 117_078_520, "398ca458f2f713eb60fc4b32ff5f178323217e71ff039f9246a8f4e7d1f63a71")]
        }
        ("rustdesk/rustdesk", "1.5.1") => &[
            ("rustdesk-1.5.1-x86_64.AppImage", 88_472_056, "bb8b7ccb3f50a201dccdc89b46385cff90f33e60b4730faeda9fa5504ac4dce6"),
            ("rustdesk-1.5.1-aarch64.dmg", 30_548_457, "1b94182cba2375ea29a196f2c8183f7feaa00588474ecd76c92c2303ffd70d7b"),
        ],
        ("rustdesk/rustdesk", "1.5.3") => &[
            ("rustdesk-1.5.3-x86_64.AppImage", 88_492_536, "ff7fff208fed218fc9f895457a36cc2dd31fefbfae0461a27b54f3d391204506"),
            ("rustdesk-1.5.3-aarch64.dmg", 31_464_911, "e6d30fa1633d772c48935907c208ca4e555a856fb2038af3aaa87a327c29360b"),
        ],
        ("pavlobu/deskreen", "v3.2.15") => &[
            ("deskreen-ce-3.2.15-arm64.dmg", 148_740_787, "bfeb75823971706af9a2123a4c212a5244c0a0000f094be703a8125dbdeba3c8"),
            ("deskreen-ce-3.2.15-x86_64.AppImage", 155_298_417, "cbabe91564be48733659675a6876ce869f4fa987d96a16de0fa6a2c2b77b2720"),
        ],
        ("pavlobu/deskreen", "v3.2.16") => &[
            ("deskreen-ce-3.2.16-arm64.dmg", 147_505_602, "09a3be94f5fa2afc13e9098b8f6ad1f3285cf41f6f7a0de64a529944398d9177"),
            ("deskreen-ce-3.2.16-x86_64.AppImage", 155_306_590, "25c54a448344587257ce977288c4b3c7e729fc1cc7061ad7471622cd09a47ab2"),
        ],
        ("zotero/zotero", tag) => {
            ZOTERO_DMG_RECEIPTS.iter().find(|&&(reviewed, _)| reviewed == tag).map_or(&[], |(_, receipt)| std::slice::from_ref(receipt))
        }
        _ => &[],
    };
    receipts.iter().filter_map(move |&(name, size, sha256)| {
        let url = if repo == crate::curated::CALIBRE.repo {
            format!("{CALIBRE_DMG_URL}{}/{name}", tag.trim_start_matches('v'))
        } else if repo == crate::curated::OPENTTD.repo {
            format!("{OPENTTD_DMG_URL}{tag}/{name}")
        } else if repo == crate::curated::ZOTERO.repo {
            format!("{ZOTERO_DMG_URL}{tag}/{name}")
        } else if repo == crate::curated::BLENDER.repo {
            let (series, _) = tag.rsplit_once('.')?;
            format!("{BLENDER_DMG_URL}Blender{series}/{name}")
        } else if repo == crate::curated::VLC.repo {
            format!("{VLC_DMG_URL}{tag}/macosx/{name}")
        } else if repo == crate::curated::INKSCAPE.repo {
            format!("{INKSCAPE_DMG_URL}{name}")
        } else if repo == crate::curated::KDENLIVE.repo {
            let series = tag.rsplit_once('.').map_or(tag, |(series, _)| series);
            format!("{KDENLIVE_DMG_URL}{series}/macOS/{name}")
        } else if repo == crate::curated::KRITA.repo {
            format!("{KRITA_DMG_URL}{tag}/{name}")
        } else if repo == crate::curated::GIMP.repo {
            let (series, _) = tag.rsplit_once('.')?;
            format!("{GIMP_DMG_URL}v{series}/macos/{name}")
        } else {
            format!("{}{tag}/{name}", download_prefix(repo))
        };
        Some(Asset { name: name.into(), size, url, sha256: Some(sha256.into()) })
    })
}

/// A release reconstructed from its tag and the published checksum manifest, when available.
pub fn release_from_sums(repo: &str, tag: &str, sums: &[(String, String)]) -> Release {
    let prefix = download_prefix(repo);
    let mut assets: Vec<Asset> = sums
        .iter()
        .map(|(hash, name)| Asset { name: name.clone(), size: 0, url: format!("{prefix}{tag}/{name}"), sha256: Some(hash.clone()) })
        .collect();
    for asset in reviewed_assets(repo, tag) {
        if let Some(existing) = assets.iter_mut().find(|existing| existing.name == asset.name) {
            *existing = asset;
        } else {
            assets.push(asset);
        }
    }
    let manifest = checksum_manifest_name(repo);
    assets.push(Asset { name: manifest.into(), size: 0, url: format!("{prefix}{tag}/{manifest}"), sha256: None });
    Release {
        tag: tag.to_string(),
        version: release_tag_version(repo, tag),
        name: tag.to_string(),
        published_at: None,
        html_url: format!("https://github.com/{repo}/releases/tag/{tag}"),
        notes: String::new(),
        prerelease: false,
        assets,
    }
}

/// The tag in a `https://github.com/<repo>/releases/tag/<tag>` redirect target.
pub fn tag_from_redirect(location: &str, repo: &str) -> Option<String> {
    let rest = location.strip_prefix(&format!("https://github.com/{repo}/releases/tag/"))?;
    valid_tag(rest).then(|| rest.to_string())
}

/// A release tag we'd put in a URL, a file or the UI: up to 64 of `[A-Za-z0-9._+-]`, not
/// starting with a dot, so no separators, control or bidi characters and no `.`/`..`.
pub fn valid_tag(tag: &str) -> bool {
    tag.len() <= 64 && !tag.starts_with('.') && plain_name(tag)
}

/// Non-empty and only `[A-Za-z0-9._+-]`, the characters every Craft release tag and asset uses.
fn plain_name(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"._-+".contains(&b))
}

/// Reads `sha256sum` output: `<hex>  <name>` (or `<hex> *<name>`) per line.
pub fn parse_sha256sums(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let (hash, rest) = line.trim().split_once(char::is_whitespace)?;
            let name = rest.trim_start().trim_start_matches('*');
            (is_hex64(hash) && valid_file_name(name)).then(|| (hash.to_ascii_lowercase(), name.to_string()))
        })
        .collect()
}

/// A bare file name we'd be willing to save: only `[A-Za-z0-9._+-]`, no leading or trailing dot,
/// and not a name Windows reserves for a device (`CON`, `nul.txt`, `COM1.msi`).
pub fn valid_file_name(name: &str) -> bool {
    name.len() <= 200 && !name.starts_with('.') && !name.ends_with('.') && plain_name(name) && !windows_device(name)
}

fn windows_device(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or_default().to_ascii_uppercase();
    matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || matches!(stem.as_bytes(), [b'C', b'O', b'M', b'1'..=b'9'] | [b'L', b'P', b'T', b'1'..=b'9'])
}

/// `s` cut to at most `max` bytes (on a char boundary) with an ellipsis, for text from a
/// release or the feed shown in the UI.
pub(crate) fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &s[..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netnewswire_mac_pointer_rejects_ios_prereleases_and_mismatched_assets() {
        let address = "https://github.com/Ranchero-Software/NetNewsWire/releases/download/mac-7.3.1/NetNewsWire7.3.1.zip";
        let current = netnewswire_mac_release_from_download(address).unwrap();
        assert!(netnewswire_mac_asset(&current, &current.assets[0]));
        assert!(!netnewswire_mac_asset(&current, &Asset { size: 1, ..current.assets[0].clone() }));
        let history = netnewswire_mac_release_history(current);
        assert_eq!(history.iter().map(|release| release.tag.as_str()).collect::<Vec<_>>(), ["mac-7.3.1", "mac-7.3"]);
        assert_eq!(history[1].version_label(), "7.3.0");
        assert!(netnewswire_mac_release_from_download(&address.replace("mac-", "iOS-")).is_none());
        assert!(netnewswire_mac_release_from_download(&address.replace("7.3.1", "7.3.1b1")).is_none());
        assert!(netnewswire_mac_release_from_download(&address.replace("NetNewsWire7.3.1.zip", "NetNewsWire7.3.2.zip")).is_none());
        assert!(netnewswire_mac_release_from_download(&address.replace("github.com", "example.org")).is_none());
    }

    #[test]
    fn zotero_stable_redirect_is_canonical_and_never_qualifies_new_packages() {
        use crate::{Arch, Os, PackageKind, Platform, curated::ZOTERO, platform::pick_asset};

        let location = "https://download.zotero.org/client/release/10.0.9/Zotero-10.0.9.dmg";
        let stable = zotero_release(zotero_stable_version(location).unwrap());
        let history = zotero_release_history(stable.clone());
        assert_eq!(history.iter().map(|release| release.tag.as_str()).collect::<Vec<_>>(), ["10.0.9", "10.0.8"]);
        assert_eq!(stable.html_url, ZOTERO.releases_url());
        for invalid in [
            location.replace("https://", "http://"),
            location.replace("download.zotero.org", "download.zotero.org.example.com"),
            location.replace("10.0.9/", "10.0.9-beta/"),
            location.replace("10.0.9/", "10.0.09/"),
            location.replace("Zotero-10.0.9", "Zotero-10.0.8"),
            format!("{location}?tracking=1"),
            format!("{location}/../"),
        ] {
            assert!(zotero_stable_version(&invalid).is_none(), "{invalid}");
        }

        let arm = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let pick = |release: &Release, platform| pick_asset(release, &[ZOTERO.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.clone());
        assert_eq!(pick(&stable, arm).as_ref().map(|asset| asset.url.as_str()), Some(location));
        assert!(pick(&history[1], arm).is_some());
        assert!(pick(&stable, Platform { arch: Arch::X64, ..arm }).is_none());
        let mut changed = stable.clone();
        changed.assets[0].sha256 = Some("0".repeat(64));
        assert!(pick(&changed, arm).is_none());
        let mut changed = stable.clone();
        changed.assets[0].url = "https://example.org/Zotero-10.0.9.dmg".into();
        assert!(pick(&changed, arm).is_none());
        let mut changed = stable.clone();
        changed.assets[0].size -= 1;
        assert!(pick(&changed, arm).is_none());
        assert!(pick(&zotero_release(version::parse("10.0.10").unwrap()), arm).is_none());
        assert_eq!(zotero_release_history(history[1].clone()).len(), 1);
    }

    #[test]
    fn deeptutor_uses_only_reviewed_apple_silicon_packages() {
        use crate::{Arch, Os, PackageKind, Platform, curated::DEEPTUTOR, platform::pick_asset};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let pick = |release: &Release, platform| pick_asset(release, &[DEEPTUTOR.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.clone());
        for tag in ["v1.7.3", "v1.7.4"] {
            let expected = reviewed_assets(DEEPTUTOR.repo, tag).next().unwrap();
            let api = serde_json::json!({
                "tag_name": tag,
                "assets": [{"name": expected.name, "size": expected.size, "browser_download_url": expected.url}]
            });
            let release = parse_github_release(&api.to_string(), DEEPTUTOR.repo).unwrap();
            assert_eq!(pick(&release, mac), Some(expected.clone()));
            assert_eq!(pick(&release_from_sums(DEEPTUTOR.repo, tag, &[]), mac), Some(expected));
            assert_eq!(pick(&release, Platform { arch: Arch::X64, ..mac }), None);
            assert_eq!(pick(&release, Platform { os: Os::Linux, arch: Arch::X64 }), None);

            let mut wrong_digest = release.clone();
            wrong_digest.assets[0].sha256 = Some("0".repeat(64));
            assert_eq!(pick(&wrong_digest, mac), None);
            let mut wrong_origin = release;
            wrong_origin.assets[0].url = "https://mirror.example/download.dmg".into();
            assert_eq!(pick(&wrong_origin, mac), None);
        }
        let unknown = release_from_sums(DEEPTUTOR.repo, "v1.7.5", &[]);
        assert_eq!(pick(&unknown, mac), None);
    }

    #[test]
    fn openwork_uses_only_reviewed_public_apple_silicon_packages() {
        use crate::{Arch, Os, PackageKind, Platform, curated::OPENWORK_DESKTOP, platform::pick_asset};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for tag in ["v0.18.72", "v0.18.73"] {
            let expected = reviewed_assets(OPENWORK_DESKTOP.repo, tag).next().unwrap();
            let api = serde_json::json!({
                "tag_name": tag,
                "assets": [{"name": expected.name, "size": expected.size, "browser_download_url": expected.url}]
            });
            let mut release = parse_github_release(&api.to_string(), OPENWORK_DESKTOP.repo).unwrap();
            let pick = |release: &Release, platform| {
                pick_asset(release, &[OPENWORK_DESKTOP.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.clone())
            };
            assert_eq!(pick(&release, mac), Some(expected.clone()));
            assert_eq!(pick(&release_from_sums(OPENWORK_DESKTOP.repo, tag, &[]), mac), Some(expected.clone()));
            assert_eq!(pick(&release, Platform { arch: Arch::X64, ..mac }), None);
            assert_eq!(pick(&release, Platform { os: Os::Linux, arch: Arch::X64 }), None);
            for edition in ["cloud", "enterprise"] {
                let mut other_edition = expected.clone();
                other_edition.name = expected.name.replacen("openwork-", &format!("openwork-{edition}-"), 1);
                other_edition.url = expected.url.replace(&expected.name, &other_edition.name);
                release.assets = vec![other_edition];
                assert_eq!(pick(&release, mac), None, "{edition}");
            }
        }
    }

    #[test]
    fn desktop_recipes_use_only_reviewed_apple_silicon_packages() {
        use crate::{
            Arch, Os, PackageKind, Platform,
            curated::{DBEAVER_CE, FREECAD, MUSESCORE, OPENMAUSBOT, ORCA_SLICER, PASEO, PRUSA_SLICER},
            platform::pick_asset,
        };

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for (app, tags, next) in [
            (PASEO, ["v0.17.6", "v0.18.0"], "v0.18.1"),
            (FREECAD, ["1.1.3", "1.1.4"], "1.1.5"),
            (OPENMAUSBOT, ["v0.1.182", "v0.1.184"], "v0.1.185"),
            (MUSESCORE, ["v4.7.6", "v4.7.7"], "v4.7.8"),
            (ORCA_SLICER, ["v2.5.1", "v2.5.2"], "v2.5.3"),
            (DBEAVER_CE, ["26.2.5", "26.2.5.2"], "26.2.6"),
            (PRUSA_SLICER, ["version_2.9.6", "version_2.9.7"], "version_2.9.8"),
        ] {
            let pick = |release: &Release, platform| pick_asset(release, &[app.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.clone());
            for tag in tags {
                let expected = reviewed_assets(app.repo, tag).find(|asset| asset.name.ends_with(".dmg")).unwrap();
                let api = serde_json::json!({
                    "tag_name": tag,
                    "assets": [{"name": expected.name, "size": expected.size, "browser_download_url": expected.url}]
                });
                let mut release = parse_github_release(&api.to_string(), app.repo).unwrap();
                assert_eq!(pick(&release, mac), Some(expected.clone()));
                assert_eq!(pick(&release_from_sums(app.repo, tag, &[]), mac), Some(expected));
                assert_eq!(pick(&release, Platform { arch: Arch::X64, ..mac }), None);
                assert_eq!(pick(&release, Platform { os: Os::Linux, arch: Arch::X64 }), None);
                release.prerelease = true;
                assert_eq!(pick(&release, mac), None);
            }
            assert_eq!(pick(&release_from_sums(app.repo, next, &[]), mac), None);
        }
    }

    #[test]
    fn calibre_archive_has_only_reviewed_official_arm64_dmgs() {
        use crate::{Arch, Os, PackageKind, Platform, curated::CALIBRE, platform::pick_asset};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let pick = |release: &Release, platform| pick_asset(release, &[CALIBRE.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.clone());
        for tag in ["v9.18.0", "v9.19.0"] {
            let name = format!("calibre-{}.dmg", &tag[1..]);
            let api = serde_json::json!({"tag_name": tag, "assets": []});
            let release = parse_github_release(&api.to_string(), CALIBRE.repo).unwrap();
            let asset = pick(&release, mac).unwrap();
            assert_eq!(asset.name, name);
            assert_eq!(asset.url, format!("{CALIBRE_DMG_URL}{}/{name}", &tag[1..]));
            assert_eq!(pick(&release_from_sums(CALIBRE.repo, tag, &[]), mac), Some(asset.clone()));
            assert_eq!(pick(&release, Platform { arch: Arch::X64, ..mac }), None);
            assert_eq!(pick(&release, Platform { os: Os::Linux, arch: Arch::X64 }), None);
            assert!(parse_github_release(&api.to_string(), "mirror/calibre").unwrap().assets.is_empty());

            let mut changed = release.clone();
            changed.assets[0].url = changed.assets[0].url.replace(CALIBRE_DMG_URL, "https://mirror.example/");
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release.clone();
            changed.assets[0].size += 1;
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release.clone();
            changed.assets[0].sha256 = Some("0".repeat(64));
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release.clone();
            changed.tag = "v9.20.0".into();
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release;
            changed.prerelease = true;
            assert_eq!(pick(&changed, mac), None);
        }
        let unknown = parse_github_release(r#"{"tag_name":"v9.20.0","assets":[]}"#, CALIBRE.repo).unwrap();
        assert_eq!(pick(&unknown, mac), None);
    }
    #[test]
    fn anki_calendar_tags_select_only_reviewed_arm64_dmgs() {
        use crate::{Arch, Os, PackageKind, Platform, curated::ANKI, platform::pick_asset};

        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        let pick = |release: &Release, platform| pick_asset(release, &[ANKI.id], platform, &[PackageKind::Dmg]).map(|(asset, _)| asset.name.clone());
        for (tag, patch) in [("26.09.3", 3), ("26.09.4", 4)] {
            let release = release_from_sums(ANKI.repo, tag, &[]);
            let name = format!("anki-{tag}-mac-apple.dmg");
            assert_eq!(release.version, Some(Version::new(26, 9, patch)));
            assert_eq!(pick(&release, mac), Some(name));
            assert_eq!(pick(&release, Platform { arch: Arch::X64, ..mac }), None);
            assert_eq!(pick(&release, Platform { os: Os::Linux, arch: Arch::X64 }), None);

            let mut changed = release.clone();
            changed.assets[0].name = changed.assets[0].name.replace("apple", "intel");
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release.clone();
            changed.assets[0].size += 1;
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release.clone();
            changed.assets[0].sha256 = Some("0".repeat(64));
            assert_eq!(pick(&changed, mac), None);
            let mut changed = release;
            changed.prerelease = true;
            assert_eq!(pick(&changed, mac), None);
            assert!(release_tag_version("other/publisher", tag).is_none());
        }
        assert_eq!(pick(&release_from_sums(ANKI.repo, "26.09.5", &[]), mac), None);
        assert!(anki_date_version("26.9.4").is_none());
        assert!(anki_date_version("26.13.4").is_none());
        assert!(anki_date_version("26.09.4-rc.1").is_none());
    }

    #[test]
    fn opencode_desktop_manifest_requires_current_stable_apple_silicon_dmg() {
        let name = "opencode-desktop-mac-arm64.dmg";
        let expected = crate::release::release_from_sums(crate::curated::OPENCODE.repo, "v2.4.2", &[]);
        let approved = &expected.assets[0];
        let linux_name = "opencode-desktop-linux-x86_64.AppImage";
        let linux = expected.assets.iter().find(|asset| asset.name == linux_name).unwrap();
        let fixture = serde_json::json!({
            "channel": "latest",
            "distribution": "opencode",
            "name": "desktop",
            "active": true,
            "version": "2.4.2",
            "metadata": {"files": {(name): {
                "size": approved.size,
                "sha256": approved.sha256,
                "url": format!("https://opencode.ai/files/bin/2.4.2/{name}"),
            }, (linux_name): {
                "size": linux.size,
                "sha256": linux.sha256,
                "url": format!("https://opencode.ai/files/bin/2.4.2/{linux_name}"),
            }}}
        });
        let parse = |value: &serde_json::Value| opencode_desktop_release(&serde_json::to_vec(value).unwrap());
        let release = parse(&fixture).unwrap();
        assert_eq!(release.tag, "v2.4.2");
        assert_eq!(&release.assets[0], approved);
        assert!(qualified_mac_dmg_asset(&crate::curated::OPENCODE, &release, approved));
        assert_eq!(&release.assets[1], linux);
        assert!(qualified_linux_appimage_asset(&crate::curated::OPENCODE, &release, linux));
        let mut invalid_linux = fixture.clone();
        invalid_linux["metadata"]["files"][linux_name]["url"] = serde_json::json!("https://elsewhere.example/download");
        assert_eq!(parse(&invalid_linux).unwrap().assets, vec![approved.clone()]);
        for (pointer, field) in [
            ("/channel", serde_json::json!("beta")),
            ("/distribution", serde_json::json!("other")),
            ("/name", serde_json::json!("cli")),
            ("/active", serde_json::json!(false)),
            ("/version", serde_json::json!("1.19.6")),
            ("/version", serde_json::json!("2.4.2-rc.1")),
            ("/version", serde_json::json!("2.4.2+1")),
            ("/metadata/files/opencode-desktop-mac-arm64.dmg/url", serde_json::json!("https://elsewhere.example/download")),
            ("/metadata/files/opencode-desktop-mac-arm64.dmg/sha256", serde_json::json!("invalid")),
            ("/metadata/files/opencode-desktop-mac-arm64.dmg/size", serde_json::json!(0)),
        ] {
            let mut changed = fixture.clone();
            *changed.pointer_mut(pointer).unwrap() = field;
            assert!(parse(&changed).is_none(), "{pointer}");
        }
    }

    const REPO: &str = "storytold/photocraft";

    #[test]
    fn generic_prerelease_updates_cannot_select_a_separate_nightly() {
        let json = r#"[
            {"tag_name":"v9.0.0-nightly.20261120","prerelease":true},
            {"tag_name":"v1.0.0-beta.1","prerelease":true},
            {"tag_name":"v0.9.0","prerelease":false}
        ]"#;
        let release = parse_github_release_list(json, "storytold/photocraft", true).unwrap().unwrap();
        assert!(!is_nightly(&release));
        assert_eq!(release.version, crate::version::parse("v1.0.0-beta.1"));
    }

    #[test]
    fn truncate_keeps_whole_characters() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdef", 3), "abc…");
        // `é` is two bytes: cutting at 2 would split it, so the cut backs off to 1.
        assert_eq!(truncate("aé", 2), "a…");
    }

    #[test]
    fn only_plain_https_links_pass() {
        assert!(safe_web_link("https://github.com/storytold/photocraft/releases/tag/v1"));
        for bad in [
            "http://example.com/",
            "javascript:alert(1)",
            "file:///C:/Windows/System32/calc.exe",
            "ms-msdt:/id",
            "https://",
            "https://user@evil.example/",
            "https://example.com/a b",
            "https://example.com/\nx",
            "https:///path",
            // Shell/cmd metacharacters: the WSL opener fallback shells out, so these must be
            // refused even inside an otherwise valid https URL (CWE-78).
            "https://example.com/a|calc",
            "https://example.com/p?x=1&y=2",
            "https://example.com/`id`",
            "https://example.com/$(whoami)",
            "https://example.com/a;b",
            "https://example.com/a>b",
            "https://example.com/%COMSPEC%",
            "https://example.com/%PATH%",
        ] {
            assert!(!safe_web_link(bad), "{bad:?}");
        }
    }
    const HASH: &str = "8a5edab282632443219e051e4ade2d1d5bbc671c781051bf1437897cbdfea0f1";

    fn api_json(url: &str) -> String {
        format!(
            r#"{{"tag_name":"v0.5.0","name":"PhotoCraft v0.5.0","html_url":"https://github.com/storytold/photocraft/releases/tag/v0.5.0",
            "published_at":"2026-10-08T14:52:50Z","prerelease":false,"body":"Notes",
            "assets":[{{"name":"photocraft-0.5.0-macos-universal.dmg","size":123,"browser_download_url":"{url}","digest":"sha256:{HASH}"}}]}}"#
        )
    }

    #[test]
    fn reads_the_latest_release() {
        let r = parse_github_release(
            &api_json("https://github.com/storytold/photocraft/releases/download/v0.5.0/photocraft-0.5.0-macos-universal.dmg"),
            REPO,
        )
        .unwrap();
        assert_eq!(r.version, Some(Version::new(0, 5, 0)));
        assert_eq!(r.name, "PhotoCraft v0.5.0");
        assert_eq!(r.published_at, Some(1_791_471_170));
        assert_eq!(r.assets.len(), 1);
        assert_eq!(r.assets[0].sha256.as_deref(), Some(HASH));
        assert_eq!(r.assets[0].size, 123);
    }

    #[test]
    fn drops_downloads_that_point_elsewhere() {
        for url in ["https://evil.example/photocraft.dmg", "https://github.com/storytold/other/releases/download/v1/x.dmg", "javascript:alert(1)"] {
            let r = parse_github_release(&api_json(url), REPO).unwrap();
            assert!(r.assets.is_empty(), "{url}");
        }
        assert_eq!(parse_github_release(r#"{"message":"Not Found"}"#, REPO).unwrap_err(), "no releases yet");
        assert!(parse_github_release("<html>", REPO).is_err());
    }

    #[test]
    fn release_html_url_rejects_traversal_and_metacharacters() {
        let repo = "storytold/photocraft";
        assert_eq!(
            safe_release_html_url(repo, "https://github.com/storytold/photocraft/releases/tag/v1.0.0"),
            Some("https://github.com/storytold/photocraft/releases/tag/v1.0.0".to_string())
        );
        assert_eq!(
            safe_release_html_url(repo, "https://github.com/storytold/photocraft/releases"),
            Some("https://github.com/storytold/photocraft/releases".to_string())
        );
        for bad in [
            "https://github.com/storytold/photocraft/releases/../../attacker/repo/releases/tag/v1",
            "https://github.com/storytold/photocraft/releases/tag/v1/../../../attacker/repo",
            "https://github.com/storytold/photocraft/releases/tag/v1|calc.exe",
            "https://github.com/storytold/photocraft/releases/tag/v1%2f..%2f..",
            "https://github.com/storytold/photocraft/releases/tag/v1?x=1",
            "https://github.com/storytold/photocraft/releases/tag/v1#hash",
            "https://github.com/storytold/other/releases/tag/v1.0.0",
            "http://github.com/storytold/photocraft/releases/tag/v1",
        ] {
            assert!(safe_release_html_url(repo, bad).is_none(), "should reject: {bad}");
        }
    }

    #[test]
    fn selected_release_cannot_download_an_asset_from_a_different_tag() {
        let wrong_tag = "https://github.com/storytold/photocraft/releases/download/v9.0.0/photocraft-0.5.0-macos-universal.dmg";
        let r = parse_github_release(&api_json(wrong_tag), REPO).unwrap();
        assert!(r.assets.is_empty());
    }

    #[test]
    fn traversal_cannot_escape_the_repository() {
        // Raw-prefix matches but the interpreted path climbs into another repository (CWE-180).
        for url in [
            "https://github.com/storytold/photocraft/releases/download/../../../../attacker/repo/releases/download/v99/photocraft-99.0.0-linux-x86_64.AppImage",
            "https://github.com/storytold/photocraft/releases/download/v1/../../../attacker/repo/x.dmg",
            "https://github.com/storytold/photocraft/releases/download/./v1/x.dmg",
            "https://github.com/storytold/photocraft/releases/download/v1/sub/x.dmg",
            "https://github.com/storytold/photocraft/releases/download/v1%2f..%2f..%2fx.dmg",
            "https://github.com/storytold/photocraft/releases/download/v1/x%2fy.dmg",
        ] {
            let prefix = download_prefix(REPO);
            assert!(!download_url_under(url, &prefix), "{url}");
            let r = parse_github_release(&api_json(url), REPO).unwrap();
            assert!(r.assets.is_empty(), "{url}");
        }
        // A plain, confined download still passes.
        let prefix = download_prefix(REPO);
        assert!(download_url_under("https://github.com/storytold/photocraft/releases/download/v0.5.0/photocraft-0.5.0-macos-universal.dmg", &prefix));
    }

    #[test]
    fn version_choices_exclude_drafts_and_distinguish_nightly_from_betas() {
        let json = r#"[
            {"tag_name":"v0.8.0-nightly.20261028","prerelease":true,"draft":false,"assets":[]},
            {"tag_name":"v0.8.0-beta.1","prerelease":true,"draft":false,"assets":[]},
            {"tag_name":"v0.7.0","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.9.0-nightly.20261029","prerelease":true,"draft":true,"assets":[]},
            {"tag_name":"not-a-version","prerelease":false,"draft":false,"assets":[]}]"#;
        let choices = parse_github_versions(json, REPO).unwrap();
        assert_eq!(choices.iter().map(|r| r.tag.as_str()).collect::<Vec<_>>(), ["v0.8.0-nightly.20261028", "v0.8.0-beta.1", "v0.7.0"]);
        assert!(is_nightly(&choices[0]));
        assert!(!is_nightly(&choices[1]));
        assert!(!is_nightly(&choices[2]));
    }

    fn nightly_release(tag: &str, published_at: Option<i64>) -> Release {
        Release {
            tag: tag.into(),
            version: version::parse(tag),
            prerelease: true,
            published_at,
            assets: vec![Asset { name: "app.zip".into(), sha256: Some(HASH.into()), ..Default::default() }],
            ..Default::default()
        }
    }

    #[test]
    fn nightly_selection_requires_canonical_matching_tags_but_not_descriptor_shape() {
        use crate::protocol::VersionTrack;
        for tag in ["1.2.3-nightly", "v1.2.3-nightly.20261120.runA", "v1.2.3-nightly+build.02"] {
            let release = nightly_release(tag, None);
            assert_eq!(canonical_nightly_version(&release), release.version, "{tag}");
            assert_eq!(choose_version(&[release], &VersionTrack::Nightly).unwrap().tag, tag);
        }
        for tag in [
            " 1.2.3-nightly",
            "1.2.3-nightly ",
            "V1.2.3-nightly",
            "artcraft-v1.2.3-nightly",
            "v1.2-nightly",
            "v01.2.3-nightly",
            "v1.2.3-Nightly",
            "v1.2.3-beta.nightly",
            "v1.2.3-nightlyish",
            "v1.2.3",
        ] {
            let release = nightly_release(tag, None);
            assert!(canonical_nightly_version(&release).is_none(), "{tag}");
            assert!(choose_version(&[release], &VersionTrack::Nightly).is_none(), "{tag}");
        }
        let canonical = nightly_release("v1.2.3-nightly+build.a", None);
        let mut unmarked = canonical.clone();
        unmarked.prerelease = false;
        let mut missing_version = canonical.clone();
        missing_version.version = None;
        let mut mismatched_version = canonical.clone();
        mismatched_version.version = version::parse("v1.2.4-nightly+build.a");
        let mut mismatched_build = canonical;
        mismatched_build.version = version::parse("v1.2.3-nightly+build.b");
        for release in [unmarked, missing_version, mismatched_version, mismatched_build] {
            assert!(canonical_nightly_version(&release).is_none());
            assert!(choose_version(&[release], &VersionTrack::Nightly).is_none());
        }
    }

    #[test]
    fn later_publication_of_an_older_nightly_cannot_replace_a_newer_version() {
        use crate::protocol::VersionTrack;
        let older = nightly_release("v1.2.3-nightly.20261120.runA", Some(200));
        let newer = nightly_release("v1.3.0-nightly.20261119.runB", Some(100));
        let old_policy = [older.clone(), newer.clone()];
        assert_eq!(
            old_policy.iter().max_by_key(|release| release.published_at).unwrap().tag,
            older.tag,
            "the old publication-time policy selects the counterexample's older SemVer"
        );
        for list in [[older.clone(), newer.clone()], [newer.clone(), older]] {
            assert_eq!(choose_version(&list, &VersionTrack::Nightly).unwrap().tag, newer.tag);
        }
    }

    #[test]
    fn rerun_dates_and_opaque_ids_are_compared_only_as_semver_components() {
        use crate::protocol::VersionTrack;
        for (older, newer) in [
            // A rerun retains its original tag date even when it is published later.
            ("v1.2.3-nightly.20261119.run999", "v1.2.3-nightly.20261120.run1"),
            // Opaque alphanumeric IDs follow SemVer lexical order, not extracted integers.
            ("v1.2.3-nightly.20261120.run10", "v1.2.3-nightly.20261120.run9"),
            // Numeric components still follow SemVer numeric order, not raw tag order.
            ("v1.2.3-nightly.20261120.9", "v1.2.3-nightly.20261120.10"),
        ] {
            let older = nightly_release(older, Some(200));
            let newer = nightly_release(newer, None);
            for list in [[older.clone(), newer.clone()], [newer.clone(), older]] {
                assert_eq!(choose_version(&list, &VersionTrack::Nightly).unwrap().tag, newer.tag);
            }
        }
    }

    #[test]
    fn equal_nightly_precedence_prefers_smallest_tag_without_metadata_or_publication_ordering() {
        use crate::protocol::VersionTrack;
        for tags in [["v1.2.3-nightly+z", "v1.2.3-nightly+a"], ["v1.2.3-nightly+a", "v1.2.3-nightly"], ["v1.2.3-nightly", "1.2.3-nightly+z"]] {
            let larger_tag = nightly_release(tags[0], Some(200));
            let smaller_tag = nightly_release(tags[1], Some(100));
            for list in [[larger_tag.clone(), smaller_tag.clone()], [smaller_tag.clone(), larger_tag]] {
                assert_eq!(choose_version(&list, &VersionTrack::Nightly).unwrap().tag, smaller_tag.tag);
            }
        }
        let older = nightly_release("v1.2.3-nightly.9+z", Some(200));
        let newer = nightly_release("v1.2.3-nightly.10+a", Some(100));
        assert_eq!(choose_version(&[older, newer.clone()], &VersionTrack::Nightly).unwrap().tag, newer.tag);
    }

    #[test]
    fn ordinary_version_list_keeps_its_order_limit_beta_filtering_and_exact_pins() {
        use crate::protocol::VersionTrack;
        let mut items: Vec<_> = (0..105).map(|i| serde_json::json!({"tag_name": format!("v0.1.{i}"), "prerelease": false})).collect();
        items[0] = serde_json::json!({"tag_name": "v99.0.0-nightly", "prerelease": true});
        items[1] = serde_json::json!({"tag_name": "v9.0.0-beta.1", "prerelease": true});
        let json = serde_json::to_string(&items).unwrap();
        let list = parse_github_versions(&json, REPO).unwrap();
        assert_eq!(list.len(), 100);
        assert_eq!(list[0].tag, "v99.0.0-nightly");
        assert_eq!(list[99].tag, "v0.1.99");
        assert_eq!(choose_version(&list, &VersionTrack::Stable).unwrap().tag, "v0.1.99");
        // Pinned releases retain exact matching, with neither checksum nor Nightly fallback.
        assert_eq!(choose_version(&list, &VersionTrack::Pinned("v99.0.0-nightly".into())).unwrap().tag, "v99.0.0-nightly");
        assert_eq!(choose_version(&list, &VersionTrack::Pinned("v0.1.99".into())).unwrap().tag, "v0.1.99");
        assert!(choose_version(&list, &VersionTrack::Pinned("v0.1.100".into())).is_none());
        assert!(choose_version(&list, &VersionTrack::Pinned("0.1.99".into())).is_none());
        assert_eq!(parse_github_release_list(&json, REPO, false).unwrap().unwrap().tag, "v0.1.104");
        assert_eq!(parse_github_release_list(&json, REPO, true).unwrap().unwrap().tag, "v9.0.0-beta.1");
    }

    #[test]
    fn a_nightly_without_published_checksums_is_unavailable() {
        let mut nightly = Release {
            tag: "v1.1.0-nightly.20261028".into(),
            version: crate::version::parse("v1.1.0-nightly.20261028"),
            prerelease: true,
            ..Default::default()
        };
        nightly.assets.push(Asset { name: "app.zip".into(), size: 3, url: "https://example.invalid/app.zip".into(), sha256: None });
        assert!(choose_version(&[nightly], &crate::protocol::VersionTrack::Nightly).is_none());
    }

    #[test]
    fn selection_follows_nightly_or_pins_an_older_release_without_beta_fallback() {
        use crate::protocol::VersionTrack;
        let json = r#"[
            {"tag_name":"v0.8.0-nightly.20261028","prerelease":true,"published_at":"2026-10-28T10:00:00Z"},
            {"tag_name":"v0.8.0-beta.1","prerelease":true,"published_at":"2026-10-28T12:00:00Z"},
            {"tag_name":"v0.8.0-nightly.20261027","prerelease":true,"published_at":"2026-10-27T10:00:00Z"},
            {"tag_name":"v0.7.0","prerelease":false},
            {"tag_name":"v0.6.0","prerelease":false}]"#;
        let mut list = parse_github_versions(json, REPO).unwrap();
        for r in list.iter_mut().filter(|r| is_nightly(r)) {
            r.assets.push(Asset { name: "app.zip".into(), size: 3, url: "https://example.invalid/app.zip".into(), sha256: Some(HASH.into()) });
        }
        assert_eq!(choose_version(&list, &VersionTrack::Stable).unwrap().tag, "v0.7.0");
        assert_eq!(choose_version(&list, &VersionTrack::Nightly).unwrap().tag, "v0.8.0-nightly.20261028");
        assert_eq!(choose_version(&list, &VersionTrack::Pinned("v0.6.0".into())).unwrap().tag, "v0.6.0");
        assert!(choose_version(&list, &VersionTrack::Pinned("v9.0.0".into())).is_none());
        assert!(choose_version(&list[3..], &VersionTrack::Nightly).is_none());
    }

    #[test]
    fn list_picks_the_newest_stable_release() {
        let json = r#"[
            {"tag_name":"v0.6.0-rc.1","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.5.1","prerelease":true,"draft":false,"assets":[]},
            {"tag_name":"v0.7.0","draft":true,"assets":[]},
            {"tag_name":"v0.5.0","prerelease":false,"draft":false,"assets":[]},
            {"tag_name":"v0.4.0","prerelease":false,"draft":false,"assets":[]}]"#;
        assert_eq!(parse_github_release_list(json, REPO, false).unwrap().unwrap().tag, "v0.5.0");
        assert_eq!(parse_github_release_list(json, REPO, true).unwrap().unwrap().tag, "v0.6.0-rc.1");
        assert_eq!(parse_github_release_list("[]", REPO, false).unwrap(), None);
    }

    #[test]
    fn reads_checksums_and_redirects() {
        let sums = parse_sha256sums(&format!("{HASH}  photocraft-0.5.0-macos-universal.dmg\n{HASH} *b.zip\nnot a line\n{HASH}  ../escape\n"));
        assert_eq!(sums, [(HASH.to_string(), "photocraft-0.5.0-macos-universal.dmg".to_string()), (HASH.to_string(), "b.zip".to_string())]);
        assert_eq!(tag_from_redirect("https://github.com/storytold/photocraft/releases/tag/v0.5.0", REPO).as_deref(), Some("v0.5.0"));
        assert_eq!(tag_from_redirect("https://github.com/storytold/photocraft/releases", REPO), None);
        assert_eq!(tag_from_redirect("https://evil/storytold/photocraft/releases/tag/v1", REPO), None);
        let r = release_from_sums(REPO, "v0.5.0", &sums);
        assert_eq!(r.assets[0].url, "https://github.com/storytold/photocraft/releases/download/v0.5.0/photocraft-0.5.0-macos-universal.dmg");
        assert_eq!(r.version, Some(Version::new(0, 5, 0)));
    }

    #[test]
    fn reviewed_package_survives_quota_free_discovery_without_a_sums_manifest() {
        let platform = crate::Platform { os: crate::Os::MacOs, arch: crate::Arch::Arm64 };
        let release = release_from_sums("cjpais/Handy", "v0.10.2", &[]);
        let (asset, _) = crate::platform::pick_asset(&release, &["handy"], platform, &[crate::PackageKind::Dmg]).unwrap();
        assert!(asset.sha256.is_some());
        for (repo, tag) in [("cjpais/Handy", "v0.10.3"), ("other/Handy", "v0.10.2")] {
            let release = release_from_sums(repo, tag, &[]);
            assert!(crate::platform::pick_asset(&release, &["handy"], platform, &[crate::PackageKind::Dmg]).is_none());
        }
        let intel = crate::Platform { os: crate::Os::MacOs, arch: crate::Arch::X64 };
        assert!(crate::platform::pick_asset(&release, &["handy"], intel, &[crate::PackageKind::Dmg]).is_none());
        let localsend = release_from_sums("localsend/localsend", "v1.19.0", &[]);
        let stems: Vec<_> = crate::curated::LOCALSEND.stems().collect();
        let linux = crate::Platform { os: crate::Os::Linux, arch: crate::Arch::X64 };
        let (image, _) = crate::platform::pick_asset(&localsend, &stems, linux, &[crate::PackageKind::AppImage]).unwrap();
        assert_eq!(image.size, 61_752_512);
        assert_eq!(image.sha256.as_deref(), Some("e72ce88383a32beec9701583c22e298037f7683f9590cea7b3a281f58c2ec2b3"));
        let (dmg, _) = crate::platform::pick_asset(&localsend, &stems, platform, &[crate::PackageKind::Dmg]).unwrap();
        assert_eq!(dmg.size, 52_878_051);
    }

    #[test]
    fn audacity_release_tag_is_parsed_for_the_official_repo() {
        let json = r#"{"tag_name":"Audacity-4.0.2"}"#;
        let official = parse_github_release(json, crate::curated::AUDACITY.repo).unwrap();
        assert_eq!(official.version, Some(Version::new(4, 0, 2)));
        assert_eq!(official.tag, "Audacity-4.0.2");
        assert!(parse_github_release(json, "example/audacity").unwrap().version.is_none());
    }

    #[test]
    fn utm_only_maps_the_reviewed_retag_to_a_stable_version() {
        let repo = crate::curated::UTM.repo;
        assert_eq!(release_tag_version(repo, "v5.0.9-2"), Some(Version::new(5, 0, 9)));
        assert!(!release_tag_version("example/UTM", "v5.0.9-2").unwrap().pre.is_empty());
        assert!(!release_tag_version(repo, "v5.0.10-2").unwrap().pre.is_empty());
    }

    #[test]
    fn file_names_cannot_escape() {
        for bad in [
            "",
            "..",
            "a/b",
            "a\\b",
            ".hidden",
            "c:x",
            "x\u{0}",
            "a b.dmg",
            "a%2e.msi",
            "a#b",
            "evil\u{202e}gpj.exe",
            "x.msi.",
            "CON",
            "nul.txt",
            "COM1.msi",
            "lpt9",
            "prn",
            "aux.log",
            "a".repeat(201).as_str(),
        ] {
            assert!(!valid_file_name(bad), "{bad:?}");
        }
        for good in [
            "photocraft-0.5.0-linux-x86_64.AppImage",
            "lightcraft_0.9.0_amd64.deb",
            "x-1.0+git.tar.gz",
            "console.zip",
            "COM10.txt",
            "COM0.msi",
            "lpt0",
        ] {
            assert!(valid_file_name(good), "{good:?}");
        }
    }

    #[test]
    fn tags_are_plain() {
        for good in ["v0.6.0", "v1.0.0-beta.2", "nightly-2026-10-29", "v1+build.7"] {
            assert!(valid_tag(good), "{good:?}");
        }
        for bad in ["", ".", "..", ".v1", "v1/x", "v1\\x", "v1\nx", "v1\u{0}x", "v1\u{202e}", "v 1", "v1?x", "v".repeat(65).as_str()] {
            assert!(!valid_tag(bad), "{bad:?}");
        }
        let api = r#"{"tag_name": "v1\u202egpj", "html_url": "", "assets": []}"#;
        assert!(parse_github_release(api, REPO).is_err());
        let prefix = download_prefix(REPO);
        assert!(download_url_under(&format!("{prefix}v1/a.dmg"), &prefix));
        assert!(!download_url_under(&format!("{prefix}v1\u{202e}/a.dmg"), &prefix), "an odd tag in a download link");
        assert!(!download_url_under(&format!("{prefix}v1/a\u{202e}.dmg"), &prefix), "an odd name in a download link");
        assert_eq!(tag_from_redirect(&format!("https://github.com/{REPO}/releases/tag/v1\u{202e}"), REPO), None);
    }
    /// A pre-release is a beta or release candidate, never a Nightly, for every app: one without
    /// its own Nightly channel (a community app) as much as one with it.
    #[test]
    fn ordinary_prerelease_lookup_never_offers_a_nightly_for_any_app() {
        // Authored representative metadata; not a claim about live SolidCraft releases.
        let json = r#"[{"tag_name":"v2.0.0-dev.nightly.1","prerelease":true},{"tag_name":"v1.0.0-beta.1","prerelease":true},{"tag_name":"v0.9.0","prerelease":false}]"#;
        for id in ["solidcraft", "filmcraft", "photocraft"] {
            let repo = crate::CraftApp::by_id(id).unwrap().repo;
            assert_eq!(parse_github_release_list(json, repo, true).unwrap().unwrap().tag, "v1.0.0-beta.1", "{id}");
            assert_eq!(parse_github_release_list(json, repo, false).unwrap().unwrap().tag, "v0.9.0", "{id}");
        }
    }
    #[test]
    fn a_feed_repo_rename_or_a_missing_prerelease_flag_cannot_bring_photocraft_nightlies_back() {
        let app = crate::CraftApp::by_id("photocraft").unwrap();
        let feed =
            crate::feed::Feed::parse(r#"{"repo_renames":[{"old_name":"storytold/photocraft","new_name":"storytold/photocraft-studio"}]}"#).unwrap();
        let query = ReleaseQuery::for_app(app, feed.repo_for(app));
        assert_eq!(query.repo, "storytold/photocraft-studio");
        let flagged =
            r#"[{"tag_name":"v2.0.0-nightly.20261123.1.1.gabcdefabcdef","prerelease":true},{"tag_name":"v1.0.0-beta.1","prerelease":true}]"#;
        assert_eq!(parse_github_release_list(flagged, &query.repo, true).unwrap().unwrap().tag, "v1.0.0-beta.1");
        let unflagged = r#"[{"tag_name":"v2.0.0-nightly.20261123.1.1.gabcdefabcdef","prerelease":false},{"tag_name":"v1.0.0","prerelease":false}]"#;
        assert_eq!(parse_github_release_list(unflagged, app.repo, true).unwrap().unwrap().tag, "v1.0.0");
        // Any other repo leaves nightlies out of ordinary lookups too.
        assert_eq!(parse_github_release_list(flagged, "storytold/photocraft-studio", true).unwrap().unwrap().tag, "v1.0.0-beta.1");
    }
}
