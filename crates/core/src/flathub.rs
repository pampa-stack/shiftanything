//! Reviewed Flathub identity and target scope for the launcher.
use crate::{Arch, Os, Platform, curated::App};

pub const RECIPES: [(&str, &str); 88] = [
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
    ("pdfarranger", "com.github.jeromerobert.pdfarranger"),
    ("okular", "org.kde.okular"),
    ("flameshot", "org.flameshot.Flameshot"),
    ("handbrake", "fr.handbrake.ghb"),
    ("shotcut", "org.shotcut.Shotcut"),
    ("free", "org.freecad.FreeCAD"),
    ("d-j-dup-backups", "org.gnome.DejaDup"),
    ("rnote", "com.github.flxzt.rnote"),
    ("mixxx-dj-software", "org.mixxx.Mixxx"),
    ("cherrytree", "net.giuspen.cherrytree"),
    ("foliate", "com.github.johnfactotum.Foliate"),
    ("pdf4qt", "io.github.JakubMelka.Pdf4qt"),
    ("gnome-calendar", "org.gnome.Calendar"),
    ("qownnotes", "org.qownnotes.QOwnNotes"),
    ("qview", "com.interversehq.qView"),
    ("czkawka", "com.github.qarmin.czkawka"),
    ("drawpile", "net.drawpile.drawpile"),
    ("kid3", "org.kde.kid3"),
    ("super-productivity", "com.super_productivity.SuperProductivity"),
    ("kate", "org.kde.kate"),
    ("ark", "org.kde.ark"),
    ("filelight", "org.kde.filelight"),
    ("glaxnimate", "org.kde.glaxnimate"),
    ("evolution", "org.gnome.Evolution"),
    ("freac", "org.freac.freac"),
    ("gcompris", "org.kde.gcompris"),
    ("simple-scan", "org.gnome.SimpleScan"),
    ("gnome-clocks", "org.gnome.clocks"),
    ("apostrophe", "org.gnome.gitlab.somas.Apostrophe"),
    ("librepcb", "org.librepcb.LibrePCB"),
    ("subtitlecomposer", "org.kde.subtitlecomposer"),
    ("qalculate", "io.github.Qalculate"),
    ("shotwell", "org.gnome.Shotwell"),
    ("remmina", "org.remmina.Remmina"),
    ("labplot", "org.kde.labplot"),
    ("upscayl", "org.upscayl.Upscayl"),
    ("db-browser-for-sqlite", "org.sqlitebrowser.sqlitebrowser"),
    ("showtime", "org.gnome.Showtime"),
    ("ruffle", "rs.ruffle.Ruffle"),
    ("tux-paint", "org.tuxpaint.Tuxpaint"),
    ("dolphin", "org.kde.dolphin"),
    ("synfig-studio", "org.synfig.SynfigStudio"),
    ("kicad", "org.kicad.KiCad"),
    ("strawberry-music-player", "org.strawberrymusicplayer.strawberry"),
    ("builder", "org.gnome.Builder"),
    ("digikam", "org.kde.digikam"),
    ("icon-library", "org.gnome.design.IconLibrary"),
    ("trilium", "org.triliumnotes.Trilium"),
    ("friction", "graphics.friction.Friction"),
    ("vokoscreenng", "com.github.vkohaupt.vokoscreenNG"),
    ("firefox", "org.mozilla.firefox"),
    ("vscodium", "com.vscodium.codium"),
    ("librecad", "org.librecad.librecad"),
    ("gnucash", "org.gnucash.GnuCash"),
    ("rssguard", "io.github.martinrotter.rssguard"),
    ("nomacs", "org.nomacs.ImageLounge"),
    ("seamly", "io.seamly.seamly2d"),
    ("flatseal", "com.github.tchx84.Flatseal"),
    ("anki", "net.ankiweb.Anki"),
    ("evince", "org.gnome.Evince"),
    ("qgis", "org.qgis.qgis"),
    ("gnome-maps", "org.gnome.Maps"),
    ("logseq", "com.logseq.Logseq"),
    ("leocad", "org.leocad.LeoCAD"),
    ("appflowy", "io.appflowy.AppFlowy"),
    ("zen", "app.zen_browser.zen"),
    ("openra", "net.openra.OpenRA"),
    ("openscad", "org.openscad.OpenSCAD"),
    ("stellarium", "org.stellarium.Stellarium"),
    ("qbittorrent", "org.qbittorrent.qBittorrent"),
    ("copyq", "com.github.hluk.copyq"),
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
        "pdfarranger" => "https://github.com/pdfarranger/pdfarranger",
        "okular" => "https://invent.kde.org/graphics/okular",
        "flameshot" => "https://github.com/flameshot-org/flameshot",
        "handbrake" => "https://github.com/HandBrake/HandBrake",
        "shotcut" => "https://github.com/mltframework/shotcut",
        "free" => "https://github.com/FreeCAD/FreeCAD",
        "d-j-dup-backups" => "https://gitlab.gnome.org/World/deja-dup",
        "rnote" => "https://github.com/flxzt/rnote",
        "mixxx-dj-software" => "https://github.com/mixxxdj/mixxx",
        "cherrytree" => "https://github.com/giuspen/cherrytree",
        "foliate" => "https://github.com/johnfactotum/foliate",
        "pdf4qt" => "https://github.com/JakubMelka/PDF4QT",
        "gnome-calendar" => "https://gitlab.gnome.org/GNOME/gnome-calendar",
        "qownnotes" => "https://github.com/pbek/QOwnNotes",
        "qview" => "https://github.com/jurplel/qView",
        "czkawka" => "https://github.com/qarmin/czkawka",
        "drawpile" => "https://github.com/drawpile/Drawpile",
        "kid3" => "https://invent.kde.org/multimedia/kid3",
        "super-productivity" => "https://github.com/super-productivity/super-productivity",
        "kate" => "https://invent.kde.org/utilities/kate",
        "ark" => "https://invent.kde.org/utilities/ark",
        "filelight" => "https://invent.kde.org/utilities/filelight",
        "glaxnimate" => "https://invent.kde.org/graphics/glaxnimate",
        "evolution" => "https://gitlab.gnome.org/GNOME/evolution",
        "freac" => "https://github.com/enzo1982/freac",
        "gcompris" => "https://invent.kde.org/education/gcompris",
        "simple-scan" => "https://gitlab.gnome.org/GNOME/simple-scan",
        "gnome-clocks" => "https://gitlab.gnome.org/GNOME/gnome-clocks",
        "apostrophe" => "https://gitlab.gnome.org/World/apostrophe",
        "librepcb" => "https://github.com/LibrePCB/LibrePCB",
        "subtitlecomposer" => "https://invent.kde.org/multimedia/subtitlecomposer",
        "qalculate" => "https://github.com/Qalculate/qalculate-gtk",
        "shotwell" => "https://gitlab.gnome.org/GNOME/shotwell",
        "remmina" => "https://gitlab.com/Remmina/Remmina",
        "labplot" => "https://invent.kde.org/education/labplot",
        "upscayl" => "https://github.com/upscayl/upscayl",
        "db-browser-for-sqlite" => "https://github.com/sqlitebrowser/sqlitebrowser",
        "showtime" => "https://gitlab.gnome.org/GNOME/showtime",
        "ruffle" => "https://github.com/ruffle-rs/ruffle",
        "tux-paint" => "https://sourceforge.net/p/tuxpaint/tuxpaint/ci/master/tree",
        "dolphin" => "https://invent.kde.org/system/dolphin",
        "synfig-studio" => "https://github.com/synfig/synfig",
        "kicad" => "https://gitlab.com/kicad/code/kicad",
        "strawberry-music-player" => "https://github.com/strawberrymusicplayer/strawberry",
        "builder" => "https://gitlab.gnome.org/GNOME/gnome-builder",
        "digikam" => "https://invent.kde.org/graphics/digikam",
        "icon-library" => "https://gitlab.gnome.org/World/design/icon-library",
        "trilium" => "https://github.com/TriliumNext/Trilium",
        "friction" => "https://github.com/friction2d/friction",
        "vokoscreenng" => "https://github.com/vkohaupt/vokoscreenNG",
        "firefox" => "https://github.com/mozilla-firefox/firefox",
        "vscodium" => "https://github.com/VSCodium/vscodium",
        "librecad" => "https://github.com/LibreCAD/LibreCAD",
        "gnucash" => "https://github.com/Gnucash/gnucash",
        "rssguard" => "https://github.com/martinrotter/rssguard",
        "nomacs" => "https://github.com/nomacs/nomacs",
        "seamly" => "https://github.com/FashionFreedom/Seamly2D",
        "flatseal" => "https://github.com/tchx84/Flatseal",
        "anki" => "https://github.com/ankitects/anki",
        "evince" => "https://gitlab.gnome.org/GNOME/evince",
        "qgis" => "https://github.com/qgis/QGIS",
        "gnome-maps" => "https://gitlab.gnome.org/GNOME/gnome-maps",
        "logseq" => "https://github.com/logseq/logseq",
        "leocad" => "https://github.com/leozide/leocad",
        "appflowy" => "https://github.com/AppFlowy-IO/AppFlowy",
        "zen" => "https://github.com/zen-browser/desktop",
        "openra" => "https://github.com/OpenRA/OpenRA",
        "openscad" => "https://github.com/openscad/openscad",
        "stellarium" => "https://github.com/Stellarium/stellarium",
        "qbittorrent" => "https://github.com/qbittorrent/qBittorrent",
        "copyq" => "https://github.com/hluk/CopyQ",
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
