use serde::Deserialize;
use std::fs;
use std::env;
use log::info;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    // Define your config fields here, e.g.,
    env: String,
}

pub fn load_config() -> Config {
    // Determine the environment (default to "local" if not set)
    let env = env::var("APP_ENV").unwrap_or_else(|_| "local".to_string());

    // Load the appropriate configuration file
    let config_file = format!("config/{}.json", env);
    let config_content = fs::read_to_string(config_file)
        .expect("Failed to read configuration file");

    // Parse the JSON configuration
    let config: Config = serde_json::from_str(&config_content)
        .expect("Failed to parse configuration file");

    info!("Loaded config for environment: {}", env);
    config
}


#[cfg(test)]
mod tests {
    use super::*;
    // use std::fs::File;
    // use std::io::Write;
    // use tempfile::tempdir;

    #[test]
    fn test_load_config_success() {
       /*  let dir = tempdir().unwrap();
        let file_path = dir.path().join("config/local.json");
        let mut file = File::create(&file_path).unwrap();

        let json_content = r#"
        {
            "env": "postgres://user:password@localhost/db_name"
        }"#;

        writeln!(file, "{}", json_content).unwrap();*/

        env::set_var("APP_ENV", "qa");

        let config = load_config();
        assert_eq!(config.env, "test");
       // assert_eq!(config.server_port, 3000);
    }

    #[test]
    #[should_panic]
    fn test_load_config_failure() {
        env::set_var("APP_ENV", "non_existent_env");
        load_config();  // This should panic because the file doesn't exist
    }
}
