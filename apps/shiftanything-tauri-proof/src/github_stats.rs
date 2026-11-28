//! Public repository metrics. The webview never supplies a URL or credentials.
use artcraft_launcher_native::{http::Http, now, paths::write_atomic};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

const API: &str = "https://api.github.com/repos/pampa-stack/shiftanything";
const PAGE_SIZE: usize = 100;
const MAX_PAGES: usize = 10;
const TTL: i64 = 15 * 60;
const MAX_CACHE_BYTES: u64 = 1024;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
struct Counts {
    stars: u64,
    downloads: Option<u64>,
    observed_at: i64,
}

#[derive(Serialize)]
pub(crate) struct Snapshot {
    counts: Option<Counts>,
    stale: bool,
}

#[derive(Default, Deserialize, Serialize)]
struct Cache {
    counts: Option<Counts>,
    next_attempt: i64,
}

impl Cache {
    fn read(path: &Path) -> Option<Self> {
        let meta = fs::symlink_metadata(path).ok()?;
        if !meta.is_file() || meta.len() > MAX_CACHE_BYTES {
            return None;
        }
        let mut bytes = Vec::new();
        File::open(path).ok()?.take(MAX_CACHE_BYTES + 1).read_to_end(&mut bytes).ok()?;
        (bytes.len() as u64 <= MAX_CACHE_BYTES).then(|| serde_json::from_slice(&bytes).ok()).flatten()
    }

    fn snapshot(&self, timestamp: i64) -> Snapshot {
        Snapshot {
            counts: self.counts,
            stale: self.counts.is_some_and(|counts| timestamp < counts.observed_at || timestamp.saturating_sub(counts.observed_at) >= TTL),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Store {
    path: PathBuf,
    cache: Arc<Mutex<Cache>>,
}

impl Store {
    pub(crate) fn new(profile: &Path) -> Self {
        let path = profile.join("github-stats.json");
        let cache = Arc::new(Mutex::new(Cache::read(&path).unwrap_or_default()));
        Self { path, cache }
    }

    fn snapshot(&self) -> Result<Snapshot, String> {
        Ok(self.cache.lock().map_err(|error| error.to_string())?.snapshot(now()))
    }

    fn refresh(&self) -> Result<Snapshot, String> {
        let timestamp = now();
        {
            let mut cache = self.cache.lock().map_err(|error| error.to_string())?;
            if cache.next_attempt > timestamp && cache.next_attempt <= timestamp + TTL {
                return Ok(cache.snapshot(timestamp));
            }
            // Reserve the attempt before HTTP so a second webview call cannot start a duplicate.
            cache.next_attempt = timestamp + TTL;
        }
        let fresh = fetch();
        let timestamp = now();
        let mut cache = self.cache.lock().map_err(|error| error.to_string())?;
        if let Ok(counts) = fresh {
            cache.counts = Some(counts);
        }
        cache.next_attempt = timestamp + TTL;
        if let Ok(bytes) = serde_json::to_vec(&*cache) {
            let _ = write_atomic(&self.path, &bytes);
        }
        Ok(cache.snapshot(timestamp))
    }
}

#[tauri::command]
pub(crate) async fn project_stats(refresh: bool, state: tauri::State<'_, Store>) -> Result<Snapshot, String> {
    if !refresh {
        return state.snapshot();
    }
    let store = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || store.refresh()).await.map_err(|error| error.to_string())?
}

#[derive(Deserialize)]
struct Repository {
    stargazers_count: u64,
}

#[derive(Deserialize)]
struct Release {
    draft: bool,
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    id: u64,
    name: String,
    state: String,
    download_count: u64,
}

fn official_package(tag: &str, name: &str) -> bool {
    let Some(version) = tag.strip_prefix('v') else { return false };
    let Some(platform) =
        name.strip_prefix("ShiftAnything-").and_then(|suffix| suffix.strip_prefix(version)).and_then(|suffix| suffix.strip_prefix('-'))
    else {
        return false;
    };
    matches!(
        platform,
        "macos-arm64.dmg" | "macos-arm64.app.tar.gz" | "macos-x64.dmg" | "macos-x64.app.tar.gz" | "linux-amd64.deb" | "linux-arm64.deb"
    )
}

fn downloads(mut page: impl FnMut(usize) -> Result<Vec<Release>, String>) -> Result<Option<u64>, String> {
    let mut total = None::<u64>;
    let mut seen = HashSet::new();
    for number in 1..=MAX_PAGES {
        let releases = page(number)?;
        let complete = releases.len() < PAGE_SIZE;
        for release in releases.into_iter().filter(|release| !release.draft) {
            for asset in release.assets {
                if asset.state == "uploaded" && official_package(&release.tag_name, &asset.name) && seen.insert(asset.id) {
                    total = Some(total.unwrap_or(0).checked_add(asset.download_count).ok_or("Download count is too large")?);
                }
            }
        }
        if complete {
            return Ok(total);
        }
    }
    Err("Release download count exceeded the pagination limit".into())
}

fn fetch() -> Result<Counts, String> {
    // get_feed is unauthenticated and uses a bounded response body and request timeout.
    let http = Http::new()?;
    let get = |url: &str| http.get_feed(url).map_err(|error| error.why);
    let repository: Repository = serde_json::from_str(&get(API)?).map_err(|error| error.to_string())?;
    let downloads = downloads(|page| {
        let url = format!("{API}/releases?per_page={PAGE_SIZE}&page={page}");
        serde_json::from_str(&get(&url)?).map_err(|error| error.to_string())
    })?;
    Ok(Counts { stars: repository.stargazers_count, downloads, observed_at: now() })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn release(id: u64, package: &str, count: u64, draft: bool) -> Release {
        Release {
            draft,
            tag_name: "v0.20.5".into(),
            assets: vec![Asset { id, name: format!("ShiftAnything-0.20.5-{package}"), state: "uploaded".into(), download_count: count }],
        }
    }

    #[test]
    fn counts_only_published_application_packages_across_release_pages() -> Result<(), String> {
        let total = downloads(|number| {
            let mut page = vec![
                release(1, "macos-arm64.dmg", 13, false),
                release(2, "linux-amd64.deb.sig", 4, false),
                release(3, "linux-amd64.deb", 5, true),
                release(4, "src.tar.gz", 9, false),
            ];
            if number == 1 {
                page.resize_with(PAGE_SIZE, || release(5, "SHA256SUMS.txt", 10, false));
            } else {
                page.extend([release(6, "macos-arm64.app.tar.gz", 40, false), release(7, "linux-amd64.deb", 21, false)]);
            }
            Ok(page)
        })?;
        assert_eq!(total, Some(74));
        assert_eq!(downloads(|_| Ok(Vec::new()))?, None);
        assert_eq!(downloads(|_| Ok(vec![release(1, "macos-arm64.dmg", 0, false)]))?, Some(0));
        Ok(())
    }

    #[test]
    fn does_not_publish_a_partial_total_from_an_interrupted_fetch() {
        let result = downloads(|number| {
            if number == 1 {
                Ok((0..PAGE_SIZE).map(|id| release(id as u64, "macos-arm64.dmg", 1, false)).collect())
            } else {
                Err("rate limited".into())
            }
        });
        assert!(result.is_err());
    }

    #[test]
    fn stale_counts_remain_available_and_file_cache_is_profile_local() -> Result<(), Box<dyn std::error::Error>> {
        let profile = tempfile::tempdir()?;
        let cache = Cache { counts: Some(Counts { stars: 9, downloads: Some(36), observed_at: 20 }), next_attempt: 0 };
        fs::write(profile.path().join("github-stats.json"), serde_json::to_vec(&cache)?)?;
        let store = Store::new(profile.path());
        let snapshot = store.snapshot()?;
        assert!(snapshot.stale);
        assert_eq!(snapshot.counts.map(|counts| counts.stars), Some(9));
        assert!(Store::new(tempfile::tempdir()?.path()).snapshot()?.counts.is_none());
        Ok(())
    }
}
