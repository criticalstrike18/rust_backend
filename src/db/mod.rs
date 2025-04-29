// src/db/mod.rs
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

pub async fn create_pool(database_url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(30))
        .max_lifetime(Duration::from_secs(60 * 30)) // 30 minutes
        .connect(database_url)
        .await
}