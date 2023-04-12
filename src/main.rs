mod config;
mod discord;
mod mongo;

use config::Config;
use discord::run_discord_bot;
use mongo::get_mongo_db;

use std::env;

#[tokio::main]
async fn main() {
    // Load the configuration from the YAML file
    let config =
        Config::from_file("src/config.yaml").expect("Failed to load configuration from YAML file");

    // Choose the enviroment: "development" or "production"
    let enviroment = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    let env_config = config
        .get_environment(&enviroment)
        .expect("Invalid environment configuration");

    let db = get_mongo_db(&env_config.mongo_uri).await;
    // List collections in the database
    let coll_names = db.list_collection_names(None).await;
    println!("Collections in database: ");
    for name in coll_names.unwrap() {
        println!("{:?}", name);
    }

    run_discord_bot(&env_config.discord_token, db).await;
}
