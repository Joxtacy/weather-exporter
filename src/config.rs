use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub user_agent: String,
    pub locations: Vec<String>,
    pub port: u16,
    pub log_level: String,
    pub cache_settings: CacheSettings,
}

#[derive(Debug, Deserialize)]
pub struct CacheSettings {
    pub enable_cache: bool,
    pub cache_duration_minutes: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default values
            .set_default("port", 9090)?
            .set_default("log_level", "info")?
            .set_default("cache_settings.enable_cache", true)?
            .set_default("cache_settings.cache_duration_minutes", 5)?
            
            // Look for config file in multiple locations
            .add_source(File::with_name("/etc/weather-exporter/config").required(false))
            .add_source(File::with_name("config/default").required(false))
            .add_source(File::with_name("config/local").required(false))
            
            // Override with environment variables (prefixed with WEATHER_)
            .add_source(config::Environment::with_prefix("WEATHER"))
            
            .build()?;

        let settings: Settings = config.try_deserialize()?;
        
        // Validate required fields
        if settings.user_agent.trim().is_empty() {
            return Err(ConfigError::Message(
                "user_agent is required and cannot be empty".to_string()
            ));
        }
        
        if settings.locations.is_empty() {
            return Err(ConfigError::Message(
                "At least one location must be specified".to_string()
            ));
        }
        
        Ok(settings)
    }
}

// Usage in main:
#[tokio::main]
async fn main() -> Result<()> {
    let settings = Settings::new()?;
    
    // Use settings.user_agent, settings.locations, etc.
}
