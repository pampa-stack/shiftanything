use std::collections::BTreeSet;

use artcraft_launcher_core::release::{
    Asset, LIBREOFFICE_STABLE_URL, Release, libreoffice_arm64_asset, libreoffice_arm64_dmg, libreoffice_stable_version,
};
use artcraft_launcher_core::version::Version;

use crate::http::Http;

const INDEX_LIMIT: usize = 64 * 1024;
const METALINK_LIMIT: usize = 256 * 1024;
const VERSION_LIMIT: usize = 8;

pub(super) fn releases(http: &Http, latest_only: bool) -> Result<Vec<Release>, String> {
    let body =
        http.get_tdf_bytes(LIBREOFFICE_STABLE_URL, INDEX_LIMIT)?.ok_or_else(|| "The LibreOffice stable directory is unavailable".to_string())?;
    let index = std::str::from_utf8(&body).map_err(|_| "The LibreOffice stable directory is unreadable".to_string())?;
    let tags = stable_tags(index);
    if tags.is_empty() {
        return Err("The LibreOffice stable directory contains no recognized releases".into());
    }
    let limit = if latest_only { 1 } else { VERSION_LIMIT };
    tags.into_iter()
        .rev()
        .take(limit)
        .map(|version| {
            let (name, url) = libreoffice_arm64_dmg(&version);
            let bytes = http
                .get_tdf_bytes(&format!("{url}.meta4"), METALINK_LIMIT)?
                .ok_or_else(|| format!("LibreOffice {version} is missing its Apple Silicon checksum"))?;
            let metalink = std::str::from_utf8(&bytes).map_err(|_| format!("The checksum for LibreOffice {version} is unreadable"))?;
            let (size, sha256) =
                package_metadata(metalink, &name).ok_or_else(|| format!("The checksum and size for LibreOffice {version} could not be verified"))?;
            let asset = Asset { name, url, size, sha256: Some(sha256) };
            let release = Release {
                tag: version.to_string(),
                name: format!("LibreOffice {version}"),
                html_url: format!("{LIBREOFFICE_STABLE_URL}{version}/"),
                version: Some(version),
                ..Release::default()
            };
            if !libreoffice_arm64_asset(&release, &asset) {
                return Err(format!("Invalid Apple Silicon package for LibreOffice {}", release.tag));
            }
            Ok(Release { assets: vec![asset], ..release })
        })
        .collect()
}

fn stable_tags(index: &str) -> BTreeSet<Version> {
    index.split("<a href=\"").skip(1).filter_map(|link| libreoffice_stable_version(link.split_once('"')?.0.strip_suffix('/')?)).collect()
}

fn package_metadata(metalink: &str, package: &str) -> Option<(u64, String)> {
    // Read the single file's size and SHA-256 from the publisher's Metalink template. It can
    // advertise insecure mirrors, so download URLs always come from the pinned recipe instead.
    if !metalink.contains("<metalink xmlns=\"urn:ietf:params:xml:ns:metalink\">") || metalink.matches("<file ").count() != 1 {
        return None;
    }
    let file = single_field(metalink, &format!("<file name=\"{package}\">"), "</file>")?;
    let size = single_field(file, "<size>", "</size>")?.parse::<u64>().ok()?;
    let sha256 = single_field(file, "<hash type=\"sha-256\">", "</hash>")?.to_ascii_lowercase();
    Some((size, sha256))
}

fn single_field<'a>(source: &'a str, before: &str, after: &str) -> Option<&'a str> {
    let mut matches = source.match_indices(before);
    let (start, _) = matches.next()?;
    if matches.next().is_some() {
        return None;
    }
    let value = source.get(start + before.len()..)?.split_once(after)?.0;
    Some(value.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_index_accepts_only_relative_canonical_tags() {
        let tags = stable_tags(
            r#"<a href="26.8.3/">x</a><a href="26.8.2/">y</a><a href="26.8.3/">duplicate</a>
<a href="https://other.example/99.0.0/">remote</a><a href="26.8.4-rc.1/">beta</a><a href="26.9.0+1/">build</a><a href="../">parent</a>"#,
        );
        assert_eq!(tags.into_iter().rev().map(|v| v.to_string()).collect::<Vec<_>>(), ["26.8.3", "26.8.2"]);
    }

    #[test]
    fn metalink_requires_one_file_and_its_sha256() {
        let package = "LibreOffice_26.8.3_MacOS_aarch64.dmg";
        let sha = "A".repeat(64);
        let data = format!(
            r#"<metalink xmlns="urn:ietf:params:xml:ns:metalink"><file name="{package}"><size>299927705</size><hash type="sha-256">{sha}</hash><url>http://mirror.example/package</url></file></metalink>"#
        );
        assert_eq!(package_metadata(&data, package), Some((299927705, "a".repeat(64))));
        assert_eq!(package_metadata(&data, "other.dmg"), None);
        assert_eq!(package_metadata(&data.replace("</file>", "<hash type=\"sha-256\">other</hash></file>"), package), None);
        assert_eq!(package_metadata(&data.replace("</metalink>", "<file name=\"other.dmg\"></file></metalink>"), package), None);
    }
}
