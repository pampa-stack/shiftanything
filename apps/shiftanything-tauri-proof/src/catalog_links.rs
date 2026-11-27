use artcraft_launcher_core::curated::{App, apps};

pub fn open(app_id: &str, target: &str) -> Result<(), String> {
    dispatch(app_id, target, artcraft_launcher_native::process::open)
}

fn dispatch(app_id: &str, target: &str, opener: impl FnOnce(&std::ffi::OsStr) -> Result<(), String>) -> Result<(), String> {
    let trusted = if let Some(id) = app_id.strip_prefix("survey:") {
        artcraft_launcher_core::kits::data().is_ok_and(|data| data.rankings.iter().any(|ranking| ranking.id == id && ranking.source == target))
    } else {
        apps().iter().find(|app| app.id == app_id).is_some_and(|app| allowed(app, target))
    };
    let url = tauri::Url::parse(target).map_err(|_| "Invalid catalog link")?;
    if url.scheme() != "https" || url.host_str().is_none() || !url.username().is_empty() || url.password().is_some() || !trusted {
        return Err("Link is outside this catalog entry".into());
    }
    opener(std::ffi::OsStr::new(url.as_str()))
}

fn allowed(app: &App, target: &str) -> bool {
    let repo = app.repo.trim_end_matches('/');
    [app.repo.as_str(), &app.website, &app.license_url].contains(&target)
        || app.agent.source.as_deref() == Some(target)
        || app.media_source.as_str() == Some(target)
        || app.review.as_ref().is_some_and(|review| review.source.as_deref() == Some(target) || review.sources.iter().any(|source| source == target))
        || app.stars.as_ref().is_some_and(|stars| stars.source == target || target == format!("{repo}/stargazers"))
        || app.popularity.as_ref().is_some_and(|popularity| popularity.page == target || popularity.source == target)
        || app.release.as_ref().is_some_and(|release| release.source == target)
        || app.platform_support.values().any(|source| source == target)
        || app.distribution.iter().any(|channel| channel.source == target || channel.download_size_source.as_deref() == Some(target))
        || crate::discovery::source_for(&app.id, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_link_is_tied_to_the_selected_app() {
        for url in ["https://github.com/cjpais/Handy", "https://github.com/cjpais/Handy/stargazers"] {
            assert_eq!(
                dispatch("handy", url, |opened| {
                    assert_eq!(opened, url);
                    Ok(())
                }),
                Ok(())
            );
        }
        for url in ["https://doop.design/docs/get-started", "https://github.com/doop-design/doop/blob/main/README.md"] {
            assert_eq!(dispatch("doop", url, |_| Ok(())), Ok(()));
            assert!(dispatch("handy", url, |_| unreachable!()).is_err());
        }
        for (id, url) in [
            ("handy", "https://github.com/storytold/photocraft"),
            ("missing", "https://github.com/cjpais/Handy"),
            ("handy", "https://github.com/cjpais/Handy/releases/download/v0/fake.dmg"),
            ("handy", "file:///tmp/test"),
        ] {
            assert!(dispatch(id, url, |_| unreachable!("unlisted target reached opener")).is_err());
        }
    }
}
