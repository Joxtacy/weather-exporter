use anyhow::Result;
use axum::{Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use chrono::{DateTime, Utc};
use clap::Parser;
use lazy_static::lazy_static;
use prometheus::{Encoder, GaugeVec, IntGaugeVec, Opts, Registry, TextEncoder};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::Duration};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

// Prometheus metrics
lazy_static! {
    static ref TEMPERATURE: GaugeVec = GaugeVec::new(
        Opts::new("weather_temperature_celsius", "Temperature in Celsius"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref HUMIDITY: GaugeVec = GaugeVec::new(
        Opts::new("weather_humidity_percent", "Relative humidity percentage"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref WIND_SPEED: GaugeVec = GaugeVec::new(
        Opts::new("weather_wind_speed_mps", "Wind speed in meters per second"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref WIND_DIRECTION: GaugeVec = GaugeVec::new(
        Opts::new(
            "weather_wind_direction_degrees",
            "Wind direction in degrees"
        ),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref PRESSURE: GaugeVec = GaugeVec::new(
        Opts::new("weather_pressure_hpa", "Air pressure in hectopascals"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref PRECIPITATION: GaugeVec = GaugeVec::new(
        Opts::new("weather_precipitation_mm", "Precipitation in millimeters"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref CLOUD_COVERAGE: GaugeVec = GaugeVec::new(
        Opts::new(
            "weather_cloud_coverage_percent",
            "Cloud coverage percentage"
        ),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref UV_INDEX: GaugeVec = GaugeVec::new(
        Opts::new("weather_uv_index", "UV index"),
        &["location", "latitude", "longitude"]
    )
    .expect("metric can be created");
    static ref WEATHER_FETCH_SUCCESS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            "weather_fetch_success",
            "Whether the last weather fetch was successful"
        ),
        &["location"]
    )
    .expect("metric can be created");
    static ref WEATHER_CACHE_HITS: IntGaugeVec = IntGaugeVec::new(
        Opts::new(
            "weather_cache_hits_total",
            "Number of times cached data was used"
        ),
        &["location"]
    )
    .expect("metric can be created");
    static ref WEATHER_API_CALLS: IntGaugeVec = IntGaugeVec::new(
        Opts::new("weather_api_calls_total", "Total number of API calls made"),
        &["location"]
    )
    .expect("metric can be created");
    static ref REGISTRY: Registry = Registry::new();
}

/// Weather exporter for Prometheus
///
/// Fetches weather data from yr.no API and exposes it as Prometheus metrics.
/// Supports monitoring multiple locations with independent caching.
#[derive(Parser, Debug)]
#[command(name = "weather-exporter")]
#[command(author = "Joxtacy <https://github.com/Joxtacy>")]
#[command(version)]
#[command(about = "Export weather data from yr.no as Prometheus metrics", long_about = None)]
#[command(after_help = "EXAMPLES:
    # Monitor a single location:
    weather-exporter --user-agent 'my-app/1.0 github.com/user/repo' --locations Oslo

    # Monitor multiple locations:
    weather-exporter -u 'my-app/1.0 contact@example.com' -l Oslo,Stockholm,Copenhagen

    # Use environment variables:
    export WEATHER_USER_AGENT='my-app/1.0 github.com/user/repo'
    weather-exporter -l London,Paris

    # Custom port:
    weather-exporter -u 'my-app/1.0' -l Oslo -p 8080

USER-AGENT FORMAT:
    The User-Agent must uniquely identify your application (required by yr.no).
    Format: <application>/<version> <contact>

    Examples:
    - 'my-weather-app/1.0 github.com/username/repo'
    - 'home-automation/2.5 https://my-website.com'
    - 'acme-corp/3.0 ops@acme.com'")]
struct Args {
    /// User-Agent for yr.no API (required)
    #[arg(
        short = 'u',
        long,
        env = "WEATHER_USER_AGENT",
        value_name = "USER_AGENT",
        help = "Unique identifier for your application"
    )]
    user_agent: String,

    /// Comma-separated list of locations to monitor
    #[arg(
        short = 'l',
        long,
        env = "WEATHER_LOCATIONS",
        default_value = "Oslo",
        value_delimiter = ',',
        value_name = "LOCATIONS",
        help = "Locations to monitor (e.g., 'Oslo,Stockholm,Copenhagen')"
    )]
    locations: Vec<String>,

    /// Port for the metrics endpoint
    #[arg(
        short = 'p',
        long,
        env = "PORT",
        default_value_t = 9090,
        value_name = "PORT",
        help = "Port to listen on"
    )]
    port: u16,

    /// Log level
    #[arg(
        long,
        env = "RUST_LOG",
        default_value = "info",
        value_name = "LEVEL",
        help = "Log level (trace, debug, info, warn, error)"
    )]
    log_level: String,

    /// Validate configuration and exit
    #[arg(long, help = "Validate configuration without starting the server")]
    check: bool,
}

// YR.no API response structures
#[derive(Debug, Deserialize)]
struct LocationSearchResponse {
    #[serde(rename = "_embedded")]
    embedded: Option<EmbeddedLocations>,
}

#[derive(Debug, Deserialize)]
struct EmbeddedLocations {
    location: Option<Vec<Location>>,
}

#[derive(Debug, Deserialize, Clone)]
struct Location {
    name: String,
    position: Position,
    category: Option<LocationCategory>,
}

#[derive(Debug, Deserialize, Clone)]
struct Position {
    lat: f64,
    lon: f64,
}

impl Position {
    // Round coordinates to 4 decimals as required by the API
    fn rounded(&self) -> (f64, f64) {
        (
            (self.lat * 10000.0).round() / 10000.0,
            (self.lon * 10000.0).round() / 10000.0,
        )
    }
}

#[derive(Debug, Deserialize, Clone)]
struct LocationCategory {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WeatherResponse {
    properties: WeatherProperties,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WeatherProperties {
    timeseries: Vec<TimeSeries>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TimeSeries {
    time: DateTime<Utc>,
    data: TimeSeriesData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct TimeSeriesData {
    instant: InstantData,
    next_1_hours: Option<NextHours>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InstantData {
    details: WeatherDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct WeatherDetails {
    air_pressure_at_sea_level: Option<f64>,
    air_temperature: Option<f64>,
    cloud_area_fraction: Option<f64>,
    relative_humidity: Option<f64>,
    wind_from_direction: Option<f64>,
    wind_speed: Option<f64>,
    ultraviolet_index_clear_sky: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NextHours {
    details: NextHoursDetails,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct NextHoursDetails {
    precipitation_amount: Option<f64>,
}

// Cache for weather data
#[derive(Clone)]
struct WeatherCache {
    data: Option<WeatherResponse>,
    expires: Option<DateTime<Utc>>,
    last_modified: Option<String>,
}

impl WeatherCache {
    fn new() -> Self {
        Self {
            data: None,
            expires: None,
            last_modified: None,
        }
    }

    fn is_expired(&self) -> bool {
        match self.expires {
            Some(expires) => Utc::now() > expires,
            None => true,
        }
    }
}

// Data for a single location
#[derive(Clone)]
struct LocationData {
    location: Option<Location>,
    cache: WeatherCache,
}

impl LocationData {
    fn new() -> Self {
        Self {
            location: None,
            cache: WeatherCache::new(),
        }
    }
}

#[derive(Clone)]
struct AppState {
    location_names: Vec<String>,
    locations: Arc<RwLock<HashMap<String, LocationData>>>,
    client: reqwest::Client,
}

impl AppState {
    fn new(location_names: Vec<String>, user_agent: String) -> Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(user_agent)
            .timeout(Duration::from_secs(30))
            .build()?;

        // Initialize HashMap with empty LocationData for each location
        let mut locations = HashMap::new();
        for name in &location_names {
            locations.insert(name.clone(), LocationData::new());
        }

        Ok(Self {
            location_names,
            locations: Arc::new(RwLock::new(locations)),
            client,
        })
    }

    async fn search_location(&self, location_name: &str) -> Result<Location> {
        let url = format!(
            "https://www.yr.no/api/v0/locations/search?q={}",
            urlencoding::encode(location_name)
        );

        info!("Searching for location: {}", location_name);

        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<LocationSearchResponse>()
            .await?;

        let location = response
            .embedded
            .and_then(|e| e.location)
            .and_then(|locs| locs.into_iter().next())
            .ok_or_else(|| anyhow::anyhow!("Location not found: {}", location_name))?;

        info!(
            "Found location: {} at ({}, {})",
            location.name, location.position.lat, location.position.lon
        );

        Ok(location)
    }

    async fn fetch_weather(
        &self,
        location_name: &str,
        location: &Location,
        cache: &WeatherCache,
    ) -> Result<WeatherCache> {
        // Check if cache is still valid
        if !cache.is_expired() && cache.data.is_some() {
            info!(
                "Using cached weather data for {} (expires: {:?})",
                location_name, cache.expires
            );
            WEATHER_CACHE_HITS.with_label_values(&[location_name]).inc();
            return Ok(cache.clone());
        }

        // Round coordinates to 4 decimals as required by the API
        let (lat, lon) = location.position.rounded();
        let url = format!(
            "https://api.met.no/weatherapi/locationforecast/2.0/compact?lat={}&lon={}",
            lat, lon
        );

        info!(
            "Fetching weather for {} (rounded coords: {}, {})",
            location_name, lat, lon
        );

        // Build request with If-Modified-Since header if we have cached data
        let mut request = self.client.get(&url);
        if let Some(ref last_mod) = cache.last_modified {
            debug!(
                "Adding If-Modified-Since header for {}: {}",
                location_name, last_mod
            );
            request = request.header("If-Modified-Since", last_mod);
        }

        let response = request.send().await?;

        WEATHER_API_CALLS.with_label_values(&[location_name]).inc();

        // Handle different status codes
        match response.status() {
            StatusCode::OK => {
                info!("Received new weather data for {}", location_name);

                // Extract headers
                let expires = response
                    .headers()
                    .get("expires")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| DateTime::parse_from_rfc2822(s).ok())
                    .map(|dt| dt.with_timezone(&Utc));

                let last_modified = response
                    .headers()
                    .get("last-modified")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.to_string());

                // Check for deprecation warning
                if response.status() == StatusCode::NON_AUTHORITATIVE_INFORMATION {
                    warn!("API endpoint is deprecated, please check for updates");
                }

                let weather_data = response.json::<WeatherResponse>().await?;

                // Return new cache
                let new_cache = WeatherCache {
                    data: Some(weather_data),
                    expires,
                    last_modified,
                };

                info!(
                    "Weather data for {} cached until: {:?}",
                    location_name, expires
                );
                Ok(new_cache)
            }
            StatusCode::NOT_MODIFIED => {
                info!(
                    "Weather data not modified for {}, using cached version",
                    location_name
                );
                WEATHER_CACHE_HITS.with_label_values(&[location_name]).inc();
                Ok(cache.clone())
            }
            StatusCode::TOO_MANY_REQUESTS => {
                error!(
                    "Rate limited by API for {} - too many requests",
                    location_name
                );
                Err(anyhow::anyhow!(
                    "Rate limited - please reduce request frequency"
                ))
            }
            StatusCode::FORBIDDEN => {
                error!("Forbidden for {} - check User-Agent header", location_name);
                Err(anyhow::anyhow!(
                    "API returned 403 Forbidden - check configuration"
                ))
            }
            _ => {
                error!(
                    "Unexpected status code for {}: {}",
                    location_name,
                    response.status()
                );
                Err(anyhow::anyhow!(
                    "Unexpected API response: {}",
                    response.status()
                ))
            }
        }
    }

    async fn update_metrics_for_location(&self, location_name: &str) -> Result<()> {
        // Get or initialize location data
        let mut locations = self.locations.write().await;
        let location_data = locations
            .get_mut(location_name)
            .ok_or_else(|| anyhow::anyhow!("Location {} not found in state", location_name))?;

        // Get or search for location coordinates
        if location_data.location.is_none() {
            match self.search_location(location_name).await {
                Ok(loc) => {
                    location_data.location = Some(loc);
                }
                Err(e) => {
                    error!("Failed to search for location {}: {}", location_name, e);
                    WEATHER_FETCH_SUCCESS
                        .with_label_values(&[location_name])
                        .set(0);
                    return Err(e);
                }
            }
        }

        let location = location_data.location.as_ref().unwrap().clone();
        let current_cache = location_data.cache.clone();

        // Release write lock before making HTTP request
        drop(locations);

        // Fetch weather data (will use cache if not expired)
        match self
            .fetch_weather(location_name, &location, &current_cache)
            .await
        {
            Ok(new_cache) => {
                // Update cache if we got new data
                let mut locations = self.locations.write().await;
                if let Some(location_data) = locations.get_mut(location_name) {
                    location_data.cache = new_cache.clone();
                }
                drop(locations);

                WEATHER_FETCH_SUCCESS
                    .with_label_values(&[location_name])
                    .set(1);

                // Update metrics from cache
                self.update_prometheus_metrics(location_name, &location, &new_cache)?;
            }
            Err(e) => {
                WEATHER_FETCH_SUCCESS
                    .with_label_values(&[location_name])
                    .set(0);
                return Err(e);
            }
        }

        Ok(())
    }

    fn update_prometheus_metrics(
        &self,
        location_name: &str,
        location: &Location,
        cache: &WeatherCache,
    ) -> Result<()> {
        let weather = cache
            .data
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("No weather data in cache for {}", location_name))?;

        // Find the timeseries entry closest to current time
        let now = Utc::now();
        let current = weather.properties.timeseries.iter().min_by_key(|ts| {
            let diff = if ts.time > now {
                ts.time - now
            } else {
                now - ts.time
            };
            diff.num_seconds().abs()
        });

        if let Some(current) = current {
            info!(
                "Using weather data for {} from {} (current time: {})",
                location_name, current.time, now
            );

            let labels = [
                location_name,
                &location.position.lat.to_string(),
                &location.position.lon.to_string(),
            ];

            let details = &current.data.instant.details;

            if let Some(temp) = details.air_temperature {
                TEMPERATURE.with_label_values(&labels).set(temp);
            }

            if let Some(humidity) = details.relative_humidity {
                HUMIDITY.with_label_values(&labels).set(humidity);
            }

            if let Some(wind_speed) = details.wind_speed {
                WIND_SPEED.with_label_values(&labels).set(wind_speed);
            }

            if let Some(wind_dir) = details.wind_from_direction {
                WIND_DIRECTION.with_label_values(&labels).set(wind_dir);
            }

            if let Some(pressure) = details.air_pressure_at_sea_level {
                PRESSURE.with_label_values(&labels).set(pressure);
            }

            if let Some(cloud) = details.cloud_area_fraction {
                CLOUD_COVERAGE.with_label_values(&labels).set(cloud);
            }

            if let Some(uv) = details.ultraviolet_index_clear_sky {
                UV_INDEX.with_label_values(&labels).set(uv);
            }

            // Precipitation from next hour forecast
            if let Some(next_hour) = &current.data.next_1_hours
                && let Some(precip) = next_hour.details.precipitation_amount
            {
                PRECIPITATION.with_label_values(&labels).set(precip);
            }

            info!("Metrics updated successfully for {}", location_name);
        } else {
            warn!("No timeseries data available for {}", location_name);
        }

        Ok(())
    }

    async fn update_all_metrics(&self) {
        // Update metrics for all locations
        for location_name in &self.location_names {
            if let Err(e) = self.update_metrics_for_location(location_name).await {
                error!("Failed to update metrics for {}: {}", location_name, e);
            }
            // Small delay between locations to avoid hitting rate limits
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

async fn metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    // Update metrics before serving them
    state.update_all_metrics().await;

    let encoder = TextEncoder::new();
    let metric_families = REGISTRY.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

async fn health_handler() -> impl IntoResponse {
    (StatusCode::OK, "OK")
}

async fn periodic_update(state: AppState) {
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Check every minute

    loop {
        interval.tick().await;

        // Check each location and update if cache expired
        for location_name in &state.location_names {
            let should_update = {
                let locations = state.locations.read().await;
                if let Some(location_data) = locations.get(location_name) {
                    location_data.cache.is_expired()
                } else {
                    true // If not initialized, we should update
                }
            };

            if should_update {
                info!(
                    "Cache expired for {}, fetching new weather data",
                    location_name
                );
                if let Err(e) = state.update_metrics_for_location(location_name).await {
                    error!(
                        "Failed to update metrics for {} in background: {}",
                        location_name, e
                    );
                }
            } else {
                debug!("Cache still valid for {}, skipping update", location_name);
            }

            // Small delay between locations
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}

fn validate_user_agent(user_agent: &str) -> Result<()> {
    let ua = user_agent.trim();

    if ua.is_empty() {
        return Err(anyhow::anyhow!(
            "User-Agent cannot be empty.\n\
            Example: 'my-app/1.0 github.com/user/repo'"
        ));
    }

    if ua.len() < 10 {
        return Err(anyhow::anyhow!(
            "User-Agent too short. Please provide a descriptive identifier.\n\
            Example: 'my-app/1.0 github.com/username/repo'"
        ));
    }

    // Check for version number or contact info
    if !ua.contains('/') && !ua.contains('@') && !ua.contains('.') {
        return Err(anyhow::anyhow!(
            "User-Agent should include version and/or contact information.\n\
            Examples:\n  \
            - 'my-app/1.0 github.com/user/repo'\n  \
            - 'weather-monitor/2.0 contact@example.com'\n  \
            - 'home-automation https://my-site.com'"
        ));
    }

    // Warn about generic user agents
    let lower = ua.to_lowercase();
    if lower.contains("test") || lower.contains("example") || lower.contains("change-me") {
        warn!(
            "User-Agent appears to be a placeholder. Please use a unique identifier for production."
        );
    }

    Ok(())
}

fn clean_locations(locations: &[String]) -> Vec<String> {
    locations
        .iter()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize tracing with the specified log level
    tracing_subscriber::fmt::init();

    // Validate user agent
    validate_user_agent(&args.user_agent)?;

    // Clean and validate locations
    let location_names = clean_locations(&args.locations);
    if location_names.is_empty() {
        return Err(anyhow::anyhow!("No valid locations provided"));
    }

    // If --check flag is set, just validate and exit
    if args.check {
        println!("✓ Configuration is valid");
        println!("  User-Agent: {}", args.user_agent);
        println!("  Locations: {}", location_names.join(", "));
        println!("  Port: {}", args.port);
        println!("  Log level: {}", args.log_level);
        return Ok(());
    }

    info!("Starting Weather Exporter v{}", env!("CARGO_PKG_VERSION"));
    info!("User-Agent: {}", args.user_agent);
    info!("Monitoring locations: {}", location_names.join(", "));
    info!("Metrics endpoint: http://0.0.0.0:{}/metrics", args.port);

    // Register metrics
    REGISTRY
        .register(Box::new(TEMPERATURE.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(HUMIDITY.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(WIND_SPEED.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(WIND_DIRECTION.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(PRESSURE.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(PRECIPITATION.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(CLOUD_COVERAGE.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(UV_INDEX.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(WEATHER_FETCH_SUCCESS.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(WEATHER_CACHE_HITS.clone()))
        .expect("collector can be registered");
    REGISTRY
        .register(Box::new(WEATHER_API_CALLS.clone()))
        .expect("collector can be registered");

    let state = AppState::new(location_names, args.user_agent)?;

    // Initial fetch to validate locations
    state.update_all_metrics().await;

    // Start background update task
    let update_state = state.clone();
    tokio::spawn(periodic_update(update_state));

    // Build the router
    let app = Router::new()
        .route("/metrics", get(metrics_handler))
        .route("/health", get(health_handler))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], args.port));

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    info!("Weather exporter listening on {}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
