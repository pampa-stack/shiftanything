use serde_json::Value;
use std::sync::OnceLock;

pub fn intents() -> Result<&'static Value, String> {
    static INTENTS: OnceLock<Result<Value, String>> = OnceLock::new();
    INTENTS
        .get_or_init(|| serde_json::from_str(include_str!("../../../crates/core/trova-app.json")).map_err(|error| error.to_string()))
        .as_ref()
        .map(|value| &value["intents"])
        .map_err(Clone::clone)
}

pub fn data() -> Result<Value, String> {
    let kits: Value = serde_json::from_str(include_str!("../../../crates/core/kit-di-lavoro.json")).map_err(|error| error.to_string())?;
    Ok(serde_json::json!({"intents": intents()?, "work": kits}))
}

pub fn source_for(app_id: &str, target: &str) -> bool {
    intents().is_ok_and(|intents| {
        intents
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|intent| intent["apps"].as_array())
            .flatten()
            .any(|app| app["id"] == app_id && app["source"] == target)
    }) || artcraft_launcher_core::kits::data().is_ok_and(|data| {
        data.kits.iter().flat_map(|kit| &kit.steps).flat_map(|step| &step.options).any(|option| option.app == app_id && option.source == target)
    })
}
