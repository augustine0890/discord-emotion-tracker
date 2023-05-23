#!/bin/bash

# Build your Rust application
cargo build --release

# Start your application in the background, redirect output to a log file,
# and store its PID in a separate file
./target/release/discord-emotion-tracker > discord-emotion-tracker.log 2>&1 & echo $! > discord-emotion-tracker.pid
