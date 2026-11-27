//! Reproducible recipe scope inventory. A candidate is not an installable release.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
fn main() -> Result<(), String> {
    use artcraft_launcher_core::{Arch, Os, Platform, curated, flathub, platform};
    let mut rows = vec![];
    for os in [Os::MacOs, Os::Windows, Os::Linux] {
        for arch in [Arch::Arm64, Arch::X64] {
            let target = Platform { os, arch };
            let mut candidates = vec![];
            let mut stable_flathub = vec![];
            let mut incompatible = vec![];
            for app in curated::apps() {
                if app.explicitly_unsupported_on(os) {
                    incompatible.push(&app.id);
                }
                if flathub::supported(&app.id, target) && flathub::approved_catalog_identity(app) {
                    stable_flathub.push(&app.id);
                }
                let recipe = curated::NATIVE_APPS.iter().find(|recipe| recipe.id == app.id && recipe.repo_url() == app.repo.trim_end_matches('/'));
                if let Some(recipe) = recipe {
                    let stems: Vec<_> = recipe.stems().collect();
                    if recipe.runs_on(os)
                        && platform::recipe_platform_verified(&stems, target)
                        && target.download_kinds().iter().any(|kind| recipe.installs(*kind))
                    {
                        candidates.push(&app.id);
                    }
                }
            }
            rows.push(serde_json::json!({
                "os":os, "arch":arch, "catalog_entries":curated::apps().len(),
                "native_recipe_candidates":candidates, "stable_flathub_recipes":stable_flathub,
                "explicitly_incompatible":incompatible,
                "status":"This inventory reports configured channels by target. Releases and lifecycle verification are tracked separately."
            }));
        }
    }
    println!("{}", serde_json::to_string_pretty(&rows).map_err(|error| error.to_string())?);
    Ok(())
}
