//! Role kits reuse curated details, comparison evidence and the guarded native bulk installer.
use std::collections::BTreeSet;

use artcraft_launcher_core::{
    automation::{Operation, Request},
    curated, kits,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{LauncherUi, Page, app::Dialog};

#[derive(Default)]
pub(crate) struct WorkKits {
    role: usize,
    ranking: bool,
    survey: usize,
    pub(crate) selected: BTreeSet<String>,
}
impl LauncherUi {
    pub(crate) fn kit_can_install(&self, id: &str) -> bool {
        self.caps.manage_apps
            && Request { id: "work-kit".into(), operation: Operation::Install, apps: vec![id.into()] }
                .validate_state(&self.automation_snapshot())
                .is_ok()
    }

    pub(crate) fn kits_page(&mut self, ui: &mut Ui) {
        let Ok(data) = kits::data() else {
            ui.label(tl!("Work kit data unavailable"));
            return;
        };
        let language = crate::i18n::current().code();
        egui::Frame::new().inner_margin(18).show(ui, |ui| {
            ui.heading(RichText::new(tl!("Work kits")).size(30.0));
            ui.style_mut().text_styles.insert(egui::TextStyle::Body, egui::FontId::proportional(12.0));
            ui.style_mut().text_styles.insert(egui::TextStyle::Button, egui::FontId::proportional(11.0));
            ui.spacing_mut().interact_size.y = 26.0;
            ui.spacing_mut().button_padding.y = 4.0;
            ui.label(tl!("Choose a role and find an app for each stage."));
            ui.add_space(14.0);
            ui.horizontal_wrapped(|ui| {
                for (index, kit) in data.kits.iter().enumerate() {
                    if ui.selectable_label(!self.kits.ranking && self.kits.role == index, kit.name.in_language(language)).clicked() {
                        self.kits.role = index;
                        self.kits.ranking = false;
                    }
                }
                if ui.selectable_label(self.kits.ranking, tl!("Ranking")).clicked() {
                    self.kits.ranking = true;
                }
            });
            ui.add_space(12.0);
            if self.kits.ranking {
                self.kit_rankings(ui, data, language);
                return;
            }
            let Some(kit) = data.kits.get(self.kits.role) else { return };
            ui.horizontal(|ui| {
                ui.strong(kit.name.in_language(language));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let eligible: Vec<String> = self.kits.selected.iter().filter(|id| self.kit_can_install(id)).cloned().collect();
                    if ui.add_enabled(!eligible.is_empty(), egui::Button::new(tl!("Review installation"))).clicked() {
                        self.dialog = Some(Dialog::InstallKit(eligible.clone()));
                    }
                    ui.label(format!("{} ({})", tl!("Selected apps"), eligible.len()));
                });
            });
            ui.label(kit.description.in_language(language));
            ui.add_space(12.0);
            let width = ui.available_width();
            TableBuilder::new(ui)
                .id_salt("work-kit-steps")
                .striped(true)
                .column(Column::exact(width * 0.16))
                .column(Column::exact(width * 0.22))
                .column(Column::remainder())
                .header(30.0, |mut header| {
                    for label in [tl!("Stage"), tl!("Paid software"), tl!("Open source alternatives")] {
                        header.col(|ui| {
                            ui.strong(label);
                        });
                    }
                })
                .body(|mut body| {
                    for step in &kit.steps {
                        body.row((step.options.len() as f32 * 48.0).max(74.0), |mut row| {
                            row.col(|ui| {
                                ui.strong(step.name.in_language(language));
                            });
                            row.col(|ui| {
                                ui.label(step.paid.join(", "));
                                if let Some(profile) = &step.compare
                                    && ui.button(tl!("Compare in SH*FTER")).clicked()
                                {
                                    self.open_comparison_profile(profile);
                                }
                            });
                            row.col(|ui| {
                                ui.push_id(&step.id, |ui| {
                                    for option in &step.options {
                                        self.kit_option(ui, &option.app, Some((option.r#use.in_language(language), &option.source)));
                                    }
                                });
                            });
                        });
                    }
                });
            ui.small(format!("{}: {}", tl!("Sources checked"), data.checked_on));
        });
    }

    fn kit_option(&mut self, ui: &mut Ui, id: &str, reason: Option<(&str, &str)>) {
        let Some(app) = curated::apps().iter().find(|app| app.id == id) else {
            ui.label(format!("{id}: {}", tl!("App unavailable in catalog")));
            return;
        };
        ui.push_id(id, |ui| {
            self.catalog_media(app);
            ui.horizontal(|ui| {
                if let Some(icon) = self.picture_source(id, "catalog-logo").or_else(|| crate::icons::app(id)) {
                    ui.add(egui::Image::new(icon).fit_to_exact_size(egui::vec2(20.0, 20.0)).alt_text(&app.name));
                }
                ui.strong(&app.name);
                if let Some(stars) = &app.stars {
                    ui.hyperlink_to(format!("★ {}", compact_count(stars.count)), &stars.source);
                } else if let Some(popularity) = &app.popularity {
                    ui.hyperlink_to(popularity.label(), &popularity.page);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                        self.curated.detail = Some(id.into());
                    }
                    if let Some((_, source)) = reason {
                        ui.hyperlink_to(tl!("Documented features"), source);
                    }
                    if reason.is_none() {
                        return;
                    }
                    if self.kit_can_install(id) {
                        let mut selected = self.kits.selected.contains(id);
                        if ui.checkbox(&mut selected, tl!("Installable")).accessible_name(format!("{} · {}", tl!("Select"), app.name)).changed() {
                            if selected {
                                self.kits.selected.insert(id.into());
                            } else {
                                self.kits.selected.remove(id);
                            }
                        }
                    } else {
                        let state = if self.entry(id).is_some_and(|entry| entry.installed.is_some()) {
                            tl!("Installed")
                        } else if self.entry(id).is_some_and(|entry| entry.job.is_some()) {
                            tl!("In progress")
                        } else if app.delivery == "self-hosted" {
                            tl!("Self-hosted")
                        } else {
                            tl!("Unmanaged installation")
                        };
                        ui.label(RichText::new(state).small()).on_hover_text(&app.name);
                    }
                });
            });
            if let Some((reason, _)) = reason {
                ui.label(RichText::new(reason).small());
            }
            ui.add_space(8.0);
        });
    }

    fn kit_rankings(&mut self, ui: &mut Ui, data: &kits::Dataset, language: &str) {
        ui.horizontal_wrapped(|ui| {
            for (index, survey) in data.rankings.iter().enumerate() {
                ui.selectable_value(&mut self.kits.survey, index, survey.name.in_language(language));
            }
        });
        let Some(survey) = data.rankings.get(self.kits.survey) else { return };
        ui.heading(survey.name.in_language(language));
        ui.label(survey.scope.in_language(language));
        ui.hyperlink_to(&survey.publisher, &survey.source);
        ui.add_space(12.0);
        let width = ui.available_width();
        TableBuilder::new(ui)
            .id_salt("work-kit-survey")
            .striped(true)
            .column(Column::exact(width * 0.24))
            .column(Column::exact(width * 0.18))
            .column(Column::remainder())
            .header(30.0, |mut header| {
                for label in [tl!("Paid software"), survey.metric.in_language(language), tl!("Open source alternatives")] {
                    header.col(|ui| {
                        ui.strong(label);
                    });
                }
            })
            .body(|mut body| {
                for entry in &survey.entries {
                    body.row((entry.apps.len() as f32 * 48.0).max(85.0), |mut row| {
                        row.col(|ui| {
                            ui.label(&entry.paid);
                            if let Some(profile) = &entry.compare
                                && ui.button(tl!("Compare in SH*FTER")).clicked()
                            {
                                self.open_comparison_profile(profile);
                            }
                        });
                        row.col(|ui| {
                            let value = format!("{}%", entry.value);
                            ui.label(if language == "it" { value.replace('.', ",") } else { value });
                        });
                        row.col(|ui| {
                            ui.push_id(&entry.paid, |ui| {
                                for id in &entry.apps {
                                    self.kit_option(ui, id, None);
                                }
                            });
                        });
                    });
                }
            });
    }

    pub(crate) fn kit_install_dialog(&mut self, ui: &mut Ui, apps: &[String]) -> bool {
        ui.heading(tl!("Review installation"));
        ui.label(tl!("Install only these new apps using the native installer. Existing apps will not be updated."));
        let valid = !apps.is_empty() && apps.iter().all(|id| self.kit_can_install(id));
        for id in apps {
            if let Some(entry) = self.entry(id) {
                ui.label(format!("{} · {}", entry.app.name, self.settings.install_locations.folder(id).unwrap_or(tl!("System default"))));
            }
        }
        if !valid {
            ui.label(tl!("App state changed. Review your selection again."));
        }
        let mut close = false;
        ui.horizontal(|ui| {
            if ui.button(tl!("Cancel")).clicked() {
                close = true;
            }
            if ui.add_enabled(valid, egui::Button::new(tl!("Install selected apps"))).clicked() {
                // Keep the expected absence through the native action boundary: races must not update an installed copy.
                self.act_command(artcraft_launcher_core::protocol::Command::InstallExpected { apps: apps.to_vec(), installed: false });
                self.kits.selected.clear();
                self.page = Page::Apps;
                close = true;
            }
        });
        close
    }
}
fn compact_count(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}k", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}
