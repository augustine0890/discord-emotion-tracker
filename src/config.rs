use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub development: EnvironmentConfig,
    pub production: EnvironmentConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub discord_token: String,
    pub mongo_uri: String,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(env::current_dir()?.join(path))?;
        let config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub fn get_environment(&self, environment: &str) -> Option<&EnvironmentConfig> {
        match environment {
            "development" => Some(&self.development),
            "production" => Some(&self.production),
            _ => None,
        }
    }
}
