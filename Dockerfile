# Build stage
FROM rust:1.89 AS builder

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

# Default locations (can be overridden)
ENV WEATHER_LOCATIONS=Oslo

# User-Agent must be provided either via:
# 1. Command-line: docker run weather-exporter --user-agent 'my-app/1.0'
# 2. Environment: docker run -e WEATHER_USER_AGENT='my-app/1.0' weather-exporter

EXPOSE 9090

ENTRYPOINT ["weather-exporter"]
