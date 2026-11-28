//! Real native HTTPS/cache media probe; writes only into an explicitly chosen test directory.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
fn main() -> Result<(), String> {
    use artcraft_launcher_native::{http::Http, paths::Paths, screenshots};
    use std::io::Write;
    let mut arguments = std::env::args_os().skip(1);
    let output = std::path::PathBuf::from(arguments.next().ok_or("supply a new temporary output directory")?);
    let requested = arguments.map(|value| value.into_string().map_err(|_| "catalog ID must be text".to_string())).collect::<Result<Vec<_>, _>>()?;
    let render_default_gallery = requested.is_empty();
    let ids = if render_default_gallery {
        vec!["photocraft", "handy", "meetily", "rizzo-pii", "normcap", "libreoffice"]
    } else {
        requested.iter().map(String::as_str).collect()
    };
    std::fs::create_dir(&output).map_err(|error| error.to_string())?;
    let config_root = output.join("automation");
    let config = artcraft_launcher_native::automation::configuration_in(&config_root)?;
    std::fs::write(output.join("mcp-config.json"), config).map_err(|error| error.to_string())?;
    println!("PASS isolated native MCP support and configuration");
    let http = Http::new()?;
    let paths = Paths::under(&output.join("profile"));
    let mut online = Vec::new();
    let mut offline = Vec::new();
    for id in ids {
        let app = artcraft_launcher_core::curated::apps().iter().find(|app| app.id == id).ok_or("unknown recipe")?;
        let mut errors = Vec::new();
        screenshots::fetch_catalog(Some(&http), &paths, app, |key, result| {
            let written = result.and_then(|bytes| {
                let mut file = std::fs::OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(output.join(format!("{id}-{key}.image")))
                    .map_err(|error| error.to_string())?;
                file.write_all(&bytes).map_err(|error| error.to_string())?;
                online.push(artcraft_launcher_core::protocol::Event::Screenshot { app: id.into(), key: key.into(), result: Ok(bytes) });
                Ok(())
            });
            if let Err(error) = written {
                errors.push(format!("{id}/{key}: {error}"));
            }
        });
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        // A second read uses only the native cache, with HTTP deliberately absent.
        screenshots::fetch_catalog(None, &paths, app, |key, result| match result {
            Ok(bytes) => offline.push(artcraft_launcher_core::protocol::Event::Screenshot { app: id.into(), key: key.into(), result: Ok(bytes) }),
            Err(error) => errors.push(format!("cached {id}/{key}: {error}")),
        });
        if !errors.is_empty() {
            return Err(errors.join("; "));
        }
        println!("PASS {id}: real native download, image budget and offline cache");
    }
    if render_default_gallery {
        render_media(&output, &online, "online")?;
        render_media(&output, &offline, "offline")?;
    }
    Ok(())
}

// Renderer proof with actual bytes from the native media path. App detection and installation
// are deliberately unavailable in this media-only backend; it never claims lifecycle success.
struct MediaOnly {
    events: Vec<artcraft_launcher_core::protocol::Event>,
}
impl artcraft_launcher_core::protocol::Backend for MediaOnly {
    fn send(&mut self, _command: artcraft_launcher_core::protocol::Command) {}
    fn poll(&mut self) -> Vec<artcraft_launcher_core::protocol::Event> {
        std::mem::take(&mut self.events)
    }
    fn capabilities(&self) -> artcraft_launcher_core::protocol::Capabilities {
        artcraft_launcher_core::protocol::Capabilities { manage_apps: false, reveal_label: "Show", self_update: false, desktop_shortcuts: false }
    }
    fn platform(&self) -> Option<artcraft_launcher_core::Platform> {
        artcraft_launcher_core::Platform::current()
    }
    fn now(&self) -> i64 {
        artcraft_launcher_native::now()
    }
}
struct MediaHost;
impl artcraft_launcher_ui_egui::Host for MediaHost {}
fn render_media(output: &std::path::Path, events: &[artcraft_launcher_core::protocol::Event], phase: &str) -> Result<(), String> {
    use artcraft_launcher_ui_egui::{LauncherUi, Settings};
    use egui_kittest::kittest::Queryable;
    for language in ["it", "en"] {
        let mut backend = Some(MediaOnly { events: events.to_vec() });
        let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(backend) = backend.take()
                {
                    *state = Some(LauncherUi::new(
                        ui.ctx(),
                        Box::new(backend),
                        Settings { language: language.into(), notify: false, ..Settings::default() },
                    ));
                }
                if let Some(launcher) = state.as_mut() {
                    launcher.logic(ui.ctx(), &mut MediaHost);
                    launcher.ui(ui, &mut MediaHost);
                }
            },
            None,
        );
        for _ in 0..20 {
            harness.run_steps(1);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
        for name in ["PhotoCraft", "Handy", "Meetily Community", "Rizzo-PII", "NormCap", "LibreOffice"] {
            if harness.query_all_by_label(&format!("{name} preview")).next().is_none() {
                return Err(format!("Missing renderer preview node: {name}"));
            }
        }
        harness.check_accessibility();
        harness
            .render()
            .map_err(|error| error.to_string())?
            .save(output.join(format!("catalog-{phase}-{language}.png")))
            .map_err(|error| error.to_string())?;
        println!("PASS {phase}/{language}: native egui renderer with six downloaded previews and AccessKit; headless, no desktop lifecycle claim");
    }
    Ok(())
}
