# Builder stage
FROM rust:1.89 AS builder

# Install necessary system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Set work directory
WORKDIR /app

# Copy manifest files first to leverage Docker cache for dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory and main.rs to cache dependencies
# This allows Docker to cache the build of dependencies when Cargo.toml/Cargo.lock don't change
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies. This command will cache the build of all dependencies listed in Cargo.toml.
# It will only re-run if Cargo.toml or Cargo.lock changes.
RUN cargo build --release --target-dir /app/target_deps

# Remove the dummy src directory and its content
RUN rm -rf src

# Copy the actual source code
COPY src ./src

# Build the application using previously cached dependencies.
# The previous step's target_deps will be leveraged, making this step faster.
RUN cargo build --release

# Runtime stage
FROM debian:stable-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN groupadd -r app && useradd -r -g app app

# Set work directory
WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/mofumofu-backend .

# Copy environment file template (optional)
# If you have an actual .env file, uncomment the line below.
# COPY .env .env

# Change ownership to app user
RUN chown -R app:app /app

# Switch to app user
USER app

# Expose port
EXPOSE 8000

# Run the application
CMD ["./mofumofu-backend"]