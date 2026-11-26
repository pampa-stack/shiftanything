//! Approved static PNG conversions supplied by Regia cafc6fb9. Original URLs and byte digests
//! are recorded in assets/catalog-media/manifest.json; no SVG/GIF/WebP decoder is enabled.
type Media = (&'static str, &'static str, &'static str, &'static [u8]);
const MEDIA: &[Media] = &[
    (
        "t3code",
        "catalog-shot-0",
        "https://raw.githubusercontent.com/pingdotgg/t3code/main/.github/readme/hero.webp",
        include_bytes!("../../../assets/catalog-media/t3code-shot-0.png"),
    ),
    (
        "solidcraft",
        "catalog-shot-0",
        "https://raw.githubusercontent.com/tangency-labs/solidcraft/main/docs/screenshots/demo.gif",
        include_bytes!("../../../assets/catalog-media/solidcraft-shot-0.png"),
    ),
    (
        "wazuh",
        "catalog-logo",
        "https://raw.githubusercontent.com/wazuh/wazuh-dashboard-plugins/main/plugins/main/public/assets/images/themes/light/icon.svg",
        include_bytes!("../../../assets/catalog-media/wazuh-logo.png"),
    ),
    (
        "uptime",
        "catalog-logo",
        "https://raw.githubusercontent.com/louislam/uptime-kuma/master/public/icon.svg",
        include_bytes!("../../../assets/catalog-media/uptime-logo.png"),
    ),
    (
        "paseo",
        "catalog-logo",
        "https://raw.githubusercontent.com/getpaseo/paseo/main/packages/website/public/logo.svg",
        include_bytes!("../../../assets/catalog-media/paseo-logo.png"),
    ),
    (
        "opencode",
        "catalog-logo",
        "https://raw.githubusercontent.com/anomalyco/opencode/dev/.github/assets/logo-ornate-dark.svg",
        include_bytes!("../../../assets/catalog-media/opencode-logo.png"),
    ),
    (
        "cutter",
        "catalog-logo",
        "https://raw.githubusercontent.com/rizinorg/cutter/dev/src/img/cutter.svg",
        include_bytes!("../../../assets/catalog-media/cutter-logo.png"),
    ),
    (
        "openutau",
        "catalog-logo",
        "https://raw.githubusercontent.com/openutau/OpenUtau/master/Logo/openutau.svg",
        include_bytes!("../../../assets/catalog-media/openutau-logo.png"),
    ),
    (
        "openutau",
        "catalog-shot-0",
        "https://raw.githubusercontent.com/openutau/OpenUtau/master/Misc/GIFs/editor.gif",
        include_bytes!("../../../assets/catalog-media/openutau-shot-0.png"),
    ),
    (
        "openutau",
        "catalog-shot-1",
        "https://raw.githubusercontent.com/openutau/OpenUtau/master/Misc/GIFs/editor2.gif",
        include_bytes!("../../../assets/catalog-media/openutau-shot-1.png"),
    ),
    (
        "openutau",
        "catalog-shot-2",
        "https://raw.githubusercontent.com/openutau/OpenUtau/master/Misc/GIFs/vibrato.gif",
        include_bytes!("../../../assets/catalog-media/openutau-shot-2.png"),
    ),
    (
        "plane",
        "catalog-shot-0",
        "https://media.docs.plane.so/GitHub-readme/github-work-items.webp",
        include_bytes!("../../../assets/catalog-media/plane-shot-0.png"),
    ),
    (
        "penpot",
        "catalog-logo",
        "https://raw.githubusercontent.com/penpot/penpot/develop/frontend/resources/images/assets/penpot-logo-icon.svg",
        include_bytes!("../../../assets/catalog-media/penpot-logo.png"),
    ),
    (
        "penpot",
        "catalog-shot-0",
        "https://raw.githubusercontent.com/penpot/penpot/develop/docs/img/interface/dashboard-dark.webp",
        include_bytes!("../../../assets/catalog-media/penpot-shot-0.png"),
    ),
    (
        "rizzo-flow",
        "catalog-shot-0",
        "https://raw.githubusercontent.com/Rizzo-AI-Academy/rizzo-flow/main/assets/playground.webp",
        include_bytes!("../../../assets/catalog-media/rizzo-flow-shot-0.png"),
    ),
];
pub(super) fn fallback(app: &str, key: &str, source: &str) -> Option<Vec<u8>> {
    MEDIA.iter().find(|(id, slot, url, _)| *id == app && *slot == key && *url == source).map(|(_, _, _, bytes)| bytes.to_vec())
}
#[cfg(test)]
mod tests {
    #[test]
    fn fallback_requires_matching_source_and_preserves_integrity_and_decoder_budget() {
        use sha2::{Digest, Sha256};
        let entries: serde_json::Value = serde_json::from_str(include_str!("../../../assets/catalog-media/manifest.json")).unwrap();
        for item in entries.as_array().unwrap() {
            let app = item["app"].as_str().unwrap();
            let key = format!("catalog-{}", item["key"].as_str().unwrap());
            let url = item["source"].as_str().unwrap();
            let bytes = super::fallback(app, &key, url).unwrap();
            assert_eq!(format!("{:x}", Sha256::digest(&bytes)), item["sha256"].as_str().unwrap());
            assert!(artcraft_launcher_core::curated::media_within_budget(&bytes));
            assert!(bytes.len() <= artcraft_launcher_core::details::MAX_SCREENSHOT_BYTES);
            assert!(super::fallback(app, &key, "https://example.com/swapped.svg").is_none());
            assert!(super::fallback("unknown-app", &key, url).is_none());
            assert!(super::fallback(app, "unknown-slot", url).is_none());
        }
    }
}
