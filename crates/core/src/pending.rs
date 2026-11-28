//! Recipes written but switched off, for the beta 0.21 lot (`docs/verification/pro-371-beta-021-lot`).
//!
//! Nothing here reaches the installer: no table below is read by release selection, by
//! [`crate::flathub::supported`] or by the verified Linux AppImage list. An entry moves into the
//! live tables in the same commit that adds its lifecycle proof (install, open, update, reopen,
//! remove, synthetic data kept) to `verified-platforms.json`, and not before.
use crate::release::{Asset, Release, download_prefix, is_hex64, release_tag_version};

/// A Flathub package that is catalogued but has no lifecycle proof yet.
pub struct FlatpakCandidate {
    /// Catalog id in `curated.json`.
    pub id: &'static str,
    /// Flathub application id.
    pub flatpak: &'static str,
    /// The catalog's repository URL for the app; the identity the recipe is bound to.
    pub upstream: &'static str,
}

/// Flathub recipes still waiting for their cycle on Ubuntu 24.04 x64.
pub const FLATPAK: [FlatpakCandidate; 3] = [
    FlatpakCandidate { id: "angryip", flatpak: "org.angryip.ipscan", upstream: "https://github.com/angryip/ipscan" },
    FlatpakCandidate {
        id: "opentabletdriver",
        flatpak: "net.opentabletdriver.OpenTabletDriver",
        upstream: "https://github.com/OpenTabletDriver/OpenTabletDriver",
    },
    FlatpakCandidate { id: "openutau", flatpak: "com.openutau.OpenUtau", upstream: "https://github.com/openutau/OpenUtau" },
];

/// The Flathub id for a candidate whose catalog identity matches, `None` otherwise.
pub fn flatpak_candidate(id: &str, repo_url: &str) -> Option<&'static str> {
    FLATPAK.iter().find(|candidate| candidate.id == id && candidate.upstream == repo_url).map(|candidate| candidate.flatpak)
}

/// One reviewed upstream file: exact release, name, size and SHA-256.
pub struct Receipt {
    pub tag: &'static str,
    pub name: &'static str,
    pub size: u64,
    pub sha256: &'static str,
}

/// Umi-OCR 2.1.4 and 2.1.5 for Linux x64. GitHub reports no digest for these files, so the
/// hashes are the ones computed from the downloaded bytes on 28 November 2026.
pub const UMI_OCR_ARCHIVES: [Receipt; 2] = [
    Receipt {
        tag: "v2.1.4",
        name: "Umi-OCR_Linux_Paddle_2.1.4.tar.xz",
        size: 277_414_412,
        sha256: "0dae3a93eb4fc8da23097944f403d143430ec27a91ce3fa563ac5d363b69246e",
    },
    Receipt {
        tag: "v2.1.5",
        name: "Umi-OCR_Linux_Paddle_2.1.5.tar.xz",
        size: 277_369_884,
        sha256: "7ff32e5dc818d67171820545100433c86e7e71155f9bad0851fad2ced9588f9b",
    },
];

/// Termix 26.11.2 and 26.11.3 AppImages for Linux x64, with the digests GitHub reports.
pub const TERMIX_APPIMAGES: [Receipt; 2] = [
    Receipt {
        tag: "v26.11.2",
        name: "termix_linux_x64_appimage.AppImage",
        size: 263_185_084,
        sha256: "a002a84337140c2a6558c5a6f7d5167b9a9f31405dbf566a56cd0697d940996a",
    },
    Receipt {
        tag: "v26.11.3",
        name: "termix_linux_x64_appimage.AppImage",
        size: 269_939_874,
        sha256: "0d8066fe07cbc64f4682bfd037329aaa0bbdfc076461f982502855528a1df926",
    },
];

/// How a self-contained app folder in a `.tar.xz` is run and what of it belongs to the user.
pub struct ArchiveLayout {
    /// Script or binary at the top of the unpacked folder that starts the app.
    pub entry: &'static str,
    /// Files inside the folder that hold the user's settings: carried into an update and left
    /// behind by a removal. Paths are relative to the folder and never contain `..`.
    pub keep: &'static [&'static str],
}

pub const UMI_OCR_LAYOUT: ArchiveLayout = ArchiveLayout { entry: "umi-ocr.sh", keep: &["UmiOCR-data/.settings", "UmiOCR-data/.pre_settings"] };

/// The layout of a reviewed archive recipe, by catalog id.
pub fn archive_layout(id: &str) -> Option<&'static ArchiveLayout> {
    (id == crate::curated::UMI_OCR.id).then_some(&UMI_OCR_LAYOUT)
}

/// Debian packages that carry the libraries the bundled Qt xcb plugin needs on Ubuntu 24.04.
/// Found with `ldd` on `libqxcb.so` (see the preflight in the lot's `umi-ocr` folder).
pub const UMI_OCR_UBUNTU_PACKAGES: [&str; 7] =
    ["libxcb-icccm4", "libxcb-image0", "libxcb-keysyms1", "libxcb-render-util0", "libxcb-xinerama0", "libxcb-xkb1", "libxkbcommon-x11-0"];

fn reviewed(app: &crate::CraftApp, receipts: &'static [Receipt], release: &Release, asset: &Asset) -> bool {
    let Some(version) = release.version.as_ref() else {
        return false;
    };
    let Some(receipt) = receipts.iter().find(|receipt| receipt.tag == release.tag) else {
        return false;
    };
    !release.prerelease
        && version.pre.is_empty()
        && version.build.is_empty()
        && release_tag_version(app.repo, &release.tag).as_ref() == Some(version)
        && asset.name == receipt.name
        && asset.url == format!("{}{}/{}", download_prefix(app.repo), release.tag, receipt.name)
        && asset.size == receipt.size
        && is_hex64(receipt.sha256)
        && asset.sha256.as_deref() == Some(receipt.sha256)
}

/// Whether `asset` is the reviewed Umi-OCR Linux archive of `release`.
pub fn umi_ocr_asset(release: &Release, asset: &Asset) -> bool {
    reviewed(&crate::curated::UMI_OCR, &UMI_OCR_ARCHIVES, release, asset)
}

/// Whether `asset` is the reviewed Termix Linux x64 AppImage of `release`.
pub fn termix_appimage_asset(release: &Release, asset: &Asset) -> bool {
    reviewed(&crate::curated::TERMIX, &TERMIX_APPIMAGES, release, asset)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Os, Platform};

    fn release(app: &crate::CraftApp, receipt: &Receipt) -> (Release, Asset) {
        let asset = Asset {
            name: receipt.name.into(),
            size: receipt.size,
            url: format!("{}{}/{}", download_prefix(app.repo), receipt.tag, receipt.name),
            sha256: Some(receipt.sha256.into()),
        };
        let release = Release {
            tag: receipt.tag.into(),
            version: release_tag_version(app.repo, receipt.tag),
            assets: vec![asset.clone()],
            ..Release::default()
        };
        (release, asset)
    }

    fn catalog(id: &str) -> &'static crate::curated::App {
        crate::curated::apps().iter().find(|app| app.id == id).unwrap_or_else(|| panic!("{id} is not in the catalog"))
    }

    #[test]
    fn every_candidate_is_a_catalogued_app_with_a_matching_identity() {
        for candidate in &FLATPAK {
            let app = catalog(candidate.id);
            assert_eq!(app.repo, candidate.upstream, "{}", candidate.id);
            assert_eq!(flatpak_candidate(candidate.id, &app.repo), Some(candidate.flatpak));
            assert_eq!(flatpak_candidate(candidate.id, "https://github.com/someone-else/fork"), None);
            assert!(app.stars.as_ref().is_some_and(|stars| stars.count > 1_000), "{} needs more than 1000 stars", candidate.id);
        }
        for id in [crate::curated::UMI_OCR.id, crate::curated::TERMIX.id] {
            assert!(catalog(id).stars.as_ref().is_some_and(|stars| stars.count > 1_000), "{id}");
        }
        let ids: std::collections::BTreeSet<_> = FLATPAK.iter().map(|candidate| candidate.id).collect();
        let flatpaks: std::collections::BTreeSet<_> = FLATPAK.iter().map(|candidate| candidate.flatpak).collect();
        assert_eq!((ids.len(), flatpaks.len()), (FLATPAK.len(), FLATPAK.len()));
    }

    #[test]
    fn candidates_stay_out_of_every_live_table() {
        let linux = Platform { os: Os::Linux, arch: crate::Arch::X64 };
        for candidate in &FLATPAK {
            assert!(!crate::flathub::supported(candidate.id, linux), "{} is live", candidate.id);
            assert!(!crate::flathub::RECIPES.iter().any(|(id, flatpak)| *id == candidate.id || *flatpak == candidate.flatpak));
        }
        for app in [&crate::curated::UMI_OCR, &crate::curated::TERMIX] {
            assert!(crate::release::qualified_linux_appimage_recipe(&[app.id]).is_none(), "{} has a live AppImage recipe", app.id);
            assert!(!crate::curated::verified_linux_appimage(app, linux), "{} is verified on Linux", app.id);
        }
        assert!(!crate::curated::NATIVE_APPS.iter().any(|app| app.id == crate::curated::UMI_OCR.id));
        assert!(!linux.default_kinds().contains(&crate::PackageKind::TarXz));
        assert!(!linux.download_kinds().contains(&crate::PackageKind::TarXz));
    }

    #[test]
    fn the_manifest_gains_nothing_before_the_lifecycle_proof() {
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../../../apps/shiftanything-tauri-proof/verified-platforms.json")).unwrap();
        let linux: Vec<&str> = manifest["linux"]["x64"].as_array().unwrap().iter().filter_map(|id| id.as_str()).collect();
        let mut waiting: Vec<&str> = FLATPAK.iter().map(|candidate| candidate.id).collect();
        waiting.extend([crate::curated::UMI_OCR.id, crate::curated::TERMIX.id]);
        for id in waiting {
            assert!(!linux.contains(&id), "{id} is in the Linux manifest: move it out of pending.rs in the same commit as its proof");
        }
    }

    #[test]
    fn receipts_are_exact_and_belong_to_two_distinct_stable_releases() {
        for receipts in [&UMI_OCR_ARCHIVES, &TERMIX_APPIMAGES] {
            assert_ne!(receipts[0].tag, receipts[1].tag);
            assert_ne!(receipts[0].sha256, receipts[1].sha256);
            for receipt in receipts {
                assert!(is_hex64(receipt.sha256) && receipt.sha256 == receipt.sha256.to_ascii_lowercase());
                assert!(receipt.size > 100_000_000, "{}", receipt.name);
            }
        }
    }

    #[test]
    fn only_the_exact_reviewed_files_pass_the_asset_gates() {
        for (app, receipts, gate) in [
            (&crate::curated::UMI_OCR, &UMI_OCR_ARCHIVES, umi_ocr_asset as fn(&Release, &Asset) -> bool),
            (&crate::curated::TERMIX, &TERMIX_APPIMAGES, termix_appimage_asset),
        ] {
            for receipt in receipts {
                let (release, asset) = release(app, receipt);
                assert!(gate(&release, &asset), "{} should pass", receipt.name);
                let tampered = [
                    Asset { size: asset.size + 1, ..asset.clone() },
                    Asset { sha256: Some("0".repeat(64)), ..asset.clone() },
                    Asset { sha256: None, ..asset.clone() },
                    Asset { name: format!("other-{}", asset.name), ..asset.clone() },
                    Asset { url: asset.url.replace(app.repo, "someone-else/fork"), ..asset.clone() },
                ];
                for bad in tampered {
                    assert!(!gate(&release, &bad), "{bad:?} must fail");
                }
                assert!(!gate(&Release { prerelease: true, ..release.clone() }, &asset));
                assert!(!gate(&Release { tag: format!("{}-rc1", release.tag), ..release.clone() }, &asset));
            }
            // A release whose file is correct but whose tag has no receipt is refused.
            let (mut unknown, asset) = release(app, &receipts[1]);
            unknown.tag = "v0.0.1".into();
            unknown.version = release_tag_version(app.repo, "v0.0.1");
            assert!(!gate(&unknown, &asset));
        }
    }

    #[test]
    fn the_archive_layout_is_bound_to_umi_ocr_and_its_user_files_stay_inside_the_folder() {
        assert!(archive_layout("umi-ocr").is_some());
        assert!(archive_layout("termix").is_none());
        let layout = &UMI_OCR_LAYOUT;
        assert!(!layout.entry.contains('/'));
        assert!(!layout.keep.is_empty());
        for path in layout.keep {
            let path = std::path::Path::new(path);
            assert!(path.is_relative() && path.components().all(|part| matches!(part, std::path::Component::Normal(_))), "{path:?}");
        }
    }
}
