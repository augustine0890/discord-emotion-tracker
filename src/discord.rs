use crate::mongo::{save_message, Message};
use crate::monitor::{monitor_memory_stats, MemoryStats};
use crate::sentiment::analyze_sentiment;
use crate::util::{
    has_minimum_word_count, replace_mentions, should_ignore_channel, should_ignore_user,
    should_not_ignore_guild,
};
use chrono::{Duration, Utc};

use mongodb::Database;
use serenity::builder::CreateEmbed;
use serenity::utils::Color;
use serenity::{
    async_trait,
    model::{channel::Channel, channel::Message as DiscordMessage, gateway::Ready, id::ChannelId},
    prelude::*,
};

struct Handler {
    db: Database,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: DiscordMessage) {
        // Skip processing messages from bots
        if msg.author.bot
            || should_ignore_user(&msg)
            || should_ignore_channel(&msg)
            || !has_minimum_word_count(&msg, 5)
            || should_not_ignore_guild(&msg)
        {
            return;
        }

        // Replace mentions in the message content
        let content = replace_mentions(&ctx, &msg).await;

        // Try to analyze sentiment and log any error that occurs
        let sentiment = analyze_sentiment(&content)
            .await
            .map_err(|err| println!("Error detecting sentiment: {}", err))
            .ok();

        // Adjust the timestamp to the local timezone (UTC+9)
        let adjusted_timestamp = Utc::now() + Duration::hours(9);

        // Get the name of the channel the message was sent in
        let channel_name = get_channel_name(ctx, &msg).await.unwrap_or_default();

        // Create a Message struct from the discord message
        let message = Message {
            id: None,
            username: msg.author.name,
            channel: channel_name,
            text: content,
            analyzed: sentiment,
            created_at: adjusted_timestamp,
            ..Default::default()
        };

        // Save the message to the database
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

    // Start monitoring and sending memory stats
    let channel_id = ChannelId(1054296641651347486); // Replace with the specific channel ID
    monitor_memory_stats(&client, channel_id).await;

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

pub fn memory_stats_embed(stats: MemoryStats) -> CreateEmbed {
    let mut embed = CreateEmbed::default();
    embed
        .title("Daily Memory Usage Report")
        .field(
            "Total Memory",
            format!("{:.2} GB", stats.total_memory),
            true,
        )
        .field("Used Memory", format!("{:.2} GB", stats.used_memory), true)
        .field("Free Memory", format!("{:.2} GB", stats.free_memory), true)
        .field(
            "Available Memory",
            format!("{:.2} GB", stats.available_memory),
            false,
        )
        .field(
            "Used Memory Percentage",
            format!("{:.2}%", stats.used_memory_percentage),
            false,
        )
        .timestamp(chrono::Utc::now().to_rfc3339())
        .color(Color::new(0x0000ff));

    embed
}
