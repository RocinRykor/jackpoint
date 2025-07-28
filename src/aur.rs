use anyhow::{Context, Result};
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize)]
struct AurResponse {
    resultcount: usize,
    results: Vec<AurPackage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")] // Maps snake_case to PascalCase
pub struct AurPackage {
    pub name: String,
    pub version: String,
    pub out_of_date: Option<u64>, // Matches "OutOfDate" in JSON
    pub maintainer: Option<String>,
}

pub async fn get_package_info(package_name: &str) -> Result<AurPackage> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]={}",
        urlencoding::encode(package_name)
    );

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("jackpoint/0.1")
        .build()?;

    let response = client
        .get(&url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch AUR info for {}", package_name))?;

    let aur_response: AurResponse = response
        .json()
        .await
        .with_context(|| format!("Failed to parse AUR response for {}", package_name))?;

    aur_response
        .results
        .into_iter()
        .next()
        .ok_or_else(|| anyhow::anyhow!("Package '{}' not found in AUR", package_name))
}