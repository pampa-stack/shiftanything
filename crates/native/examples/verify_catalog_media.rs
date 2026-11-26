//! Real native HTTPS/cache media probe; writes only into an explicitly chosen test directory.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
fn main() -> Result<(), String> {
    use artcraft_launcher_native::{http::Http, paths::Paths, screenshots};
    use std::io::Write;
    let output = std::path::PathBuf::from(std::env::args_os().nth(1).ok_or("supply a new temporary output directory")?);
    std::fs::create_dir(&output).map_err(|error| error.to_string())?;
    let config_root = output.join("automation");
    let config = artcraft_launcher_native::automation::configuration_in(&config_root)?;
    std::fs::write(output.join("mcp-config.json"), config).map_err(|error| error.to_string())?;
    println!("PASS isolated native MCP support and configuration");
    let http = Http::new()?;
    let paths = Paths::under(&output.join("profile"));
    for id in ["photocraft", "handy", "meetily", "rizzo-pii", "normcap", "libreoffice"] {
        let app = artcraft_launcher_core::curated::apps().iter().find(|app| app.id == id).ok_or("unknown recipe")?;
        let mut errors = Vec::new();
        screenshots::fetch_catalog(Some(&http), &paths, app, |key, result| {
            let written = result.and_then(|bytes| {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output.join(format!("{id}-{key}.image")))
                    .map_err(|error| error.to_string())?;
                file.write_all(&bytes).map_err(|error| error.to_string())
            });
            if let Err(error) = written {
                errors.push(format!("{id}/{key}: {error}"));
            }
        });
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        // A second read uses only the native cache, with HTTP deliberately absent.
        screenshots::fetch_catalog(None, &paths, app, |key, result| {
            if let Err(error) = result {
                errors.push(format!("cached {id}/{key}: {error}"));
            }
        });
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        println!("PASS {id}: real native download, image budget and offline cache");
    }
    Ok(())
}
