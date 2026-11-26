//! Opt-in macOS arm64 proof: actual NativeBackend downloads/jobs behind egui + stdio MCP.
//! The parent creates a new HOME/CF/XDG profile; no personal app data or Trash is used.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn main() -> Result<(), String> {
    Err("This probe requires macOS Apple Silicon".into())
}
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn main() -> Result<(), String> {
    proof::run()
}

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
mod proof {
    use artcraft_launcher_core::{
        PackageKind, Platform,
        curated::{self, HANDY},
        protocol::{Backend, Capabilities, Command, Event, JobPhase, LatestRelease},
        release::ReleaseQuery,
        version,
    };
    use artcraft_launcher_native::{
        NativeBackend, automation, http, install, manifest,
        paths::Paths,
        process,
        source::{GitHubSource, ReleaseSource},
    };
    use artcraft_launcher_ui_egui::{Host, LauncherUi, Page, Settings};
    use egui_kittest::kittest::Queryable;
    use std::{
        os::unix::process::CommandExt,
        path::{Path, PathBuf},
        sync::{Arc, Mutex},
        time::{Duration, Instant},
    };

    pub fn run() -> Result<(), String> {
        let args: Vec<_> = std::env::args_os().skip(1).collect();
        if args.first().is_some_and(|a| a == "--child") {
            return child(
                Path::new(args.get(1).ok_or("missing profile")?),
                Path::new(args.get(2).ok_or("missing previous DMG")?),
                args.iter().any(|a| a == "--no-open"),
            );
        }
        let no_open = args.iter().any(|a| a == "--no-open");
        if !no_open && Path::new("/tmp/com_pais_handy_si.sock").exists() {
            return Err("Handy single-instance socket already exists; refusing to contact a personal instance. Use --no-open for the explicitly partial lifecycle proof".into());
        }
        let previous = PathBuf::from(args.first().ok_or("Supply the official Handy 0.10.1 arm64 DMG")?);
        if http::sha256_file(&previous)? != "7cdef5de7d1bfcea70813320b897540cbead2190f5280db6d9760a59e5995463" {
            return Err("Unexpected previous artifact digest".into());
        }
        let root = tempfile::Builder::new().prefix("shift-official-mcp-").tempdir().map_err(|e| e.to_string())?.keep();
        let home = root.join("home");
        for dir in [&home, &home.join(".Trash"), &home.join("config"), &home.join("data"), &home.join("cache")] {
            std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        }
        println!("PROOF_ROOT={}", root.display());
        let mut command = std::process::Command::new(std::env::current_exe().map_err(|e| e.to_string())?);

        let status = command
            .arg("--child")
            .arg(&root)
            .arg(previous)
            .args(if no_open { vec!["--no-open"] } else { vec![] })
            .env("HOME", &home)
            .env("CFFIXED_USER_HOME", &home)
            .env("CRAFT_VERSIONS_FILE", home.join("Library/Application Support/ShiftAnything/versions.json"))
            .env("XDG_CONFIG_HOME", home.join("config"))
            .env("XDG_DATA_HOME", home.join("data"))
            .env("XDG_CACHE_HOME", home.join("cache"))
            .env_remove("ARTCRAFT_LAUNCHER_GITHUB_TOKEN")
            .env_remove("ARTCRAFT_LAUNCHER_DOWNLOAD_BASE")
            .env_remove("ARTCRAFT_LAUNCHER_FEED_URL")
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() { Ok(()) } else { Err(format!("Official cycle failed; inspect {}", root.display())) }
    }

    // Only background catalog refresh/media are suppressed. The real, freshly fetched official
    // release cache is replayed by NativeBackend; every mutating command and native job is real.
    struct Observed {
        inner: NativeBackend,
        events: Arc<Mutex<Vec<Event>>>,
    }
    impl Backend for Observed {
        fn send(&mut self, command: Command) {
            if !matches!(
                command,
                Command::CheckForUpdates { .. } | Command::FetchCatalogMedia { .. } | Command::FetchScreenshots { .. } | Command::McpState
            ) {
                self.inner.send(command);
            }
        }
        fn poll(&mut self) -> Vec<Event> {
            let events = self.inner.poll();
            for event in &events {
                if matches!(event, Event::Progress { app, .. } | Event::JobFinished { app, .. } | Event::Detected { app, .. } if app == "handy") {
                    println!("NATIVE {event:?}");
                }
            }
            self.events.lock().unwrap_or_else(std::sync::PoisonError::into_inner).extend(events.clone());
            events
        }
        fn capabilities(&self) -> Capabilities {
            self.inner.capabilities()
        }
        fn platform(&self) -> Option<Platform> {
            self.inner.platform()
        }
        fn now(&self) -> i64 {
            self.inner.now()
        }
        fn has_active_jobs(&self) -> bool {
            self.inner.has_active_jobs()
        }
        fn download_size(&self, id: &str) -> Option<(PackageKind, u64)> {
            self.inner.download_size(id)
        }
    }
    struct Client(std::process::Child);
    impl Drop for Client {
        fn drop(&mut self) {
            let _ = std::process::Command::new("/bin/kill").args(["-TERM", "--", &format!("-{}", self.0.id())]).status();
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    struct TemporaryApp(PathBuf);
    impl Drop for TemporaryApp {
        fn drop(&mut self) {
            if process::is_running(&self.0) {
                let _ = stop_exact_process(&self.0.join("Contents/MacOS/handy"));
            }
        }
    }
    struct ScopedHost;
    impl Host for ScopedHost {
        fn automation_request(&mut self) -> Option<artcraft_launcher_core::automation::Request> {
            automation::take_request().filter(|request| request.apps.iter().all(|id| id == "handy"))
        }
        fn automation_publish(&mut self, snapshot: &artcraft_launcher_core::automation::Snapshot) {
            automation::publish(snapshot);
        }
    }

    fn child(root: &Path, previous: &Path, no_open: bool) -> Result<(), String> {
        let home = root.join("home");
        if std::env::var_os("HOME").as_deref() != Some(home.as_os_str()) || std::env::var_os("CFFIXED_USER_HOME").as_deref() != Some(home.as_os_str())
        {
            return Err("Refusing a non-isolated process".into());
        }
        let paths = Paths::new()?;
        if !paths.data.starts_with(&home) || !paths.cache.starts_with(&home) {
            return Err("Native paths escaped profile".into());
        }
        prove_launch_home(root, &home)?;
        let apps = home.join("Applications");
        std::fs::create_dir_all(&apps).map_err(|e| e.to_string())?;
        let source = GitHubSource::new(http::Http::new()?, paths.release_cache());
        let answer = source.latest(&ReleaseQuery::for_app(&HANDY, HANDY.repo), false);
        let LatestRelease::Found(release) = answer.latest else {
            return Err(format!("Official release lookup failed: {:?}", answer.latest));
        };
        if release.tag != "v0.10.2" {
            return Err("Current release differs from the reviewed recipe; re-review before running".into());
        }
        source.save();
        let old = version::parse("0.10.1").ok_or("invalid previous version")?;
        let placed = install::macos::install_dmg(
            &HANDY,
            previous,
            None,
            Some(&apps),
            None,
            &paths,
            &install::macos::Expect { team: HANDY.mac_team, version: Some(&old) },
        )?;
        let _temporary_app = TemporaryApp(placed.clone());
        let mut manifest = manifest::Manifest::default();
        manifest.apps.insert(
            "handy".into(),
            manifest::Record {
                version: old.to_string(),
                kind: PackageKind::Dmg,
                path: placed.clone(),
                installed_at: artcraft_launcher_native::now(),
                files: vec![],
                needs_root: false,
            },
        );
        manifest.save(&paths.manifest())?;
        let sentinel = home.join("Library/Application Support/com.pais.handy/shift-proof-user-data");
        std::fs::create_dir_all(sentinel.parent().ok_or("no sentinel parent")?).map_err(|e| e.to_string())?;
        std::fs::write(&sentinel, "synthetic-data-keep").map_err(|e| e.to_string())?;
        let config = automation::configuration()?;
        let config_path = root.join("config.json");
        std::fs::write(&config_path, &config).map_err(|e| e.to_string())?;
        let uv = serde_json::from_str::<serde_json::Value>(&config).map_err(|e| e.to_string())?["mcpServers"]["shiftanything"]["command"]
            .as_str()
            .ok_or("uv command absent")?
            .to_string();
        let events = Arc::new(Mutex::new(vec![]));
        let mut backend = Some(Observed { inner: NativeBackend::new_isolated_macos_profile(|| {})?, events: events.clone() });
        let mut host = ScopedHost;
        let applications = apps.display().to_string();
        let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
            move |ui, state: &mut Option<LauncherUi>| {
                if state.is_none()
                    && let Some(backend) = backend.take()
                {
                    let mut settings = Settings { language: "en".into(), notify: false, ..Settings::default() };
                    settings.install_locations.per_app.insert("handy".into(), applications.clone());
                    settings.hidden.extend(curated::NATIVE_APPS.iter().filter(|app| app.id != "handy").map(|app| app.id.to_string()));
                    let mut launcher = LauncherUi::new(ui.ctx(), Box::new(backend), settings);
                    launcher.set_page(Page::Apps);
                    *state = Some(launcher);
                }
                if let Some(launcher) = state.as_mut() {
                    launcher.logic(ui.ctx(), &mut host);
                    launcher.ui(ui, &mut host);
                }
            },
            None,
        );
        harness.run_steps(6);
        let mut client = Client(
            std::process::Command::new(uv)
                .args(["run", concat!(env!("CARGO_MANIFEST_DIR"), "/../../tools/verify_official_mcp.py")])
                .arg(&config_path)
                .arg(root)
                .args(if no_open { vec!["--no-open"] } else { vec![] })
                .process_group(0)
                .stdout(std::fs::File::create(root.join("client.log")).map_err(|e| e.to_string())?)
                .stderr(std::fs::File::create(root.join("client-error.log")).map_err(|e| e.to_string())?)
                .spawn()
                .map_err(|e| e.to_string())?,
        );
        let deadline = Instant::now() + Duration::from_secs(300);
        let mut rejected = false;
        let mut launches = 0;
        let outcome = loop {
            harness.run_steps(2);
            if harness.query_by_label("Approve").is_some() {
                harness
                    .render()
                    .map_err(|e| e.to_string())?
                    .save(root.join(format!("approval-{}.png", artcraft_launcher_native::now())))
                    .map_err(|e| e.to_string())?;
                if rejected {
                    harness.get_by_label("Approve").click_accesskit();
                } else {
                    harness.get_by_label("Reject").click_accesskit();
                    rejected = true;
                }
            } else {
                for label in ["Install", "Uninstall"] {
                    if let Some(button) = harness.query_by_label(label) {
                        button.click_accesskit();
                        break;
                    }
                }
            }
            // The client waits for this marker before requesting removal. It is separate from
            // Launch Services' acceptance: verify an actual process at the unique temporary path.
            if !no_open && exact_process(&placed.join("Contents/MacOS/handy"))?.is_some() {
                stop_exact_process(&placed.join("Contents/MacOS/handy"))?;
                launches += 1;
                std::fs::write(root.join(format!("launch-{launches}")), "actual process observed and stopped").map_err(|e| e.to_string())?;
            }
            if let Some(status) = client.0.try_wait().map_err(|e| e.to_string())? {
                break status;
            }
            if Instant::now() >= deadline {
                let _ = client.0.kill();
                let _ = client.0.wait();
                return Err(format!("Timed out; logs in {}", root.display()));
            }
            std::thread::sleep(Duration::from_millis(30));
        };
        let client_output = std::fs::read_to_string(root.join("client.log")).map_err(|e| e.to_string())?;
        println!("{client_output}");
        if !outcome.success() {
            return Err(std::fs::read_to_string(root.join("client-error.log")).map_err(|e| e.to_string())?);
        }
        harness.check_accessibility();
        if launches != if no_open { 0 } else { 2 }
            || placed.exists()
            || std::fs::read_to_string(&sentinel).map_err(|e| e.to_string())? != "synthetic-data-keep"
        {
            return Err("Lifecycle or profile preservation mismatch".into());
        }
        let events = events.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if !events.iter().any(|event| matches!(event,Event::Progress { app, phase: JobPhase::Downloading {done,..} } if app=="handy" && *done>0)) {
            return Err("No real native download observed".into());
        }
        for expected in ["0.10.1", "0.10.2"] {
            if !events.iter().any(|event| matches!(event,Event::Detected { app, installed:Some(installed) } if app=="handy" && installed.version.as_ref().is_some_and(|version|version.to_string()==expected))) { return Err(format!("Native version {expected} not detected")); }
        }
        if no_open {
            println!(
                "PASS PARTIAL official NativeBackend download/verify/update/remove/install/remove via stdio + egui approvals; synthetic profile preserved. OPEN NOT TESTED: global single-instance conflict"
            );
        } else {
            println!(
                "PASS official NativeBackend download/verify/update/open/remove/install/open/remove via stdio + egui approvals; temporary processes observed; synthetic profile preserved"
            );
        }
        Ok(())
    }

    fn exact_process(executable: &Path) -> Result<Option<String>, String> {
        let output = std::process::Command::new("/bin/ps").args(["-axo", "pid=,args="]).output().map_err(|e| e.to_string())?;
        let executable = executable.to_string_lossy();
        let mut found = None;
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            let Some((pid, args)) = line.trim().split_once(char::is_whitespace) else { continue };
            if args.trim() == executable || args.trim().starts_with(&format!("{executable} ")) {
                if !pid.bytes().all(|b| b.is_ascii_digit()) || found.is_some() {
                    return Err("Ambiguous temporary process identity".into());
                }
                found = Some(pid.to_string());
            }
        }
        Ok(found)
    }

    fn stop_exact_process(executable: &Path) -> Result<(), String> {
        let pid = exact_process(executable)?.ok_or("Temporary process absent")?;
        let status = std::process::Command::new("/bin/kill").args(["-TERM", &pid]).status().map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("Could not stop temporary Handy process".into());
        }
        let deadline = Instant::now() + Duration::from_secs(10);
        while exact_process(executable)?.is_some() {
            if Instant::now() >= deadline {
                return Err("Temporary process did not exit".into());
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        Ok(())
    }

    fn prove_launch_home(root: &Path, home: &Path) -> Result<(), String> {
        let bundle = root.join("ProfileProof.app");
        let macos = bundle.join("Contents/MacOS");
        std::fs::create_dir_all(&macos).map_err(|e| e.to_string())?;
        std::fs::write(bundle.join("Contents/Info.plist"), "<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>CFBundleExecutable</key><string>ProfileProof</string><key>CFBundleIdentifier</key><string>it.promptica.shiftanything.profile-proof</string><key>CFBundlePackageType</key><string>APPL</string></dict></plist>").map_err(|e|e.to_string())?;
        let source = root.join("profile-proof.m");
        std::fs::write(&source, "#import <Foundation/Foundation.h>\nint main(void){@autoreleasepool{NSString *home=NSHomeDirectory();[home writeToFile:[home stringByAppendingPathComponent:@\"launch-profile-proof\"] atomically:YES encoding:NSUTF8StringEncoding error:nil];}return 0;}\n").map_err(|e|e.to_string())?;
        let status = std::process::Command::new("/usr/bin/xcrun")
            .args(["clang", "-framework", "Foundation"])
            .arg(&source)
            .arg("-o")
            .arg(macos.join("ProfileProof"))
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("Could not build our own Foundation profile fixture".into());
        }
        process::open_app(&bundle, &[])?;
        let marker = home.join("launch-profile-proof");
        let deadline = Instant::now() + Duration::from_secs(10);
        while !marker.exists() && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(30));
        }
        if std::fs::read_to_string(marker).map_err(|e| e.to_string())? != home.display().to_string() {
            return Err("Launch Services did not preserve Foundation home".into());
        }
        println!("PASS actual Launch Services + Foundation fixture wrote only to the explicitly redirected home");
        Ok(())
    }
}
