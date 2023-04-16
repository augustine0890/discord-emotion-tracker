// #![allow(unused_mut, unused_variables, unused_imports)]
use chrono::prelude::*;
use chrono_tz::Asia::Seoul;
use cron::Schedule;
use serenity::client::Client;
use serenity::model::id::ChannelId;
use std::str::FromStr;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use tokio::time::interval_at;

use crate::discord::{memory_stats_alert_embed, memory_stats_embed, send_embed_to_user};

#[derive(Debug, Clone, Copy)]
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

    println!("-----------------------------------------------------------");
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
    println!("Used memory percentage: {:.2}%", used_memory_percent);
    println!("-----------------------------------------------------------");
}

pub async fn monitor_memory_stats(client: Client, channel_id: ChannelId) {
    // Set up the intervals for monitoring, printing, and alerting
    let monitoring_interval = Duration::from_secs(2 * 60); // 2 minutes
    let print_interval = Duration::from_secs(24 * 60 * 60); // 24 hours
    let alert_interval = Duration::from_secs(60 * 60); // 1 hour

    // Create timers for each interval
    let mut monitoring_timer = interval_at(tokio::time::Instant::now(), monitoring_interval);
    let mut print_timer = interval_at(tokio::time::Instant::now(), print_interval);
    let mut alert_timer = interval_at(tokio::time::Instant::now(), alert_interval);

    // Initialize the memory stats
    let mut stats = get_memory_stats();

    let http = client.cache_and_http.http.clone();
    let sending_task = async move {
        // Set up the cron schedule for sending the memory_stats_embed at 10 AM every day
        let cron_expression = "0 56 22 * * *"; // 10:0 AM every day
        let schedule =
            Schedule::from_str(cron_expression).expect("Failed to parse the cron schedule");

        loop {
            let seoul_now = Utc::now().with_timezone(&Seoul);
            let next_event = schedule.upcoming(Seoul).next().unwrap();

            let duration_until_next_event = (next_event - seoul_now).to_std().unwrap();
            tokio::time::sleep(duration_until_next_event).await;

            let embed = memory_stats_embed(stats.clone());
            let _ = channel_id
                .send_message(&http, |m| m.set_embed(embed.clone()))
                .await;
        }
    };

    // Send the memory_stats_embed at 10 AM every day
    tokio::spawn(sending_task);

    loop {
        tokio::select! {
            // Update memory stats every 2 minutes
            _ = monitoring_timer.tick() => {
                stats = get_memory_stats();
            },

            // Check if an alert needs to be sent every hour
            _ = alert_timer.tick() => {
                // If used_memory_percentage is greater than 95%, send an alert
                if stats.used_memory_percentage > 95.0 {
                    let embed = memory_stats_alert_embed(stats.clone());

                    // Send the embed to the channel
                    let _ = channel_id.send_message(&client.cache_and_http.http, |m| {
                        m.set_embed(embed.clone())
                    }).await;

                    // Send the alert embed as a direct message
                    let user_id: u64 = 623155071735037982; // Replace with the target user's ID
                    if let Err(e) = send_embed_to_user(&client, user_id, embed).await {
                        println!("Error sending DM: {:?}", e);
                    }
                }
            },

            _ = print_timer.tick() => {
                display_memory_stats();
            },
        }
    }
}
