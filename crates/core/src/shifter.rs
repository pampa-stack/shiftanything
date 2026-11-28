//! Evidence-based comparisons across the catalog.
use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

use crate::curated;

#[derive(Debug, Deserialize)]
pub struct Dataset {
    pub schema_version: u32,
    pub checked_on: String,
    pub profiles: Vec<Profile>,
    #[serde(default)]
    pub suggested_software: Vec<String>,
}
#[derive(Debug, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub features: Vec<Feature>,
    pub candidates: Vec<Candidate>,
}
#[derive(Debug, Deserialize)]
pub struct Feature {
    pub id: String,
    pub name: Text,
    pub source: String,
}
#[derive(Debug, Deserialize)]
pub struct Text {
    pub it: String,
    pub en: String,
}
impl Text {
    pub fn in_language(&self, language: &str) -> &str {
        if language == "it" { &self.it } else { &self.en }
    }
}
#[derive(Debug, Deserialize)]
pub struct Candidate {
    pub app_id: String,
    pub evidence: BTreeMap<String, Evidence>,
}
#[derive(Debug, Deserialize)]
pub struct Evidence {
    pub state: State,
    pub source: String,
    pub note: Text,
}
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Supported,
    Partial,
    Unsupported,
    #[default]
    Unverified,
}
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct Score {
    pub app_id: String,
    pub supported: usize,
    pub partial: usize,
    pub unsupported: usize,
    pub unverified: usize,
    pub coverage: Option<usize>,
    pub missing_required: Vec<String>,
}

impl Score {
    pub fn has_positive_coverage(&self) -> bool {
        self.coverage.is_some_and(|coverage| coverage > 0)
    }
}

pub fn data() -> Result<&'static Dataset, &'static str> {
    static DATA: OnceLock<Result<Dataset, String>> = OnceLock::new();
    DATA.get_or_init(|| serde_json::from_str(include_str!("../shifter.json")).map_err(|error| error.to_string())).as_ref().map_err(String::as_str)
}

/// Ignore case, accents, spacing and literal trademark signs, not unknown product names.
pub fn normalize(value: &str) -> String {
    value.replace(['™', '®', '℠'], "").nfkd().filter(|c| !is_combining_mark(*c) && c.is_alphanumeric()).flat_map(char::to_lowercase).collect()
}
impl Profile {
    pub fn matches(&self, query: &str) -> bool {
        let query = normalize(query);
        !query.is_empty() && std::iter::once(&self.name).chain(&self.aliases).any(|name| normalize(name).contains(&query))
    }
    /// True when the app has confirmed support for at least one documented feature.
    pub fn confirms(&self, app_id: &str) -> bool {
        self.candidates
            .iter()
            .any(|candidate| candidate.app_id == app_id && candidate.evidence.values().any(|evidence| evidence.state == State::Supported))
    }
    pub fn evidence(&self, app_id: &str, feature: &str) -> Option<&Evidence> {
        self.candidates.iter().find(|candidate| candidate.app_id == app_id)?.evidence.get(feature)
    }
    pub fn compare(&self, selected: &[String], required: &[String]) -> Result<Vec<Score>, String> {
        let allowed: BTreeSet<_> = self.features.iter().map(|feature| feature.id.as_str()).collect();
        let selected_set: BTreeSet<_> = selected.iter().map(String::as_str).collect();
        let required_set: BTreeSet<_> = required.iter().map(String::as_str).collect();
        if selected_set.len() != selected.len() || required_set.len() != required.len() {
            return Err("Feature IDs must be unique".into());
        }
        if !selected_set.is_subset(&allowed) || !required_set.is_subset(&selected_set) {
            return Err("Unknown feature or required feature outside the selection".into());
        }
        let mut scores: Vec<_> = curated::apps().iter().map(|app| self.score(&app.id, selected, required)).collect();
        sort_scores(&mut scores);
        Ok(scores)
    }
    fn score(&self, app_id: &str, selected: &[String], required: &[String]) -> Score {
        score_evidence(app_id, selected, required, |feature| self.evidence(app_id, feature).map(|evidence| evidence.state))
    }
}

fn score_evidence(app_id: &str, selected: &[String], required: &[String], evidence: impl Fn(&str) -> Option<State>) -> Score {
    let mut score =
        Score { app_id: app_id.into(), supported: 0, partial: 0, unsupported: 0, unverified: 0, coverage: None, missing_required: vec![] };
    for feature in selected {
        match evidence(feature).unwrap_or_default() {
            State::Supported => score.supported += 1,
            State::Partial => score.partial += 1,
            State::Unsupported => score.unsupported += 1,
            State::Unverified => score.unverified += 1,
        }
    }
    if !selected.is_empty() && score.unverified < selected.len() {
        // Integer rounding half up; partial and missing evidence remain in the denominator.
        score.coverage = Some((200 * score.supported + selected.len()) / (2 * selected.len()));
    }
    score.missing_required = required.iter().filter(|feature| evidence(feature) != Some(State::Supported)).cloned().collect();
    score
}
fn catalog_sort_names() -> &'static BTreeMap<&'static str, String> {
    static NAMES: OnceLock<BTreeMap<&'static str, String>> = OnceLock::new();
    NAMES.get_or_init(|| curated::apps().iter().map(|app| (app.id.as_str(), normalize(&app.name))).collect())
}

fn sort_scores(scores: &mut [Score]) {
    scores.sort_by_key(|score| {
        let (id, name) = catalog_sort_names().get_key_value(score.app_id.as_str()).map_or(("", ""), |(&id, name)| (id, name.as_str()));
        (score.missing_required.len(), std::cmp::Reverse(score.coverage), name, id)
    });
}
pub struct Software<'a> {
    pub name: &'a str,
    pub profile: Option<&'a Profile>,
}
impl Dataset {
    /// Suggestions only discover software. Only explicit profile selection supplies evidence.
    pub fn software(&self, query: &str) -> Vec<Software<'_>> {
        let query = normalize(query);
        let mut names = BTreeMap::new();
        for name in &self.suggested_software {
            names.insert(normalize(name), Software { name, profile: None });
        }
        for profile in &self.profiles {
            names.insert(normalize(&profile.name), Software { name: &profile.name, profile: Some(profile) });
        }
        names
            .into_values()
            .filter(|software| {
                query.is_empty() || normalize(software.name).contains(&query) || software.profile.is_some_and(|profile| profile.matches(&query))
            })
            .collect()
    }

    /// Exact product match on a name or alias, for links from curated lists such as the work kits.
    pub fn profile_named(&self, name: &str) -> Option<&Profile> {
        let name = normalize(name);
        self.profiles.iter().find(|profile| std::iter::once(&profile.name).chain(&profile.aliases).any(|known| normalize(known) == name))
    }

    /// One row per profile: the catalog apps with at least one confirmed feature, most first.
    /// It counts documented alternatives; it does not measure popularity or quality.
    pub fn confirmed_alternatives(&self) -> Vec<(&Profile, Vec<&str>)> {
        let catalog = catalog_sort_names();
        let mut rows: Vec<_> = self
            .profiles
            .iter()
            .map(|profile| {
                let apps = profile
                    .candidates
                    .iter()
                    .map(|candidate| candidate.app_id.as_str())
                    .filter(|app_id| catalog.contains_key(app_id) && profile.confirms(app_id))
                    .collect::<BTreeSet<_>>();
                (profile, apps.into_iter().collect::<Vec<_>>())
            })
            .collect();
        rows.sort_by_cached_key(|(profile, apps)| (std::cmp::Reverse(apps.len()), normalize(&profile.name)));
        rows
    }

    /// Explicit qualified IDs reuse evidence; free labels never acquire evidence by name.
    pub fn feature(&self, key: &str) -> Option<(&Profile, &Feature)> {
        let (profile_id, feature_id) = key.split_once(':')?;
        let profile = self.profiles.iter().find(|profile| profile.id == profile_id)?;
        Some((profile, profile.features.iter().find(|feature| feature.id == feature_id)?))
    }
    pub fn evidence(&self, app_id: &str, key: &str) -> Option<&Evidence> {
        let (profile, feature) = self.feature(key)?;
        profile.evidence(app_id, &feature.id)
    }
    pub fn compare_manual(&self, app_ids: &[String], selected: &[String], required: &[String]) -> Result<Vec<Score>, String> {
        let unique = |ids: &[String]| ids.iter().collect::<BTreeSet<_>>().len() == ids.len();
        let catalog = catalog_sort_names();
        if !unique(app_ids)
            || !unique(selected)
            || !unique(required)
            || !required.iter().all(|feature| selected.contains(feature))
            || app_ids.iter().any(|id| !catalog.contains_key(id.as_str()))
        {
            return Err("Unknown app, duplicate IDs, or required feature outside the selection".into());
        }
        let app_ids: Vec<_> = if app_ids.is_empty() { catalog.keys().copied().collect() } else { app_ids.iter().map(String::as_str).collect() };
        let mut scores: Vec<_> = app_ids
            .into_iter()
            .map(|app_id| score_evidence(app_id, selected, required, |feature| self.evidence(app_id, feature).map(|evidence| evidence.state)))
            .collect();
        sort_scores(&mut scores);
        Ok(scores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn software_suggestions_filter_aliases_without_creating_evidence() {
        let data = data().unwrap();
        let choices = data.software("");
        assert!(choices.windows(2).all(|pair| normalize(pair[0].name) < normalize(pair[1].name)));
        assert!(data.software("PHÓTOSHÓP®").iter().any(|software| software.profile.is_some_and(|profile| profile.id == "photoshop")));
        assert!(data.software("Slack").iter().any(|software| software.profile.is_none()));
        let password = data.software("One Password").into_iter().find_map(|software| software.profile).unwrap();
        assert_eq!(password.id, "1password");
        let selected = ["vault", "shared-vault"].map(String::from);
        let scores = password.compare(&selected, &selected[1..]).unwrap();
        let vaultwarden = scores.iter().find(|score| score.app_id == "vaultwarden").unwrap();
        let keepassxc = scores.iter().find(|score| score.app_id == "keepassxc").unwrap();
        assert_eq!(vaultwarden.supported, 2);
        assert!(vaultwarden.missing_required.is_empty());
        assert_eq!(keepassxc.supported, 1);
        assert_eq!(keepassxc.missing_required, selected[1..]);
        assert!(data.software("no documented product by this name").is_empty());
        let arq = data.profiles.iter().find(|profile| profile.id == "arq").unwrap();
        let selected: Vec<_> = arq.features.iter().map(|feature| feature.id.clone()).collect();
        let scores = arq.compare(&selected, &[]).unwrap();
        assert!(scores.iter().any(|score| score.app_id == "kopia" && score.supported == selected.len()));
        assert!(scores.iter().any(|score| score.app_id == "backrest" && score.supported < selected.len()));
    }

    #[test]
    fn guided_profiles_change_recommendations_with_the_selected_features() {
        let data = data().unwrap();
        for (query, id) in [("Word 365", "microsoft-word"), ("Zoom", "zoom"), ("Notion", "notion")] {
            let profile = data.software(query).into_iter().find_map(|choice| choice.profile).unwrap();
            assert_eq!(profile.id, id);
            assert!(profile.features.len() >= 4);
        }
        let zoom = data.software("Zoom").into_iter().find_map(|choice| choice.profile).unwrap();
        let breakout = zoom.features.iter().find(|feature| feature.id.contains("breakout")).unwrap().id.clone();
        let selected = [breakout];
        let scores = zoom.compare(&selected, &selected).unwrap();
        assert!(scores.iter().any(|score| score.app_id == "jitsi-meet" && score.coverage == Some(100) && score.missing_required.is_empty()));
        assert!(scores.iter().any(|score| score.app_id == "element" && score.supported == 0 && !score.missing_required.is_empty()));
    }

    #[test]
    fn evidence_gaps_and_partial_support_never_become_confirmed_support() {
        let profile: Profile = serde_json::from_value(serde_json::json!({
            "id":"fixture", "name":"Fixture", "aliases":[],
            "features":[], "candidates":[{"app_id":"fixture", "evidence": {
                "yes":{"state":"supported","source":"https://example.org","note":{"it":"sì","en":"yes"}},
                "some":{"state":"partial","source":"https://example.org","note":{"it":"parte","en":"some"}},
                "no":{"state":"unsupported","source":"https://example.org","note":{"it":"no","en":"no"}}
            }}]
        }))
        .unwrap();
        let features: Vec<_> = ["yes", "some", "no", "unknown"].map(String::from).into();
        let score = profile.score("fixture", &features, &features);
        assert_eq!(score.coverage, Some(25));
        assert_eq!(score.missing_required, features[1..]);
        assert_eq!((score.supported, score.partial, score.unsupported, score.unverified), (1, 1, 1, 1));
        assert_eq!(profile.score("unknown-app", &features, &[]).coverage, None);
        assert_eq!(profile.score("fixture", &[], &[]).coverage, None);
        assert_eq!(profile.score("fixture", &features[..3], &[]).coverage, Some(33));
        let eight: Vec<_> = ["yes", "a", "b", "c", "d", "e", "f", "g"].map(String::from).into();
        assert_eq!(profile.score("fixture", &eight, &[]).coverage, Some(13));
    }
    #[test]
    fn manual_comparison_reuses_only_explicit_feature_ids_and_prioritizes_required_gaps() {
        let data = data().unwrap();
        let apps = vec!["photocraft".into(), "gimp".into()];
        let selected = vec!["photoshop:layers".into(), "custom:layers".into()];
        let scores = data.compare_manual(&apps, &selected, &selected).unwrap();
        assert!(scores.iter().all(|score| score.coverage == Some(50) && score.missing_required == ["custom:layers"]));
        assert!(data.compare_manual(&["missing-app".into()], &selected, &[]).is_err());
        assert!(data.compare_manual(&apps, &["layers".into()], &[]).unwrap().iter().all(|score| score.coverage.is_none()));
        // Requirements outrank coverage: GIMP's partial adjustments are still uncovered.
        let selected = vec!["photoshop:reversible".into()];
        let scores = data.compare_manual(&apps, &selected, &selected).unwrap();
        assert!(scores[0].missing_required.len() < scores[1].missing_required.len());
    }

    #[test]
    fn imported_evidence_is_scoped_to_catalog_apps_and_declared_features() {
        let data = data().unwrap();
        assert_eq!(data.schema_version, 1);
        for profile in &data.profiles {
            for candidate in &profile.candidates {
                assert!(curated::apps().iter().any(|app| app.id == candidate.app_id));
                for (feature, evidence) in &candidate.evidence {
                    assert!(profile.features.iter().any(|known| known.id == *feature));
                    assert!(evidence.source.starts_with("https://"));
                }
            }
            let selected: Vec<_> = profile.features.iter().map(|feature| feature.id.clone()).collect();
            let scores = profile.compare(&selected, &selected).unwrap();
            let all: BTreeSet<_> = scores.iter().map(|score| score.app_id.as_str()).collect();
            assert_eq!(all, catalog_sort_names().keys().copied().collect());
            assert!(
                scores
                    .iter()
                    .filter(|score| !profile.candidates.iter().any(|candidate| candidate.app_id == score.app_id))
                    .all(|score| { score.coverage.is_none() && score.unsupported == 0 && score.unverified == selected.len() })
            );
            assert!(profile.compare(&["unknown".into()], &[]).is_err());
            assert!(profile.compare(&[], &selected).is_err());
        }
    }
    #[test]
    fn product_search_accepts_accents_case_and_trademark_signs_without_selecting_free_text() {
        let profiles = &data().unwrap().profiles;
        assert!(profiles.iter().any(|profile| profile.matches("PHÓTOSHÓP®")));
        assert!(!profiles.iter().any(|profile| profile.matches("a product with no profile")));
    }

    #[test]
    fn figma_alternatives_agree_across_comparison_kits_and_rankings() {
        let comparison = data().unwrap().software("Figma Design").into_iter().find_map(|suggestion| suggestion.profile).unwrap();
        let migration = vec!["migration".into()];
        let scores = comparison.compare(&migration, &migration).unwrap();
        let candidates: BTreeSet<_> = scores.iter().filter(|score| score.coverage.is_some()).map(|score| score.app_id.as_str()).collect();
        assert_eq!(candidates, BTreeSet::from(["doop", "figcraft", "openpencil", "penpot"]));
        let openpencil = scores.iter().find(|score| score.app_id == "openpencil").unwrap();
        assert_eq!(openpencil.supported, 1);
        assert!(openpencil.missing_required.is_empty());
        assert!(scores.iter().filter(|score| score.app_id != "openpencil").all(|score| score.missing_required == migration));
        let doop = scores.iter().find(|score| score.app_id == "doop").unwrap();
        assert_eq!(doop.unsupported, 1);
        assert!(scores.iter().filter(|score| matches!(score.app_id.as_str(), "figcraft" | "penpot")).all(|score| score.partial == 1));

        let kits = crate::kits::data().unwrap();
        let step = kits.kits.iter().flat_map(|kit| &kit.steps).find(|step| step.paid.iter().any(|name| name == "Figma")).unwrap();
        assert_eq!(step.compare.as_deref(), Some(comparison.id.as_str()));
        assert_eq!(step.options.iter().map(|option| option.app.as_str()).collect::<BTreeSet<_>>(), candidates);
        for entry in kits.rankings.iter().flat_map(|ranking| &ranking.entries).filter(|entry| entry.paid == "Figma") {
            assert_eq!(entry.compare.as_deref(), Some(comparison.id.as_str()));
            assert_eq!(entry.apps.iter().map(String::as_str).collect::<BTreeSet<_>>(), candidates);
        }
    }
}
