mod config;

use config::Config;

use std::env;

fn main() {
    // Load the configuration from the YAML file
    let config =
        Config::from_file("src/config.yaml").expect("Failed to load configuration from YAML file");

    // Choose the enviroment: "development" or "production"
    let enviroment = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    let env_config = config
        .get_environment(&enviroment)
        .expect("Invalid environment configuration");

    println!("{:?}", env_config);
}
