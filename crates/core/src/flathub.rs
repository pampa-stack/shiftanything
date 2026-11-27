//! Reviewed Flathub identity and target scope for the launcher.
use crate::{Arch, Os, Platform, curated::App};

pub const RECIPES: [(&str, &str); 17] = [
    ("gnome-calculator", "org.gnome.Calculator"),
    ("gnome-text-editor", "org.gnome.TextEditor"),
    ("loupe", "org.gnome.Loupe"),
    ("papers", "org.gnome.Papers"),
    ("krita", "org.kde.krita"),
    ("obs", "com.obsproject.Studio"),
    ("celluloid", "io.github.celluloid_player.Celluloid"),
    ("gimp", "org.gimp.GIMP"),
    ("inks", "org.inkscape.Inkscape"),
    ("kdenlive", "org.kde.kdenlive"),
    ("pika-backup", "org.gnome.World.PikaBackup"),
    ("tenacity", "org.tenacityaudio.Tenacity"),
    ("peazip", "io.github.peazip.PeaZip"),
    ("pinta", "com.github.PintaProject.Pinta"),
    ("keepassxc", "org.keepassxc.KeePassXC"),
    ("xournal", "com.github.xournalpp.xournalpp"),
    ("libreoffice", "org.libreoffice.LibreOffice"),
];

pub const IDS: [&str; RECIPES.len()] = {
    let mut ids = [""; RECIPES.len()];
    let mut index = 0;
    while index < RECIPES.len() {
        ids[index] = RECIPES[index].0;
        index += 1;
    }
    ids
};

pub fn supported(id: &str, platform: Platform) -> bool {
    platform.os == Os::Linux && platform.arch == Arch::X64 && IDS.contains(&id)
}

pub fn approved_catalog_identity(app: &App) -> bool {
    let upstream = match app.id.as_str() {
        "gnome-calculator" => "https://gitlab.gnome.org/GNOME/gnome-calculator",
        "gnome-text-editor" => "https://gitlab.gnome.org/GNOME/gnome-text-editor",
        "loupe" => "https://gitlab.gnome.org/GNOME/loupe",
        "papers" => "https://gitlab.gnome.org/GNOME/papers",
        "krita" => "https://invent.kde.org/graphics/krita",
        "obs" => "https://github.com/obsproject/obs-studio",
        "celluloid" => "https://github.com/celluloid-player/celluloid",
        "gimp" => "https://gitlab.gnome.org/GNOME/gimp",
        "inks" => "https://gitlab.com/inkscape/inkscape",
        "kdenlive" => "https://invent.kde.org/multimedia/kdenlive",
        "pika-backup" => "https://gitlab.gnome.org/World/pika-backup",
        "tenacity" => "https://codeberg.org/tenacityteam/tenacity",
        "peazip" => "https://github.com/peazip/PeaZip",
        "pinta" => "https://github.com/PintaProject/Pinta",
        "keepassxc" => "https://github.com/keepassxreboot/keepassxc",
        "xournal" => "https://github.com/xournalpp/xournalpp",
        "libreoffice" => "https://git.libreoffice.org/core",
        _ => return false,
    };
    RECIPES.iter().any(|(id, flatpak_id)| {
        app.id == *id
            && app.repo.trim_end_matches('/') == upstream
            && app.distribution.iter().any(|channel| {
                channel.platform == "linux"
                    && channel.manager == "flatpak"
                    && channel.id == *flatpak_id
                    && channel.source == format!("https://flathub.org/apps/{flatpak_id}")
            })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reviewed_flatpak_ids_match_canonical_curated_channels_only_on_linux_x64() {
        let linux = Platform { os: Os::Linux, arch: Arch::X64 };
        let arm = Platform { arch: Arch::Arm64, ..linux };
        let mac = Platform { os: Os::MacOs, arch: Arch::Arm64 };
        for id in IDS {
            let app = crate::curated::apps().iter().find(|app| app.id == id).unwrap();
            assert!(approved_catalog_identity(app), "{id} catalog identity");
            assert!(supported(id, linux), "{id} Linux x64");
            assert!(!supported(id, arm), "{id} Linux arm64");
            assert!(!supported(id, mac), "{id} macOS");
        }
        assert!(!supported("vlc", linux));
    }
}
