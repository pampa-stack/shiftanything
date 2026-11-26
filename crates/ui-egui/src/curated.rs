//! Curated discovery delegates every managed action to the existing native UI flow.
use crate::LauncherUi;
use artcraft_launcher_core::curated;

#[derive(Default)]
pub(crate) struct Catalog {
    search: String,
    pub(crate) level: String,
    author: String,
    category: String,
    sort: String,
    pub(crate) detail: Option<String>,
}
impl LauncherUi {
    pub(crate) fn curated_page(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_top(|ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().interact_size.y = 24.0;
                ui.spacing_mut().button_padding.y = 4.0;
                ui.spacing_mut().item_spacing.y = 3.0;
                ui.set_width(194.0);
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
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                self.curated_contents(ui);
            });
        });
    }

    fn curated_contents(&mut self, ui: &mut egui::Ui) {
        let t = crate::theme::Tokens::get(ui.ctx());
        let all = curated::apps();
        ui.heading(tl!("All apps"));
        ui.add_space(12.0);
        ui.horizontal_wrapped(|ui| {
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
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(label).size(10.0).color(t.muted));
                    let category_label = |value: &str| {
                        CATEGORIES
                            .iter()
                            .find(|(key, _)| *key == value)
                            .map_or_else(|| value.to_string(), |(_, label)| crate::i18n::t(label).to_string())
                    };
                    let selected = if field.is_empty() {
                        tl!("All").to_string()
                    } else if id == "curated-category" {
                        category_label(field)
                    } else {
                        field.clone()
                    };
                    craft_ui::ComboBox::from_id_salt(id)
                        .selected_text(selected)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(field, String::new(), tl!("All"));
                            for value in values {
                                let display = if id == "curated-category" { category_label(value) } else { value.to_string() };
                                ui.selectable_value(field, value.to_string(), display);
                            }
                        })
                        .response
                        .accessible_name(label);
                });
            }
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(tl!("Sort")).size(10.0).color(t.muted));
                craft_ui::ComboBox::from_id_salt("catalog-sort")
                    .selected_text(match self.curated.sort.as_str() {
                        "stars" => tl!("GitHub stars"),
                        "name" => tl!("Name"),
                        "latest" => tl!("Latest release"),
                        _ => tl!("Featured"),
                    })
                    .show_ui(ui, |ui| {
                        for (value, label) in
                            [("", tl!("Featured")), ("name", tl!("Name")), ("stars", tl!("GitHub stars")), ("latest", tl!("Latest release"))]
                        {
                            ui.selectable_value(&mut self.curated.sort, value.to_string(), label);
                        }
                    })
                    .response
                    .accessible_name(tl!("Sort"));
            });
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
        match self.curated.sort.as_str() {
            "stars" => apps.sort_by_key(|a| std::cmp::Reverse(a.stars.as_ref().map_or(0, |s| s.count))),
            "name" => apps.sort_by_key(|a| a.name.to_lowercase()),
            "latest" => apps.sort_by_key(|a| std::cmp::Reverse(a.release.as_ref().map(|release| release.date.as_str()))),
            _ => {}
        }
        ui.label(format!("{} · {}", tl!("Catalog"), apps.len()));
        egui::ScrollArea::vertical().show(ui, |ui| {
            if self.settings.view == crate::settings::ViewMode::Table {
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    egui::Grid::new("curated-table").striped(true).min_col_width(85.0).max_col_width(160.0).show(ui, |ui| {
                        for title in [tl!("Name"), tl!("Publisher"), tl!("Agent control"), tl!("License"), tl!("Popularity"), tl!("App details")] {
                            ui.strong(title);
                        }
                        ui.end_row();
                        for app in &apps {
                            if ui.is_rect_visible(egui::Rect::from_min_size(ui.cursor().min, egui::vec2(ui.available_width(), 48.0))) {
                                self.catalog_media(app);
                            }
                            ui.horizontal(|ui| {
                                if let Some(icon) = self.picture_source(&app.id, "catalog-logo").or_else(|| crate::icons::app(&app.id)) {
                                    ui.add(egui::Image::new(icon).fit_to_exact_size(egui::vec2(28.0, 28.0)).alt_text(format!("{} logo", app.name)));
                                }
                                if ui.button(&app.name).clicked() {
                                    self.curated.detail = Some(app.id.clone());
                                }
                            });
                            ui.label(&app.author);
                            let control = match app.agent.kind.as_str() {
                                "mcp" => tl!("MCP available"),
                                "cli" => tl!("CLI control"),
                                "api" => tl!("API control"),
                                _ => tl!("Control unverified"),
                            };
                            if let Some(source) = &app.agent.source {
                                ui.hyperlink_to(control, source);
                            } else {
                                ui.label(control);
                            }
                            ui.hyperlink_to(&app.license, &app.license_url);
                            if let Some(stars) = &app.stars {
                                let count = if stars.count >= 1000 {
                                    format!("★ {:.1}k", stars.count as f64 / 1000.0)
                                } else {
                                    format!("★ {}", stars.count)
                                };
                                ui.hyperlink_to(egui::RichText::new(count).color(egui::Color32::from_rgb(187, 143, 45)), &stars.source)
                                    .on_hover_text(&stars.date);
                            } else if let Some(popularity) = &app.popularity {
                                ui.hyperlink_to(popularity.label(), &popularity.page).on_hover_text(&popularity.date);
                            } else {
                                ui.label("—");
                            }
                            ui.horizontal(|ui| {
                                ui.hyperlink_to(if app.repo.starts_with("https://github.com/") { "GitHub" } else { tl!("Source code") }, &app.repo);
                                if let Some(entry) = self
                                    .entries
                                    .iter()
                                    .find(|entry| {
                                        entry.app.id == app.id && app.repo.trim_end_matches('/') == format!("https://github.com/{}", entry.app.repo)
                                    })
                                    .cloned()
                                {
                                    let status = entry.status(self.platform, self.caps.manage_apps);
                                    self.catalog_action(ui, &entry, &status, 112.0);
                                } else {
                                    ui.label("—").on_hover_text(tl!("Managed installation not available"));
                                }
                                if ui.button(tl!("App details")).accessible_name(format!("{} · {}", tl!("App details"), app.name)).clicked() {
                                    self.curated.detail = Some(app.id.clone());
                                }
                            });
                            ui.end_row();
                        }
                    });
                });
                if apps.is_empty() {
                    ui.label(tl!("No apps match"));
                }
                return;
            }
            let columns = (ui.available_width() / 350.0).floor().max(1.0) as usize;
            let width = ((ui.available_width() - (columns - 1) as f32 * ui.spacing().item_spacing.x) / columns as f32 - 28.0).max(80.0);
            for row in apps.chunks(columns) {
                ui.horizontal_top(|ui| {
                    for app in row {
                        egui::Frame::new().stroke(t.hairline()).inner_margin(14).show(ui, |ui| {
                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                ui.set_width(width);
                                ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Wrap);
                                self.curated_card(ui, app, width);
                            });
                        });
                    }
                });
            }
            if apps.is_empty() {
                ui.label(tl!("No apps match"));
            }
        });
    }
    pub(crate) fn curated_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(tl!("Check for updates")).clicked() {
                self.check_now_from_ui();
            }
            crate::bar::view_toggle(ui, &mut self.settings.view, [tl!("Show apps as cards"), tl!("Show apps as a table")]);
            ui.add(egui::TextEdit::singleline(&mut self.curated.search).hint_text(tl!("Search apps")).desired_width(230.0))
                .accessible_name(tl!("Search apps"));
        });
    }

    pub(crate) fn catalog_media(&mut self, app: &curated::App) {
        let key = (app.id.clone(), "catalog-logo".into());
        if let std::collections::hash_map::Entry::Vacant(entry) = self.pictures.entry(key) {
            entry.insert(crate::details::Picture::Loading);
            self.act_command(artcraft_launcher_core::protocol::Command::FetchCatalogMedia { app: app.id.clone() });
        }
    }

    fn curated_card(&mut self, ui: &mut egui::Ui, app: &curated::App, width: f32) {
        let t = crate::theme::Tokens::get(ui.ctx());
        ui.style_mut().override_font_id = Some(egui::FontId::proportional(12.0));
        if ui.is_rect_visible(egui::Rect::from_min_size(ui.cursor().min, egui::vec2(width, 380.0))) {
            self.catalog_media(app);
        }
        ui.horizontal_top(|ui| {
            let icon = self.picture_source(&app.id, "catalog-logo").or_else(|| crate::icons::app(&app.id));
            if let Some(icon) = icon {
                ui.add(egui::Image::new(icon).fit_to_exact_size(egui::vec2(40.0, 40.0)).alt_text(format!("{} logo", app.name)));
            }
            ui.vertical(|ui| {
                ui.label(egui::RichText::new(&app.name).font(egui::FontId::new(18.0, crate::theme::display())));
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(&app.author).size(11.0).color(t.muted));
                    if let Some(stars) = &app.stars {
                        let label =
                            if stars.count >= 1000 { format!("★ {:.1}k", stars.count as f64 / 1000.0) } else { format!("★ {}", stars.count) };
                        ui.hyperlink_to(egui::RichText::new(label).size(11.0).color(egui::Color32::from_rgb(187, 143, 45)), &stars.source)
                            .on_hover_text(&stars.date);
                    } else if let Some(popularity) = &app.popularity
                        && popularity.kind == "flathub_installs"
                    {
                        ui.hyperlink_to(popularity.label(), &popularity.page).on_hover_text(format!("{}\n{}", popularity.date, popularity.source));
                    }
                });
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                ui.hyperlink_to(if app.repo.starts_with("https://github.com/") { "GitHub" } else { tl!("Source code") }, &app.repo);
            });
        });
        ui.add(egui::Label::new(if crate::i18n::current().code() == "it" { &app.description.it } else { &app.description.en }).truncate());
        let size = egui::vec2(width, 134.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
        ui.painter().rect_filled(rect, 4.0, t.sunken);
        if let Some(source) = self.picture_source(&app.id, "catalog-shot-0") {
            ui.scope_builder(egui::UiBuilder::new().max_rect(rect).layout(egui::Layout::top_down(egui::Align::Center)), |ui| {
                ui.add(egui::Image::new(source).fit_to_exact_size(size).alt_text(format!("{} preview", app.name)));
            });
        } else {
            ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, tl!("Preview unavailable"), egui::FontId::proportional(12.0), t.muted);
        }
        response.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, format!("{} · {}", tl!("App details"), app.name)));
        if response.clicked() {
            self.curated.detail = Some(app.id.clone());
        }
        ui.horizontal(|ui| {
            ui.label(if app.delivery == "self-hosted" { tl!("Self-hosted") } else { tl!("Desktop app") });
            ui.hyperlink_to(&app.license, &app.license_url);
            let control = match app.agent.kind.as_str() {
                "mcp" => "MCP",
                "cli" => tl!("CLI control"),
                "api" => tl!("API control"),
                _ => tl!("Control unverified"),
            };
            if let Some(source) = &app.agent.source {
                ui.hyperlink_to(control, source);
            } else {
                ui.label(control);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                for (platform, label, image) in [
                    // This layout places the first item at the right edge.
                    ("linux", "Linux", egui::include_image!("../../../assets/shiftanything/linux.png")),
                    ("windows", "Windows", egui::include_image!("../../../assets/shiftanything/windows.png")),
                    ("macos", "macOS", egui::include_image!("../../../assets/shiftanything/macos.svg")),
                ] {
                    if let Some(source) = app.platform_source(platform) {
                        ui.add(egui::Image::new(image).fit_to_exact_size(egui::vec2(15.0, 15.0)).alt_text(label))
                            .on_hover_text(format!("{label}\n{source}"));
                    }
                }
            });
        });
        if !app.alternatives.is_empty() {
            ui.label(format!("{} · {}", tl!("Alternative to"), app.alternatives.join(", ")));
        }
        ui.columns(2, |columns| {
            columns[0].label(egui::RichText::new(tl!("Last update")).color(t.muted).size(10.0));
            columns[1].label(egui::RichText::new(tl!("Download size")).color(t.muted).size(10.0));
            if let Some(release) = &app.release {
                columns[0].hyperlink_to(format!("{} · {}", release.date, release.version), &release.source);
            } else {
                columns[0].label("—");
            }
            let os = self.platform.map(|p| match p.os {
                artcraft_launcher_core::Os::MacOs => "macos",
                artcraft_launcher_core::Os::Windows => "windows",
                artcraft_launcher_core::Os::Linux => "linux",
                artcraft_launcher_core::Os::FreeBsd => "freebsd",
            });
            if let Some(size) = app.download_size(os) {
                let platform = match size.platform {
                    "macos" => "macOS",
                    "windows" => "Windows",
                    "linux" => "Linux",
                    "freebsd" => "FreeBSD",
                    other => other,
                };
                let range =
                    size.maximum.filter(|max| *max != size.minimum).map(|max| format!("–{:.1}", max as f64 / 1_000_000.0)).unwrap_or_default();
                let channel = size.manager.map(|manager| format!(" / {manager}")).unwrap_or_default();
                columns[1].hyperlink_to(format!("{:.1}{range} MB · {platform}{channel}", size.minimum as f64 / 1_000_000.0), size.source);
            } else {
                columns[1].label("—");
            }
        });
        ui.horizontal(|ui| {
            let action_width = (width - ui.spacing().item_spacing.x) / 2.0;
            if let Some(entry) = self
                .entries
                .iter()
                .find(|e| e.app.id == app.id && app.repo.trim_end_matches('/') == format!("https://github.com/{}", e.app.repo))
                .cloned()
            {
                let status = entry.status(self.platform, self.caps.manage_apps);
                self.catalog_action(ui, &entry, &status, action_width);
            } else {
                ui.add_sized([action_width, crate::theme::CONTROL_H], egui::Label::new(tl!("Managed installation not available")).wrap());
            }
            if ui.add_sized([action_width, crate::theme::CONTROL_H], egui::Button::new(tl!("App details"))).clicked() {
                self.curated.detail = Some(app.id.clone());
            }
        });
    }

    pub(crate) fn curated_details(&mut self, ctx: &egui::Context) {
        let Some(id) = self.curated.detail.clone() else { return };
        let Some(app) = curated::apps().iter().find(|app| app.id == id) else {
            self.curated.detail = None;
            return;
        };
        let mut open = true;
        egui::Window::new(&app.name).open(&mut open).default_width(680.0).show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height(ctx.content_rect().height() * 0.75).show(ui, |ui| {
                self.curated_card(ui, app, ui.available_width());
                ui.hyperlink_to(tl!("Website"), &app.website);
                for index in 1..app.screenshots.len() {
                    if let Some(source) = self.picture_source(&id, &format!("catalog-shot-{index}")) {
                        ui.add(egui::Image::new(source).max_width(ui.available_width()).alt_text(format!("{} {}", app.name, index + 1)));
                    }
                }
                ui.hyperlink_to(tl!("Media source"), app.media_source.as_str().unwrap_or(&app.repo));
                if self
                    .entries
                    .iter()
                    .any(|entry| entry.app.id == id && app.repo.trim_end_matches('/') == format!("https://github.com/{}", entry.app.repo))
                    && ui.button(tl!("Manage app")).clicked()
                {
                    self.detail = Some(id.clone());
                    self.curated.detail = None;
                }
            });
        });
        if !open || ctx.input(|input| input.key_pressed(egui::Key::Escape)) {
            self.curated.detail = None;
        }
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
