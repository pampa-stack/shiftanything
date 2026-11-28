use artcraft_launcher_core::protocol::{Backend, Command, Event};
use artcraft_launcher_native::NativeBackend;
use std::time::{Duration, Instant};

fn wait(backend: &mut NativeBackend, description: &str, limit: Duration, mut done: impl FnMut(&NativeBackend, Vec<Event>) -> bool) {
    let start = Instant::now();
    loop {
        let events = backend.poll();
        if done(backend, events) {
            return;
        }
        assert!(start.elapsed() < limit, "timed out waiting for {description}");
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn job(backend: &mut NativeBackend, app: &str, command: Command, succeeds: bool) {
    backend.send(command);
    wait(backend, "job", Duration::from_secs(300), |_, events| {
        events
            .into_iter()
            .find_map(|event| match event {
                Event::JobFinished { app: id, result } if id == app => {
                    println!("{app} JobFinished: {result:?}");
                    assert_eq!(result.is_ok(), succeeds);
                    Some(true)
                }
                _ => None,
            })
            .unwrap_or(false)
    });
}

fn main() {
    let mut args = std::env::args().skip(1);
    let stage = args.next().unwrap();
    let app = args.next().unwrap_or_else(|| "gnome-calculator".into());
    let home = std::path::PathBuf::from(std::env::var_os("HOME").unwrap());
    let mut backend = NativeBackend::new_isolated_linux_profile(&home, || {}).unwrap();
    backend.send(Command::Detect);
    wait(&mut backend, "initial Flatpak observation", Duration::from_secs(60), |backend, events| {
        events.into_iter().any(|event| matches!(event, Event::Detected { app: id, .. } if id == app)) && backend.flatpak_status(&app).is_some()
    });
    let status = backend.flatpak_status(&app).unwrap();
    println!("{app} before {stage}: {status:?}");
    match stage.as_str() {
        "install" => {
            assert!(status.installable && status.installed.is_none());
            job(&mut backend, &app, Command::InstallExpected { apps: vec![app.clone()], installed: false }, true);
            let state = backend.flatpak_status(&app).unwrap();
            assert!(state.can_open && state.can_remove && !state.update_available);
        }
        "external" => {
            assert!(!status.update_available && !status.can_remove);
            assert!(status.can_open && status.installed.unwrap().externally_managed);
            job(&mut backend, &app, Command::InstallExpected { apps: vec![app.clone()], installed: true }, false);
            job(&mut backend, &app, Command::Uninstall { app: app.clone() }, false);
        }
        "update" => {
            assert!(status.update_available && status.can_remove && status.can_open);
            job(&mut backend, &app, Command::InstallExpected { apps: vec![app.clone()], installed: true }, true);
            let state = backend.flatpak_status(&app).unwrap();
            assert!(state.can_open && state.can_remove && !state.update_available);
        }
        "open" => {
            assert!(status.can_open);
            job(&mut backend, &app, Command::Launch { app: app.clone(), args: Vec::new() }, true);
        }
        "remove" => {
            assert!(status.can_remove);
            job(&mut backend, &app, Command::Uninstall { app: app.clone() }, true);
            let state = backend.flatpak_status(&app).unwrap();
            assert!(state.installed.is_none() && state.installable && !state.can_open && !state.can_remove);
        }
        "untrusted" => {
            assert!(!status.installable && status.installed.is_none());
            job(&mut backend, &app, Command::Install { app: app.clone() }, false);
        }
        _ => panic!("unknown stage"),
    }
    println!("{app} after {stage}: {:?}", backend.flatpak_status(&app));
}
