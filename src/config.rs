
use serde::Deserialize;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Deserialize)]
pub struct Config {
    pub repo: RepoConfig,
    pub server: ServerConfig,
    pub aur: AurConfig,
}

#[derive(Deserialize)]
pub struct RepoConfig {
    pub path: PathBuf,
    pub architecture: String,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

#[derive(Deserialize)]
pub struct AurConfig {
    pub check_interval: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .build()?;

        Ok(settings.try_deserialize()?)
    }
}