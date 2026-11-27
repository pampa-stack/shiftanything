// Included only in macOS installer tests. The adapter executes real native primitives against
// our unsigned DMGs; it does not pretend to test the production signature/download/job pipeline.
#[test]
fn scoped_mcp_egui_approval_executes_native_fixture_lifecycle() {
    use artcraft_launcher_core::protocol::{Backend, Capabilities, Command, Event, LatestRelease, Origin};
    use artcraft_launcher_core::{Arch, Os, Platform};
    use artcraft_launcher_ui_egui::{Host, LauncherUi, Page, Settings};
    use egui_kittest::kittest::Queryable;
    use std::os::unix::fs::PermissionsExt;

    struct Fixture {
        root: PathBuf,
        images: [PathBuf; 2],
        installed: Option<Installed>,
        events: Vec<Event>,
    }
    fn latest(version: &str) -> Event {
        let json = serde_json::json!({"tag_name": format!("v{version}"), "assets":[{"name":format!("photocraft-{version}-macos-universal.dmg"),"browser_download_url":format!("https://github.com/storytold/photocraft/releases/download/v{version}/photocraft-{version}-macos-universal.dmg")}]}).to_string();
        Event::Latest {
            app: "photocraft".into(),
            latest: LatestRelease::Found(artcraft_launcher_core::release::parse_github_release(&json, "storytold/photocraft").unwrap()),
            from: Origin::GitHub,
        }
    }
    impl Fixture {
        fn detected(&mut self) {
            self.events.push(Event::Detected { app: "photocraft".into(), installed: self.installed.clone() });
            self.events.push(Event::Installable { app: "photocraft".into(), installable: true });
        }
        fn operation(&mut self, command: Command) -> Result<String, String> {
            match command {
                Command::InstallExpected { apps, installed } => {
                    assert_eq!(apps, ["photocraft"]);
                    assert_eq!(self.installed.is_some(), installed);
                    let index = usize::from(installed);
                    let placed = install_dmg(
                        photocraft(),
                        &self.images[index],
                        self.installed.as_ref(),
                        Some(&self.root.join("Applications")),
                        None,
                        &Expect { team: None, version: None },
                    )?;
                    let executable = placed.join("Contents/MacOS/PhotoCraft");
                    std::fs::write(&executable, "#!/bin/sh\nprintf launched > \"$1\"\n").unwrap();
                    std::fs::set_permissions(&executable, std::fs::Permissions::from_mode(0o755)).unwrap();
                    let mut manifest = crate::manifest::Manifest::default();
                    manifest
                        .apps
                        .insert("photocraft".into(), record(if installed { "0.7.0" } else { "0.6.0" }, PackageKind::Dmg, placed, vec![], false));
                    self.installed = crate::detect::detect(photocraft(), &manifest, &crate::detect::Snapshot::default());
                    assert!(self.installed.is_some());
                    assert_eq!(self.installed.as_ref().unwrap().version.as_ref().unwrap().to_string(), if installed { "0.7.0" } else { "0.6.0" });
                    self.detected();
                    Ok(if installed { "fixture updated to 0.7.0" } else { "fixture installed at 0.6.0" }.into())
                }
                Command::Launch { app, args } => {
                    assert_eq!(app, "photocraft");
                    assert!(args.is_empty());
                    let path = PathBuf::from(&self.installed.as_ref().ok_or("not installed")?.location);
                    let marker = self.root.join("launched");
                    crate::process::launch_binary(&path.join("Contents/MacOS/PhotoCraft"), &[marker.display().to_string()])?;
                    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                    while !marker.exists() && std::time::Instant::now() < deadline {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                    assert_eq!(std::fs::read_to_string(marker).unwrap(), "launched");
                    self.events.push(latest("0.7.0"));
                    Ok("fixture executable really ran".into())
                }
                Command::Uninstall { app } => {
                    assert_eq!(app, "photocraft");
                    let installed = self.installed.take().ok_or("not installed")?;
                    super::macos::move_to_trash_in(Path::new(&installed.location), &self.root.join("Trash"))?;
                    self.detected();
                    Ok("fixture moved to temporary Trash".into())
                }
                Command::Install { .. } | Command::InstallAll { .. } => panic!("MCP install/update must keep its state guard"),
                _ => Ok(String::new()),
            }
        }
    }
    impl Backend for Fixture {
        fn send(&mut self, command: Command) {
            let meaningful = matches!(
                command,
                Command::InstallExpected { .. }
                    | Command::Launch { .. }
                    | Command::Uninstall { .. }
                    | Command::Install { .. }
                    | Command::InstallAll { .. }
            );
            let result = self.operation(command);
            if meaningful {
                self.events.push(Event::JobFinished { app: "photocraft".into(), result });
            }
        }
        fn poll(&mut self) -> Vec<Event> {
            std::mem::take(&mut self.events)
        }
        fn platform(&self) -> Option<Platform> {
            Some(Platform { os: Os::MacOs, arch: Arch::Arm64 })
        }
        fn capabilities(&self) -> Capabilities {
            Capabilities { manage_apps: true, self_update: false, desktop_shortcuts: false, reveal_label: "Show" }
        }
        fn now(&self) -> i64 {
            crate::now()
        }
    }
    struct ScopedHost(PathBuf);
    impl Host for ScopedHost {
        fn automation_request(&mut self) -> Option<artcraft_launcher_core::automation::Request> {
            crate::automation::take_request_in(&self.0)
        }
        fn automation_publish(&mut self, snapshot: &artcraft_launcher_core::automation::Snapshot) {
            crate::automation::publish_in(&self.0, snapshot);
        }
    }
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    for folder in ["Applications", "Trash", "automation"] {
        std::fs::create_dir(root.join(folder)).unwrap();
    }
    std::fs::write(root.join("user-document"), "keep me").unwrap();
    let config = crate::automation::configuration_in(&root.join("automation")).unwrap();
    assert!(!crate::automation::connection_in(&root.join("automation")).unwrap());
    let config_path = root.join("config.json");
    let uv = serde_json::from_str::<serde_json::Value>(&config).unwrap()["mcpServers"]["shiftanything"]["command"].as_str().unwrap().to_owned();
    assert!(Path::new(&uv).is_absolute());
    std::fs::write(&config_path, config).unwrap();
    let mut backend = Fixture { root: root.into(), images: [dmg(root, "0.6.0"), dmg(root, "0.7.0")], installed: None, events: vec![latest("0.6.0")] };
    backend.detected();
    let mut backend = Some(backend);
    let mut host = ScopedHost(root.join("automation"));
    let applications = root.join("Applications").display().to_string();
    let language = std::env::var("SHIFTANYTHING_FIXTURE_LANG").unwrap_or_else(|_| "en".into());
    assert!(["en", "it"].contains(&language.as_str()));
    let labels = if language == "it" { ["Approva", "Rifiuta", "Installa", "Rimuovi"] } else { ["Approve", "Reject", "Install", "Uninstall"] };
    let mut harness = egui_kittest::Harness::builder().with_size(egui::vec2(1440.0, 900.0)).build_ui_state(
        move |ui, state: &mut Option<LauncherUi>| {
            if state.is_none() {
                let mut settings = Settings { language: language.clone(), notify: false, ..Settings::default() };
                settings.install_locations.per_app.insert("photocraft".into(), applications.clone());
                let mut launcher = LauncherUi::new(ui.ctx(), Box::new(backend.take().unwrap()), settings);
                launcher.set_page(Page::Apps);
                *state = Some(launcher);
            }
            let launcher = state.as_mut().unwrap();
            launcher.logic(ui.ctx(), &mut host);
            launcher.ui(ui, &mut host);
        },
        None,
    );
    harness.run_steps(6);
    let stdout = std::fs::File::create(root.join("client.log")).unwrap();
    let stderr = std::fs::File::create(root.join("client-error.log")).unwrap();
    struct Client(std::process::Child);
    impl Drop for Client {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
    let mut client = Client(
        std::process::Command::new(uv)
            .args(["run", concat!(env!("CARGO_MANIFEST_DIR"), "/../../tools/verify_mcp_cycle.py")])
            .arg(&config_path)
            .env("PATH", "/usr/bin:/bin:/usr/sbin:/sbin")
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .unwrap(),
    );
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(90);
    let mut rejected = false;
    let mut approvals = 0;
    let status = loop {
        harness.run_steps(2);
        // These are actual AccessKit button clicks, including the native install/remove confirmation.
        if harness.query_by_label(labels[0]).is_some() {
            if let Some(folder) = std::env::var_os("SHIFTANYTHING_MCP_FIXTURE_SCREENSHOTS") {
                std::fs::create_dir_all(&folder).unwrap();
                harness.render().unwrap().save(Path::new(&folder).join(format!("approval-{approvals}.png"))).unwrap();
            }
            if rejected {
                harness.get_by_label(labels[0]).click_accesskit();
            } else {
                harness.get_by_label(labels[1]).click_accesskit();
                rejected = true;
            }
            approvals += 1;
        } else {
            for label in [labels[2], labels[3]] {
                if let Some(button) = harness.query_by_label(label) {
                    button.click_accesskit();
                    break;
                }
            }
        }
        if let Some(status) = client.0.try_wait().unwrap() {
            break status;
        }
        if std::time::Instant::now() >= deadline {
            client.0.kill().unwrap();
            let _ = client.0.wait();
            panic!("MCP fixture timed out: {}", std::fs::read_to_string(root.join("client-error.log")).unwrap());
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    };
    let output = std::fs::read_to_string(root.join("client.log")).unwrap();
    assert!(status.success(), "{}\n{}", output, std::fs::read_to_string(root.join("client-error.log")).unwrap());
    assert!(output.contains("PASS real stdio"));
    assert!(crate::automation::connection_in(&root.join("automation")).unwrap());
    assert!(!root.join("Applications/PhotoCraft.app").exists());
    assert!(root.join("Trash/PhotoCraft.app").is_dir());
    assert_eq!(std::fs::read_to_string(root.join("user-document")).unwrap(), "keep me");
    println!("{output}");
}
