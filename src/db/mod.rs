use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub mod users;
pub mod sessions;
pub mod votes;
pub mod feedback;
pub mod speakers;
pub mod rooms;
pub mod categories;
pub mod podcast;
pub mod sync;

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(60 * 30)) // 30 minutes
        .connect(database_url)
        .await
}

pub async fn create_pool_with_retry(
    url: &str, 
    max_connections: u32,
    max_retries: u32
) -> Result<PgPool, sqlx::Error> {
    let mut attempts = 0;
    let mut last_error = None;
    
    while attempts < max_retries {
        match create_pool(url, max_connections).await {
            Ok(pool) => {
                log::info!("Database connection established successfully");
                return Ok(pool);
            },
            Err(err) => {
                last_error = Some(err);
                attempts += 1;
                log::warn!("Database connection attempt {} failed, retrying in 5 seconds...", attempts);
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| {
        sqlx::Error::Configuration("Failed to connect to database after retries".into())
    }))
}