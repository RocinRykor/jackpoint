use std::cmp::PartialEq;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum BuildStatus {
    Pending,
    Building,
    Success,
    Failed(String),  // Error message
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageState {
    pub aur_version: String,
    pub built_version: Option<String>,
    pub last_checked: DateTime<Utc>,
    pub last_built: Option<DateTime<Utc>>,
    pub build_status: BuildStatus,
}

pub struct PackageDb {
    path: PathBuf,
    state: HashMap<String, PackageState>,
}

impl PartialEq for BuildStatus {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PackageDb {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        let state = if path.exists() {
            let data = fs::read_to_string(&path)?;
            serde_json::from_str(&data)?
        } else {
            HashMap::new()
        };

        Ok(Self { path, state })
    }

    pub fn needs_build(&self, package: &str, new_version: &str) -> bool {
        match self.state.get(package) {
            Some(state) => {
                state.build_status != BuildStatus::Building &&  // Not currently building
                    state.aur_version != new_version &&             // Version changed
                    state.built_version.as_ref() != Some(&new_version.to_string())  // Not already built
            }
            None => true,
        }
    }

    pub fn mark_building(&mut self, package: &str, aur_version: &str) -> Result<()> {
        let state = PackageState {
            aur_version: aur_version.to_string(),
            built_version: None,
            last_checked: Utc::now(),
            last_built: None,
            build_status: BuildStatus::Building,
        };

        self.state.insert(package.to_string(), state);
        self.save()
    }

    pub fn mark_built(&mut self, package: &str, built_version: &str) -> Result<()> {
        if let Some(state) = self.state.get_mut(package) {
            state.built_version = Some(built_version.to_string());
            state.last_built = Some(Utc::now());
            state.build_status = BuildStatus::Success;
        }
        self.save()
    }

    pub fn mark_failed(&mut self, package: &str, error: &str) -> Result<()> {
        if let Some(state) = self.state.get_mut(package) {
            state.build_status = BuildStatus::Failed(error.to_string());
        }
        self.save()
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.state)?;
        fs::write(&self.path, data)?;
        Ok(())
    }
}