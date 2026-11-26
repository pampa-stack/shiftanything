//! The comparator uses the same documented evidence and integer scoring contract as MCP.
use artcraft_launcher_core::{curated, shifter};
use egui::{RichText, Ui};

use crate::LauncherUi;

#[derive(Default)]
pub(crate) struct Comparison {
    query: String,
    pub(crate) sources: Option<(String, Vec<String>)>,
    profile: Option<String>,
    selected: Vec<String>,
    required: Vec<String>,
    manual_name: Option<String>,
    manual_feature: String,
    manual_features: Vec<String>,
    manual_required: Vec<String>,
    app_query: String,
    manual_apps: Vec<String>,
}
impl LauncherUi {
    pub(crate) fn open_comparison_profile(&mut self, id: &str) {
        if let Ok(data) = shifter::data()
            && let Some(profile) = data.profiles.iter().find(|profile| profile.id == id)
        {
            self.comparison.profile = Some(profile.id.clone());
            self.comparison.manual_name = None;
            self.comparison.query.clear();
            self.comparison.selected = profile.features.iter().map(|feature| feature.id.clone()).collect();
            self.comparison.required.clear();
            self.page = crate::Page::Shifter;
        }
    }
    pub(crate) fn shifter_page(&mut self, ui: &mut Ui) {
        let Ok(data) = shifter::data() else {
            ui.label(tl!("Comparison data unavailable"));
            return;
        };
        let language = crate::i18n::current().code();
        egui::Frame::new().inner_margin(18).show(ui, |ui| {
            ui.columns(2, |columns| {
                columns[0].heading("SH*FTER");
                columns[0].label(tl!("Choose what matters. Compare documented features."));
                self.shifter_search(&mut columns[1]);
            });
            ui.add_space(12.0);
            self.shifter_profiles(ui, data);
            ui.add_space(12.0);
            ui.horizontal_top(|ui| {
                let tokens = crate::theme::Tokens::get(ui.ctx());
                egui::Frame::new().fill(tokens.sunken).stroke(egui::Stroke::new(1.0, tokens.line)).inner_margin(12).show(ui, |ui| {
                    ui.vertical(|ui| {
                        ui.set_width(320.0);
                        egui::ScrollArea::vertical().id_salt("shifter-controls").max_height(ui.available_height()).show(ui, |ui| {
                            self.shifter_selection(ui, data, language);
                        });
                    });
                });
                ui.add_space(6.0);
                ui.vertical(|ui| {
                    ui.set_width(ui.available_width());
                    if let Some(profile) = self.comparison.profile.as_ref().and_then(|id| data.profiles.iter().find(|profile| profile.id == *id)) {
                        self.shifter_results(ui, profile, language);
                    } else if self.comparison.manual_name.is_some() {
                        ui.heading(self.comparison.manual_name.as_deref().unwrap_or_default());
                        ui.label(tl!("Manual comparison. Reuse documented features or add your own."));
                        self.shifter_manual_results(ui, data, language);
                    } else {
                        ui.label(tl!("Select a documented profile or start a manual comparison."));
                    }
                });
            });
        });
        self.shifter_sources(ui, data, language);
    }

    fn shifter_search(&mut self, ui: &mut Ui) {
        ui.label(RichText::new(tl!("Paid software")).size(10.0).family(egui::FontFamily::Monospace));
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.comparison.query).desired_width(260.0)).accessible_name(tl!("Paid software"));
            if ui.add_enabled(!self.comparison.query.trim().is_empty(), egui::Button::new(tl!("Compare manually"))).clicked() {
                self.comparison.manual_name = Some(self.comparison.query.trim().to_owned());
                self.comparison.profile = None;
                self.comparison.manual_features.clear();
                self.comparison.manual_required.clear();
                let name = shifter::normalize(&self.comparison.query);
                self.comparison.manual_apps = curated::apps()
                    .iter()
                    .filter(|app| app.alternatives.iter().any(|alternative| shifter::normalize(alternative) == name))
                    .map(|app| app.id.clone())
                    .collect();
            }
        });
    }

    fn shifter_profiles(&mut self, ui: &mut Ui, data: &shifter::Dataset) {
        ui.horizontal_wrapped(|ui| {
            for profile in &data.profiles {
                if (self.comparison.query.is_empty() || profile.matches(&self.comparison.query))
                    && ui.selectable_label(self.comparison.profile.as_deref() == Some(&profile.id), &profile.name).clicked()
                {
                    self.comparison.profile = Some(profile.id.clone());
                    self.comparison.manual_name = None;
                    self.comparison.selected = profile.features.iter().map(|feature| feature.id.clone()).collect();
                    self.comparison.required.clear();
                }
            }
        });
    }

    fn shifter_selection(&mut self, ui: &mut Ui, data: &shifter::Dataset, language: &str) {
        if let Some(profile) = self.comparison.profile.as_ref().and_then(|id| data.profiles.iter().find(|profile| profile.id == *id)) {
            ui.strong(tl!("Features to compare"));
            for feature in &profile.features {
                let name = feature.name.in_language(language);
                let mut selected = self.comparison.selected.contains(&feature.id);
                if ui.checkbox(&mut selected, name).changed() {
                    self.comparison.selected.retain(|id| id != &feature.id);
                    if selected {
                        self.comparison.selected.push(feature.id.clone());
                    } else {
                        self.comparison.required.retain(|id| id != &feature.id);
                    }
                }
                ui.horizontal(|ui| {
                    let mut required = self.comparison.required.contains(&feature.id);
                    if ui
                        .add_enabled(selected, egui::Checkbox::new(&mut required, tl!("Required")))
                        .accessible_name(format!("{} · {name}", tl!("Required")))
                        .changed()
                    {
                        self.comparison.required.retain(|id| id != &feature.id);
                        if required {
                            self.comparison.required.push(feature.id.clone());
                        }
                    }
                    ui.hyperlink_to(tl!("Source"), &feature.source);
                });
            }
        }
        if self.comparison.manual_name.is_some() {
            self.shifter_manual_controls(ui, data, language);
        }
        ui.add_space(12.0);
        ui.label(format!("{} · {}", tl!("Evidence checked"), data.checked_on));
        ui.label(tl!("Coverage counts confirmed features only. Partial and unverified remain separate."));
    }

    fn shifter_results(&mut self, ui: &mut Ui, profile: &shifter::Profile, language: &str) {
        use egui_extras::{Column, TableBuilder};
        let selected = self.comparison.selected.clone();
        let required = self.comparison.required.clone();
        let scores = match profile.compare(&selected, &required) {
            Ok(scores) => scores,
            Err(error) => {
                ui.label(error);
                return;
            }
        };
        let tokens = crate::theme::Tokens::get(ui.ctx());
        egui::Frame::new().fill(tokens.sunken).stroke(egui::Stroke::new(1.0, tokens.line)).inner_margin(12).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.strong(tl!("Open-source alternatives"));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(&profile.name);
                });
            });
            ui.separator();
            if selected.is_empty() {
                ui.label(tl!("Select at least one feature. No percentage is calculated."));
            }
            let width = ui.available_width();
            TableBuilder::new(ui)
                .id_salt("shifter-results")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(width * 0.27))
                .column(Column::exact(width * 0.23))
                .column(Column::remainder())
                .column(Column::exact(200.0))
                .header(32.0, |mut header| {
                    for title in [tl!("App"), tl!("Documented coverage"), tl!("Required features"), tl!("Sources / details")] {
                        header.col(|ui| {
                            ui.label(RichText::new(title).size(11.0).family(egui::FontFamily::Monospace));
                        });
                    }
                })
                .body(|mut body| {
                    for score in scores {
                        let Some(app) = curated::apps().iter().find(|app| app.id == score.app_id) else { continue };
                        self.catalog_media(app);
                        body.row(48.0, |mut row| {
                            row.col(|ui| {
                                if let Some(icon) = self.picture_source(&app.id, "catalog-logo").or_else(|| crate::icons::app(&app.id)) {
                                    ui.add(egui::Image::new(icon).fit_to_exact_size(egui::vec2(24.0, 24.0)).alt_text(format!("{} logo", app.name)));
                                }
                                ui.strong(&app.name);
                            });
                            row.col(|ui| {
                                if let Some(coverage) = score.coverage {
                                    ui.label(RichText::new(format!("{coverage}%")).size(18.0).strong().color(tokens.good));
                                    ui.label(format!("{}/{}", score.supported, selected.len()));
                                } else {
                                    ui.label(tl!("No verified evidence"));
                                }
                            });
                            row.col(|ui| {
                                if required.is_empty() {
                                    ui.label("—");
                                } else if score.missing_required.is_empty() {
                                    ui.label(tl!("All required features confirmed"));
                                } else {
                                    let names = score
                                        .missing_required
                                        .iter()
                                        .filter_map(|id| profile.features.iter().find(|feature| feature.id == *id))
                                        .map(|feature| feature.name.in_language(language))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    ui.label(
                                        RichText::new(format!("⚠ {}: {}", tl!("Required feature not confirmed"), score.missing_required.len()))
                                            .color(tokens.warn),
                                    )
                                    .on_hover_text(names);
                                }
                            });
                            row.col(|ui| {
                                if ui.button(tl!("Sources")).accessible_name(format!("{} · {}", tl!("Sources"), app.name)).clicked() {
                                    self.comparison.sources =
                                        Some((app.id.clone(), selected.iter().map(|id| format!("{}:{id}", profile.id)).collect()));
                                }
                                if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                                    self.curated.detail = Some(app.id.clone());
                                }
                            });
                        });
                    }
                });
            ui.separator();
            ui.label(format!("{} · {}", tl!("Evidence checked"), shifter::data().map_or("—", |data| data.checked_on.as_str())));
        });
    }

    fn shifter_sources(&mut self, ui: &mut Ui, data: &shifter::Dataset, language: &str) {
        let Some((app_id, features)) = self.comparison.sources.clone() else { return };
        let Some(app) = curated::apps().iter().find(|app| app.id == app_id) else { return };
        let mut open = true;
        egui::Window::new(format!("{} · {}", tl!("Sources"), app.name))
            .id(egui::Id::unique("shifter-sources"))
            .open(&mut open)
            .default_width(620.0)
            .resizable(true)
            .show(ui.ctx(), |ui| {
                for key in features {
                    let Some((_, feature)) = data.feature(&key) else { continue };
                    ui.strong(feature.name.in_language(language));
                    evidence_cell(ui, data.evidence(&app.id, &key), language);
                    if let Some(evidence) = data.evidence(&app.id, &key) {
                        ui.label(evidence.note.in_language(language));
                    }
                    ui.separator();
                }
                ui.label(format!("{} · {}", tl!("Evidence checked"), data.checked_on));
            });
        if !open || ui.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.comparison.sources = None;
        }
    }

    fn shifter_manual_controls(&mut self, ui: &mut Ui, data: &shifter::Dataset, language: &str) {
        ui.label(tl!("Add a feature"));
        ui.horizontal(|ui| {
            ui.text_edit_singleline(&mut self.comparison.manual_feature).accessible_name(tl!("Add a feature"));
            if ui.add_enabled(!self.comparison.manual_feature.trim().is_empty(), egui::Button::new(tl!("Add"))).clicked() {
                let feature = format!("custom:{}", self.comparison.manual_feature.trim());
                if !self.comparison.manual_features.contains(&feature) {
                    self.comparison.manual_features.push(feature);
                }
                self.comparison.manual_feature.clear();
            }
        });
        craft_ui::ComboBox::from_id_salt("reuse-feature")
            .selected_text(tl!("Reuse a documented feature"))
            .show_ui(ui, |ui| {
                for profile in &data.profiles {
                    for feature in &profile.features {
                        let key = format!("{}:{}", profile.id, feature.id);
                        if ui
                            .selectable_label(
                                self.comparison.manual_features.contains(&key),
                                format!("{} · {}", profile.name, feature.name.in_language(language)),
                            )
                            .clicked()
                            && !self.comparison.manual_features.contains(&key)
                        {
                            self.comparison.manual_features.push(key);
                        }
                    }
                }
            })
            .response
            .accessible_name(tl!("Reuse a documented feature"));
        let mut removed = None;
        for feature in &self.comparison.manual_features {
            ui.horizontal(|ui| {
                let name = manual_feature_name(data, feature, language);
                ui.label(&name);
                let mut required = self.comparison.manual_required.contains(feature);
                if ui.checkbox(&mut required, tl!("Required")).accessible_name(format!("{} · {name}", tl!("Required"))).changed() {
                    self.comparison.manual_required.retain(|id| id != feature);
                    if required {
                        self.comparison.manual_required.push(feature.clone());
                    }
                }
                if ui.button("×").accessible_name(format!("{} · {name}", tl!("Remove feature"))).clicked() {
                    removed = Some(feature.clone());
                }
            });
        }
        if let Some(removed) = removed {
            self.comparison.manual_features.retain(|feature| *feature != removed);
            self.comparison.manual_required.retain(|feature| *feature != removed);
        }
        ui.text_edit_singleline(&mut self.comparison.app_query).accessible_name(tl!("Choose catalog apps"));
        egui::ScrollArea::vertical().id_salt("manual-app-picker").max_height(180.0).show(ui, |ui| {
            let query = shifter::normalize(&self.comparison.app_query);
            for app in curated::apps().iter().filter(|app| shifter::normalize(&app.name).contains(&query)) {
                let mut selected = self.comparison.manual_apps.contains(&app.id);
                if ui.checkbox(&mut selected, &app.name).changed() {
                    self.comparison.manual_apps.retain(|id| id != &app.id);
                    if selected {
                        self.comparison.manual_apps.push(app.id.clone());
                    }
                }
            }
        });
    }
    fn shifter_manual_results(&mut self, ui: &mut Ui, data: &shifter::Dataset, language: &str) {
        let scores = match data.compare_manual(&self.comparison.manual_apps, &self.comparison.manual_features, &self.comparison.manual_required) {
            Ok(scores) => scores,
            Err(error) => {
                ui.label(error);
                return;
            }
        };
        if scores.is_empty() {
            ui.label(tl!("No apps selected"));
        }
        egui::ScrollArea::both().id_salt("manual-results").show(ui, |ui| {
            egui::Grid::new("manual-comparison").striped(true).max_col_width(140.0).spacing([14.0, 16.0]).show(ui, |ui| {
                ui.strong(tl!("App"));
                ui.strong(tl!("Documented coverage"));
                ui.strong(tl!("Required features"));
                for feature in &self.comparison.manual_features {
                    ui.strong(manual_feature_name(data, feature, language));
                }
                ui.strong(tl!("App details"));
                ui.end_row();
                for score in scores {
                    let Some(app) = curated::apps().iter().find(|app| app.id == score.app_id) else { continue };
                    ui.label(&app.name);
                    ui.label(score.coverage.map_or_else(
                        || tl!("No verified evidence").to_owned(),
                        |coverage| format!("{coverage}% · {}/{}", score.supported, self.comparison.manual_features.len()),
                    ));
                    if score.missing_required.is_empty() {
                        ui.label("—");
                    } else {
                        ui.label(format!(
                            "⚠ {}: {}",
                            tl!("Required feature not confirmed"),
                            score.missing_required.iter().map(|key| manual_feature_name(data, key, language)).collect::<Vec<_>>().join(", ")
                        ));
                    }
                    for feature in &self.comparison.manual_features {
                        evidence_cell(ui, data.evidence(&app.id, feature), language);
                    }
                    if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                        self.curated.detail = Some(app.id.clone());
                    }
                    ui.end_row();
                }
            });
        });
    }
}
fn manual_feature_name(data: &shifter::Dataset, key: &str, language: &str) -> String {
    data.feature(key).map_or_else(
        || key.strip_prefix("custom:").unwrap_or(key).to_owned(),
        |(profile, feature)| format!("{} · {}", profile.name, feature.name.in_language(language)),
    )
}
fn evidence_cell(ui: &mut Ui, evidence: Option<&shifter::Evidence>, language: &str) {
    let state = evidence.map_or(shifter::State::Unverified, |evidence| evidence.state);
    let label = match state {
        shifter::State::Supported => tl!("Confirmed"),
        shifter::State::Partial => tl!("Partial"),
        shifter::State::Unsupported => tl!("Absent"),
        shifter::State::Unverified => tl!("Unverified"),
    };
    if let Some(evidence) = evidence {
        ui.hyperlink_to(label, &evidence.source).on_hover_text(evidence.note.in_language(language));
    } else {
        ui.label(label);
    }
}
