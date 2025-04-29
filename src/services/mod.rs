// src/services/mod.rs
pub mod sync;
pub mod admin;

use sqlx::PgPool;
use std::time::Duration;

pub fn start_sync_service(pool: PgPool, interval_minutes: u64, url: String) {
    tokio::spawn(async move {
        let interval_duration = Duration::from_secs(interval_minutes * 60);
        
        loop {
            match sync::synchronize_with_sessionize(&pool, &url).await {
                Ok(_) => log::info!("Successfully synchronized with Sessionize"),
                Err(e) => log::error!("Failed to synchronize with Sessionize: {:?}", e),
            }
            
            tokio::time::sleep(interval_duration).await;
        }
    });
}