//! The comparator uses the same documented evidence and integer scoring contract as MCP.
use artcraft_launcher_core::{curated, shifter};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};
use std::{collections::BTreeMap, sync::OnceLock};

use crate::LauncherUi;

#[derive(Default)]
pub(crate) struct Comparison {
    query: String,
    search_open: bool,
    search_filter: bool,
    search_keyboard: bool,
    search_index: Option<usize>,
    pub(crate) sources: Option<(String, Vec<String>)>,
    profile: Option<String>,
    selected: Vec<String>,
    required: Vec<String>,
    manual_name: Option<String>,
    manual_feature: String,
    manual_features: Vec<String>,
    manual_required: Vec<String>,
    app_query: String,
}
impl LauncherUi {
    pub(crate) fn open_comparison_profile(&mut self, id: &str) {
        if let Ok(data) = shifter::data()
            && let Some(profile) = data.profiles.iter().find(|profile| profile.id == id)
        {
            self.comparison.profile = Some(profile.id.clone());
            self.comparison.manual_name = None;
            self.comparison.query.clear();
            self.comparison.app_query.clear();
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
                self.shifter_search(&mut columns[1], data);
            });
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

    fn start_manual_comparison(&mut self, name: String) {
        self.comparison.manual_name = Some(name);
        self.comparison.profile = None;
        self.comparison.manual_features.clear();
        self.comparison.manual_required.clear();
        self.comparison.app_query.clear();
    }

    fn shifter_search(&mut self, ui: &mut Ui, data: &shifter::Dataset) {
        use egui::Key;
        ui.label(RichText::new(tl!("Paid software")).size(10.0).family(egui::FontFamily::Monospace));
        let search_id = ui.scope_id().with("shifter-search");
        let active = self.comparison.search_open || ui.memory(|memory| memory.has_focus(search_id));
        let down = active && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, Key::ArrowDown));
        let up = active && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, Key::ArrowUp));
        let enter = active && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, Key::Enter));
        let escape = self.comparison.search_open && ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, Key::Escape));
        let mut manual = false;
        let mut toggle = false;
        let search = ui
            .horizontal(|ui| {
                let search = ui
                    .add(egui::TextEdit::singleline(&mut self.comparison.query).id(search_id).desired_width(300.0))
                    .accessible_name(tl!("Paid software"));
                let arrow = ui.add(egui::Button::new("").min_size(egui::vec2(24.0, 24.0))).accessible_name(tl!("Paid software"));
                let center = arrow.rect.center();
                ui.painter().add(egui::Shape::convex_polygon(
                    vec![center + egui::vec2(-4.0, -2.0), center + egui::vec2(4.0, -2.0), center + egui::vec2(0.0, 3.0)],
                    ui.visuals().text_color(),
                    egui::Stroke::NONE,
                ));
                toggle = arrow.clicked();
                manual = ui.add_enabled(!self.comparison.query.trim().is_empty(), egui::Button::new(tl!("Compare manually"))).clicked();
                search
            })
            .inner;
        if toggle {
            self.comparison.search_open = !self.comparison.search_open;
            self.comparison.search_filter = false;
            self.comparison.search_keyboard = false;
            self.comparison.search_index = None;
            search.request_focus();
        }
        if search.clicked() || search.changed() {
            self.comparison.search_filter = search.changed();
            self.comparison.search_keyboard = false;
            self.comparison.search_open = true;
            self.comparison.search_index = None;
        }
        let choices = data.software(if self.comparison.search_filter { &self.comparison.query } else { "" });
        if ui.input(|input| input.key_pressed(Key::Tab)) {
            self.comparison.search_open = false;
            self.comparison.search_keyboard = false;
        }
        if ui.input(|input| input.pointer.any_pressed()) {
            self.comparison.search_keyboard = false;
        }
        let mut focus_choice = false;
        if (down || up) && !choices.is_empty() {
            self.comparison.search_open = true;
            self.comparison.search_index = Some(match self.comparison.search_index {
                None => {
                    if down {
                        0
                    } else {
                        choices.len() - 1
                    }
                }
                Some(index) => {
                    if down {
                        (index + 1).min(choices.len() - 1)
                    } else {
                        index.saturating_sub(1)
                    }
                }
            });
            focus_choice = true;
            self.comparison.search_keyboard = true;
        }
        let mut chosen = if enter { choices.get(self.comparison.search_index.unwrap_or(0)) } else { None };
        if escape {
            self.comparison.search_open = false;
            search.request_focus();
        }
        if enter && chosen.is_none() && !self.comparison.query.trim().is_empty() {
            manual = true;
        }
        let mut open = self.comparison.search_open && !escape && !manual;
        egui::Popup::from_response(&search)
            .open_bool(&mut open)
            .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
            .width(search.rect.width())
            .accessible_name(tl!("Paid software"))
            .show(|ui| {
                ui.set_width(search.rect.width());
                ui.spacing_mut().item_spacing.y = 0.0;
                egui::ScrollArea::vertical().id_salt("software-suggestions").max_height(280.0).show(ui, |ui| {
                    for (index, software) in choices.iter().enumerate() {
                        let row = ui.add_sized(
                            egui::vec2(ui.available_width(), 32.0),
                            egui::Button::selectable(self.comparison.search_index == Some(index), software.name),
                        );
                        if self.comparison.search_keyboard && self.comparison.search_index == Some(index) {
                            row.request_focus();
                            if focus_choice {
                                row.scroll_to_me(Some(egui::Align::Center));
                            }
                        }
                        if row.clicked() {
                            chosen = Some(software);
                        }
                    }
                });
            });
        self.comparison.search_open = open;
        ui.ctx().accesskit_node_builder(search.id, |node| node.set_expanded(open));
        if let Some(software) = chosen {
            if let Some(profile) = software.profile {
                self.open_comparison_profile(&profile.id);
            } else {
                self.start_manual_comparison(software.name.to_string());
            }
            self.comparison.search_filter = false;
            self.comparison.query = software.name.to_string();
            self.comparison.search_open = false;
            search.request_focus();
        } else if manual {
            self.start_manual_comparison(self.comparison.query.trim().to_owned());
            self.comparison.search_open = false;
            search.request_focus();
        }
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
            let (rows, search_changed) = self.shifter_result_search(ui, scores);
            let width = ui.available_width();
            let mut table = TableBuilder::new(ui)
                .id_salt(("shifter-results", &profile.id))
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(width * 0.27))
                .column(Column::remainder());
            if !required.is_empty() {
                table = table.column(Column::exact(width * 0.25));
            }
            table = table.column(Column::exact(200.0));
            if search_changed {
                table = table.vertical_scroll_offset(0.0);
            }
            table
                .header(32.0, |mut header| {
                    for title in [
                        Some(tl!("App")),
                        Some(tl!("Documented coverage")),
                        (!required.is_empty()).then(|| tl!("Required features")),
                        Some(tl!("Sources / details")),
                    ]
                    .into_iter()
                    .flatten()
                    {
                        header.col(|ui| {
                            ui.label(RichText::new(title).size(11.0).family(egui::FontFamily::Monospace));
                        });
                    }
                })
                .body(|body| {
                    body.rows(48.0, rows.len(), |mut row| {
                        let (score, app) = &rows[row.index()];
                        row.col(|ui| {
                            if ui.is_rect_visible(ui.max_rect()) {
                                self.catalog_media(app);
                            }
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
                                ui.label("—").on_hover_text(tl!("No verified evidence"));
                            }
                        });
                        if !required.is_empty() {
                            row.col(|ui| {
                                if score.missing_required.is_empty() {
                                    ui.label(tl!("All required features confirmed"));
                                } else if required.iter().any(|id| profile.evidence(&app.id, id).is_some()) {
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
                                } else {
                                    ui.label("—").on_hover_text(tl!("No verified evidence"));
                                }
                            });
                        }
                        row.col(|ui| {
                            if selected.iter().any(|id| profile.evidence(&app.id, id).is_some())
                                && ui.button(tl!("Sources")).accessible_name(format!("{} · {}", tl!("Sources"), app.name)).clicked()
                            {
                                self.comparison.sources = Some((app.id.clone(), selected.iter().map(|id| format!("{}:{id}", profile.id)).collect()));
                            }
                            if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                                self.curated.detail = Some(app.id.clone());
                            }
                        });
                    });
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
    }

    fn shifter_result_search(&mut self, ui: &mut Ui, scores: Vec<shifter::Score>) -> (Vec<(shifter::Score, &'static curated::App)>, bool) {
        let mut search_changed = false;
        let rows = ui
            .horizontal(|ui| {
                search_changed = ui
                    .add(egui::TextEdit::singleline(&mut self.comparison.app_query).hint_text(tl!("Search apps")).desired_width(230.0))
                    .accessible_name(tl!("Search apps"))
                    .changed();
                let total = scores.len();
                let rows = comparison_rows(scores, &self.comparison.app_query);
                let count = ui.label(format!("{} · {}/{}", tl!("All apps"), rows.len(), total));
                ui.ctx().accesskit_node_builder(count.id, |node| node.set_live(egui::accesskit::Live::Polite));
                rows
            })
            .inner;
        if rows.is_empty() {
            ui.label(tl!("No apps match"));
        }
        (rows, search_changed)
    }

    fn shifter_manual_results(&mut self, ui: &mut Ui, data: &shifter::Dataset, language: &str) {
        let scores = match data.compare_manual(&[], &self.comparison.manual_features, &self.comparison.manual_required) {
            Ok(scores) => scores,
            Err(error) => {
                ui.label(error);
                return;
            }
        };
        let (rows, search_changed) = self.shifter_result_search(ui, scores);
        if self.comparison.manual_features.is_empty() {
            ui.label(tl!("Select at least one feature. No percentage is calculated."));
        }
        let features = &self.comparison.manual_features;
        let required = &self.comparison.manual_required;
        egui::ScrollArea::horizontal().id_salt("manual-results-width").show(ui, |ui| {
            let mut table = TableBuilder::new(ui)
                .id_salt(("manual-results", &self.comparison.manual_name))
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(150.0).clip(true))
                .column(Column::exact(175.0));
            if !required.is_empty() {
                table = table.column(Column::exact(190.0));
            }
            table = table.columns(Column::exact(160.0).clip(true), features.len()).column(Column::exact(140.0));
            if search_changed {
                table = table.vertical_scroll_offset(0.0);
            }
            table
                .header(44.0, |mut header| {
                    for title in [Some(tl!("App")), Some(tl!("Documented coverage")), (!required.is_empty()).then(|| tl!("Required features"))]
                        .into_iter()
                        .flatten()
                    {
                        header.col(|ui| {
                            ui.strong(title);
                        });
                    }
                    for feature in features {
                        header.col(|ui| {
                            let name = manual_feature_name(data, feature, language);
                            ui.strong(&name).on_hover_text(name);
                        });
                    }
                    header.col(|ui| {
                        ui.strong(tl!("App details"));
                    });
                })
                .body(|body| {
                    body.rows(48.0, rows.len(), |mut row| {
                        let (score, app) = &rows[row.index()];
                        row.col(|ui| {
                            ui.label(&app.name).on_hover_text(&app.name);
                        });
                        row.col(|ui| {
                            if let Some(coverage) = score.coverage {
                                ui.label(format!("{coverage}% · {}/{}", score.supported, features.len()));
                            } else {
                                ui.label("—").on_hover_text(tl!("No verified evidence"));
                            }
                        });
                        if !required.is_empty() {
                            row.col(|ui| {
                                if score.missing_required.is_empty() {
                                    ui.label(tl!("All required features confirmed"));
                                } else if required.iter().any(|key| data.evidence(&app.id, key).is_some()) {
                                    let names = score
                                        .missing_required
                                        .iter()
                                        .map(|key| manual_feature_name(data, key, language))
                                        .collect::<Vec<_>>()
                                        .join(", ");
                                    ui.label(format!("⚠ {}: {}", tl!("Required feature not confirmed"), score.missing_required.len()))
                                        .on_hover_text(names);
                                } else {
                                    ui.label("—").on_hover_text(tl!("No verified evidence"));
                                }
                            });
                        }
                        for feature in features {
                            row.col(|ui| {
                                if let Some(evidence) = data.evidence(&app.id, feature) {
                                    evidence_cell(ui, Some(evidence), language);
                                } else {
                                    ui.label("—").on_hover_text(tl!("No verified evidence"));
                                }
                            });
                        }
                        row.col(|ui| {
                            if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                                self.curated.detail = Some(app.id.clone());
                            }
                        });
                    });
                });
        });
    }
}
fn comparison_rows(scores: Vec<shifter::Score>, query: &str) -> Vec<(shifter::Score, &'static curated::App)> {
    static APPS: OnceLock<BTreeMap<&str, (&curated::App, String)>> = OnceLock::new();
    let apps = APPS.get_or_init(|| {
        curated::apps()
            .iter()
            .map(|app| {
                let searchable = shifter::normalize(&format!(
                    "{} {} {} {} {} {} {}",
                    app.id,
                    app.name,
                    app.author,
                    app.category,
                    app.description.it,
                    app.description.en,
                    app.alternatives.join(" "),
                ));
                (app.id.as_str(), (app, searchable))
            })
            .collect()
    });
    let query = shifter::normalize(query);
    scores
        .into_iter()
        .filter_map(|score| {
            let (app, searchable) = apps.get(score.app_id.as_str())?;
            searchable.contains(&query).then_some((score, *app))
        })
        .collect()
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
