//! Reversible local-only visual proof. No mutating backend command is exposed.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use artcraft_launcher_core::{
    Platform,
    protocol::{Backend, Command, Event},
    status::Installed,
};
use artcraft_launcher_native::NativeBackend;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf, sync::Mutex};
use tauri::{WebviewUrl, WebviewWindowBuilder};

struct NativeState {
    backend: NativeBackend,
    detected: BTreeMap<String, Option<Installed>>,
    diagnostics: PathBuf,
}
#[derive(Serialize)]
struct Snapshot {
    platform: Option<Platform>,
    detected: BTreeMap<String, Option<Installed>>,
}
#[tauri::command]
fn catalog_snapshot() -> Result<serde_json::Value, String> {
    eprintln!("Native catalog requested");
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../crates/core/curated.json"))).map_err(|e| e.to_string())
}
#[tauri::command]
fn native_snapshot(state: tauri::State<'_, Mutex<NativeState>>) -> Result<Snapshot, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    if state.detected.is_empty() {
        eprintln!("Native platform/detection requested");
    }
    for event in state.backend.poll() {
        if let Event::Detected { app, installed } = event {
            state.detected.insert(app, installed);
        }
    }
    Ok(Snapshot { platform: state.backend.platform(), detected: state.detected.clone() })
}
#[derive(Deserialize, Serialize)]
struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}
#[derive(Deserialize, Serialize)]
struct Card {
    id: String,
    rect: Rect,
    preview: Rect,
    logo_loaded: bool,
    preview_loaded: bool,
}
#[derive(Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Language {
    It,
    En,
}
impl Language {
    fn code(self) -> &'static str {
        match self {
            Self::It => "it",
            Self::En => "en",
        }
    }
}
#[derive(Deserialize, Serialize)]
struct VisualReport {
    language: Language,
    viewport: Rect,
    header: Rect,
    toolbar: Rect,
    rail: Rect,
    navigation: Vec<String>,
    cards: Vec<Card>,
}
// A bounded diagnostic write to one fixed file in this proof's private profile. No path argument.
#[tauri::command]
fn record_visual_metrics(report: VisualReport, state: tauri::State<'_, Mutex<NativeState>>) -> Result<(), String> {
    if report.cards.len() > 298
        || report.navigation.len() > 8
        || report.navigation.iter().any(|s| s.len() > 80)
        || report.cards.iter().any(|card| !artcraft_launcher_core::curated::apps().iter().any(|app| app.id == card.id))
    {
        return Err("Invalid visual report".into());
    }
    let state = state.lock().map_err(|e| e.to_string())?;
    let path = state.diagnostics.join(format!("visual-{}.json", report.language.code()));
    artcraft_launcher_native::paths::write_atomic(&path, &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?)
}
fn local_navigation(url: &tauri::Url) -> bool {
    let local_origin =
        (url.scheme() == "tauri" && url.host_str() == Some("localhost")) || (url.scheme() == "http" && url.host_str() == Some("tauri.localhost"));
    local_origin && matches!(url.path(), "/index.html" | "/it.html" | "/en.html") && url.query().is_none()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_none_or(|arg| arg != "--child") {
        let root = PathBuf::from(args.first().ok_or("Supply a new absolute temporary profile directory, followed by it/en")?);
        if !root.is_absolute() {
            return Err("Profile must be absolute".into());
        }
        std::fs::create_dir(&root)?;
        let home = root.join("home");
        for path in [&home, &home.join(".Trash"), &home.join("config"), &home.join("data"), &home.join("cache")] {
            std::fs::create_dir_all(path)?;
        }
        let language = args.get(1).and_then(|s| s.to_str()).unwrap_or("it");
        if !matches!(language, "it" | "en") {
            return Err("Use it or en".into());
        }
        println!("PROOF_ROOT={}", root.display());
        let status = std::process::Command::new(std::env::current_exe()?)
            .arg("--child")
            .arg(&root)
            .arg(language)
            .env("HOME", &home)
            .env("CFFIXED_USER_HOME", &home)
            .env("XDG_CONFIG_HOME", home.join("config"))
            .env("XDG_DATA_HOME", home.join("data"))
            .env("XDG_CACHE_HOME", home.join("cache"))
            .env("CRAFT_VERSIONS_FILE", home.join("Library/Application Support/ShiftAnything/versions.json"))
            .env_remove("ARTCRAFT_LAUNCHER_GITHUB_TOKEN")
            .env_remove("ARTCRAFT_LAUNCHER_DOWNLOAD_BASE")
            .status()?;
        return if status.success() { Ok(()) } else { Err("Desktop proof failed".into()) };
    }
    let root = PathBuf::from(args.get(1).ok_or("Missing profile")?).canonicalize()?;
    let language = args.get(2).and_then(|s| s.to_str()).ok_or("Missing language")?;
    if !matches!(language, "it" | "en") {
        return Err("Use it or en".into());
    }
    let mut backend = NativeBackend::new_isolated_macos_profile(|| {})?;
    backend.send(Command::Detect);
    println!("DESKTOP_PID={}", std::process::id());
    let page = format!("{language}.html");
    let builder = tauri::Builder::default();
    #[cfg(target_os = "macos")]
    let builder = builder.activate_ignoring_other_apps(false);
    builder
        .manage(Mutex::new(NativeState { backend, detected: BTreeMap::new(), diagnostics: root.clone() }))
        .invoke_handler(tauri::generate_handler![catalog_snapshot, native_snapshot, record_visual_metrics])
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            WebviewWindowBuilder::new(app, "proof", WebviewUrl::App(page.into()))
                .title("Sh*ftAnything · Tauri proof")
                .inner_size(1440.0, 900.0)
                .resizable(false)
                .decorations(false)
                .focused(false)
                .data_directory(root.join("webview"))
                .on_navigation(local_navigation)
                .on_new_window(|_, _| tauri::webview::NewWindowResponse::Deny)
                .build()?;
            println!("Local-only window created; native catalog/platform/detection reads enabled; lifecycle commands absent");
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn only_exact_embedded_documents_can_navigate() -> Result<(), Box<dyn std::error::Error>> {
        for local in ["tauri://localhost/it.html", "tauri://localhost/en.html", "http://tauri.localhost/index.html"] {
            assert!(super::local_navigation(&tauri::Url::parse(local)?));
        }
        for foreign in [
            "https://github.com/",
            "https://tauri.localhost/it.html",
            "tauri://localhost/unknown.html",
            "tauri://localhost/it.html?url=https://example.com",
            "file:///etc/passwd",
            "tauri://other/it.html",
        ] {
            assert!(!super::local_navigation(&tauri::Url::parse(foreign)?));
        }
        Ok(())
    }
}
