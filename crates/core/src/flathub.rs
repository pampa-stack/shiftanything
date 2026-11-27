//! Reviewed Flathub identity and target scope for the launcher.
use crate::{Arch, Os, Platform, curated::App};

pub const RECIPES: [(&str, &str); 4] = [
    ("gnome-calculator", "org.gnome.Calculator"),
    ("gnome-text-editor", "org.gnome.TextEditor"),
    ("loupe", "org.gnome.Loupe"),
    ("papers", "org.gnome.Papers"),
];

pub const IDS: [&str; 4] = [RECIPES[0].0, RECIPES[1].0, RECIPES[2].0, RECIPES[3].0];

pub fn supported(id: &str, platform: Platform) -> bool {
    platform.os == Os::Linux && platform.arch == Arch::X64 && IDS.contains(&id)
}

pub fn approved_catalog_identity(app: &App) -> bool {
    RECIPES.iter().any(|(id, flatpak_id)| {
        app.id == *id
            && app.repo.trim_end_matches('/') == format!("https://gitlab.gnome.org/GNOME/{id}")
            && app.distribution.iter().any(|channel| {
                channel.platform == "linux"
                    && channel.manager == "flatpak"
                    && channel.id == *flatpak_id
                    && channel.source == format!("https://flathub.org/apps/{flatpak_id}")
            })
    })
}
