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

### Quick Start

```bash
# Basic usage with required user-agent
weather-exporter --user-agent 'my-app/1.0 github.com/myuser/myrepo'

# Short form
weather-exporter -u 'my-app/1.0 contact@example.com' -l Oslo

# Multiple locations
weather-exporter -u 'my-app/1.0' -l Oslo,Stockholm,Copenhagen

# See all options
weather-exporter --help
```

### Command-Line Options

| Option | Short | Environment Variable | Description | Default |
|--------|-------|---------------------|-------------|---------|
| `--user-agent` | `-u` | `WEATHER_USER_AGENT` | **Required:** Unique identifier for yr.no API | - |
| `--locations` | `-l` | `WEATHER_LOCATIONS` | Comma-separated list of locations | `Oslo` |
| `--port` | `-p` | `PORT` | Port for metrics endpoint | `9090` |
| `--log-level` | - | `RUST_LOG` | Log level (trace/debug/info/warn/error) | `info` |
| `--check` | - | - | Validate configuration and exit | - |
| `--help` | `-h` | - | Show help information | - |
| `--version` | `-V` | - | Show version information | - |

### User-Agent Format

The yr.no API requires a unique User-Agent. Format: `<app-name>/<version> <contact>`

Good examples:
- `my-weather-app/1.0 github.com/username/repo`
- `home-automation/2.5 https://my-website.com`
- `personal-station/1.0 contact@example.com`
- `acme-corp/3.0 ops@acme.com`

### Examples

```bash
# Single location
weather-exporter -u 'my-app/1.0 github.com/user/repo' -l Stockholm

# Multiple locations with custom port
weather-exporter \
  --user-agent 'home-weather/1.0 me@example.com' \
  --locations 'New York,Los Angeles,Chicago' \
  --port 8080

# Using environment variables (still works!)
export WEATHER_USER_AGENT='my-app/1.0 github.com/user/repo'
export WEATHER_LOCATIONS='London,Paris,Berlin'
weather-exporter

# Mixed: env var for user-agent, CLI for locations
export WEATHER_USER_AGENT='my-app/1.0'
weather-exporter -l Tokyo,Seoul,Beijing

# Check configuration without starting
weather-exporter -u 'test/1.0' -l Oslo --check

# Debug logging
weather-exporter -u 'my-app/1.0' --log-level debug
```

## Docker

Build the Docker image:
```bash
docker build -t weather-exporter .
```

Run with Docker:
```bash
# Using command-line arguments
docker run -d \
  -p 9090:9090 \
  weather-exporter \
  --user-agent 'my-app/1.0 github.com/user/repo' \
  --locations Berlin,Munich

# Using environment variables (for compatibility)
docker run -d \
  -p 9090:9090 \
  -e WEATHER_USER_AGENT='my-app/1.0 github.com/user/repo' \
  -e WEATHER_LOCATIONS='Berlin,Munich,Hamburg' \
  weather-exporter

# Mixed approach
docker run -d \
  -p 9090:9090 \
  -e WEATHER_USER_AGENT='my-app/1.0' \
  weather-exporter -l Stockholm,Oslo
```

## Configuration

### Priority Order

Configuration values are resolved in the following order (highest to lowest priority):
1. Command-line arguments
2. Environment variables
3. Default values

### Configuration Options

| Option | CLI Argument | Environment Variable | Required | Default | Description |
|--------|-------------|---------------------|----------|---------|-------------|
| User-Agent | `--user-agent`, `-u` | `WEATHER_USER_AGENT` | **Yes** | - | Unique identifier for yr.no API |
| Locations | `--locations`, `-l` | `WEATHER_LOCATIONS` | No | `Oslo` | Comma-separated list of locations |
| Port | `--port`, `-p` | `PORT` | No | `9090` | Port for metrics endpoint |
| Log Level | `--log-level` | `RUST_LOG` | No | `info` | Log verbosity (trace/debug/info/warn/error) |

### User-Agent Format

The yr.no API requires a unique User-Agent to identify your application. The format should be:

```
<application-name>/<version> <contact-information>
```

Good examples:
- `my-weather-app/1.0 github.com/myusername/my-weather-app`
- `home-automation/2.5 https://my-website.com`
- `personal-weather-station/1.0 contact@example.com`
- `acme-corp-weather/3.0 weather-admin@acme.com`

The User-Agent helps yr.no:
- Contact you if there are issues with your usage
- Provide better service by understanding usage patterns
- Ensure fair use of their free API

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
