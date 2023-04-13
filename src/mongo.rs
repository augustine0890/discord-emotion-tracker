use chrono::Utc;
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::error::Error;
use mongodb::{bson, options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub channel: String,
    pub text: String,
    #[serde(rename = "sentiment", skip_serializing_if = "Option::is_none")]
    pub hugging_face: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotion: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: chrono::DateTime<Utc>,
}

pub async fn get_mongo_db(uri: &str) -> Database {
    let client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse MongoDB client otions");

    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB client");

    client.database("discord-stats")
}

pub async fn save_message(db: &Database, message: &Message) -> Result<(), Error> {
    let message_collection = db.collection("messages");
    let message_doc = bson::to_bson(&message)
        .unwrap()
        .as_document()
        .unwrap()
        .clone();
    message_collection
        .insert_one(message_doc, None)
        .await
        .map(|_| ())
}