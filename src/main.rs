mod config;
mod errors;
mod features;

use anyhow::Result;
use tracing::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("Starting application…");

    // Load config
    let cfg = config::AppConfig::from_env()?;
    info!("Loaded config: {:?}", cfg);

    // Run an example async task
    features::run_example().await?;

    Ok(())
}
