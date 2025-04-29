// src/db/users.rs
use sqlx::PgPool;

pub async fn validate_user(pool: &PgPool, uuid: &str) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT COUNT(*) as count FROM users WHERE uuid = $1",
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(result.count.unwrap_or(0) > 0)
}

pub async fn create_user(pool: &PgPool, uuid: &str, timestamp: &str) -> Result<bool, sqlx::Error> {
    // Check if user exists
    let exists = validate_user(pool, uuid).await?;
    if exists {
        return Ok(false);
    }

    // Insert the new user
    sqlx::query!(
        "INSERT INTO users (uuid, timestamp) VALUES ($1, $2)",
        uuid,
        timestamp
    )
    .execute(pool)
    .await?;

    Ok(true)
}