use serde::Deserialize;
use config::{Config, ConfigError, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default values
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?
            .set_default("database.url", "postgres://postgres:postgres@localhost:5432/kotlinconfg")?
            .set_default("database.max_connections", 5)?
            // Add in settings from environment variables (with a prefix of APP)
            // E.g., `APP_SERVER__PORT=5001 would set server.port`
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        // Deserialize the configuration into our AppConfig struct
        config.try_deserialize()
    }
}