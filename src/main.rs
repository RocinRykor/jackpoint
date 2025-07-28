mod aur;
mod db;
mod config;

use anyhow::Result;
use config::Config;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

struct AppState {
    config: Config,
    package_db: Mutex<db::PackageDb>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    let package_db = db::PackageDb::new("state.json")?;

    let state = Arc::new(AppState {
        config,
        package_db: Mutex::new(package_db),
    });

    // Start monitoring for AUR packages
    let monitor_handle = tokio::spawn(monitor_aur_packages(state.clone()));

    // ... (Web server would go here later)

    monitor_handle.await??;
    Ok(())
}

async fn monitor_aur_packages(state: Arc<AppState>) -> Result<()> {
    let packages = &["yay", "google-chrome", "visual-studio-code-bin"]; // Hardcoded for now

    loop {
        for package in packages {
            check_and_update_package(state.clone(), package).await?;
        }

        sleep(Duration::from_secs(state.config.aur.check_interval)).await;
    }
}

async fn check_and_update_package(state: Arc<AppState>, package: &str) -> Result<()> {
    // Fetch AUR package info
    let aur_info = match aur::get_package_info(package).await {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Error fetching {}: {}", package, e);
            return Ok(());
        }
    };

    let mut db = state.package_db.lock().await;

    // Check if we need to build
    if db.needs_build(package, &aur_info.version) {
        println!("Update needed for {}: {}", package, aur_info.version);

        // Mark as building in DB
        db.mark_building(package, &aur_info.version)?;

        // Release lock while building
        drop(db);

        // Build package (Phase 3 will implement this)
        match build_package(package, &aur_info.version).await {
            Ok(_) => {
                let mut db = state.package_db.lock().await;
                db.mark_built(package, &aur_info.version)?;
            }
            Err(e) => {
                let mut db = state.package_db.lock().await;
                db.mark_failed(package, &e.to_string())?;
            }
        }
    }

    Ok(())
}

// Stub for Phase 3
async fn build_package(_package: &str, _version: &str) -> Result<()> {
    // Will be implemented in Phase 3
    Ok(())
}