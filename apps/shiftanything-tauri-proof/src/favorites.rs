//! Profile-local favorites. No installer or backend command is reachable from this store.
use std::{collections::BTreeSet, io::Read, path::PathBuf};

pub struct Favorites {
    path: PathBuf,
    ids: BTreeSet<String>,
}

impl Favorites {
    pub fn load(profile: &std::path::Path) -> Result<Self, String> {
        let path = profile.join("favorites.json");
        let ids = match std::fs::symlink_metadata(&path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => BTreeSet::new(),
            Err(error) => return Err(error.to_string()),
            Ok(metadata) => {
                if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 16_384 {
                    return Err("Invalid favorites file".into());
                }
                let mut bytes = Vec::new();
                std::fs::File::open(&path)
                    .map_err(|error| error.to_string())?
                    .take(16_385)
                    .read_to_end(&mut bytes)
                    .map_err(|error| error.to_string())?;
                if bytes.len() > 16_384 {
                    return Err("Favorites file is too large".into());
                }
                let ids: BTreeSet<String> = serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
                ids.into_iter().filter(|id| known(id)).collect()
            }
        };
        Ok(Self { path, ids })
    }

    pub fn ids(&self) -> &BTreeSet<String> {
        &self.ids
    }

    pub fn set(&mut self, id: String, favorite: bool) -> Result<BTreeSet<String>, String> {
        if !known(&id) {
            return Err("Unknown curated app".into());
        }
        let mut ids = self.ids.clone();
        if favorite {
            ids.insert(id);
        } else {
            ids.remove(&id);
        }
        artcraft_launcher_native::paths::write_atomic(&self.path, &serde_json::to_vec(&ids).map_err(|error| error.to_string())?)?;
        self.ids = ids;
        Ok(self.ids.clone())
    }
}

fn known(id: &str) -> bool {
    artcraft_launcher_core::curated::apps().iter().any(|app| app.id == id)
}

#[cfg(test)]
mod tests {
    #[test]
    fn favorites_survive_restart_and_remain_in_their_profile() -> Result<(), String> {
        let first = tempfile::tempdir().map_err(|error| error.to_string())?;
        let second = tempfile::tempdir().map_err(|error| error.to_string())?;
        let mut store = super::Favorites::load(first.path())?;
        store.set("handy".into(), true)?;
        assert!(super::Favorites::load(first.path())?.ids().contains("handy"));
        assert!(super::Favorites::load(second.path())?.ids().is_empty());
        assert!(store.set("unknown-app".into(), true).is_err());
        assert_eq!(super::Favorites::load(first.path())?.ids().len(), 1);
        store.set("handy".into(), false)?;
        assert!(super::Favorites::load(first.path())?.ids().is_empty());
        Ok(())
    }
}
