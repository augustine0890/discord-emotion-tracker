use crate::mongo::delete_messages;
use chrono::Utc;
use chrono_tz::Asia::Seoul;
use cron::Schedule;
use mongodb::Database;
use std::str::FromStr;
use tokio::time::sleep;

pub async fn start_scheduler(db: &Database) {
    // Define the cron expression for scheduling the task.
    // The task will run at 10:00 AM every Monday.
    let cron_expression = "0 0 10 * * MON";
    // let cron_expression = "0 * * * * *"; // Runs every minute
    let schedule = Schedule::from_str(cron_expression).unwrap();

    loop {
        // Get the current time in the Seoul timezone
        let seoul_now = Utc::now().with_timezone(&Seoul);

        // Find the next scheduled event based on the cron expression
        let next_event = schedule.upcoming(Seoul).next().unwrap();

        // Calculate the duration until the next scheduled event
        let duration_until_next_event = (next_event - seoul_now).to_std().unwrap();
        let days = duration_until_next_event.as_secs() / (24 * 3600);
        let hours = (duration_until_next_event.as_secs() % (24 * 3600)) / 3600;

        let timestamp = Utc::now();
        let formatted_timestamp = timestamp.format("%Y-%m-%d %H:%M:%S");

        // Print the waiting time util the next scheduled event
        println!(
            "Waiting until next scheduled event [{}]: in {} days and {} hours.",
            next_event, days, hours
        );

        // Sleep the current task for the calculated duration
        sleep(duration_until_next_event).await;

        // After waking up, run the delete_messages function
        println!("[{}] Running delete messages", formatted_timestamp);
        // If there is an error while running delete_messages, print the error
        match delete_messages(&db).await {
            Ok(result) => {
                let deleted_count = result.deleted_count;
                println!(
                    "[{}] Deleted {} message(s)",
                    formatted_timestamp, deleted_count
                );
            }
            Err(e) => {
                println!("[{}] Error deleting messages: {:?}", formatted_timestamp, e);
            }
        }
    }
}
