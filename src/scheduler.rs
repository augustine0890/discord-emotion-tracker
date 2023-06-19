use crate::mongo::delete_messages;
use chrono::Utc;
use cron::Schedule;
use mongodb::Database;
use std::str::FromStr;
use tokio::time::sleep;

pub async fn start_scheduler(db: &Database) {
    // Define the cron expression for scheduling the task.
    let cron_expression = "0 0 1 * * MON";
    // let cron_expression = "0 * * * * *"; // Runs every minute
    let schedule = Schedule::from_str(cron_expression).unwrap();

    loop {
        // Find the next scheduled event based on the cron expression
        let next_event = schedule.upcoming(chrono::Utc).next().unwrap();
        // Get the current time
        let now = Utc::now();

        // Calculate the duration until the next scheduled event
        let duration_until_next_event = (next_event - now).to_std().unwrap();
        let days = duration_until_next_event.as_secs() / (24 * 3600);
        let hours = (duration_until_next_event.as_secs() % (24 * 3600)) / 3600;

        // Print the waiting time until the next scheduled event
        println!(
            "[Delete Documents] Waiting until next scheduled event [{}]: in {} days and {} hours.",
            next_event, days, hours
        );

        // Sleep the current task for the calculated duration
        sleep(duration_until_next_event).await;

        // Get the current timestamp
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");

        // Print the message for running delete messages
        println!("[{}] Running delete messages", timestamp);
        let mut task_succeeded = false;

        while !task_succeeded {
            // Run the delete_messages function
            match delete_messages(&db).await {
                Ok(result) => {
                    let deleted_count = result.deleted_count;
                    task_succeeded = true;
                    // Print the success message with the number of deleted messages
                    println!("[{}] Deleted {} message(s)", timestamp, deleted_count);
                }
                Err(e) => {
                    // Print the error message if there's an error deleting messages
                    println!("[{}] Error deleting messages: {:?}", timestamp, e);
                    // Sleep for 5 minutes before retrying
                    tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
                }
            }
        }
    }
}
