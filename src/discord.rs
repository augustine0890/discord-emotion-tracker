use chrono::{Duration, Utc};
// use mongodb::{options::ClientOptions, Client, Database};
use crate::mongo::{save_message, Message};
use mongodb::Database;
use serenity::{
    async_trait,
    model::{channel::Channel, channel::Message as DiscordMessage, gateway::Ready},
    prelude::*,
};

#[allow(dead_code)]
struct Handler {
    db: Database,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        if msg.author.bot {
            return;
        }

        // Add 9 hours to the system time
        let adjusted_timestamp = Utc::now() + Duration::hours(9);

        let channel_name = get_channel_name(ctx, &msg).await.unwrap_or_default();
        let message = Message {
            id: None,
            username: msg.author.name,
            channel: channel_name,
            text: msg.content,
            hugging_face: None,
            emotion: None,
            created_at: adjusted_timestamp,
        };

        if let Err(e) = save_message(&self.db, &message).await {
            println!("Error saving message: {:?}", e);
        }
    }
}

pub async fn run_discord_bot(token: &str, db: Database) -> tokio::task::JoinHandle<()> {
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { db })
        .await
        .expect("Error creating Discord client");

    let handler = tokio::spawn(async move {
        client.start().await.expect("Error starting Discord client");
    });
    handler
}

async fn get_channel_name(ctx: Context, message: &DiscordMessage) -> Option<String> {
    let channel_id = message.channel_id;
    let channel = channel_id.to_channel(&ctx).await.ok()?;
    match channel {
        Channel::Guild(channel) => Some(channel.name),
        Channel::Private(channel) => Some(channel.name()),
        _ => None,
    }
}
