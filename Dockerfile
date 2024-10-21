# Dockerfile
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Copy the Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use Ubuntu 22.04 as the runtime base image
FROM ubuntu:22.04

# Install SSL certificates
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/api_server .

# Make the binary executable
RUN chmod +x api_server

# Create a script to start the application
RUN echo '#!/bin/bash\n\
PORT="${PORT:-8080}"\n\
./api_server' > start.sh && \
    chmod +x start.sh

# Command to run the application
CMD ["./start.sh"]