use env_logger;
use log::{info};
use crate::config::{load_config};
use std::env;


pub async fn setup() {

    // Get the log level from the environment or default to "info"
    let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    env::set_var("RUST_LOG", &log_level);
    
    // Initialize the logger with the log level
    env_logger::init();
    info!("Logger initialized with level: {}", log_level);

    // Load the configuration
    let config = load_config();
    info!("Loaded configuration: {:?}", config);
}


#[cfg(test)]
mod tests {
    use super::*;
    //use crate::config::load_config;

    #[tokio::test]
    async fn test_setup() {
        let _ = setup().await;
        assert!(true, "Setup completed successfully");
    }
}
