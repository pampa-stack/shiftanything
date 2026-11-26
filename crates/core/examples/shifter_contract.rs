//! Export all selected/required feature subsets for cross-language MCP parity verification.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
fn main() -> Result<(), String> {
    let data = artcraft_launcher_core::shifter::data()?;
    let mut cases = vec![];
    for profile in &data.profiles {
        if profile.features.len() > 10 {
            return Err("contract sweep is bounded to ten features per profile".into());
        }
        for selection in 0..(1usize << profile.features.len()) {
            for required in 0..(1usize << profile.features.len()) {
                if required & !selection != 0 {
                    continue;
                }
                let features = |mask| {
                    profile
                        .features
                        .iter()
                        .enumerate()
                        .filter(|(index, _)| mask & (1 << index) != 0)
                        .map(|(_, feature)| feature.id.clone())
                        .collect::<Vec<_>>()
                };
                let selected = features(selection);
                let required = features(required);
                cases.push(serde_json::json!({"profile_id":profile.id,"selected":selected,"required":required,"results":profile.compare(&selected,&required)?}));
            }
        }
    }
    for selected in [vec!["custom:fixture".into()], vec!["photoshop:layers".into(), "custom:fixture".into()], vec![]] {
        let apps = vec!["photocraft".into(), "gimp".into()];
        cases.push(serde_json::json!({"manual":true,"app_ids":apps,"selected":selected,"required":selected,"results":data.compare_manual(&apps,&selected,&selected)?}));
    }
    println!("{}", serde_json::to_string(&cases).map_err(|error| error.to_string())?);
    Ok(())
}
