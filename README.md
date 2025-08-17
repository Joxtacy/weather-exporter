# Weather Exporter for Prometheus

[![Build and Test](https://github.com/Joxtacy/weather-exporter/actions/workflows/build.yml/badge.svg)](https://github.com/Joxtacy/weather-exporter/actions/workflows/build.yml)
[![Security Audit](https://github.com/Joxtacy/weather-exporter/actions/workflows/security.yml/badge.svg)](https://github.com/Joxtacy/weather-exporter/actions/workflows/security.yml)
[![Docker Pulls](https://img.shields.io/docker/pulls/joxtacy/weather-exporter)](https://hub.docker.com/r/joxtacy/weather-exporter)
[![GitHub Release](https://img.shields.io/github/v/release/Joxtacy/weather-exporter)](https://github.com/Joxtacy/weather-exporter/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust-based weather metrics exporter that fetches weather data from yr.no API and exposes it in Prometheus format. Supports monitoring multiple locations simultaneously.

## Features

- Fetches real-time weather data from yr.no (Norwegian Meteorological Institute)
- **Supports multiple locations** - monitor weather for multiple cities at once
- Automatic location search by place name
- Respects API rate limits with proper caching and conditional requests
- Exposes metrics in Prometheus format
- Independent cache management per location
- Comprehensive weather metrics including:
  - Temperature (Celsius)
  - Humidity (%)
  - Wind speed (m/s) and direction (degrees)
  - Air pressure (hPa)
  - Precipitation (mm)
  - Cloud coverage (%)
  - UV index

## Installation

### Pre-built Binaries

Download the latest release from the [GitHub Releases](https://github.com/Joxtacy/weather-exporter/releases) page.

```bash
# Linux (amd64)
curl -L https://github.com/Joxtacy/weather-exporter/releases/latest/download/weather-exporter-linux-amd64.tar.gz | tar xz

# Linux (arm64)
curl -L https://github.com/Joxtacy/weather-exporter/releases/latest/download/weather-exporter-linux-arm64.tar.gz | tar xz

# macOS (Intel)
curl -L https://github.com/Joxtacy/weather-exporter/releases/latest/download/weather-exporter-macos-amd64.tar.gz | tar xz

# macOS (Apple Silicon)
curl -L https://github.com/Joxtacy/weather-exporter/releases/latest/download/weather-exporter-macos-arm64.tar.gz | tar xz

# Make it executable
chmod +x weather-exporter
```

### From Source

```bash
git clone https://github.com/Joxtacy/weather-exporter.git
cd weather-exporter
cargo build --release
```

### Docker

```bash
# From Docker Hub
docker pull joxtacy/weather-exporter:latest

# From GitHub Container Registry
docker pull ghcr.io/joxtacy/weather-exporter:latest
```

## Building

```bash
cargo build --release
```

## Running

### Single location:
```bash
cargo run -- "Stockholm"
# or
./target/release/weather-exporter "Oslo"
```

### Multiple locations (comma-separated):
```bash
cargo run -- "Stockholm, Oslo, Copenhagen, Helsinki"
# or
./target/release/weather-exporter "New York, Los Angeles, Chicago"
```

### Using environment variable:
```bash
export WEATHER_LOCATIONS="London, Paris, Berlin, Rome"
cargo run
```

### Custom port:
```bash
export PORT=8080
export WEATHER_LOCATIONS="Tokyo, Seoul, Beijing"
cargo run
```

## Docker

Build the Docker image:
```bash
docker build -t weather-exporter .
```

Run with Docker:
```bash
# Single location
docker run -d \
  -p 9090:9090 \
  -e WEATHER_LOCATIONS="Berlin" \
  weather-exporter

# Multiple locations
docker run -d \
  -p 9090:9090 \
  -e WEATHER_LOCATIONS="Berlin, Munich, Hamburg, Frankfurt" \
  weather-exporter
```

## Prometheus Configuration

Add this to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'weather'
    static_configs:
      - targets: ['localhost:9090']
    scrape_interval: 5m
```

## Endpoints

- `/metrics` - Prometheus metrics endpoint
- `/health` - Health check endpoint

## Metrics

| Metric | Description | Labels |
|--------|-------------|--------|
| `weather_temperature_celsius` | Temperature in Celsius | location, latitude, longitude |
| `weather_humidity_percent` | Relative humidity percentage | location, latitude, longitude |
| `weather_wind_speed_mps` | Wind speed in meters per second | location, latitude, longitude |
| `weather_wind_direction_degrees` | Wind direction in degrees | location, latitude, longitude |
| `weather_pressure_hpa` | Air pressure in hectopascals | location, latitude, longitude |
| `weather_precipitation_mm` | Precipitation in millimeters | location, latitude, longitude |
| `weather_cloud_coverage_percent` | Cloud coverage percentage | location, latitude, longitude |
| `weather_uv_index` | UV index | location, latitude, longitude |
| `weather_fetch_success` | Whether the last weather fetch was successful (1 or 0) | location |

## Example Prometheus Queries

```promql
# Current temperature for a specific location
weather_temperature_celsius{location="Oslo"}

# Compare temperatures across multiple cities
weather_temperature_celsius

# Average temperature across all monitored locations
avg(weather_temperature_celsius)

# Highest wind speed among all locations
max(weather_wind_speed_mps)

# Locations with humidity above 80%
weather_humidity_percent > 80

# Average wind speed over the last hour for Stockholm
avg_over_time(weather_wind_speed_mps{location="Stockholm"}[1h])

# Alert when temperature drops below freezing in any location
weather_temperature_celsius < 0

# Cache hit rate per location
rate(weather_cache_hits_total[5m])
```

## Grafana Dashboard

You can visualize these metrics in Grafana. Example panel queries:

- **Temperature comparison**: `weather_temperature_celsius` (use legend format `{{location}}`)
- **Weather overview table**: Multiple queries with location as a variable
- **Wind speed time series**: `weather_wind_speed_mps{location=~"$location"}`
- **Humidity heatmap**: `weather_humidity_percent`
- **Cache efficiency**: `rate(weather_cache_hits_total[5m]) / rate(weather_api_calls_total[5m])`

Create a Grafana variable for location selection:
- Variable type: Query
- Query: `label_values(weather_temperature_celsius, location)`
- Multi-value: enabled

## API Attribution

This application uses the yr.no API provided by the Norwegian Meteorological Institute. Please respect their [terms of service](https://developer.yr.no/doc/TermsOfService/).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Development

```bash
# Run tests
cargo test
# or
make test

# Run with logging
RUST_LOG=debug cargo run -- "Oslo"
# or
make run-debug LOCATIONS="Oslo"

# Format code
cargo fmt
# or
make fmt

# Run clippy
cargo clippy -- -D warnings
# or
make lint

# Run all checks
make check
```

### Releasing

To create a new release:

```bash
# Patch release (bug fixes)
make release-patch
# or
./scripts/release.sh patch

# Minor release (new features)
make release-minor
# or
./scripts/release.sh minor

# Major release (breaking changes)
make release-major
# or
./scripts/release.sh major
```

See [RELEASING.md](RELEASING.md) for detailed release instructions.

## Author

[Joxtacy](https://github.com/Joxtacy)

## License

MIT
