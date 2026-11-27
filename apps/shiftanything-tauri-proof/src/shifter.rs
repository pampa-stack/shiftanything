//! Tauri's read-only bridge to the shared comparison engine and its reviewed sources.
use artcraft_launcher_core::{curated, shifter};
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;

#[derive(Serialize)]
pub(crate) struct Workspace {
    dataset: serde_json::Value,
    apps: Vec<CatalogEntry>,
}

#[derive(Serialize)]
struct CatalogEntry {
    id: &'static str,
    name: &'static str,
    alternatives: &'static [String],
}

#[tauri::command]
pub(crate) fn shifter_workspace() -> Result<Workspace, String> {
    let dataset = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../crates/core/shifter.json")))
        .map_err(|error| error.to_string())?;
    let apps = curated::apps().iter().map(|app| CatalogEntry { id: &app.id, name: &app.name, alternatives: &app.alternatives }).collect();
    Ok(Workspace { dataset, apps })
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub(crate) enum Comparison {
    Profile { profile_id: String },
    Manual { app_ids: Vec<String> },
}

#[tauri::command]
pub(crate) fn compare_shifter(comparison: Comparison, selected: Vec<String>, required: Vec<String>) -> Result<Vec<shifter::Score>, String> {
    const MAX_FEATURES: usize = 100;
    const MAX_ID_BYTES: usize = 140;
    let data = shifter::data()?;
    if selected.len() > MAX_FEATURES || required.len() > selected.len() || selected.iter().chain(&required).any(|id| id.len() > MAX_ID_BYTES) {
        return Err("Invalid feature selection".into());
    }
    match comparison {
        Comparison::Profile { profile_id } => {
            data.profiles.iter().find(|profile| profile.id == profile_id).ok_or("Unknown profile")?.compare(&selected, &required)
        }
        Comparison::Manual { app_ids } => {
            if app_ids.len() > curated::apps().len() || app_ids.iter().any(|id| id.len() > MAX_ID_BYTES) {
                return Err("Invalid app selection".into());
            }
            for id in &selected {
                let custom = id.strip_prefix("custom:").is_some_and(|suffix| suffix.parse::<u32>().is_ok());
                if !custom && data.feature(id).is_none() {
                    return Err("Unknown manual feature".into());
                }
            }
            data.compare_manual(&app_ids, &selected, &required)
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Source {
    Baseline { feature_id: String },
    Evidence { app_id: String, feature_id: String },
}

#[tauri::command]
pub(crate) fn open_shifter_source(source: Source) -> Result<(), String> {
    dispatch_source(source, artcraft_launcher_native::process::open)
}

fn dispatch_source(source: Source, open: impl FnOnce(&OsStr) -> Result<(), String>) -> Result<(), String> {
    let data = shifter::data()?;
    let url = match source {
        Source::Baseline { feature_id } => data.feature(&feature_id).map(|(_, feature)| feature.source.as_str()),
        Source::Evidence { app_id, feature_id } => data.evidence(&app_id, &feature_id).map(|evidence| evidence.source.as_str()),
    }
    .ok_or("This feature has no documented source")?;
    let parsed = tauri::Url::parse(url).map_err(|_| "Invalid documentation URL")?;
    if parsed.scheme() != "https" || parsed.host_str().is_none() || !parsed.username().is_empty() || parsed.password().is_some() {
        return Err("Documentation must use HTTPS".into());
    }
    open(OsStr::new(url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_profile_and_reuses_only_qualified_manual_evidence() -> Result<(), String> {
        let profile = Comparison::Profile { profile_id: "photoshop".into() };
        let selected = ["layers", "reversible", "psd", "genfill"].map(String::from);
        let scores = compare_shifter(profile, selected.to_vec(), vec!["genfill".into()])?;
        let photocraft = scores.iter().find(|score| score.app_id == "photocraft").ok_or("PhotoCraft missing")?;
        assert_eq!(photocraft.coverage, Some(75));
        assert_eq!(photocraft.missing_required, ["genfill"]);

        let manual = Comparison::Manual { app_ids: vec!["photocraft".into()] };
        let selected = vec!["photoshop:layers".into(), "custom:0".into()];
        let scores = compare_shifter(manual, selected, vec!["custom:0".into()])?;
        assert_eq!(scores[0].coverage, Some(50));
        assert_eq!(scores[0].unverified, 1);
        assert_eq!(scores[0].missing_required, ["custom:0"]);
        Ok(())
    }

    #[test]
    fn sources_resolve_only_documented_ids() -> Result<(), String> {
        let mut opened = None;
        dispatch_source(Source::Evidence { app_id: "photocraft".into(), feature_id: "photoshop:layers".into() }, |url| {
            opened = Some(url.to_owned());
            Ok(())
        })?;
        let expected = shifter::data()?.evidence("photocraft", "photoshop:layers").ok_or("Missing fixture evidence")?;
        assert_eq!(opened.as_deref(), Some(OsStr::new(&expected.source)));
        assert!(dispatch_source(Source::Baseline { feature_id: "file:///tmp/private".into() }, |_| Ok(())).is_err());
        assert!(dispatch_source(Source::Evidence { app_id: "gimp".into(), feature_id: "custom:0".into() }, |_| Ok(())).is_err());
        Ok(())
    }
}
