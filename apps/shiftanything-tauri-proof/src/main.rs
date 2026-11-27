//! Local desktop launcher with approved native lifecycle operations.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod catalog_links;
mod discovery;
mod favorites;
mod launcher_update;
mod media;
mod session;
mod shifter;
mod startup;
#[cfg(target_os = "macos")]
mod verification;
use artcraft_launcher_core::automation::{Operation, Request};
use artcraft_launcher_core::mcp_config::{McpConfigFormat, catalog_config};
use serde::{Deserialize, Serialize};
use session::NativeState;
use std::sync::Mutex;
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
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum McpClient {
    Codex,
    ClaudeCode,
    Cursor,
    VsCode,
    OpenCode,
}

impl McpClient {
    fn docs(&self) -> &'static str {
        match self {
            Self::Codex => "https://developers.openai.com/codex/mcp",
            Self::ClaudeCode => "https://code.claude.com/docs/en/mcp",
            Self::Cursor => "https://cursor.com/docs/mcp",
            Self::VsCode => "https://code.visualstudio.com/docs/agent-customization/mcp-servers",
            Self::OpenCode => "https://opencode.ai/docs/mcp-servers/",
        }
    }

    fn format(&self) -> McpConfigFormat {
        match self {
            Self::Codex => McpConfigFormat::CodexToml,
            Self::ClaudeCode | Self::Cursor => McpConfigFormat::Json,
            Self::VsCode => McpConfigFormat::VsCode,
            Self::OpenCode => McpConfigFormat::OpenCode,
        }
    }
}

#[tauri::command]
fn mcp_configuration(client: McpClient, state: tauri::State<'_, Mutex<NativeState>>) -> Result<String, String> {
    if !state.lock().map_err(|error| error.to_string())?.lifecycle_enabled() {
        return Err("MCP setup requires an active lifecycle profile".into());
    }
    catalog_config(client.format(), &artcraft_launcher_native::automation::configuration()?)
}
#[tauri::command]
fn mcp_connection(state: tauri::State<'_, Mutex<NativeState>>) -> Result<bool, String> {
    if !state.lock().map_err(|error| error.to_string())?.lifecycle_enabled() {
        return Ok(false);
    }
    artcraft_launcher_native::automation::connection()
}
#[tauri::command]
fn open_mcp_docs(client: McpClient) -> Result<(), String> {
    artcraft_launcher_native::process::open(std::ffi::OsStr::new(client.docs()))
}
#[tauri::command]
fn native_snapshot(state: tauri::State<'_, Mutex<NativeState>>) -> Result<session::Snapshot, String> {
    let mut state = state.lock().map_err(|e| e.to_string())?;
    let snapshot = state.snapshot();
    if state.isolated_profile() {
        artcraft_launcher_native::paths::write_atomic(
            &state.diagnostics.join("native-state.json"),
            &serde_json::to_vec_pretty(&snapshot).map_err(|e| e.to_string())?,
        )?;
    }
    Ok(snapshot)
}
#[tauri::command]
fn request_native_action(operation: Operation, apps: Vec<String>, state: tauri::State<'_, Mutex<NativeState>>) -> Result<Request, String> {
    state.lock().map_err(|e| e.to_string())?.request(operation, apps)
}
#[tauri::command]
fn decide_native_action(
    request_id: String,
    approve: bool,
    open_after_install: Option<bool>,
    state: tauri::State<'_, Mutex<NativeState>>,
) -> Result<(), String> {
    state.lock().map_err(|e| e.to_string())?.decide(&request_id, approve, open_after_install.unwrap_or(false))
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
#[tauri::command]
fn set_language(language: Language, state: tauri::State<'_, Mutex<NativeState>>) -> Result<(), String> {
    let state = state.lock().map_err(|error| error.to_string())?;
    artcraft_launcher_native::paths::write_atomic(&state.diagnostics.join(startup::LANGUAGE_FILE), language.code().as_bytes())
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
    if !state.isolated_profile() {
        return Err("Visual metrics require a verification profile".into());
    }
    let path = state.diagnostics.join(format!("visual-{}.json", report.language.code()));
    artcraft_launcher_native::paths::write_atomic(&path, &serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?)
}
fn local_navigation(url: &tauri::Url) -> bool {
    let local_origin =
        (url.scheme() == "tauri" && url.host_str() == Some("localhost")) || (url.scheme() == "http" && url.host_str() == Some("tauri.localhost"));
    local_origin && matches!(url.path(), "/index.html" | "/it.html" | "/en.html") && url.query().is_none()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let Some(launch) = startup::start(std::env::args_os().skip(1).collect())? else { return Ok(()) };
    // Package replacement can invalidate current_exe() on the old process.
    let executable = std::env::current_exe();
    let isolated = launch.isolated;
    let capture_window = launch.capture_window;
    let lifecycle = launch.lifecycle;
    let state = NativeState::new(launch.root.clone(), lifecycle, isolated)?;
    let catalog_media = media::Media::new(state.catalog_media());
    println!("DESKTOP_PID={}", std::process::id());
    let page = format!("{}.html", launch.language);
    let webview_directory = launch.root.join("webview");
    let mut context = tauri::generate_context!();
    let launcher_updater = launcher_update::configure(&mut context, isolated);
    let builder = tauri::Builder::default();
    #[cfg(target_os = "macos")]
    let builder = builder.activate_ignoring_other_apps(false);
    let exit_code = builder
        .manage(Mutex::new(state))
        .manage(catalog_media)
        .manage(launcher_updater)
        .invoke_handler(tauri::generate_handler![
            catalog_snapshot,
            media::fetch_catalog_media,
            discovery_data,
            open_social_link,
            open_catalog_link,
            native_snapshot,
            record_visual_metrics,
            request_native_action,
            decide_native_action,
            set_favorite,
            set_language,
            check_native_updates,
            launcher_update::launcher_update_state,
            launcher_update::launcher_update_check,
            launcher_update::launcher_update_install,
            launcher_update::launcher_update_restart,
            mcp_configuration,
            mcp_connection,
            open_mcp_docs,
            shifter::shifter_workspace,
            shifter::compare_shifter,
            shifter::open_shifter_source
        ])
        .setup(move |app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(if capture_window { tauri::ActivationPolicy::Accessory } else { tauri::ActivationPolicy::Regular });
            WebviewWindowBuilder::new(app, "proof", WebviewUrl::App(page.into()))
                .title(if isolated { "Sh*ftAnything · Tauri proof" } else { "Sh*ftAnything" })
                .inner_size(1440.0, 900.0)
                .resizable(!capture_window)
                .decorations(!capture_window)
                .focused(!isolated)
                .incognito(isolated)
                .data_directory(webview_directory)
                .on_navigation(local_navigation)
                .on_new_window(|_, _| tauri::webview::NewWindowResponse::Deny)
                .build()?;
            launcher_update::start(app.handle())?;
            println!("Local-only window created; native lifecycle enabled: {lifecycle}");
            Ok(())
        })
        .build(context)?
        .run_return(|_, _| {});
    if exit_code == launcher_update::RESTART_EXIT_CODE {
        // The new process must be able to acquire the same desktop instance lock.
        drop(launch);
        startup::relaunch(&executable?)?;
    } else if exit_code != 0 {
        std::process::exit(exit_code);
    }
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
