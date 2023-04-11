use mongodb::bson::{doc, oid::ObjectId};
use mongodb::{bson, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub channel: String,
    pub text: String,
    #[serde(rename = "sentiment")]
    pub hugging_face: String,
    pub emotion: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: bson::DateTime,
}

pub async fn get_mongo_db(uri: &str) -> Database {
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse MongoDB client otions");

    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB client");

    client.database("discord-stats")
}
