# Weather Exporter for Prometheus

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

## Author

[Joxtacy](https://github.com/Joxtacy)

## License

MIT
