//! Closed launcher operations: no commands, arguments, paths or URLs supplied by agents.
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Install,
    Update,
    Open,
    Remove,
    UpdateAll,
    Cancel,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub id: String,
    pub operation: Operation,
    pub apps: Vec<String>,
}
impl Request {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty() || self.id.len() > 64 || !self.id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
            return Err("invalid request identity".into());
        }
        if self.apps.is_empty() || self.apps.len() > 32 || (self.operation != Operation::UpdateAll && self.apps.len() != 1) {
            return Err("invalid app count".into());
        }
        let mut ids = std::collections::HashSet::new();
        for id in &self.apps {
            let entry = crate::curated::apps().iter().find(|a| &a.id == id).ok_or("unknown curated app")?;
            let native = crate::CraftApp::by_id(id).ok_or("no native recipe")?;
            if !ids.insert(id) || entry.repo.trim_end_matches('/') != format!("https://github.com/{}", native.repo) {
                return Err("unapproved recipe identity".into());
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct InventoryEntry {
    pub id: String,
    pub detected: bool,
    pub installed: bool,
    pub compatible: bool,
    pub installable: bool,
    pub update_available: bool,
    pub installed_version: Option<String>,
    pub latest_version: Option<String>,
    pub job: Option<String>,
}
impl Request {
    pub fn validate_state(&self, snapshot: &Snapshot) -> Result<(), String> {
        self.validate()?;
        for id in &self.apps {
            let app = snapshot.apps.iter().find(|entry| &entry.id == id).ok_or("app not in native inventory")?;
            if !app.detected || (app.job.is_some() && self.operation != Operation::Cancel) {
                return Err("native state unavailable or app busy".into());
            }
            let allowed = match self.operation {
                Operation::Install => !app.installed && app.compatible && app.installable,
                Operation::Update | Operation::UpdateAll => app.installed && app.compatible && app.installable && app.update_available,
                Operation::Open | Operation::Remove => app.installed,
                Operation::Cancel => app.job.is_some(),
            };
            if !allowed {
                return Err(format!("operation {:?} is unavailable for {id} in its current native state", self.operation));
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Snapshot {
    pub updated_at: i64,
    pub apps: Vec<InventoryEntry>,
    pub results: std::collections::BTreeMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(operation: Operation) -> Request {
        Request { id: "test-request".into(), operation, apps: vec!["handy".into()] }
    }
    fn inventory() -> Snapshot {
        Snapshot {
            apps: vec![InventoryEntry { id: "handy".into(), detected: true, compatible: true, installable: true, ..Default::default() }],
            ..Default::default()
        }
    }
    #[test]
    fn operations_require_the_expected_native_state() {
        let mut state = inventory();
        assert!(request(Operation::Install).validate_state(&state).is_ok());
        for operation in [Operation::Update, Operation::UpdateAll, Operation::Open, Operation::Remove, Operation::Cancel] {
            assert!(request(operation).validate_state(&state).is_err());
        }
        state.apps[0].compatible = false;
        assert!(request(Operation::Install).validate_state(&state).is_err());
        state.apps[0].compatible = true;
        state.apps[0].installed = true;
        assert!(request(Operation::Install).validate_state(&state).is_err());
        assert!(request(Operation::Open).validate_state(&state).is_ok());
        assert!(request(Operation::Remove).validate_state(&state).is_ok());
        assert!(request(Operation::Update).validate_state(&state).is_err());
        state.apps[0].update_available = true;
        for operation in [Operation::Update, Operation::UpdateAll] {
            assert!(request(operation).validate_state(&state).is_ok());
        }
        state.apps[0].job = Some("Downloading".into());
        assert!(request(Operation::Update).validate_state(&state).is_err());
        assert!(request(Operation::Cancel).validate_state(&state).is_ok());
        state.apps[0].detected = false;
        assert!(request(Operation::Cancel).validate_state(&state).is_err());
    }
    #[test]
    fn approval_revalidates_state_and_rejects_unknown_inputs() {
        let mut state = inventory();
        let mut req = request(Operation::Install);
        assert!(req.validate_state(&state).is_ok());
        state.apps[0].installed = true;
        assert!(req.validate_state(&state).is_err());
        req.apps = vec!["unknown-app".into()];
        assert!(req.validate_state(&state).is_err());
        req.apps = vec!["handy".into(), "handy".into()];
        req.operation = Operation::UpdateAll;
        assert!(req.validate().is_err());
        assert!(serde_json::from_str::<Request>(r#"{"id":"test","operation":"shell","apps":["handy"]}"#).is_err());
        assert!(serde_json::from_str::<Request>(r#"{"id":"test","operation":"open","apps":["handy"],"command":"echo"}"#).is_err());
    }
}
