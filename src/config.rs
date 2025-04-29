// src/config.rs
use serde::Deserialize;
use config::{Config, ConfigError, Environment, File};

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
pub struct SessionizeConfig {
    pub url: String,
    pub images_url: String,
    pub interval: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceConfig {
    pub environment: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub sessionize: SessionizeConfig,
    pub service: ServiceConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Start with default values matching application.yaml
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("database.url", "postgres://postgres:postgres@db:5432/kotlinconfg")?
            .set_default("database.max_connections", 5)?
            .set_default("sessionize.interval", 60)?
            .set_default("service.environment", "production")?
            .set_default("service.secret", "admin")?
            // Try to load from a file if it exists
            .add_source(File::with_name("config").required(false))
            // Add environment variables (with prefix)
            .add_source(Environment::with_prefix("APP").separator("__"))
            .build()?;

        // Deserialize
        config.try_deserialize()
    }
}