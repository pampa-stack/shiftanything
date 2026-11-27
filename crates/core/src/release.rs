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
        self.version.as_ref().map_or_else(|| self.tag.clone(), ToString::to_string)
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
                || [crate::curated::KOPIA.repo, crate::curated::TERMIX.repo].iter().any(|repo| {
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
    } else if repo == crate::curated::TERMIX.repo {
        "SHA256SUMS"
    } else {
        "SHA256SUMS.txt"
    }
}

const MAX_NOTES: usize = 64 * 1024;

pub(crate) fn is_hex64(s: &str) -> bool {
    s.len() == 64 && s.bytes().all(|b| b.is_ascii_hexdigit())
}

pub const LIBREOFFICE_STABLE_URL: &str = "https://download.documentfoundation.org/libreoffice/stable/";
pub const LIBREOFFICE_DMG_MAX_SIZE: u64 = 1024 * 1024 * 1024;

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
    let assets = v["assets"]
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

fn release_tag_version(repo: &str, tag: &str) -> Option<Version> {
    if repo == crate::curated::AUDACITY.repo {
        version::parse(tag.strip_prefix("Audacity-")?)
    } else if repo == crate::curated::ENTE_AUTH.repo {
        ente_auth_version(tag)
    } else if repo == crate::curated::TERMIX.repo {
        termix_desktop_version(tag)
    } else if repo == crate::curated::PICARD.repo {
        picard_version(tag)
    } else if repo == crate::curated::GODOT_RELEASES_REPO {
        godot_stable_version(tag)
    } else {
        version::parse(tag)
    }
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

// Official package receipts apply only to the original repository, tag and exact asset.
fn reviewed_assets<'a>(repo: &'a str, tag: &'a str) -> impl Iterator<Item = Asset> + 'a {
    let receipts: &[(&str, u64, &str)] = match (repo, tag) {
        ("cjpais/Handy", "v0.10.2") => {
            &[("Handy_0.10.2_aarch64.dmg", 14_979_688, "b8b893fbf9e8262abb0e363cb0ac6715cc69eac2fd6ffd8c0785278083a0e3ca")]
        }
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
        ("keepassxreboot/keepassxc", "2.7.12") => {
            &[("KeePassXC-2.7.12-arm64.dmg", 34_360_675, "65f4f63607180c0a15794b4a4068f85e99ed5391c87c1fb9312648f1b36fed40")]
        }
        ("keepassxreboot/keepassxc", "2.8.0") => {
            &[("KeePassXC-2.8.0-1-arm64.dmg", 47_786_914, "85f0e75e258ef513f0fc7f5bf470cad0943b7d7e8191b2171f9898ad32e62b9c")]
        }
        ("laurent22/joplin", "v3.7.22") => {
            &[("Joplin-3.7.22-arm64.DMG", 211_992_036, "185395eba8d9dac2d8656884cbb0e5b96bcb40b41d80c67126ce5526cb5186c8")]
        }
        ("laurent22/joplin", "v3.7.24") => {
            &[("Joplin-3.7.24-arm64.DMG", 212_067_439, "d95c2376035c7630dfb94f6eecca2f6edb3a27531626553a344f2b2308a5830e")]
        }
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
        ("jgraph/drawio-desktop", "v32.6.10") => &[
            ("drawio-x86_64-32.6.10.AppImage", 152_706_651, "94158126a43016b4e6f24bbc2c840ab8b5a51c6bf2796e1b3dadc64390b2d256"),
            ("draw.io-arm64-32.6.10.dmg", 157_367_951, "83e4027837f89bfba9dafbcc5c0e4befa76e2ac7f4e37f7a7b40c817141a6727"),
        ],
        ("jgraph/drawio-desktop", "v32.7.4") => &[
            ("drawio-x86_64-32.7.4.AppImage", 154_092_007, "7d78c781d4ec33d86dc48251c3cadc9c18dcea6bc5d581322c0335ccb33b222c"),
            ("draw.io-arm64-32.7.4.dmg", 158_930_055, "b0ba01224cdc4817ebf218d73b2a45ff3569f8c9135ad422c5157655062900b3"),
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
        ("cryptomator/cryptomator", "1.19.6") => {
            &[("Cryptomator-1.19.6-arm64.dmg", 58_714_109, "fdfc8ef50b10baca1078b7a9edf404e330363c8a3a40fda29f86dd5d3a0a0b02")]
        }
        ("cryptomator/cryptomator", "1.19.7") => {
            &[("Cryptomator-1.19.7-arm64.dmg", 58_716_219, "c4818d38cb3f9919bfe276c812714596cc50da4d01a82ac2f22bf2e64b441677")]
        }
        ("ActivityWatch/activitywatch", "v0.14.2") => {
            &[("activitywatch-v0.14.2-macos-arm64.dmg", 65_835_856, "8341d56b5b43a60ca04ea2a3cc5ffbfb5e661cdb6430145d9ceb0f49dca63d65")]
        }
        ("ActivityWatch/activitywatch", "v0.14.3") => {
            &[("activitywatch-v0.14.3-macos-arm64.dmg", 67_250_584, "5462f3b90f9e1c0352061888d5f041029abb2ccfbe05e42c6dc63d91ff26bd4a")]
        }
        ("borgbase/vorta", "v0.12.1") => &[("Vorta-v0.12.1-arm.dmg", 45_686_573, "b65da5b0fc3dcb1792897eaf2dad62e0b2cbc1db4a577ff39668b28c7ae2600b")],
        ("borgbase/vorta", "v0.12.2") => &[("Vorta-v0.12.2-arm.dmg", 45_694_460, "2a377fa5e04068cdc4f8c9316e34046cc1bae11acca4e8daf20e4ca7fdb8723d")],
        ("getopenscreen/openscreen", "v2.3.0") => &[(
            "Openscreen-macOS-Apple-Silicon-2.3.0.dmg",
            141_157_755,
            "db2f75451ac364e19c4d56e45a85b8572056312c54461979f13c7b1053ea2d82",
        )],
        ("getopenscreen/openscreen", "v2.4.0") => &[(
            "Openscreen-macOS-Apple-Silicon-2.4.0.dmg",
            141_308_945,
            "177374fd4b1b9290661db535010032247e0cea42f9bdf281c918c5e418876a51",
        )],
        ("thewh1teagle/vibe", "v3.3.5") => &[("vibe_3.3.5_aarch64.dmg", 38_715_249, "efaf965175cf9d6b7c6782e38eb42b50800a4645c06771584fcbd46c74d75807")],
        ("thewh1teagle/vibe", "v3.3.8") => &[("vibe_3.3.8_aarch64.dmg", 38_640_733, "48dd4ffd5b4fc6a9466fc2b68ee04d398e794404758577e46f27163614138a39")],
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
        ("musescore/MuseScore", "v4.7.6") => {
            &[("MuseScore-Studio-4.7.6.261022093-x86_64.AppImage", 192_616_952, "5fcbd1f292270eca5c3867d56b767c31a173a155216a415f5caee22553f8e7a6")]
        }
        ("musescore/MuseScore", "v4.7.7") => {
            &[("MuseScore-Studio-4.7.7.261109195-x86_64.AppImage", 192_743_928, "8597105814359676483807ce4c4051eaf439bd723ebccaa9160632f56c05a02d")]
        }
        ("mifi/lossless-cut", "v3.70.1") => {
            &[("LosslessCut-linux-x86_64.AppImage", 221_418_564, "04c87c97ff3980b8ebb6a62f0394204232e73a7dff89ea86ef49be3c372b0842")]
        }
        ("mifi/lossless-cut", "v3.70.2") => {
            &[("LosslessCut-linux-x86_64.AppImage", 221_357_823, "e9ea00c9cf6cdda10db9d9cd13fc9ca10d61cdc046a60f5ee340db6643164a4a")]
        }
        ("OrcaSlicer/OrcaSlicer", "v2.5.1") => &[(
            "OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.1.AppImage",
            201_116_152,
            "073b8105c928356141db0ddbce88b81d52c7a9e2442f9baae0a9089c89ff04eb",
        )],
        ("OrcaSlicer/OrcaSlicer", "v2.5.2") => &[(
            "OrcaSlicer_Linux_AppImage_Ubuntu2404_V2.5.2.AppImage",
            201_034_232,
            "22fa88884dec3fdbd29e90cba9912fcff6fb57a60b1bdc1a6c189c61a6b0a870",
        )],
        ("Zettlr/Zettlr", "v4.10.0") => {
            &[("Zettlr-4.10.0-x86_64.AppImage", 172_628_308, "bf6f7458c2377818d5d4ed008a721c295bec2c25d570cf8f54d053f8819508fc")]
        }
        ("Zettlr/Zettlr", "v4.11.0") => {
            &[("Zettlr-4.11.0-x86_64.AppImage", 172_657_896, "e9b5406d62e2c9e5fef3adb092877669d849d16a217375c4e1736cf9c7aa9bd7")]
        }
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
        _ => &[],
    };
    receipts.iter().map(move |&(name, size, sha256)| Asset {
        name: name.into(),
        size,
        url: format!("{}{tag}/{name}", download_prefix(repo)),
        sha256: Some(sha256.into()),
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
