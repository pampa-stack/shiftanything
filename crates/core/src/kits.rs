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
