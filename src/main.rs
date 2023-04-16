mod config;
mod discord;
mod mongo;
mod monitor;
mod scheduler;
mod sentiment;
mod util;

use discord::run_discord_bot;
use mongo::get_mongo_db;
use scheduler::start_scheduler;

use std::env;
use tokio::spawn;

#[tokio::main]
async fn main() {
    // Start monitoring and printing memory stats
    monitor::display_memory_stats();

    let config_path = env::var("CONFIG_PATH").unwrap_or_else(|_| "config.yaml".to_string());
    // Load the configuration from the YAML file
    let config = config::load_config(&config_path);

    // Choose the environment: "development" or "production"
    let environment = env::var("APP_ENV").unwrap_or_else(|_| "production".to_string());
    let env_config = config
        .get_environment(&environment)
        .expect("Invalid environment configuration");

    config::set_env_variables(env_config);

    let db = get_mongo_db(&env_config.mongo_uri).await;

    // Create a clone of the database connection
    let db_clone = db.clone();
    // Start the scheduler for deleting messages, without blocking the main function.
    spawn(async move {
        start_scheduler(&db_clone).await;
    });

    // List collections in the database
    let coll_names = db.list_collection_names(None).await;
    println!("Collections in database: ");
    for name in coll_names.unwrap() {
        println!("{:?}", name);
    }

    let discord_bot_handle = run_discord_bot(&env_config.discord_token, db).await;
    if let Err(err) = discord_bot_handle.await {
        println!("An error occurred while running the Discord Bot: {}", err);
    }
}
