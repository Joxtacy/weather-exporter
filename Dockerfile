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
ENV WEATHER_LOCATIONS=Oslo

EXPOSE 9090

CMD ["weather-exporter"]
