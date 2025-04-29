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
pub struct ServiceConfig {
    pub environment: String,
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub service: ServiceConfig,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // Load .env file first
        dotenv::dotenv().ok();
        
        // Determine database URL based on environment
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                if std::env::var("DOCKER_ENV").is_ok() {
                    "postgres://postgres:postgres@db:5432/kotlinconfg".to_string()
                } else {
                    "postgres://postgres:postgres@localhost:5432/kotlinconfg".to_string()
                }
            });

        let config = Config::builder()
            // Default values
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default("database.url", database_url)?
            .set_default("database.max_connections", 5)?
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