# Use the official Rust image as the base image for building your application
FROM rust:latest as builder

# Create a new directory for the application and set it as the working directory
WORKDIR /app

# Copy the `Cargo.toml`, `Cargo.lock`, and configuration files into the container
COPY Cargo.toml Cargo.lock config.yaml ./

# Copy the source code into the container
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Start a new stage for the final deployment image
FROM debian:bullseye-slim

# Install the necessary dependencies for the app
RUN apt-get update && \
    apt-get install -y ca-certificates tzdata && \
    rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage to the current stage
COPY --from=builder /app/target/release/discord-emotion-tracker /usr/local/bin/

# Copy the configuration file from the builder stage to the current stage
COPY --from=builder /app/config.yaml /app/

# Set the working directory
WORKDIR /app

# Set the default command to run your application
CMD ["discord-emotion-tracker"]
