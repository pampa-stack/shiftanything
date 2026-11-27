//! Agent actions stay pending until an explicit approval in this window.
use crate::{Host, LauncherUi};
use artcraft_launcher_core::{
    automation::{InventoryEntry, Operation, Snapshot},
    protocol::Command,
};
impl LauncherUi {
    pub(crate) fn automation_dialog(&mut self, ctx: &egui::Context, host: &mut dyn Host) {
        while self.automation_results.len() > 256 {
            self.automation_results.pop_first();
        }
        if self.automation_pending.is_none()
            && self.dialog.is_none()
            && let Some(request) = host.automation_request()
        {
            match self.validate_automation_request(&request) {
                Ok(()) => {
                    self.automation_results.insert(request.id.clone(), "awaiting approval".into());
                    self.automation_pending = Some(request);
                }
                Err(error) => {
                    self.automation_results.insert(request.id, format!("rejected: {error}"));
                }
            }
        }
        if let Some(request) = self.automation_pending.clone() {
            let mut decision = None;
            egui::Window::new(tl!("Agent request")).anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO).collapsible(false).resizable(false).show(
                ctx,
                |ui| {
                    let label = match request.operation {
                        Operation::Install => tl!("Install"),
                        Operation::Update | Operation::UpdateAll => tl!("Update"),
                        Operation::Open => tl!("Open"),
                        Operation::Remove => tl!("Uninstall"),
                        Operation::Cancel => tl!("Cancel"),
                    };
                    let apps =
                        request.apps.iter().map(|id| self.entry(id).map_or(id.as_str(), |entry| entry.app.name)).collect::<Vec<_>>().join(", ");
                    ui.label(format!("{label}: {apps}"));
                    ui.label(tl!("Approve this action in the launcher"));
                    ui.horizontal(|ui| {
                        if ui.button(tl!("Reject")).clicked() {
                            decision = Some(false);
                        }
                        if ui.button(tl!("Approve")).clicked() {
                            decision = Some(true);
                        }
                    });
                },
            );
            if let Some(approve) = decision {
                self.automation_pending = None;
                // Revalidate at approval, not only when the request arrived.
                if approve && request.validate_state(&self.automation_snapshot()).is_ok() && self.caps.manage_apps {
                    self.automation_results.insert(request.id.clone(), "approved; native operation requested (not completed)".into());
                    for id in &request.apps {
                        self.automation_results.remove(&format!("app:{id}"));
                        self.automation_links.entry(id.clone()).or_default().push(request.id.clone());
                        if matches!(request.operation, Operation::Install | Operation::Update | Operation::UpdateAll) {
                            self.automation_expected.insert(id.clone(), request.operation != Operation::Install);
                        }
                    }
                    match request.operation {
                        Operation::Install | Operation::Update => self.install_app(&request.apps[0]),
                        Operation::Remove => self.dialog = Some(crate::app::Dialog::ConfirmUninstall(request.apps[0].clone())),
                        Operation::Cancel => self.act_command(Command::Cancel { app: request.apps[0].clone() }),
                        Operation::Open => self.act_command(Command::Launch { app: request.apps[0].clone(), args: vec![] }),
                        Operation::UpdateAll => self.act_command(Command::InstallAll { apps: request.apps }),
                    }
                } else {
                    self.automation_results.insert(request.id, "rejected".into());
                }
            }
        }
        host.automation_publish(&self.automation_snapshot());
        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }

    pub(crate) fn agent_setup(&mut self, ui: &mut egui::Ui, host: &mut dyn Host) -> bool {
        use artcraft_launcher_core::mcp_config::{McpConfigFormat, catalog_config};
        ui.set_width(760.0f32.min(ui.ctx().content_rect().width() - 80.0));
        ui.heading(tl!("Connect agent"));
        ui.label(tl!("Search the catalog and request app actions. Approve each action in this launcher."));
        egui::ScrollArea::vertical().max_height(ui.ctx().content_rect().height() * 0.7).show(ui, |ui| {
            ui.label(tl!("Requires uv. Copy this configuration into your MCP client, then keep the launcher open."));
            ui.horizontal_wrapped(|ui| {
                for (format, label) in [(McpConfigFormat::Json, "JSON"), (McpConfigFormat::VsCode, "VS Code"), (McpConfigFormat::CodexToml, "TOML")] {
                    ui.selectable_value(&mut self.automation_format, format, label);
                }
            });
            let configuration = self.automation_config.get_or_insert_with(|| host.automation_configuration()).clone();
            match configuration.and_then(|config| catalog_config(self.automation_format, &config)) {
                Ok(config) => {
                    if ui.button(tl!("Copy configuration")).clicked() { ui.ctx().copy_text(config.clone()); }
                    ui.group(|ui| { ui.monospace(config); });
                }
                Err(error) => { ui.label(tl!("Setup unavailable")); ui.label(error); }
            }
            ui.horizontal_wrapped(|ui| {
                if ui.button(tl!("Check connection")).clicked() { self.automation_connection = Some(host.automation_connection()); }
                ui.hyperlink_to(tl!("Documentation"), "https://github.com/lorenzotalamucci0-stack/shiftanything/blob/f/pro-371-launcher-desktop/docs/mcp.md");
            });
            if let Some(status) = &self.automation_connection {
                match status {
                    Ok(true) => { ui.label(tl!("Agent contacted this launcher in the last minute")); }
                    Ok(false) => { ui.label(tl!("No recent agent call. Ask your client to call get_inventory.")); }
                    Err(error) => { ui.label(tl!("Setup unavailable")); ui.label(error); }
                }
            }
            ui.separator();
            ui.heading(tl!("Examples"));
            let examples = "search_apps {\"query\":\"speech to text\",\"level\":\"base\"}\nget_app {\"app_id\":\"handy\"}\nget_inventory {}\nrequest_action {\"operation\":\"install\",\"app_ids\":[\"handy\"]}\nget_request_status {\"request_id\":\"<returned request_id>\"}";
            ui.monospace(examples);
            if ui.button(tl!("Copy examples")).clicked() { ui.ctx().copy_text(examples.to_string()); }
            ui.separator();
            ui.heading(tl!("Request history"));
            if self.automation_results.is_empty() { ui.label(tl!("No requests in this session")); }
            for (request, status) in &self.automation_results { ui.label(format!("{request} · {status}")); }
        });
        ui.button(tl!("Close")).clicked()
    }

    fn validate_automation_request(&self, request: &artcraft_launcher_core::automation::Request) -> Result<(), String> {
        if !self.caps.manage_apps || self.automation_results.contains_key(&request.id) {
            return Err("management disabled or request already processed".into());
        }
        request.validate_state(&self.automation_snapshot())
    }

    pub(crate) fn automation_dialog_closed(&mut self, dialog: &crate::app::Dialog) {
        use crate::app::{Dialog, UiAction};
        let (Dialog::Install { app, .. } | Dialog::ConfirmUninstall(app) | Dialog::Replace { app, .. }) = dialog else { return };
        let dispatched = self.pending.iter().any(|action| {
            matches!(action,
            UiAction::Command(Command::Install { app: id } | Command::Uninstall { app: id } | Command::Replace { app: id, .. }) if id == app)
        });
        if !dispatched {
            self.automation_expected.remove(app);
            for request in self.automation_links.remove(app).unwrap_or_default() {
                self.automation_results.insert(format!("{request}:{app}"), "cancelled before native operation".into());
            }
        }
    }

    pub(crate) fn automation_guarded_command(&mut self, command: Command) -> Command {
        match &command {
            Command::Install { app } => {
                if let Some(installed) = self.automation_expected.get(app).copied() {
                    return Command::InstallExpected { apps: vec![app.clone()], installed };
                }
            }
            Command::InstallAll { apps } if apps.iter().all(|id| self.automation_expected.get(id) == Some(&true)) => {
                return Command::InstallExpected { apps: apps.clone(), installed: true };
            }
            _ => {}
        }
        command
    }

    pub(crate) fn automation_snapshot(&self) -> Snapshot {
        Snapshot {
            updated_at: self.backend.now(),
            apps: self
                .entries
                .iter()
                .map(|entry| InventoryEntry {
                    id: entry.app.id.to_string(),
                    detected: entry.detected,
                    installed: entry.installed.is_some(),
                    compatible: self.platform.is_some_and(|platform| entry.app.runs_on(platform.os)) && entry.has_download(self.platform),
                    installable: self.caps.manage_apps && entry.installs_here(self.platform),
                    update_available: entry.status(self.platform, self.caps.manage_apps) == artcraft_launcher_core::AppStatus::UpdateAvailable,
                    installed_version: entry.installed.as_ref().and_then(|installed| installed.version.as_ref().map(ToString::to_string)),
                    latest_version: entry.release().and_then(|release| release.version.as_ref().map(ToString::to_string)),
                    job: entry.job.map(|phase| format!("{phase:?}")),
                    can_open: None,
                    can_remove: None,
                })
                .collect(),
            results: self.automation_results.clone(),
        }
    }
}
