//! Local-only desktop proof with explicitly enabled, approved native lifecycle operations.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod catalog_links;
mod discovery;
mod favorites;
mod session;
mod shifter;
#[cfg(target_os = "macos")]
mod verification;
use artcraft_launcher_core::automation::{Operation, Request};
use serde::{Deserialize, Serialize};
use session::NativeState;
use std::{path::PathBuf, sync::Mutex};
use tauri::{WebviewUrl, WebviewWindowBuilder};

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum SocialLink {
    Website,
    Linkedin,
    Instagram,
    Reddit,
    X,
    Discord,
}
impl SocialLink {
    fn url(&self) -> &'static str {
        match self {
            Self::Website => "https://promptica.it/",
            Self::Linkedin => "https://www.linkedin.com/company/promptica/",
            Self::Instagram => "https://www.instagram.com/promptica_ai",
            Self::Reddit => "https://www.reddit.com/user/Comfortable_Ice_4281/",
            Self::X => "https://x.com/Pampaeyes",
            Self::Discord => "https://discord.gg/Xrg2kksz2m",
        }
    }
}
#[tauri::command]
fn open_social_link(link: SocialLink) -> Result<(), String> {
    dispatch_social_link(link, artcraft_launcher_native::process::open)
}
#[tauri::command]
fn open_catalog_link(app_id: String, target: String) -> Result<(), String> {
    catalog_links::open(&app_id, &target)
}
fn dispatch_social_link(link: SocialLink, opener: impl FnOnce(&std::ffi::OsStr) -> Result<(), String>) -> Result<(), String> {
    opener(std::ffi::OsStr::new(link.url()))?;
    eprintln!("EXTERNAL_LINK_REQUESTED {}", link.url());
    Ok(())
}
#[tauri::command]
fn catalog_snapshot() -> Result<serde_json::Value, String> {
    eprintln!("Native catalog requested");
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../crates/core/curated.json"))).map_err(|e| e.to_string())
}
#[tauri::command]
fn discovery_data() -> Result<serde_json::Value, String> {
    discovery::data()
}
#[tauri::command]
fn native_snapshot(state: tauri::State<'_, Mutex<NativeState>>) -> Result<session::Snapshot, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let snapshot = state.snapshot();
    artcraft_launcher_native::paths::write_atomic(
        &state.diagnostics.join("native-state.json"),
        &serde_json::to_vec_pretty(&snapshot).map_err(|e| e.to_string())?,
    )?;
    Ok(snapshot)
}
#[tauri::command]
fn request_native_action(operation: Operation, apps: Vec<String>, state: tauri::State<'_, Mutex<NativeState>>) -> Result<Request, String> {
    state.lock().map_err(|e| e.to_string())?.request(operation, apps)
}
#[tauri::command]
fn decide_native_action(request_id: String, approve: bool, state: tauri::State<'_, Mutex<NativeState>>) -> Result<(), String> {
    state.lock().map_err(|e| e.to_string())?.decide(&request_id, approve)
}
#[tauri::command]
fn check_native_updates(state: tauri::State<'_, Mutex<NativeState>>) -> Result<(), String> {
    state.lock().map_err(|error| error.to_string())?.check_updates()
}
#[tauri::command]
fn set_favorite(app_id: String, favorite: bool, state: tauri::State<'_, Mutex<NativeState>>) -> Result<std::collections::BTreeSet<String>, String> {
    state.lock().map_err(|error| error.to_string())?.favorites.set(app_id, favorite)
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
    if report.cards.len() > artcraft_launcher_core::curated::apps().len()
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
    let lifecycle = args.iter().any(|arg| arg == "--lifecycle");
    if args.first().is_none_or(|arg| arg != "--child") {
        let root = PathBuf::from(args.first().ok_or("Supply a new absolute temporary profile directory, followed by it/en")?);
        if !root.is_absolute() {
            return Err("Profile must be absolute".into());
        }
        let marker = root.join(".shiftanything-proof");
        if args.iter().any(|argument| argument == "--resume") {
            let metadata = std::fs::symlink_metadata(&marker)?;
            if !metadata.is_file()
                || metadata.file_type().is_symlink()
                || metadata.len() > 64
                || std::fs::read(&marker)? != b"shiftanything-tauri-proof-v1\n"
            {
                return Err("Resume requires a profile created by this proof".into());
            }
        } else {
            std::fs::create_dir(&root)?;
            std::fs::write(marker, b"shiftanything-tauri-proof-v1\n")?;
        }
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
            .args(args.iter().skip(2))
            .env("HOME", &home)
            .env("CFFIXED_USER_HOME", &home)
            .env("XDG_CONFIG_HOME", home.join("config"))
            .env("XDG_DATA_HOME", home.join("data"))
            .env("XDG_CACHE_HOME", home.join("cache"))
            .env("CRAFT_VERSIONS_FILE", home.join("Library/Application Support/ShiftAnything/versions.json"))
            .env_remove("ARTCRAFT_LAUNCHER_GITHUB_TOKEN")
            .env_remove("ARTCRAFT_LAUNCHER_DOWNLOAD_BASE")
            .env_remove("ARTCRAFT_LAUNCHER_FEED_URL")
            .status()?;
        return if status.success() { Ok(()) } else { Err("Desktop proof failed".into()) };
    }
    let root = PathBuf::from(args.get(1).ok_or("Missing profile")?).canonicalize()?;
    let language = args.get(2).and_then(|s| s.to_str()).ok_or("Missing language")?;
    if !matches!(language, "it" | "en") {
        return Err("Use it or en".into());
    }
    if let Some(position) = args.iter().position(|argument| argument == "--seed-handy-previous") {
        if !lifecycle {
            return Err("Previous release preparation requires --lifecycle".into());
        }
        let previous = std::path::Path::new(args.get(position + 1).ok_or("Missing previous Handy DMG")?);
        #[cfg(target_os = "macos")]
        verification::seed_previous_handy(&root, previous)?;
        #[cfg(not(target_os = "macos"))]
        {
            let _ = previous;
            return Err("Previous Handy preparation requires macOS".into());
        }
    }
    let state = NativeState::new(root.clone(), lifecycle)?;
    println!("DESKTOP_PID={}", std::process::id());
    let page = format!("{language}.html");
    let builder = tauri::Builder::default();
    #[cfg(target_os = "macos")]
    let builder = builder.activate_ignoring_other_apps(false);
    builder
        .manage(Mutex::new(state))
        .invoke_handler(tauri::generate_handler![
            catalog_snapshot,
            discovery_data,
            open_social_link,
            open_catalog_link,
            native_snapshot,
            record_visual_metrics,
            request_native_action,
            decide_native_action,
            set_favorite,
            check_native_updates,
            shifter::shifter_workspace,
            shifter::compare_shifter,
            shifter::open_shifter_source
        ])
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
            println!("Local-only window created; native lifecycle enabled: {lifecycle}");
            Ok(())
        })
        .run(tauri::generate_context!())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn social_opener_accepts_only_closed_ids_and_propagates_failure() {
        for (id, url) in [
            ("website", "https://promptica.it/"),
            ("linkedin", "https://www.linkedin.com/company/promptica/"),
            ("instagram", "https://www.instagram.com/promptica_ai"),
            ("reddit", "https://www.reddit.com/user/Comfortable_Ice_4281/"),
            ("x", "https://x.com/Pampaeyes"),
            ("discord", "https://discord.gg/Xrg2kksz2m"),
        ] {
            let link: super::SocialLink = serde_json::from_value(serde_json::json!(id)).unwrap();
            let mut opened = None;
            super::dispatch_social_link(link, |target| {
                opened = Some(target.to_owned());
                Ok(())
            })
            .unwrap();
            assert_eq!(opened.as_deref(), Some(std::ffi::OsStr::new(url)));
        }
        for value in ["https://example.com/", "file:///tmp/private", "--args", "unknown", "Discord"] {
            assert!(serde_json::from_value::<super::SocialLink>(serde_json::json!(value)).is_err());
        }
        let error = super::dispatch_social_link(super::SocialLink::Discord, |_| Err("opener unavailable".into())).unwrap_err();
        assert_eq!(error, "opener unavailable");
    }
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
