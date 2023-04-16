use serenity::client::Client;
use serenity::model::id::ChannelId;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use tokio::time::interval_at;

use crate::discord::{memory_stats_alert_embed, memory_stats_embed, send_embed_to_user};

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_memory: f64,
    pub used_memory: f64,
    pub free_memory: f64,
    pub available_memory: f64,
    pub used_memory_percentage: f64,
}

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}

fn get_memory_stats() -> MemoryStats {
    let mut system = System::new_all();
    system.refresh_all();

    let total_memory = bytes_to_gb(system.total_memory());
    let used_memory = bytes_to_gb(system.used_memory());
    let free_memory = bytes_to_gb(system.free_memory());
    let available_memory = bytes_to_gb(system.available_memory());
    let used_memory_percentage = used_memory as f64 / total_memory as f64 * 100.0;

    MemoryStats {
        total_memory,
        used_memory,
        free_memory,
        available_memory,
        used_memory_percentage,
    }
}

fn display_memory_stats() {
    let mut system = System::new_all();
    system.refresh_all();

    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let free_memory = system.free_memory();
    // "free" memory refers to unallocated memory whereas "available" memory refers to memory that is available for (re)use.
    let available_memory = system.available_memory();
    let used_memory_percent = used_memory as f64 / total_memory as f64 * 100.0;

    println!(
        "Total memory: {} bytes ({:.2} GB)",
        total_memory,
        bytes_to_gb(total_memory)
    );
    println!(
        "Used memory: {} bytes ({:.2} GB)",
        used_memory,
        bytes_to_gb(used_memory)
    );
    println!(
        "Free memory: {} bytes ({:.2} GB)",
        free_memory,
        bytes_to_gb(free_memory)
    );
    println!(
        "Available memory: {} bytes ({:.2} GB)",
        available_memory,
        bytes_to_gb(available_memory)
    );
    println!("Used memory percentage: {:.2}%\n", used_memory_percent);
    println!("-----------------------------------------------------------");
}

pub async fn monitor_memory_stats(client: &Client, channel_id: ChannelId) {
    let monitoring_interval = Duration::from_secs(2 * 60); // 2 minutes
    let print_interval = Duration::from_secs(24 * 60 * 60); // 24 hours
    let sending_interval = Duration::from_secs(24 * 60 * 60); // 24 hours
    let alert_interval = Duration::from_secs(60 * 60); // 1 hour

    let mut monitoring_timer = interval_at(tokio::time::Instant::now(), monitoring_interval);
    let mut print_timer = interval_at(tokio::time::Instant::now(), print_interval);
    let mut sending_timer = interval_at(tokio::time::Instant::now(), sending_interval);
    let mut alert_timer = interval_at(tokio::time::Instant::now(), alert_interval);

    let mut stats = get_memory_stats();

    loop {
        tokio::select! {
            _ = monitoring_timer.tick() => {
                stats = get_memory_stats();
            },

            _ = alert_timer.tick() => {
                if stats.used_memory_percentage > 95.0 {
                    let embed = memory_stats_alert_embed(stats.clone());

                    // Send the embed to the channel
                    let _ = channel_id.send_message(&client.cache_and_http.http, |m| {
                        m.set_embed(embed.clone())
                    }).await;

                    // Send the embed as direct message
                    let user_id: u64 = 623155071735037982; // Replace with the target user's ID
                    if let Err(e) = send_embed_to_user(&client, user_id, embed).await {
                        println!("Error sending DM: {:?}", e);
                    }
                }
            },

            _ = sending_timer.tick() => {
                let embed = memory_stats_embed(stats.clone());

                // Send the embed to the channel
                let _ = channel_id.send_message(&client.cache_and_http.http, |m| {
                    m.set_embed(embed.clone())
                }).await;
            },

            _ = print_timer.tick() => {
                display_memory_stats();
            },
        }
    }
}
