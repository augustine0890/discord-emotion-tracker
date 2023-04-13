# Use the latest Rust base image
FROM rust:latest

# Update the package repository and install dependencies
RUN apt-get update && \
    apt-get install -y software-properties-common python3-dev python3-pip libopenblas-dev libopenmpi-dev

# Add the PyTorch repository to the system's package manager
RUN add-apt-repository ppa:ubuntu-toolchain-r/test

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

# Set the container to run the Discord emotion tracker executable when it starts
CMD ["./target/release/discord-emotion-tracker"]
