# Weather Exporter for Prometheus

A Rust-based weather metrics exporter that fetches weather data from yr.no API and exposes it in Prometheus format.

## Features

- Fetches real-time weather data from yr.no (Norwegian Meteorological Institute)
- Automatic location search by place name
- Exposes metrics in Prometheus format
- Periodic updates every 5 minutes
- Comprehensive weather metrics including:
  - Temperature (Celsius)
  - Humidity (%)
  - Wind speed (m/s) and direction (degrees)
  - Air pressure (hPa)
  - Precipitation (mm)
  - Cloud coverage (%)
  - UV index

## Building

```bash
cargo build --release
```

## Running

### Using command line argument:
```bash
cargo run -- "Stockholm"
# or
./target/release/weather-exporter "New York"
```

### Using environment variable:
```bash
export WEATHER_LOCATION="London"
cargo run
```

### Custom port:
```bash
export PORT=8080
cargo run -- "Paris"
```

## Docker

Build the Docker image:
```bash
docker build -t weather-exporter .
```

Run with Docker:
```bash
docker run -d \
  -p 9090:9090 \
  -e WEATHER_LOCATION="Berlin" \
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
# Current temperature
weather_temperature_celsius{location="Oslo"}

# Average wind speed over the last hour
avg_over_time(weather_wind_speed_mps{location="Oslo"}[1h])

# Alert when temperature drops below freezing
weather_temperature_celsius{location="Oslo"} < 0
```

## Grafana Dashboard

You can visualize these metrics in Grafana. Example panel queries:

- Temperature gauge: `weather_temperature_celsius{location="$location"}`
- Wind speed time series: `weather_wind_speed_mps{location="$location"}`
- Humidity percentage: `weather_humidity_percent{location="$location"}`

## API Attribution

This application uses the yr.no API provided by the Norwegian Meteorological Institute. Please respect their [terms of service](https://developer.yr.no/doc/TermsOfService/).

## Author

[Joxtacy](https://github.com/Joxtacy)

## License

MIT
