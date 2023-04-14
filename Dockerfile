# Use the Rust slim base image
FROM rust:slim AS builder

# Update the package repository and install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends software-properties-common python3-dev python3-pip libopenblas-dev libopenmpi-dev libtinfo5 curl unzip libssl-dev pkg-config g++

# Set the working directory to '/app'
WORKDIR /app

# Download and unzip the libtorch CPU version (version 1.13.1) and remove the zip file after extraction
RUN curl -LJO https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcpu.zip && \
    unzip libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcpu.zip && \
    rm libtorch-cxx11-abi-shared-with-deps-1.13.1%2Bcpu.zip

# Copy the application code from the current directory to the working directory in the container
COPY . .

# Set the LIBTORCH and LD_LIBRARY_PATH environment variables to enable the application to find the PyTorch library
ENV LIBTORCH='/app/libtorch'
ENV LD_LIBRARY_PATH="${LIBTORCH}/lib:$LD_LIBRARY_PATH"

# Build the Rust application in release mode
RUN cargo build --release

# Use a minimal Debian-based image for the final stage
FROM debian:bullseye-slim

# Install necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends libopenblas-dev libopenmpi-dev libtinfo5 libssl1.1 libgomp1 && \
    rm -rf /var/lib/apt/lists/*

# Copy the application binary, libtorch, and config.yaml from the builder stage
COPY --from=builder /app/target/release/discord-emotion-tracker /usr/local/bin/
COPY --from=builder /app/libtorch /libtorch
COPY --from=builder /app/config.yaml /app/config.yaml

# Set the CONFIG_PATH environment variable
ENV CONFIG_PATH="/app/config.yaml"

# Set the LD_LIBRARY_PATH environment variable to enable the application to find the PyTorch library
ENV LD_LIBRARY_PATH="/libtorch/lib:$LD_LIBRARY_PATH"

# Set the container to run the Discord emotion tracker executable when it starts
CMD ["discord-emotion-tracker"]
