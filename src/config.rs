use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub development: EnvConfig,
    pub production: EnvConfig,
}

#[derive(Debug, Deserialize)]
pub struct EnvConfig {
    pub discord_token: String,
    pub mongo_uri: String,
    pub discord_guild: String,
    pub aws_access_key_id: Option<String>,
    pub aws_secret_access_key: Option<String>,
    pub aws_region: Option<String>,
}

impl Config {
    pub fn get_environment(&self, environment: &str) -> Option<&EnvConfig> {
        match environment {
            "production" => Some(&self.production),
            "development" => Some(&self.development),
            _ => None,
        }
    }
}

pub fn load_config(file_path: &str) -> Config {
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let config: Config = serde_yaml::from_str(&contents).expect("Error while parsing YAML config");
    config
}

pub fn set_env_variables(config: &EnvConfig) {
    env::set_var("DISCORD_TOKEN", &config.discord_token);
    env::set_var("MONGO_URI", &config.mongo_uri);
    env::set_var("DISCORD_GUILD", &config.discord_guild);
    if let Some(aws_access_key_id) = &config.aws_access_key_id {
        env::set_var("AWS_ACCESS_KEY_ID", aws_access_key_id);
    }
    if let Some(aws_secret_access_key) = &config.aws_secret_access_key {
        env::set_var("AWS_SECRET_ACCESS_KEY", aws_secret_access_key);
    }
    if let Some(aws_region) = &config.aws_region {
        env::set_var("AWS_REGION", aws_region);
    }
}
