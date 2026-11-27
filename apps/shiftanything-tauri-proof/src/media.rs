use artcraft_launcher_native::screenshots::CatalogMedia;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const MAX_WORKERS: usize = 3;

pub struct Media {
    catalog: CatalogMedia,
    active: Arc<AtomicUsize>,
}

impl Media {
    pub fn new(catalog: CatalogMedia) -> Self {
        Self { catalog, active: Arc::new(AtomicUsize::new(0)) }
    }
}

struct Permit(Arc<AtomicUsize>);

impl Permit {
    fn try_acquire(active: &Arc<AtomicUsize>) -> Result<Self, String> {
        active
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |count| (count < MAX_WORKERS).then_some(count + 1))
            .map_err(|_| "Catalog media workers are busy".to_string())?;
        Ok(Self(active.clone()))
    }
}

impl Drop for Permit {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
    }
}

/// The webview sends a curated app ID and optional screenshot index, never a URL or file path.
#[tauri::command]
pub async fn fetch_catalog_media(app_id: String, screenshot: Option<usize>, state: tauri::State<'_, Media>) -> Result<tauri::ipc::Response, String> {
    let permit = Permit::try_acquire(&state.active)?;
    let catalog = state.catalog.clone();
    let bytes = tauri::async_runtime::spawn_blocking(move || {
        let _permit = permit;
        catalog.fetch(&app_id, screenshot)
    })
    .await
    .map_err(|error| error.to_string())??;
    Ok(tauri::ipc::Response::new(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_finished_fetch_readmits_another_worker() -> Result<(), String> {
        let active = Arc::new(AtomicUsize::new(0));
        let mut workers = (0..MAX_WORKERS).map(|_| Permit::try_acquire(&active)).collect::<Result<Vec<_>, _>>()?;
        assert!(Permit::try_acquire(&active).is_err());
        drop(workers.pop());
        assert!(Permit::try_acquire(&active).is_ok());
        Ok(())
    }
}
