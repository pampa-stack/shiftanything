//! Export all feature subsets for documented candidates, with full-catalog coverage at the boundaries.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
fn main() -> Result<(), String> {
    let data = artcraft_launcher_core::shifter::data()?;
    let mut cases = vec![];
    for profile in &data.profiles {
        if profile.features.len() > 10 {
            return Err("contract sweep is bounded to ten features per profile".into());
        }
        let all_features = (1usize << profile.features.len()) - 1;
        let reviewed: Vec<_> = profile.candidates.iter().map(|candidate| candidate.app_id.as_str()).collect();
        for selection in 0..=all_features {
            for required in 0..=all_features {
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
                let full_catalog = selection == 0 || selection == all_features && (required == 0 || required == all_features);
                let selected = features(selection);
                let required = features(required);
                let mut results = profile.compare(&selected, &required)?;
                let mut case = serde_json::json!({"profile_id": profile.id, "selected": selected, "required": required});
                if !full_catalog {
                    results.retain(|score| reviewed.contains(&score.app_id.as_str()));
                    case["result_app_ids"] = serde_json::json!(&reviewed);
                }
                case["results"] = serde_json::json!(results);
                cases.push(case);
            }
        }
    }
    for selected in [vec!["custom:fixture".into()], vec!["photoshop:layers".into(), "custom:fixture".into()], vec![]] {
        let apps = vec!["photocraft".into(), "gimp".into()];
        cases.push(serde_json::json!({"manual":true,"app_ids":apps,"selected":selected,"required":selected,"results":data.compare_manual(&apps,&selected,&selected)?}));
        cases.push(serde_json::json!({"manual":true,"app_ids":[],"selected":selected,"required":selected,"results":data.compare_manual(&[],&selected,&selected)?}));
    }
    println!("{}", serde_json::to_string(&cases).map_err(|error| error.to_string())?);
    Ok(())
}
