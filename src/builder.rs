use std::net::SocketAddr;

pub struct WeatherExporterBuilder {
    user_agent: Option<String>,
    locations: Vec<String>,
    port: u16,
    log_level: String,
}

impl WeatherExporterBuilder {
    pub fn new() -> Self {
        Self {
            user_agent: None,
            locations: Vec::new(),
            port: 9090,
            log_level: "info".to_string(),
        }
    }
    
    /// Required: Set the User-Agent for yr.no API
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }
    
    pub fn add_location(mut self, location: impl Into<String>) -> Self {
        self.locations.push(location.into());
        self
    }
    
    pub fn locations(mut self, locations: Vec<String>) -> Self {
        self.locations = locations;
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }
    
    pub fn log_level(mut self, level: impl Into<String>) -> Self {
        self.log_level = level.into();
        self
    }
    
    pub fn build(self) -> Result<WeatherExporter, BuilderError> {
        let user_agent = self.user_agent
            .ok_or(BuilderError::MissingUserAgent)?;
            
        if self.locations.is_empty() {
            return Err(BuilderError::NoLocations);
        }
        
        validate_user_agent(&user_agent)?;
        
        Ok(WeatherExporter {
            user_agent,
            locations: self.locations,
            port: self.port,
            log_level: self.log_level,
        })
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BuilderError {
    #[error("User-Agent is required for yr.no API compliance")]
    MissingUserAgent,
    
    #[error("At least one location must be specified")]
    NoLocations,
    
    #[error("Invalid User-Agent format: {0}")]
    InvalidUserAgent(String),
}

// Usage:
let exporter = WeatherExporterBuilder::new()
    .user_agent("my-app/1.0 github.com/user/repo")  // Required
    .add_location("Oslo")
    .add_location("Stockholm")
    .port(8080)
    .build()?;  // Fails at compile time if user_agent not called
