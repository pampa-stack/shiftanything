//! Menu entries (freedesktop `.desktop` files) for AppImages the launcher installs.
//!
//! An AppImage installs nothing by itself, so without an entry the app is missing from the
//! desktop's menu and, on Wayland, the dock can't match the window's app id to an icon. The
//! launcher writes `<data>/applications/ai.storyteller.<app>.desktop` (the id each app's window
//! reports) and marks it as its own, so it only ever replaces or removes entries it wrote.
//!
//! The text is built by pure functions, tested on every platform; only Linux writes the files.

#![cfg_attr(not(target_os = "linux"), allow(dead_code))]

use std::path::Path;

use artcraft_launcher_core::CraftApp;

/// The key that marks an entry as written by the launcher.
pub const MARKER: &str = "X-ArtCraft-Launcher=true";

/// The menu categories and the window class each app's own `.desktop` file declares.
pub fn meta(app: &CraftApp) -> (&'static str, Option<&'static str>) {
    if let Some(meta) = artcraft_launcher_core::hosted_catalog::metadata(app.id) {
        return (meta.desktop_categories.as_str(), Some(meta.startup_wm_class.as_str()));
    }
    match app.id {
        "pencil2d" => ("Graphics;2DGraphics;VectorGraphics;RasterGraphics;Qt;", Some("Pencil2D")),
        "openshot-video-editor" => ("GNOME;GTK;AudioVideo;Video;AudioVideoEditing;", Some("openshot")),
        "musescore" => ("AudioVideo;Audio;Music;Qt;", Some("MuseScore4")),
        "orca-slicer" => ("Graphics;3DGraphics;", Some("OrcaSlicer")),
        "losslesscut" => ("AudioVideo;Video;AudioVideoEditing;", Some("losslesscut")),
        "zettlr" => ("Office;WordProcessor;", Some("zettlr")),
        "onlyoffice" => ("Office;WordProcessor;Spreadsheet;Presentation;", Some("ONLYOFFICE")),
        "audacity" => ("AudioVideo;Audio;AudioVideoEditing;", Some("Audacity4")),
        "drawio" => ("Graphics;", Some("draw-io")),
        "nextcloud-desktop" => ("Utility;X-SuSE-SyncUtility;", Some("Nextcloud")),
        "cryptomator" => ("Utility;Security;FileTools;", Some("org.cryptomator.launcher.Cryptomator$MainApp")),
        "joplin" => ("Office;", Some("appimagekit-joplin")),
        "notepadnext" => ("Qt;TextEditor;Utility;", Some("NotepadNext")),
        "rapidraw" => ("Graphics;Photography;", Some("RapidRAW")),
        "rustdesk" => ("Utility;Network;", Some("rustdesk")),
        "activitywatch" => ("Utility;", Some("aw-qt")),
        "artcraft" => ("Graphics;AudioVideo;2DGraphics;", Some("artcraft")),
        "photocraft" => ("Graphics;2DGraphics;RasterGraphics;Photography;", Some("photocraft")),
        "pdfcraft" => ("Office;Viewer;", Some("pdfcraft")),
        "lightcraft" => ("Graphics;2DGraphics;RasterGraphics;Photography;", None),
        "filmcraft" => ("AudioVideo;Video;AudioVideoEditing;", None),
        "designcraft" => ("Graphics;Publishing;", None),
        "vectorcraft" => ("Graphics;2DGraphics;VectorGraphics;", None),
        "effectcraft" => ("AudioVideo;Video;Graphics;2DGraphics;", None),
        "anicraft" => ("Graphics;2DGraphics;VectorGraphics;AudioVideo;", Some("anicraft")),
        "wordcraft" => ("Office;WordProcessor;", None),
        "deckcraft" => ("Office;Presentation;", None),
        "gridcraft" => ("Office;Spreadsheet;", None),
        "cadcraft" => ("Graphics;Engineering;VectorGraphics;", None),
        "soundcraft" => ("AudioVideo;Audio;Midi;Mixer;Sequencer;Recorder;", None),
        "figcraft" => ("Graphics;2DGraphics;VectorGraphics;X-Design;", Some("figcraft")),
        "texturecraft" => ("Graphics;3DGraphics;", Some("texturecraft")),
        "materialcraft" => ("Graphics;3DGraphics;", Some("materialcraft")),
        "craft-anything" => ("Development;Debugger;", None),
        _ => ("Graphics;", None),
    }
}

/// `path` as one quoted argument of an `Exec=` key, per the Desktop Entry spec: inside double
/// quotes `"`, `` ` ``, `$` and `\` take a backslash, then the string-value rule doubles every
/// backslash, and `%` (field codes) is written `%%`. `None` for paths that can't be written
/// safely (not UTF-8, holding a control character such as a line break, or holding `=`, which the
/// spec forbids in the executable name or path even when quoted).
pub fn exec_quote(path: &Path) -> Option<String> {
    let s = path.to_str()?;
    if s.chars().any(|c| c.is_control() || c == '=') {
        return None;
    }
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' | '`' | '$' => {
                out.push_str("\\\\");
                out.push(c);
            }
            '\\' => out.push_str("\\\\\\\\"),
            '%' => out.push_str("%%"),
            c => out.push(c),
        }
    }
    out.push('"');
    Some(out)
}

/// A localestring/string value: backslashes and line breaks escaped.
fn value(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t")
}

/// These reviewed AppImages support extracting themselves on systems without a usable FUSE mount.
pub(crate) fn appimage_runtime_flag(app: &CraftApp, path: &Path) -> Option<&'static str> {
    use artcraft_launcher_core::curated::{
        ACTIVITYWATCH, AUDACITY, CRYPTOMATOR, DRAWIO, JOPLIN, LOCALSEND, LOSSLESSCUT, MUSESCORE, NEXTCLOUD_DESKTOP, NOTEPAD_NEXT, ONLYOFFICE,
        OPENSHOT, ORCA_SLICER, PENCIL2D, RAPIDRAW, RUSTDESK, ZETTLR,
    };
    (path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("appimage"))
        && [
            LOCALSEND,
            LOSSLESSCUT,
            MUSESCORE,
            ONLYOFFICE,
            OPENSHOT,
            ORCA_SLICER,
            PENCIL2D,
            ZETTLR,
            AUDACITY,
            DRAWIO,
            NEXTCLOUD_DESKTOP,
            CRYPTOMATOR,
            JOPLIN,
            NOTEPAD_NEXT,
            RAPIDRAW,
            RUSTDESK,
            ACTIVITYWATCH,
        ]
        .iter()
        .any(|candidate| (app.id, app.repo) == (candidate.id, candidate.repo)))
    .then_some("--appimage-extract-and-run")
}

/// The entry for `app` installed as the AppImage `appimage`; `icon` when its icon is installed
/// under the app's id. `None` when the path can't be put in an `Exec=` line.
pub fn render(app: &CraftApp, appimage: &Path, icon: bool) -> Option<String> {
    let exec = exec_quote(appimage)?;
    let (categories, wm_class) = meta(app);
    let mut text = String::from("[Desktop Entry]\nType=Application\n");
    text.push_str(&format!("Name={}\n", value(app.name)));
    text.push_str(&format!("Comment={}\n", value(app.tagline)));
    text.push_str(&format!("Exec={exec}"));
    if let Some(flag) = appimage_runtime_flag(app, appimage) {
        text.push(' ');
        text.push_str(flag);
    }
    text.push_str(" %F\n");
    if icon {
        text.push_str(&format!("Icon={}\n", app.linux_desktop_id()));
    }
    text.push_str("Terminal=false\n");
    text.push_str(&format!("Categories={categories}\n"));
    text.push_str(&format!("StartupWMClass={}\n", wm_class.map_or_else(|| app.linux_desktop_id(), str::to_string)));
    text.push_str(MARKER);
    text.push('\n');
    Some(text)
}

/// Whether the launcher wrote this entry.
pub fn is_ours(text: &str) -> bool {
    text.lines().any(|l| l.trim() == MARKER)
}

/// The program an entry's `Exec=` runs: its first argument, unquoted. Reads the launcher's own
/// quoting and the plain `Exec="<path>" %F` the apps' AppImages write for themselves.
pub fn exec_program(text: &str) -> Option<String> {
    let line = text.lines().find_map(|l| l.strip_prefix("Exec="))?;
    // String-value escapes first, then the quoting rule.
    let mut unescaped = String::with_capacity(line.len());
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('s') => unescaped.push(' '),
                Some('n') => unescaped.push('\n'),
                Some('t') => unescaped.push('\t'),
                Some('r') => unescaped.push('\r'),
                Some(other) => unescaped.push(other),
                None => unescaped.push('\\'),
            }
        } else {
            unescaped.push(c);
        }
    }
    let mut out = String::new();
    let mut chars = unescaped.trim_start().chars();
    if unescaped.trim_start().starts_with('"') {
        chars.next();
        loop {
            match chars.next()? {
                '"' => break,
                '\\' => out.push(chars.next()?),
                c => out.push(c),
            }
        }
    } else {
        out.extend(chars.take_while(|c| !c.is_whitespace()));
    }
    Some(out.replace("%%", "%"))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn app(id: &str) -> &'static CraftApp {
        CraftApp::by_id(id).unwrap()
    }

    #[test]
    fn writes_a_complete_entry() {
        let text = render(app("photocraft"), Path::new("/home/ana/Applications/photocraft-0.6.0-linux-x86_64.AppImage"), true).unwrap();
        let expected = format!(
            "[Desktop Entry]\nType=Application\nName=PhotoCraft\nComment={}\n\
             Exec=\"/home/ana/Applications/photocraft-0.6.0-linux-x86_64.AppImage\" %F\n\
             Icon=ai.storyteller.photocraft\nTerminal=false\nCategories=Graphics;2DGraphics;RasterGraphics;Photography;\n\
             StartupWMClass=photocraft\nX-ArtCraft-Launcher=true\n",
            app("photocraft").tagline
        );
        assert_eq!(text, expected);
        assert!(is_ours(&text));
        let no_icon = render(app("wordcraft"), Path::new("/x/w.AppImage"), false).unwrap();
        assert!(!no_icon.contains("Icon="));
        assert!(no_icon.contains("StartupWMClass=ai.storyteller.wordcraft\n"));
        assert!(no_icon.contains("Categories=Office;WordProcessor;\n"));
    }

    #[test]
    fn quotes_exec_paths_per_the_spec() {
        assert_eq!(exec_quote(Path::new("/home/ana/My Apps/p.AppImage")).unwrap(), "\"/home/ana/My Apps/p.AppImage\"");
        assert_eq!(exec_quote(Path::new("/a/\"q\" $HOME `x`.AppImage")).unwrap(), r#""/a/\\"q\\" \\$HOME \\`x\\`.AppImage""#);
        assert_eq!(exec_quote(Path::new(r"/a\b/100%.AppImage")).unwrap(), r#""/a\\\\b/100%%.AppImage""#);
        assert_eq!(exec_quote(Path::new("/a/line\nbreak.AppImage")), None);
        // The spec forbids `=` in the executable name or path, even quoted.
        assert_eq!(exec_quote(Path::new("/opt/artcraft=beta/app")), None);
        assert!(render(app("photocraft"), Path::new("/a/line\nbreak.AppImage"), true).is_none());
    }

    #[test]
    fn localsend_desktop_entry_runs_without_fuse_and_keeps_the_owned_path_readable() {
        let path = Path::new("/home/ana/Applications/LocalSend-1.19.0-linux-x86-64.AppImage");
        let text = render(app("localsend"), path, false).unwrap();
        assert!(text.contains(&format!("Exec=\"{}\" --appimage-extract-and-run %F\n", path.display())));
        assert_eq!(exec_program(&text).map(PathBuf::from).as_deref(), Some(path));
        assert!(appimage_runtime_flag(app("localsend"), Path::new("/usr/bin/localsend")).is_none());
        assert!(appimage_runtime_flag(app("photocraft"), path).is_none());
    }

    #[test]
    fn reads_back_the_program_it_wrote_and_the_apps_own_entries() {
        for p in ["/home/ana/My Apps/p.AppImage", "/a/\"q\" $HOME `x`.AppImage", r"/a\b/100%.AppImage", "/plain.AppImage"] {
            let text = render(app("gridcraft"), Path::new(p), false).unwrap();
            assert_eq!(exec_program(&text).map(PathBuf::from), Some(PathBuf::from(p)), "{p}");
        }
        // PhotoCraft's AppRun writes its own entry, unmarked.
        let theirs = "[Desktop Entry]\nName=PhotoCraft\nExec=\"/home/ana/Applications/photocraft-0.5.0-linux-x86_64.AppImage\" %F\n";
        assert_eq!(exec_program(theirs).as_deref(), Some("/home/ana/Applications/photocraft-0.5.0-linux-x86_64.AppImage"));
        assert!(!is_ours(theirs));
        assert_eq!(exec_program("[Desktop Entry]\nExec=photocraft %F\n").as_deref(), Some("photocraft"));
        assert_eq!(exec_program("[Desktop Entry]\nName=x\n"), None);
    }
}
