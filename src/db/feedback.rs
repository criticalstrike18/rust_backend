// src/db/feedback.rs
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::feedback::FeedbackInfo;

pub async fn set_feedback(
    pool: &PgPool,
    user_id: &str,
    session_id: &str,
    feedback_value: &str,
    timestamp: DateTime<Utc>,
) -> Result<bool, ServiceError> {
    let timestamp_str = timestamp.to_rfc3339();
    
    let result = sqlx::query!(
        r#"
        INSERT INTO feedback (uuid, sessionId, feedback, timestamp)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        session_id,
        feedback_value,
        timestamp_str
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn get_feedback_summary(pool: &PgPool) -> Result<Vec<FeedbackInfo>, ServiceError> {
    let feedback = sqlx::query!(
        r#"
        SELECT sessionId, feedback
        FROM feedback
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(feedback
        .into_iter()
        .map(|row| FeedbackInfo {
            session_id: row.sessionid,
            value: row.feedback,
        })
        .collect())
}