# Build stage
FROM rust:1.75 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/weather-exporter /usr/local/bin/weather-exporter

ENV PORT=9090
# WEATHER_USER_AGENT must be set when running the container
# Example: -e WEATHER_USER_AGENT='my-app/1.0 github.com/username/repo'

# WEATHER_LOCATIONS is optional, defaults to Oslo if not set
# Example: -e WEATHER_LOCATIONS='Oslo, Stockholm, Copenhagen'

EXPOSE 9090

CMD ["weather-exporter"]
