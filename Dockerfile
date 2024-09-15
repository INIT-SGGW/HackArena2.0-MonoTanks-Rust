# Step 1: Build the Rust application
FROM rust:1.79.0-slim-bookworm AS builder

# Create a new directory for the application
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the application
RUN cargo build --release


# Step 2: Create a runtime image using slim
FROM debian:bookworm-20231120-slim

# Upgrading the image to minimize vulnerabilities
RUN apt-get update && apt-get upgrade -y

# Create a new directory for the application
WORKDIR /app

# Copy the built application from the builder stage
COPY --from=builder /app/target/release/hackaton_2024_h2_rust_client .

# Set the entrypoint to pass CLI arguments to the Rust application
ENTRYPOINT ["./hackaton_2024_h2_rust_client"]
