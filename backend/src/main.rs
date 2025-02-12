use crate::setup::setup;
use log::error;
use std::env;

mod hive;


mod health;
mod metrics;
mod setup;
mod config;

#[tokio::main]
pub async fn main() {
    // Setup logging and configuration
    let config = setup().await;

    hive::run(config).await;
        
}
