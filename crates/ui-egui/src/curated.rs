//! Curated discovery delegates every managed action to the existing native UI flow.
use crate::LauncherUi;
use artcraft_launcher_core::curated;

#[derive(Default)]
pub(crate) struct Catalog {
    search: String,
    level: String,
    author: String,
    category: String,
    stars: bool,
}
impl LauncherUi {
    pub(crate) fn curated_page(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.set_width(180.0);
                if ui.selectable_label(self.curated.category.is_empty(), tl!("All apps")).clicked() {
                    self.curated.category.clear();
                }
                for (id, label) in CATEGORIES {
                    let count = curated::apps().iter().filter(|app| app.category == *id).count();
                    if ui.selectable_label(self.curated.category == *id, format!("{} ({count})", crate::i18n::t(label))).clicked() {
                        self.curated.category = id.to_string();
                    }
                }
            });
            ui.separator();
            ui.vertical(|ui| self.curated_contents(ui));
        });
    }

    fn curated_contents(&mut self, ui: &mut egui::Ui) {
        let t = crate::theme::Tokens::get(ui.ctx());
        let all = curated::apps();
        ui.horizontal_wrapped(|ui| {
            ui.text_edit_singleline(&mut self.curated.search).labelled_by(ui.label(tl!("Search")).id);
            for (value, label) in [("", tl!("All")), ("base", tl!("Basic")), ("avanz", tl!("Advanced"))] {
                ui.selectable_value(&mut self.curated.level, value.to_string(), label);
            }
            for (id, label, field, values) in [
                (
                    "curated-author",
                    tl!("Publisher"),
                    &mut self.curated.author,
                    all.iter().map(|a| a.author.as_str()).collect::<std::collections::BTreeSet<_>>(),
                ),
                ("curated-category", tl!("Category"), &mut self.curated.category, all.iter().map(|a| a.category.as_str()).collect()),
            ] {
                craft_ui::ComboBox::from_id_salt(id).selected_text(if field.is_empty() { label } else { field.as_str() }).show_ui(ui, |ui| {
                    ui.selectable_value(field, String::new(), tl!("All"));
                    for value in values {
                        ui.selectable_value(field, value.to_string(), value);
                    }
                });
            }
            ui.checkbox(&mut self.curated.stars, tl!("GitHub stars"));
            crate::bar::view_toggle(ui, &mut self.settings.view, [tl!("Show apps as cards"), tl!("Show apps as a table")]);
            if ui.button(tl!("Reset filters")).clicked() {
                self.curated = Catalog::default();
            }
        });
        let q = self.curated.search.to_lowercase();
        let mut apps: Vec<_> = all
            .iter()
            .filter(|a| {
                (self.curated.level.is_empty() || a.level == self.curated.level)
                    && (self.curated.author.is_empty() || a.author == self.curated.author)
                    && (self.curated.category.is_empty() || a.category == self.curated.category)
                    && format!("{} {} {} {} {} {}", a.name, a.author, a.category, a.description.en, a.description.it, a.alternatives.join(" "))
                        .to_lowercase()
                        .contains(&q)
            })
            .collect();
        if self.curated.stars {
            apps.sort_by_key(|a| std::cmp::Reverse(a.stars.as_ref().map_or(0, |s| s.count)));
        }
        ui.label(format!("{} · {}", tl!("Catalog"), apps.len()));
        egui::ScrollArea::vertical().show(ui, |ui| {
            let columns =
                if self.settings.view == crate::settings::ViewMode::Table { 1 } else { (ui.available_width() / 350.0).floor().max(1.0) as usize };
            let width = (ui.available_width() / columns as f32 - 12.0).max(240.0);
            for row in apps.chunks(columns) {
                ui.horizontal_top(|ui| {
                    for app in row {
                        egui::Frame::new().stroke(t.hairline()).inner_margin(14).show(ui, |ui| {
                            ui.set_width(width);
                            ui.heading(&app.name);
                            ui.label(&app.author);
                            ui.label(&app.description.en);
                            ui.label(&app.license);
                            ui.label(format!("{}: {}", tl!("Alternative to"), app.alternatives.join(", ")));
                            ui.hyperlink_to("GitHub / source", &app.repo);
                            ui.hyperlink_to(tl!("Website"), &app.website);
                            if let Some(stars) = &app.stars {
                                ui.hyperlink_to(format!("★ {} · {}", stars.count, stars.date), &stars.source);
                            }
                            if let Some(release) = &app.release {
                                ui.hyperlink_to(format!("{} · {}", release.version, release.date), &release.source);
                                for package in &release.packages {
                                    let size = match (package.size_min, package.size_max) {
                                        (Some(min), Some(max)) if min != max => {
                                            format!("{:.1}–{:.1} MB", min as f64 / 1_000_000.0, max as f64 / 1_000_000.0)
                                        }
                                        (Some(size), _) | (_, Some(size)) => format!("{:.1} MB", size as f64 / 1_000_000.0),
                                        _ => "—".to_string(),
                                    };
                                    ui.label(format!("{} · {size}", package.platform));
                                }
                            }
                            if let Some(source) = &app.agent.source {
                                ui.hyperlink_to(format!("{} · {}", tl!("Agent control"), app.agent.kind), source);
                            }
                            if let Some(entry) = self
                                .entries
                                .iter()
                                .find(|e| e.app.id == app.id && app.repo.trim_end_matches('/') == format!("https://github.com/{}", e.app.repo))
                                .cloned()
                            {
                                let status = entry.status(self.platform, self.caps.manage_apps);
                                self.actions_in(ui, &entry, &status, width, crate::views::Place::Row);
                                if ui.button(tl!("App details")).clicked() {
                                    self.detail = Some(app.id.clone());
                                }
                            } else {
                                ui.label(tl!("Managed installation not available"));
                            }
                        });
                    }
                });
            }
            if apps.is_empty() {
                ui.label(tl!("No apps match"));
            }
            ui.label("Powered by Promptica · ArtCraft engine · MIT / Apache-2.0");
        });
    }
}

const CATEGORIES: &[(&str, &str)] = &[
    ("ufficio", "Office and PDF"),
    ("foto", "Photo and images"),
    ("design", "Design and 3D"),
    ("video", "Audio, video and animation"),
    ("voce", "Voice and meetings"),
    ("comm", "Communication"),
    ("sviluppo", "Coding and agents"),
    ("re", "Reverse engineering"),
    ("agenti", "AI assistants"),
    ("sic", "Defensive security"),
    ("infra", "Monitoring"),
    ("sistema", "System and files"),
    ("rete", "Networking"),
    ("dati", "Data and analytics"),
];
