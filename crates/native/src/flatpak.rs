//! Reviewed Flathub application refs. Editorial metadata and GitHub releases cannot add one.

use artcraft_launcher_core::Installed;
use serde::{Deserialize, Serialize};

pub use artcraft_launcher_core::flathub::{IDS, supported};

/// Last observed state. A pre-existing deployment is visible and openable, but is never adopted.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Status {
    pub installable: bool,
    pub installed: Option<Installed>,
    pub installed_version_label: Option<String>,
    pub update_available: bool,
    pub can_remove: bool,
    pub can_open: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Owned {
    reference: String,
    origin: String,
    commit: String,
    #[serde(default)]
    device: u64,
    #[serde(default)]
    inode: u64,
    #[serde(default)]
    modified_seconds: i64,
    #[serde(default)]
    modified_nanos: i64,
}

#[cfg(target_os = "linux")]
pub(crate) use linux::{Action, InventoryScan, UserFlatpak};

#[cfg(target_os = "linux")]
mod linux {
    use super::{Owned, Status};
    use artcraft_launcher_core::flathub::RECIPES;
    use artcraft_launcher_core::{InstallSource, Installed, version};
    use sha2::{Digest, Sha256};
    use std::os::unix::fs::MetadataExt;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::Duration;

    use crate::{
        http::{Http, Stop},
        paths, process,
    };

    const REMOTE: &str = "flathub";
    const URL: &str = "https://dl.flathub.org/repo/";
    const DESCRIPTOR: &str = "https://dl.flathub.org/repo/flathub.flatpakrepo";
    const DESCRIPTOR_SHA256: &str = "3371dd250e61d9e1633630073fefda153cd4426f72f4afa0c3373ae2e8fea03a";
    const PRIMARY_KEY: &str = "6E5C05D979C76DAF93C081354184DD4D907A7CAE";
    // Official descriptor key and the same key imported by libostree 2025.7.
    const KEYRING_SHA256: [&str; 2] =
        ["8bdc20abc4e19c0796460beb5bfe0e7aa4138716999e19c6f2dbdd78cc41aeaa", "c504fa5dc891df6cfcc10021dd9addf08459007f1787f40b9c6fd3c7e58416ea"];
    const LOCAL_TIMEOUT: Duration = Duration::from_secs(5);
    const NETWORK_TIMEOUT: Duration = Duration::from_secs(25);

    pub(crate) enum Action<'a> {
        Install,
        Update,
        Remove,
        Open(&'a [String]),
    }

    #[cfg(test)]
    type QueryStub = std::sync::Arc<dyn Fn(&[&str], Duration) -> Result<String, String> + Send + Sync>;

    #[derive(Clone)]
    pub(crate) struct UserFlatpak {
        data: PathBuf,
        profile: Option<PathBuf>,
        #[cfg(test)]
        query_stub: Option<QueryStub>,
    }

    /// List and remote state shared by one inventory refresh. Deployment identity is still checked per ref.
    pub(crate) struct InventoryScan {
        rows: Result<String, String>,
        trusted: bool,
        remote_missing: bool,
    }

    struct Deployment {
        origin: String,
        commit: String,
        scope: String,
        version: String,
    }

    impl Deployment {
        fn version_label(&self) -> Option<&str> {
            let raw = self.version.trim();
            // Some upstream AppStream metadata serializes a release number as Python bytes.
            let label =
                raw.strip_prefix("b'").and_then(|value| value.strip_suffix('\'')).filter(|value| version::parse(value).is_some()).unwrap_or(raw);
            (!label.is_empty() && label.len() <= 96 && !label.chars().any(char::is_control)).then_some(label)
        }
    }

    impl UserFlatpak {
        pub(crate) fn new(profile: Option<&Path>) -> Result<Self, String> {
            // The caller must see the same Flatpak config and keyring as the process it starts.
            if process::in_sandbox() {
                return Err("Flathub management requires the native launcher outside a Flatpak sandbox".into());
            }
            let data = if let Some(home) = profile {
                let data = home.join("data/flatpak");
                let system = home.join("system-flatpak");
                let config = home.join("config/flatpak-system");
                if !home.is_absolute() || [&data, &system, &config].into_iter().any(|path| paths::resolve_existing(path) != *path) {
                    return Err("Flatpak directories escaped the verification profile".into());
                }
                data
            } else if let Some(data) = std::env::var_os("FLATPAK_USER_DIR").map(PathBuf::from) {
                data
            } else {
                let data = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).filter(|p| p.is_absolute());
                data.or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")))
                    .ok_or("No Flatpak user directory")?
                    .join("flatpak")
            };
            if !data.is_absolute() {
                return Err("Flatpak user directory must be absolute".into());
            }
            Ok(Self {
                data,
                profile: profile.map(Path::to_path_buf),
                #[cfg(test)]
                query_stub: None,
            })
        }

        fn command(&self) -> Command {
            let mut command = process::flatpak_command();
            command.env("FLATPAK_USER_DIR", &self.data);
            if let Some(profile) = &self.profile {
                command
                    .env("HOME", profile)
                    .env("XDG_DATA_HOME", profile.join("data"))
                    .env("XDG_CONFIG_HOME", profile.join("config"))
                    .env("XDG_CACHE_HOME", profile.join("cache"))
                    .env("XDG_STATE_HOME", profile.join("state"))
                    .env("FLATPAK_SYSTEM_DIR", profile.join("system-flatpak"))
                    .env("FLATPAK_CONFIG_DIR", profile.join("config/flatpak-system"));
            }
            command
        }

        fn maintenance_command(&self) -> Command {
            let mut command = self.command();
            command.env("LC_ALL", "C");
            command
        }

        fn query(&self, args: &[&str], timeout: Duration) -> Result<String, String> {
            #[cfg(test)]
            if let Some(query) = &self.query_stub {
                return query(args, timeout);
            }
            let mut command = self.maintenance_command();
            command.args(args);
            process::run_command(command, "flatpak", timeout)
        }

        fn verify_remote(&self) -> Result<(), String> {
            let repo = self.data.join("repo");
            let config = read_small(&repo.join("config"), 256 * 1024)?;
            if !trusted_config(&config) {
                return Err("The user Flathub remote needs the official URL and both GPG verification settings".into());
            }
            let keyring = repo.join("flathub.trustedkeys.gpg");
            let bytes = read_bounded(&keyring, 64 * 1024)?;
            if KEYRING_SHA256.contains(&format!("{:x}", Sha256::digest(&bytes)).as_str()) {
                return Ok(());
            }
            let scratch = tempfile::tempdir().map_err(|e| e.to_string())?;
            let keyring = scratch.path().join("flathub.gpg");
            std::fs::write(&keyring, bytes).map_err(|e| e.to_string())?;
            let mut command = process::command("gpg");
            command
                .args(["--no-options", "--batch", "--homedir"])
                .arg(scratch.path())
                .args(["--with-colons", "--fingerprint", "--show-keys"])
                .arg(keyring);
            let keys = process::run_command(command, "gpg", LOCAL_TIMEOUT)?;
            if !trusted_keys(&keys) {
                return Err("The configured Flathub signing key does not match the reviewed key".into());
            }
            Ok(())
        }

        fn remote_missing(&self) -> Result<bool, String> {
            let remotes = self.query(&["remotes", "--user", "--show-disabled", "--columns=name:full"], LOCAL_TIMEOUT)?;
            Ok(!remotes.lines().any(|line| line.trim() == REMOTE))
        }

        /// Runs only from an approved new installation. A pre-existing remote, even disabled,
        /// is never replaced or reconfigured.
        fn prepare_remote_for_install(&self, stop: &Stop) -> Result<(), String> {
            if self.remote_missing()? {
                let contents = Http::new()?.get_bytes(DESCRIPTOR, 16 * 1024)?.ok_or("Flathub descriptor was not found")?;
                if format!("{:x}", Sha256::digest(&contents)) != DESCRIPTOR_SHA256 {
                    return Err("Flathub changed its repository descriptor. The launcher needs an updated trust policy".into());
                }
                let scratch = tempfile::tempdir().map_err(|e| e.to_string())?;
                let descriptor = scratch.path().join("flathub.flatpakrepo");
                std::fs::write(&descriptor, contents).map_err(|e| e.to_string())?;
                stop.check()?;
                let mut command = self.maintenance_command();
                command.args(["remote-add", "--user", REMOTE]).arg(descriptor);
                process::run_command(command, "flatpak", NETWORK_TIMEOUT)?;
            }
            self.verify_remote()
        }

        fn list_deployments(&self) -> Result<String, String> {
            let scope = if self.profile.is_some() { &["--user"][..] } else { &[][..] };
            let mut args = vec!["list", "--app", "--columns=ref:full,origin:full,installation:full,version:full"];
            args.extend_from_slice(scope);
            self.query(&args, LOCAL_TIMEOUT)
        }

        fn deployments_from_rows(&self, rows: &str, reference: &str) -> Result<Vec<Deployment>, String> {
            // The list command abbreviates OSTree commits even with the :full column suffix.
            parse_deployments(rows, reference, |scope| {
                self.query(&["info", &scope_arg(scope), "--show-commit", reference], LOCAL_TIMEOUT).map(|commit| commit.trim().to_owned())
            })
        }

        fn deployments(&self, reference: &str) -> Result<Vec<Deployment>, String> {
            self.deployments_from_rows(&self.list_deployments()?, reference)
        }

        pub(crate) fn inventory_scan(&self) -> InventoryScan {
            self.inventory_scan_with_trust(|| self.verify_remote().is_ok())
        }

        fn inventory_scan_with_trust(&self, verify_trust: impl FnOnce() -> bool) -> InventoryScan {
            let rows = self.list_deployments();
            let trusted = rows.is_ok() && verify_trust();
            let remote_missing = rows.is_ok() && !trusted && self.remote_missing().unwrap_or(false);
            InventoryScan { rows, trusted, remote_missing }
        }

        fn remote_commit(&self, reference: &str) -> Result<String, String> {
            let value = self.query(&["remote-info", "--user", "--show-commit", REMOTE, reference], NETWORK_TIMEOUT)?;
            let digest = value.trim();
            if !valid_commit(digest) {
                return Err("Flathub did not return an exact commit for this app".into());
            }
            Ok(digest.into())
        }

        fn location(&self, reference: &str, deployed: &Deployment) -> Result<PathBuf, String> {
            let scope = scope_arg(&deployed.scope);
            let path = PathBuf::from(self.query(&["info", &scope, "--show-location", reference], LOCAL_TIMEOUT)?.trim());
            if !path.is_absolute() {
                return Err("Flatpak did not return an absolute deployment location".into());
            }
            Ok(path)
        }

        fn reviewed_deployment(&self, id: &str, reference: &str, deployed: &Deployment) -> Result<Owned, String> {
            if deployed.scope != "user" || deployed.origin != REMOTE {
                return Err("This Flatpak is managed outside the launcher".into());
            }
            let path = self.location(reference, deployed)?;
            let branch = self.data.join(reference);
            let branch = branch.canonicalize().map_err(|e| e.to_string())?;
            let path = path.canonicalize().map_err(|e| e.to_string())?;
            let file = path.join("deploy");
            let prefix = format!("{}-", deployed.commit);
            if path.parent() != Some(branch.as_path())
                || !path.file_name().and_then(|n| n.to_str()).is_some_and(|name| name == deployed.commit || name.starts_with(&prefix))
                || !exported_gui(&path, id)
            {
                return Err("The Flatpak deployment does not match the reviewed desktop app".into());
            }
            let meta = std::fs::symlink_metadata(file).map_err(|e| e.to_string())?;
            if !meta.is_file() {
                return Err("Invalid Flatpak deployment record".into());
            }
            Ok(Owned {
                reference: reference.into(),
                origin: REMOTE.into(),
                commit: deployed.commit.clone(),
                device: meta.dev(),
                inode: meta.ino(),
                modified_seconds: meta.mtime(),
                modified_nanos: meta.mtime_nsec(),
            })
        }

        pub(crate) fn status(&self, catalog_id: &str, record: Option<&Owned>, remote_latest: bool) -> Result<Status, String> {
            self.status_using(catalog_id, record, remote_latest, None)
        }

        pub(crate) fn status_from_scan(
            &self,
            catalog_id: &str,
            record: Option<&Owned>,
            remote_latest: bool,
            scan: &InventoryScan,
        ) -> Result<Status, String> {
            self.status_using(catalog_id, record, remote_latest, Some(scan))
        }

        fn status_using(
            &self,
            catalog_id: &str,
            record: Option<&Owned>,
            remote_latest: bool,
            scan: Option<&InventoryScan>,
        ) -> Result<Status, String> {
            let (id, reference) = recipe(catalog_id)?;
            let deployments = match scan {
                Some(scan) => self.deployments_from_rows(scan.rows.as_ref().map_err(Clone::clone)?, &reference),
                None => self.deployments(&reference),
            }?;
            let trusted = scan.map_or_else(|| self.verify_remote().is_ok(), |scan| scan.trusted);
            if deployments.is_empty() {
                let installable = trusted || scan.map_or_else(|| self.remote_missing().unwrap_or(false), |scan| scan.remote_missing);
                return Ok(Status { installable, ..Status::default() });
            }
            let deployed = selected(&deployments);
            let location = self.location(&reference, deployed)?;
            let can_open = exported_gui(&location, id);
            let managed = trusted && record.is_some_and(|r| self.reviewed_deployment(id, &reference, deployed).is_ok_and(|found| &found == r));
            let update_available = managed && remote_latest && self.remote_commit(&reference).is_ok_and(|commit| commit != deployed.commit);
            let version_label = deployed.version_label();
            Ok(Status {
                installable: managed,
                installed: Some(Installed {
                    version: version_label.and_then(version::parse),
                    source: InstallSource::Flatpak,
                    location: location.display().to_string(),
                    package: Some(id.into()),
                    externally_managed: !managed,
                    origin: Some(deployed.origin.clone()),
                    installation: Some(deployed.scope.clone()),
                }),
                installed_version_label: version_label.map(str::to_owned),
                update_available,
                can_remove: managed,
                can_open,
            })
        }

        /// Rechecks the exact live deployment immediately before acting. The backend serializes
        /// its own Flatpak operations and only saves ownership after this returns successfully.
        pub(crate) fn execute(&self, catalog_id: &str, record: Option<&Owned>, action: Action<'_>, stop: &Stop) -> Result<Option<Owned>, String> {
            stop.check()?;
            let (id, reference) = recipe(catalog_id)?;
            let deployments = self.deployments(&reference)?;
            let deployed = (!deployments.is_empty()).then(|| selected(&deployments));
            if let Action::Open(args) = action {
                let deployed = deployed.ok_or("This Flatpak is not installed")?;
                if !exported_gui(&self.location(&reference, deployed)?, id) {
                    return Err("No verified desktop entry for this Flatpak".into());
                }
                // A desktop app inherits the session locale. Forcing C here prevents GTK apps
                // such as SolveSpace from starting because they require a UTF-8 locale.
                let mut command = self.command();
                command.args(["run", &scope_arg(&deployed.scope)]);
                if id == "dev.zed.Zed" {
                    // The community wrapper otherwise starts Zed through the host portal.
                    command.args(["--env=ZED_FLATPAK_NO_ESCAPE=1", "--no-talk-name=org.freedesktop.Flatpak"]);
                }
                command.arg(&reference);
                // The Octave Flatpak runs the CLI by default, unlike its desktop entry.
                if id == "org.octave.Octave" {
                    command.arg("--gui");
                } else if id == "io.mpv.Mpv" {
                    command.arg("--player-operation-mode=pseudo-gui");
                }
                command.args(args);
                process::check_desktop_start(command, Duration::from_secs(1))?;
                return Ok(None);
            }
            let at_start = match action {
                Action::Install if deployed.is_none() => {
                    self.prepare_remote_for_install(stop)?;
                    None
                }
                Action::Install => return Err("This Flatpak is already installed outside this install request".into()),
                _ => {
                    self.verify_remote()?;
                    let deployed = deployed.ok_or("This Flatpak is not installed")?;
                    let observed = self.reviewed_deployment(id, &reference, deployed)?;
                    if Some(&observed) != record {
                        return Err("This Flatpak is managed outside the launcher".into());
                    }
                    Some(observed)
                }
            };
            let target = match action {
                Action::Install | Action::Update => Some(self.remote_commit(&reference)?),
                Action::Remove | Action::Open(_) => None,
            };
            // Revalidate after querying the network. Never add --reinstall or --delete-data.
            self.verify_remote()?;
            let before = self.deployments(&reference)?;
            if before.is_empty() != at_start.is_none()
                || at_start.as_ref().is_some_and(|record| self.reviewed_deployment(id, &reference, selected(&before)).ok().as_ref() != Some(record))
            {
                return Err("The Flatpak deployment changed before the operation".into());
            }
            if matches!((&target, &at_start), (Some(target), Some(previous)) if target == &previous.commit) {
                stop.check()?;
                return Ok(at_start);
            }
            let mut command = self.maintenance_command();
            match action {
                Action::Install => command.args(["install", "--user", "--noninteractive", "--assumeyes", REMOTE, &reference]),
                Action::Update => command
                    .args(["update", "--user", "--noninteractive", "--assumeyes"])
                    .arg(format!("--commit={}", target.as_deref().ok_or("Missing verified Flatpak target")?))
                    .arg(&reference),
                Action::Remove => command.args(["uninstall", "--user", "--noninteractive", "--assumeyes", &reference]),
                Action::Open(_) => return Err("Already handled this Flatpak launch".into()),
            };
            let output = process::cancellable_output(command, || stop.check())?;
            if !output.status.success() {
                let reason = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Flatpak operation failed: {}", reason.trim()));
            }
            let remaining = self.deployments(&reference)?;
            if matches!(action, Action::Remove) {
                return if remaining.iter().any(|d| d.scope == "user") { Err("The user Flatpak is still installed".into()) } else { Ok(None) };
            }
            self.verify_remote()?;
            let after = remaining.iter().find(|d| d.scope == "user").ok_or("Flatpak did not deploy the app for this user")?;
            if target.as_deref() != Some(after.commit.as_str()) {
                return Err("The deployed Flatpak commit differs from the selected Flathub commit".into());
            }
            let result = self.reviewed_deployment(id, &reference, after)?;
            Ok(Some(result))
        }
    }

    fn recipe(catalog_id: &str) -> Result<(&'static str, String), String> {
        let (_, id) = RECIPES.iter().find(|(known, _)| *known == catalog_id).ok_or("Unknown Flatpak app")?;
        Ok((id, format!("app/{id}/x86_64/stable")))
    }

    fn valid_commit(input: &str) -> bool {
        input.len() == 64 && input.bytes().all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    }

    fn valid_scope(input: &str) -> bool {
        !input.is_empty() && input.bytes().all(|c| c.is_ascii_alphanumeric() || b"-_".contains(&c))
    }

    fn scope_arg(value: &str) -> String {
        match value {
            "user" => "--user".into(),
            "system" | "default" => "--system".into(),
            name => format!("--installation={name}"),
        }
    }

    fn selected(deployments: &[Deployment]) -> &Deployment {
        deployments.iter().find(|d| d.scope == "user").unwrap_or(&deployments[0])
    }

    fn parse_deployments(
        rows: &str,
        reference: &str,
        mut read_commit: impl FnMut(&str) -> Result<String, String>,
    ) -> Result<Vec<Deployment>, String> {
        let mut deployments = Vec::new();
        for row in rows.lines() {
            let mut columns = row.splitn(4, '\t');
            let Some(found) = columns.next() else { continue };
            if found != reference && Some(found) != reference.strip_prefix("app/") {
                continue;
            }
            let origin = columns.next().ok_or("Missing Flatpak origin")?;
            let scope = columns.next().ok_or("Missing Flatpak installation scope")?;
            let version = columns.next().unwrap_or_default();
            if !valid_scope(scope) || deployments.iter().any(|d: &Deployment| d.scope == scope) {
                return Err("Invalid or duplicate Flatpak deployment".into());
            }
            let commit = read_commit(scope)?;
            if !valid_commit(&commit) {
                return Err("Flatpak did not return an exact deployment commit".into());
            }
            // Some AppStream versions are stale. Use the observed app version only
            // for the exact Flathub deployments verified in QA.
            let version = match (reference, origin, version, commit.as_str()) {
                (
                    "app/io.github.guillaumechereau.Goxel/x86_64/stable",
                    REMOTE,
                    "0.13.0",
                    "6529af40848649a25b3bce47c5104af29a6fff9cd5f1f6959211a458d5061fe6",
                ) => "0.14.0",
                (
                    "app/io.github.guillaumechereau.Goxel/x86_64/stable",
                    REMOTE,
                    "0.13.0",
                    "0ca337f50e5934f6f827237c3c30d3b475c3382720f61925f92cb087da2bc61e",
                ) => "0.15.1",
                ("app/org.jaspstats.JASP/x86_64/stable", REMOTE, "0.98.1", "a142fa6e6685d28a398463259fed880160efebbbe8a7b6440266381a8bdfa14a") => {
                    "0.99.0"
                }
                _ => version,
            };
            deployments.push(Deployment { origin: origin.into(), commit, scope: scope.into(), version: version.into() });
        }
        Ok(deployments)
    }

    fn read_small(path: &Path, size: u64) -> Result<String, String> {
        String::from_utf8(read_bounded(path, size)?).map_err(|e| e.to_string())
    }

    fn read_bounded(path: &Path, size: u64) -> Result<Vec<u8>, String> {
        use std::io::Read;
        if !std::fs::metadata(path).is_ok_and(|m| m.is_file()) {
            return Err("No regular file for the Flatpak configuration or desktop entry".into());
        }
        let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
        let mut output = Vec::new();
        file.take(size + 1).read_to_end(&mut output).map_err(|e| e.to_string())?;
        if output.len() as u64 > size {
            return Err("Flatpak configuration file is too large".into());
        }
        Ok(output)
    }

    fn trusted_config(text: &str) -> bool {
        let mut entries = std::collections::BTreeMap::new();
        let mut matching = false;
        let mut seen = false;
        for line in text.lines().map(str::trim) {
            if line.starts_with('[') {
                matching = line == "[remote \"flathub\"]";
                if matching && std::mem::replace(&mut seen, true) {
                    return false;
                }
            } else if matching && !line.starts_with('#') && !line.starts_with(';') && !line.is_empty() {
                let Some((key, value)) = line.split_once('=') else { return false };
                if entries.insert(key.trim(), value.trim()).is_some() {
                    return false;
                }
            }
        }
        entries.get("url") == Some(&URL)
            && entries.get("gpg-verify") == Some(&"true")
            && entries.get("gpg-verify-summary") == Some(&"true")
            && entries.get("xa.disable").is_none_or(|value| *value == "false")
            && !["contenturl", "metalink", "gpgkeypath", "collection-id", "xa.collection-id", "tls-permissive"]
                .iter()
                .any(|key| entries.contains_key(key))
    }

    fn trusted_keys(output: &str) -> bool {
        let mut primary = None;
        let mut fingerprint_next = false;
        for line in output.lines() {
            let mut fields = line.split(':');
            match fields.next() {
                Some("pub") => {
                    if primary.is_some() || fingerprint_next {
                        return false;
                    }
                    fingerprint_next = true;
                }
                Some("fpr") if fingerprint_next => {
                    primary = fields.nth(8);
                    fingerprint_next = false;
                }
                _ => {}
            }
        }
        primary == Some(PRIMARY_KEY) && !fingerprint_next
    }

    fn exported_gui(path: &Path, id: &str) -> bool {
        let path = path.join("export/share/applications").join(format!("{id}.desktop"));
        let Ok(text) = read_small(&path, 64 * 1024) else { return false };
        let mut main = false;
        let (mut kind, mut app, mut exec) = (None, None, None);
        for line in text.lines() {
            if line.starts_with('[') {
                if main {
                    break;
                }
                main = line == "[Desktop Entry]";
            } else if main && let Some((key, value)) = line.split_once('=') {
                match key {
                    "Type" => kind = Some(value),
                    "X-Flatpak" => app = Some(value),
                    "Exec" => exec = Some(value),
                    _ => {}
                }
            }
        }
        kind == Some("Application")
            && app == Some(id)
            && exec.is_some_and(|command| {
                let parts: Vec<_> = command.split_whitespace().collect();
                parts.starts_with(&["/usr/bin/flatpak", "run"])
                    && parts.contains(&"--branch=stable")
                    && parts.contains(&"--arch=x86_64")
                    && parts.contains(&id)
            })
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::sync::{Arc, Mutex};

        const STABLE: &str = "app/org.gnome.Papers/x86_64/stable";
        const COMMIT: &str = "dee8c261c19ed25ca1e959e8cebd1d3c1dd3889e7acca56677249e741effb53c";

        #[test]
        fn flatpak_desktop_inherits_session_locale_while_maintenance_uses_c() {
            let scratch = tempfile::tempdir().unwrap();
            let flatpak = UserFlatpak { data: scratch.path().join("flatpak"), profile: None, query_stub: None };
            assert!(!flatpak.command().get_envs().any(|(name, _)| name == "LC_ALL"));
            assert!(flatpak.maintenance_command().get_envs().any(|(name, value)| name == "LC_ALL" && value == Some("C".as_ref())));
        }

        #[test]
        fn flatpak_status_can_display_a_four_component_version_label() {
            let reference = "app/org.kde.krita/x86_64/stable";
            let rows = format!("{reference}\tflathub\tuser\t5.3.4.1");
            let deployments = parse_deployments(&rows, reference, |_| Ok(COMMIT.into())).unwrap();
            let deployed = selected(&deployments);
            assert_eq!(deployed.version_label(), Some("5.3.4.1"));
            assert!(version::parse(&deployed.version).is_none());
        }

        #[test]
        fn flatpak_normalizes_the_bytes_version_label_published_by_cherrytree() {
            let reference = "app/net.giuspen.cherrytree/x86_64/stable";
            let rows = format!("{reference}\tflathub\tuser\tb'1.7.3'");
            let deployments = parse_deployments(&rows, reference, |_| Ok(COMMIT.into())).unwrap();
            let label = selected(&deployments).version_label();
            assert_eq!(label, Some("1.7.3"));
            assert_eq!(label.and_then(version::parse), version::parse("1.7.3"));
        }

        #[test]
        fn corrects_stale_goxel_version_only_for_the_reviewed_flathub_commits() {
            let reference = "app/io.github.guillaumechereau.Goxel/x86_64/stable";
            let rows = format!("{reference}\tflathub\tuser\t0.13.0");
            let current = "0ca337f50e5934f6f827237c3c30d3b475c3382720f61925f92cb087da2bc61e";
            let label = |rows: &str, commit: &str| {
                let deployments = parse_deployments(rows, reference, |_| Ok(commit.into())).unwrap();
                selected(&deployments).version_label().map(str::to_owned)
            };
            assert_eq!(label(&rows, current).as_deref(), Some("0.15.1"));
            assert_eq!(label(&rows, COMMIT).as_deref(), Some("0.13.0"));
            assert_eq!(label(&rows.replace("flathub", "other"), current).as_deref(), Some("0.13.0"));
            assert_eq!(label(&rows.replace("0.13.0", "0.15.1"), current).as_deref(), Some("0.15.1"));
        }

        #[test]
        fn corrects_stale_jasp_version_only_for_the_reviewed_flathub_commit() {
            let reference = "app/org.jaspstats.JASP/x86_64/stable";
            let current = "a142fa6e6685d28a398463259fed880160efebbbe8a7b6440266381a8bdfa14a";
            let previous = "0c963797dbc4497bbe14842058f81b05847edcfa94c084fbbbd184915217577d";
            let label = |reference: &str, origin: &str, version: &str, commit: &str| {
                let rows = format!("{reference}\t{origin}\tuser\t{version}");
                let deployments = parse_deployments(&rows, reference, |_| Ok(commit.into())).unwrap();
                selected(&deployments).version_label().map(str::to_owned)
            };
            assert_eq!(label(reference, "flathub", "0.98.1", current).as_deref(), Some("0.99.0"));
            assert_eq!(label(reference, "flathub", "0.98.1", previous).as_deref(), Some("0.98.1"));
            assert_eq!(label(reference, "other", "0.98.1", current).as_deref(), Some("0.98.1"));
            assert_eq!(label(reference, "flathub", "0.99.1", current).as_deref(), Some("0.99.1"));
            let beta = "app/org.jaspstats.JASP/x86_64/beta";
            assert_eq!(label(beta, "flathub", "0.98.1", current).as_deref(), Some("0.98.1"));
        }

        #[test]
        fn only_exact_gui_ref_counts_and_user_scope_is_preserved() {
            let (_, exact) = recipe("papers").unwrap();
            let list = format!(
                "app/org.gnome.Papers/x86_64/beta\tflathub\tuser\t1.0\n\
                 app/org.gnome.Papers/aarch64/stable\tflathub\tuser\t1.0\n\
                 {STABLE}\tthird-party\tsystem\t2.0\n\
                 {STABLE}\tflathub\tuser\t2.0\n"
            );
            let rows = parse_deployments(&list, &exact, |_| Ok(COMMIT.into())).unwrap();
            assert_eq!(rows.len(), 2);
            assert_eq!(selected(&rows).scope, "user");
            assert_eq!(scope_arg(&rows[0].scope), "--system");
            assert!(parse_deployments(&list, &exact, |_| Ok("short".into())).is_err());
        }

        #[test]
        fn bulk_inventory_reuses_the_list_and_remote_probe_and_preserves_scope() {
            let scratch = tempfile::tempdir().unwrap();
            let user_path = scratch.path().join("user-deployment");
            let entries = user_path.join("export/share/applications");
            std::fs::create_dir_all(&entries).unwrap();
            std::fs::write(
                entries.join("org.gnome.Papers.desktop"),
                "[Desktop Entry]\nType=Application\nX-Flatpak=org.gnome.Papers\nExec=/usr/bin/flatpak run --branch=stable --arch=x86_64 org.gnome.Papers\n",
            )
            .unwrap();

            // The remote exists, but without the canonical repo configuration it is not trusted.
            let calls = Arc::new(Mutex::new(Vec::new()));
            let recorded = calls.clone();
            let flatpak = UserFlatpak {
                data: scratch.path().join("flatpak"),
                profile: None,
                query_stub: Some(Arc::new(move |args, timeout| {
                    assert_eq!(timeout, LOCAL_TIMEOUT);
                    recorded.lock().unwrap().push(args.iter().map(|value| (*value).to_owned()).collect::<Vec<_>>());
                    match args {
                        ["list", "--app", "--columns=ref:full,origin:full,installation:full,version:full"] => Ok(format!(
                            "app/org.gnome.Papers/aarch64/stable\tflathub\tuser\t2.1\n\
                             {STABLE}\tthird-party\tsystem\t1.0\n{STABLE}\tflathub\tuser\t2.0\n"
                        )),
                        ["remotes", "--user", "--show-disabled", "--columns=name:full"] => Ok("flathub\n".into()),
                        ["info", "--system" | "--user", "--show-commit", STABLE] => Ok(COMMIT.into()),
                        ["info", "--user", "--show-location", STABLE] => Ok(user_path.to_str().unwrap().into()),
                        _ => panic!("unexpected Flatpak command: {args:?}"),
                    }
                })),
            };

            let scan = flatpak.inventory_scan();
            let mut observed = None;
            for id in super::super::IDS {
                let status = flatpak.status_from_scan(id, None, true, &scan).unwrap();
                if id == "papers" {
                    let installed = status.installed.as_ref().unwrap();
                    assert_eq!(installed.installation.as_deref(), Some("user"));
                    assert_eq!(status.installed_version_label.as_deref(), Some("2.0"));
                    assert!(installed.externally_managed);
                    assert!(status.can_open);
                    assert!(!status.can_remove);
                    assert!(!status.update_available);
                    assert!(!status.installable);
                    observed = Some(status);
                } else {
                    assert_eq!(status, Status::default());
                }
            }

            {
                let calls = calls.lock().unwrap();
                assert_eq!(calls.iter().filter(|args| args[0] == "list").count(), 1);
                assert_eq!(calls.iter().filter(|args| args[0] == "remotes").count(), 1);
                assert_eq!(calls.len(), 5);
            }

            assert_eq!(flatpak.status("papers", None, true).unwrap(), observed.unwrap());
            assert_eq!(flatpak.status("krita", None, true).unwrap(), Status::default());
            assert_eq!(calls.lock().unwrap().iter().filter(|args| args[0] == "list").count(), 3);
        }

        #[test]
        fn bulk_inventory_keeps_the_remote_bootstrap_policy() {
            use std::cell::Cell;

            let scratch = tempfile::tempdir().unwrap();
            for (trusted, remotes, installable, expected_queries) in [
                (false, Ok(""), true, vec!["list", "remotes"]),
                (false, Err("remotes unavailable"), false, vec!["list", "remotes"]),
                (true, Ok("flathub"), true, vec!["list"]),
            ] {
                let calls = Arc::new(Mutex::new(Vec::new()));
                let recorded = calls.clone();
                let flatpak = UserFlatpak {
                    data: scratch.path().join("flatpak"),
                    profile: None,
                    query_stub: Some(Arc::new(move |args, _| {
                        recorded.lock().unwrap().push(args[0].to_owned());
                        match args[0] {
                            "list" => Ok(String::new()),
                            "remotes" => remotes.map(str::to_owned).map_err(str::to_owned),
                            _ => panic!("unexpected Flatpak command: {args:?}"),
                        }
                    })),
                };
                let trust_checks = Cell::new(0);
                let scan = flatpak.inventory_scan_with_trust(|| {
                    trust_checks.set(trust_checks.get() + 1);
                    trusted
                });
                for id in super::super::IDS {
                    assert_eq!(flatpak.status_from_scan(id, None, true, &scan).unwrap(), Status { installable, ..Status::default() });
                }
                assert_eq!(trust_checks.get(), 1);
                assert_eq!(*calls.lock().unwrap(), expected_queries);
            }
        }

        #[test]
        fn bulk_inventory_propagates_a_failed_list_without_probing_the_remote() {
            let scratch = tempfile::tempdir().unwrap();
            let flatpak = UserFlatpak {
                data: scratch.path().join("data/flatpak"),
                profile: Some(scratch.path().into()),
                query_stub: Some(Arc::new(|args, _| {
                    assert_eq!(args, ["list", "--app", "--columns=ref:full,origin:full,installation:full,version:full", "--user"]);
                    Err("list failed".into())
                })),
            };
            let scan = flatpak.inventory_scan();
            for id in ["papers", "krita"] {
                assert_eq!(flatpak.status_from_scan(id, None, true, &scan), Err("list failed".into()));
            }
        }

        #[test]
        fn known_name_is_insufficient_for_repository_trust() {
            let official = format!("[core]\n[remote \"flathub\"]\nurl={URL}\ngpg-verify=true\ngpg-verify-summary=true\n");
            assert!(trusted_config(&official));
            assert!(!trusted_config(&official.replace(URL, "https://example.com/flathub")));
            assert!(!trusted_config(&official.replace("verify-summary=true", "verify-summary=false")));
            assert!(!trusted_config(&format!("{official}gpg-verify=false\n")));
            assert!(!trusted_config(&format!("{official}collection-id=org.flathub.Stable\n")));
            assert!(!trusted_config(&format!("{official}contenturl=https://example.com\n")));
            let known = format!("pub:u:4096:1:key:1:2:::::\nfpr:::::::::{PRIMARY_KEY}:\n");
            assert!(trusted_keys(&known));
            assert!(!trusted_keys(&known.repeat(2)));
            assert!(!trusted_keys(&known.replace(PRIMARY_KEY, "different")));
        }
    }
}
