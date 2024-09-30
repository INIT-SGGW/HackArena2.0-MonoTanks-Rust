# Stage 1: Build the Rust application
FROM rust:1.81.0-alpine3.20 as builder

# Install musl-tools and other necessary dependencies
RUN apk add --no-cache musl-dev

# Add the x86_64-unknown-linux-musl target to Rust toolchain
RUN rustup target add x86_64-unknown-linux-musl

# Create a new directory for the app and set it as the working directory
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock (if available) to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs file to force cargo to build dependencies first
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build the dependencies only (caching this layer)
RUN cargo build --release --target x86_64-unknown-linux-musl

# Now copy the actual source code of the application
COPY . .

# Build the application, creating a static binary
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Create minimal runtime image using 'scratch'
FROM scratch

# Copy the static binary from the builder stage
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/hackathon_2024_h2_rust_client /app/hackathon_2024_h2_rust_client

# Use a non-root user
USER 1000

# Set the binary as the entry point
ENTRYPOINT ["/app/hackathon_2024_h2_rust_client"]