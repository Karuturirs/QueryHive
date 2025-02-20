use crate::setup::setup;
use log::error;

mod hive;
mod health;
mod metrics;
mod setup;
mod config;
mod indexing;

#[tokio::main]
pub async fn main() {
    // Setup logging and configuration
    let config = setup().await;

    hive::run(config).await;
    if let Err(e) = indexing::start_indexing().await {
        error!("Error starting indexing: {}", e);
    }
        
}
