//! Curated workflows and survey samples. These descriptions grant no native capability.
use crate::shifter::Text;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Debug, Deserialize)]
pub struct Dataset {
    pub checked_on: String,
    pub kits: Vec<Kit>,
    pub rankings: Vec<Ranking>,
}
#[derive(Debug, Deserialize)]
pub struct Kit {
    pub id: String,
    pub name: Text,
    pub description: Text,
    pub steps: Vec<Step>,
}
#[derive(Debug, Deserialize)]
pub struct Step {
    pub id: String,
    pub name: Text,
    pub paid: Vec<String>,
    pub compare: Option<String>,
    pub options: Vec<OptionApp>,
}
#[derive(Debug, Deserialize)]
pub struct OptionApp {
    pub app: String,
    pub r#use: Text,
    pub source: String,
}
#[derive(Debug, Deserialize)]
pub struct Ranking {
    pub id: String,
    pub name: Text,
    pub metric: Text,
    pub scope: Text,
    pub source: String,
    pub publisher: String,
    pub entries: Vec<RankingEntry>,
}
#[derive(Debug, Deserialize)]
pub struct RankingEntry {
    pub paid: String,
    pub value: f64,
    pub apps: Vec<String>,
    pub compare: Option<String>,
}
pub fn data() -> Result<&'static Dataset, &'static str> {
    static DATA: OnceLock<Result<Dataset, String>> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("../kit-di-lavoro.json")).map_err(|error| error.to_string()))
        .as_ref()
        .map_err(String::as_str)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn kits_only_offer_catalog_apps_and_documented_comparisons() {
        let catalog: BTreeSet<_> = crate::curated::apps().iter().map(|app| app.id.as_str()).collect();
        let profiles: BTreeSet<_> = crate::shifter::data().unwrap().profiles.iter().map(|profile| profile.id.as_str()).collect();
        let mut kit_ids = BTreeSet::new();
        for kit in &data().unwrap().kits {
            assert!(kit_ids.insert(&kit.id), "duplicate kit {}", kit.id);
            assert!(!kit.steps.is_empty(), "empty kit {}", kit.id);
            let mut steps = BTreeSet::new();
            for step in &kit.steps {
                assert!(steps.insert(&step.id), "duplicate step {}/{}", kit.id, step.id);
                assert!(!step.paid.is_empty() && !step.options.is_empty(), "empty step {}/{}", kit.id, step.id);
                assert!(step.compare.as_deref().is_none_or(|profile| profiles.contains(profile)));
                let mut apps = BTreeSet::new();
                for option in &step.options {
                    assert!(apps.insert(&option.app) && catalog.contains(option.app.as_str()), "invalid option {}", option.app);
                    assert!(option.source.starts_with("https://"), "missing source for {}", option.app);
                }
            }
        }
    }
}
