use anyhow::Result;
use tracing::*;

pub async fn run_example() -> Result<()> {
    info!("Running example feature…");

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    info!("Feature completed");
    Ok(())
}
