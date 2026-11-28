//! The pictures on an app's details page. They are not part of the launcher: opening a page
//! fetches them once and keeps them in the cache folder, so the next time (and any time offline)
//! they come from disk.

#[path = "catalog_media.rs"]
mod catalog_media;

use std::path::Path;

use artcraft_launcher_core::details::{AppDetails, MAX_SCREENSHOT_BYTES, looks_like_jpeg, screenshot_within_budget};

use crate::http::Http;
use crate::paths::{Paths, write_atomic};

/// The picture saved at `saved`, if there is one and it's still a JPEG of a sane size. A damaged or
/// swapped file is ignored, and the picture is fetched again over it.
fn saved_copy(saved: &Path) -> Option<Vec<u8>> {
    if std::fs::metadata(saved).ok()?.len() > MAX_SCREENSHOT_BYTES as u64 {
        return None;
    }
    std::fs::read(saved).ok().filter(|bytes| looks_like_jpeg(bytes) && screenshot_within_budget(bytes))
}

/// Why a picture couldn't be had.
#[derive(Debug, PartialEq, Eq)]
enum Failure {
    /// This picture only: it isn't published, or what came back isn't a picture.
    Picture(String),
    /// The network or the server: the other pictures would fail the same way.
    Network(String),
}

/// Downloads the picture at `url` and saves it at `saved`. A saved file that can't be written is
/// only a missed cache: the picture is still shown.
fn download(http: &Http, url: &str, saved: &Path) -> Result<Vec<u8>, Failure> {
    let bytes = match http.get_bytes(url, MAX_SCREENSHOT_BYTES) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => return Err(Failure::Picture("this picture isn't published".into())),
        Err(why) => return Err(Failure::Network(why)),
    };
    // A captive portal's login page or a proxy's error page arrives as a 200: don't keep it.
    if !looks_like_jpeg(&bytes) {
        return Err(Failure::Picture("the answer isn't a JPEG picture".into()));
    }
    // A small JPEG can declare enormous dimensions; the image loader allocates from the header
    // before decoding, so refuse anything outside the pixel budget before it is shown or saved.
    if !screenshot_within_budget(&bytes) {
        return Err(Failure::Picture("the picture's dimensions are too large".into()));
    }
    if let Err(e) = write_atomic(saved, &bytes) {
        log::warn!("couldn't keep {}: {e}", saved.display());
    }
    Ok(bytes)
}

/// Every picture of `details`, lead first, handed to `emit` one by one as it is had. Saved copies
/// need no network. Once the network or the server has failed, the pictures still to fetch fail at
/// once with the same reason instead of each waiting for its own timeout; saved ones are still
/// served. A picture that is merely missing doesn't hold the others back.
pub fn fetch_all(http: Option<&Http>, paths: &Paths, details: &AppDetails, mut emit: impl FnMut(&'static str, Result<Vec<u8>, String>)) {
    let mut failed: Option<String> = None;
    for shot in details.shots {
        let saved = paths.screenshot(details.id, shot.key);
        if let Some(bytes) = saved_copy(&saved) {
            emit(shot.key, Ok(bytes));
            continue;
        }
        if let Some(why) = &failed {
            emit(shot.key, Err(why.clone()));
            continue;
        }
        let result = match (http, details.screenshot_url(shot.key)) {
            (Some(http), Some(url)) => download(http, &url, &saved),
            (None, _) => Err(Failure::Network("HTTPS isn't available".into())),
            (_, None) => Err(Failure::Picture("this page has no such picture".into())),
        };
        let result = result.map_err(|failure| {
            let why = match failure {
                Failure::Network(why) => {
                    failed = Some(why.clone());
                    why
                }
                Failure::Picture(why) => why,
            };
            log::warn!("picture {}/{} unavailable: {why}", details.id, shot.key);
            why
        });
        emit(shot.key, result);
    }
}

/// Reads individual curated media slots through the existing backend HTTPS and profile cache.
#[derive(Clone)]
pub struct CatalogMedia {
    http: Option<Http>,
    paths: Paths,
}

impl CatalogMedia {
    pub(crate) fn new(http: Option<Http>, paths: Paths) -> Self {
        Self { http, paths }
    }

    /// A missing index requests the logo. Slots and URLs always come from the curated catalog.
    pub fn fetch(&self, app_id: &str, screenshot: Option<usize>) -> Result<Vec<u8>, String> {
        let entry =
            artcraft_launcher_core::curated::apps().iter().find(|entry| entry.id == app_id).ok_or_else(|| "Unknown catalog app".to_string())?;
        let (key, url) = match screenshot {
            Some(index) => {
                let url = entry.screenshots.get(index).ok_or_else(|| "Unknown catalog screenshot".to_string())?;
                (format!("catalog-shot-{index}"), url.as_str())
            }
            None => ("catalog-logo".to_string(), entry.logo.as_str()),
        };
        load_catalog_picture(self.http.as_ref(), &self.paths, app_id, &key, url)
    }
}

/// Uses the same bounded native HTTPS/cache path for frontends that request whole galleries.
pub fn fetch_catalog(
    http: Option<&Http>,
    paths: &Paths,
    app: &artcraft_launcher_core::curated::App,
    mut emit: impl FnMut(&str, Result<Vec<u8>, String>),
) {
    let media = std::iter::once(("catalog-logo".to_string(), app.logo.as_str()))
        .chain(app.screenshots.iter().enumerate().map(|(i, url)| (format!("catalog-shot-{i}"), url.as_str())));
    for (key, url) in media {
        emit(&key, load_catalog_picture(http, paths, &app.id, &key, url));
    }
}

fn load_catalog_picture(http: Option<&Http>, paths: &Paths, app_id: &str, key: &str, url: &str) -> Result<Vec<u8>, String> {
    use sha2::{Digest, Sha256};
    use std::io::Read;
    let saved = paths.cache.join("curated-media").join(format!("{:x}", Sha256::digest(url.as_bytes())));
    let valid = |bytes: &[u8]| bytes.len() <= MAX_SCREENSHOT_BYTES && artcraft_launcher_core::curated::media_within_budget(bytes);
    if std::fs::metadata(&saved).is_ok_and(|metadata| metadata.is_file() && metadata.len() <= MAX_SCREENSHOT_BYTES as u64)
        && let Ok(file) = std::fs::File::open(&saved)
    {
        let mut bytes = Vec::new();
        if file.take(MAX_SCREENSHOT_BYTES as u64 + 1).read_to_end(&mut bytes).is_ok() && valid(&bytes) {
            return Ok(bytes);
        }
    }
    if let Some(bytes) = catalog_media::fallback(app_id, key, url).filter(|bytes| valid(bytes)) {
        let _ = write_atomic(&saved, &bytes);
        return Ok(bytes);
    }
    let allowed = [
        "https://raw.githubusercontent.com/",
        "https://getartcraft.com/",
        "https://dl.flathub.org/",
        "https://paseo.sh/",
        "https://docs.freeplane.org/images/",
        "https://deskflow.org/screenshots/",
        "https://docs.langflow.org/assets/images/quickstart-simple-agent-",
        "https://docs.syncthing.net/_images/gs",
        "https://jellyfin.org/assets/images/setup-wizard-4-media-popup-",
        "https://cdn.mac-stats.com/assets/",
        "https://www.home-assistant.io/images/blog/2026-02/new-overview.png",
        "https://uptime.kuma.pet/",
        "https://getsharex.com/",
        "https://media.docs.plane.so/",
        "https://github.com/",
        "https://static-site.onlyoffice.com/",
        "https://b3log.org/",
        "https://activitywatch.net/",
        "https://anythingllm.com/",
        "https://docmost.com/",
        "https://files.ollama.com/",
        "https://files.sumatrapdfreader.org/",
        "https://iina.io/",
        "https://nickemanarin.github.io/",
        "https://pdfsam.org/",
        "https://www.librechat.ai/",
        "https://www.naps2.com/",
    ];
    let reviewed_pictures = [
        "https://a-us.storyblok.com/f/1022730/1999x1020/36a8151d38/embed-dashboards_sample-dashboard.png",
        "https://umami.is/_next/static/immutable/media/app.06ov74hy3esf9.jpg",
        "https://rectangleapp.com/assets/images/shortcutScreenshot.jpg",
        "https://rectangleapp.com/assets/images/snapAreaScreenshot.jpg",
        "https://icemenubar.app/gallery/menu-bar-layout.png?ts=1737648866",
        "https://icemenubar.app/gallery/ice-bar.png?ts=1737648866",
        "https://maccy.app/img/maccy/Instructions.png",
        "https://iterm2.com/img/screenshots/split_panes.png",
        "https://iterm2.com/img/screenshots/find.png",
        "https://docs.github.com/assets/cb-66158/images/help/desktop/diff-selection.png",
        "https://docs.github.com/assets/cb-16555/images/help/desktop/commit-all.png",
        "https://guacamole.apache.org/doc/1.6.0/gug/_images/guacamole-home-screen.png",
        "https://guacamole.apache.org/doc/1.6.0/gug/_images/guacamole-client-interface.png",
    ];
    if !reviewed_pictures.contains(&url) && !allowed.iter().any(|base| url.starts_with(base)) {
        return Err("Unapproved catalog media origin".into());
    }
    let bytes = http
        .ok_or_else(|| "HTTPS isn't available".to_string())?
        .get_bytes(url, MAX_SCREENSHOT_BYTES)?
        .ok_or_else(|| "Catalog image unavailable".to_string())?;
    if !valid(&bytes) {
        return Err("Unsupported image or image exceeds dimensions budget".into());
    }
    let _ = write_atomic(&saved, &bytes);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::tests::{Reply, serve};

    /// A body that reads as a valid, in-budget JPEG (SOI · APP0 · SOF0 declaring 1200×750), padded
    /// to a realistic size. Its address on the test server is returned alongside where used.
    fn jpeg() -> Vec<u8> {
        let mut bytes = vec![0xFF, 0xD8]; // SOI
        bytes.extend_from_slice(&[0xFF, 0xE0, 0x00, 0x04, 0x00, 0x00]); // APP0, length 4
        bytes.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x11, 0x08]); // SOF0, length 17, precision 8
        bytes.extend_from_slice(&750u16.to_be_bytes()); // height
        bytes.extend_from_slice(&1200u16.to_be_bytes()); // width
        bytes.extend((0..5_000u32).map(|i| (i % 251) as u8));
        bytes
    }

    /// A tiny JPEG whose SOF0 declares a pathological 65535×65535, to exercise the pixel budget.
    fn huge_jpeg() -> Vec<u8> {
        let mut bytes = vec![0xFF, 0xD8];
        bytes.extend_from_slice(&[0xFF, 0xC0, 0x00, 0x11, 0x08]);
        bytes.extend_from_slice(&65535u16.to_be_bytes());
        bytes.extend_from_slice(&65535u16.to_be_bytes());
        bytes.extend_from_slice(&[0x03, 0x01, 0x11, 0x00]);
        bytes
    }

    #[test]
    fn a_downloaded_picture_is_saved_where_it_will_be_read_from() {
        let dir = tempfile::tempdir().unwrap();
        let saved = dir.path().join("x").join("pic.jpg");
        let http = Http::new().unwrap();
        let (url, _) = serve(jpeg(), vec![Reply::Full]);
        assert_eq!(download(&http, &url, &saved).unwrap(), jpeg());
        assert_eq!(saved_copy(&saved), Some(jpeg()));
    }

    #[test]
    fn what_is_not_a_picture_is_not_kept() {
        let dir = tempfile::tempdir().unwrap();
        let saved = dir.path().join("pic.jpg");
        let http = Http::new().unwrap();
        let (url, _) = serve(b"<html>Sign in to the Wi-Fi</html>".to_vec(), vec![Reply::Full]);
        assert_eq!(download(&http, &url, &saved), Err(Failure::Picture("the answer isn't a JPEG picture".into())));
        assert!(!saved.exists());
        let (url, _) = serve(Vec::new(), vec![Reply::Missing]);
        assert_eq!(download(&http, &url, &saved), Err(Failure::Picture("this picture isn't published".into())));
        assert!(!saved.exists());
        // A server that can't be reached is the network's failure, not this picture's.
        let gone = std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        assert!(matches!(download(&http, &format!("http://{gone}/pic.jpg"), &saved), Err(Failure::Network(_))));
    }

    #[test]
    fn a_picture_declaring_enormous_dimensions_is_refused_and_not_saved() {
        let dir = tempfile::tempdir().unwrap();
        let saved = dir.path().join("pic.jpg");
        let http = Http::new().unwrap();
        let (url, _) = serve(huge_jpeg(), vec![Reply::Full]);
        assert_eq!(download(&http, &url, &saved), Err(Failure::Picture("the picture's dimensions are too large".into())));
        assert!(!saved.exists());
        // Even if one is already on disk (e.g. from an older build), it is ignored.
        std::fs::create_dir_all(saved.parent().unwrap()).unwrap();
        std::fs::write(&saved, huge_jpeg()).unwrap();
        assert_eq!(saved_copy(&saved), None, "an oversized saved copy is not trusted");
    }

    #[test]
    fn a_damaged_saved_copy_is_ignored() {
        let dir = tempfile::tempdir().unwrap();
        let saved = dir.path().join("pic.jpg");
        assert_eq!(saved_copy(&saved), None, "nothing saved");
        std::fs::write(&saved, b"<html>oops</html>").unwrap();
        assert_eq!(saved_copy(&saved), None, "not a JPEG");
        std::fs::write(&saved, vec![0xFF; MAX_SCREENSHOT_BYTES + 1]).unwrap();
        assert_eq!(saved_copy(&saved), None, "too big");
        std::fs::write(&saved, jpeg()).unwrap();
        assert_eq!(saved_copy(&saved), Some(jpeg()));
    }

    #[test]
    fn individual_catalog_slots_are_validated_and_share_the_existing_cache() {
        use sha2::{Digest, Sha256};
        let profile = tempfile::tempdir().unwrap();
        let paths = Paths::under(profile.path());
        let media = CatalogMedia::new(None, paths.clone());
        assert!(media.fetch("https://example.test/logo.png", None).is_err());
        assert!(media.fetch("localsend", Some(usize::MAX)).is_err());
        let app = artcraft_launcher_core::curated::apps().iter().find(|entry| entry.id == "localsend").unwrap();
        let cache = paths.cache.join("curated-media");
        let first = cache.join(format!("{:x}", Sha256::digest(app.screenshots[0].as_bytes())));
        write_atomic(&first, &jpeg()).unwrap();
        assert_eq!(media.fetch(&app.id, Some(0)).unwrap(), jpeg());
        assert_eq!(std::fs::read_dir(&cache).unwrap().count(), 1);
        assert!(media.fetch(&app.id, Some(1)).is_err());
        write_atomic(&first, &vec![0xff; MAX_SCREENSHOT_BYTES + 1]).unwrap();
        assert!(media.fetch(&app.id, Some(0)).is_err());
        assert!(media.fetch("wazuh", None).unwrap().starts_with(b"\x89PNG"));
    }

    #[test]
    fn saved_pictures_need_no_network_and_a_failure_ends_the_run() {
        let paths = Paths::under(tempfile::tempdir().unwrap().path());
        let page = AppDetails::by_id("photocraft").unwrap();
        // The second picture is saved already; the first and the others need the network, which is not there.
        let second = paths.screenshot(page.id, page.shots[1].key);
        write_atomic(&second, &jpeg()).unwrap();
        let mut seen = Vec::new();
        fetch_all(None, &paths, page, |key, result| seen.push((key, result)));
        let keys: Vec<_> = seen.iter().map(|(k, _)| *k).collect();
        assert_eq!(keys, page.shots.iter().map(|s| s.key).collect::<Vec<_>>(), "every picture is answered, in order");
        assert!(seen[0].1.is_err() && seen[2].1.is_err() && seen[3].1.is_err());
        assert_eq!(seen[1].1.as_ref().unwrap(), &jpeg());
    }
}
