//! Close an exact macOS bundle through AppKit's normal Quit path before an approved update.

use std::collections::BTreeMap;

pub(crate) type OpenBundles = BTreeMap<String, String>;

const STILL_OPEN: &str = "App is still open. Save your work, quit it, and try updating again.";

#[cfg(target_os = "macos")]
const APPKIT: &str = r#"
function run(argv) {
    ObjC.import('AppKit');
    // Foundation aliases /private/tmp to /tmp, unlike Rust's canonicalize.
    const targets = new Map(argv.slice(1).map(path => [ObjC.unwrap($(path).stringByResolvingSymlinksInPath), path]));
    function running() {
        const apps = $.NSWorkspace.sharedWorkspace.runningApplications;
        const matches = [];
        for (let i = 0; i < Number(apps.count); i++) {
            const app = apps.objectAtIndex(i);
            const url = app.bundleURL;
            if (app.terminated || url.isNil()) continue;
            const path = ObjC.unwrap(url.URLByResolvingSymlinksInPath.path);
            if (targets.has(path)) matches.push({app, path: targets.get(path)});
        }
        return matches;
    }
    let matches = running();
    if (argv[0] === 'quit' && matches.length) {
        for (const {app} of matches) app.terminate;
        for (let i = 0; i < 50 && matches.length; i++) {
            // NSWorkspace updates its running application list on the run loop.
            $.NSRunLoop.currentRunLoop.runUntilDate($.NSDate.dateWithTimeIntervalSinceNow(0.2));
            matches = running();
        }
    }
    return JSON.stringify(matches.map(match => match.path));
}
"#;

fn remaining(bundles: &OpenBundles, quit: bool) -> Result<OpenBundles, String> {
    if bundles.is_empty() {
        return Ok(OpenBundles::new());
    }
    #[cfg(target_os = "macos")]
    {
        let mut args = vec!["-l", "JavaScript", "-e", APPKIT, "--", if quit { "quit" } else { "list" }];
        args.extend(bundles.values().map(String::as_str));
        let timeout = std::time::Duration::from_secs(if quit { 15 } else { 3 });
        let result = artcraft_launcher_native::process::run("/usr/bin/osascript", &args, timeout)?;
        let paths: std::collections::BTreeSet<String> = serde_json::from_str(&result).map_err(|error| error.to_string())?;
        Ok(bundles.iter().filter(|(_, path)| paths.contains(*path)).map(|(id, path)| (id.clone(), path.clone())).collect())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = quit;
        Ok(OpenBundles::new())
    }
}

/// Paths come only from the native inventory, and must identify actual app bundles.
pub(crate) fn detect<'a>(apps: impl Iterator<Item = (&'a str, &'a str)>) -> Result<OpenBundles, String> {
    let bundles = apps
        .filter_map(|(id, path)| {
            let path = std::path::Path::new(path);
            if !path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("app")) {
                return None;
            }
            let real = path.canonicalize().ok()?;
            if !real.is_dir() {
                return None;
            }
            Some((id.to_string(), real.to_str()?.to_string()))
        })
        .collect();
    remaining(&bundles, false)
}

pub(crate) fn quit_for_update(bundles: &OpenBundles) -> BTreeMap<String, Result<(), String>> {
    let still_open = remaining(bundles, true);
    bundles
        .keys()
        .map(|id| {
            let result = match &still_open {
                Ok(entries) if !entries.contains_key(id) => Ok(()),
                Ok(_) => Err(STILL_OPEN.into()),
                Err(error) => Err(format!("Could not ask the app to quit: {error}")),
            };
            (id.clone(), result)
        })
        .collect()
}
