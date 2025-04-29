// src/db/votes.rs
use chrono::Utc;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::vote::{Score, VoteInfo};

pub async fn get_votes(pool: &PgPool, user_id: &str) -> Result<Vec<VoteInfo>, ServiceError> {
    let votes = sqlx::query!(
        r#"
        SELECT sessionId, rating
        FROM votes
        WHERE uuid = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(votes
        .into_iter()
        .filter_map(|row| {
            Score::from_value(row.rating).map(|score| VoteInfo {
                session_id: row.sessionid,
                score: Some(score),
            })
        })
        .collect())
}

pub async fn get_all_votes(pool: &PgPool) -> Result<Vec<VoteInfo>, ServiceError> {
    let votes = sqlx::query!(
        r#"
        SELECT sessionId, rating
        FROM votes
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(votes
        .into_iter()
        .filter_map(|row| {
            Score::from_value(row.rating).map(|score| VoteInfo {
                session_id: row.sessionid,
                score: Some(score),
            })
        })
        .collect())
}

pub async fn change_vote(
    pool: &PgPool,
    user_id: &str,
    session_id: &str,
    score: Option<Score>,
    timestamp: DateTime<Utc>,
) -> Result<(), ServiceError> {
    // If score is None, delete the vote
    if score.is_none() {
        return delete_vote(pool, user_id, session_id).await;
    }

    let score_value = score.unwrap().value();
    let timestamp_str = timestamp.to_rfc3339();

    // Check if vote exists
    let exists = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM votes
        WHERE uuid = $1 AND sessionId = $2
        "#,
        user_id,
        session_id
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0)
        > 0;

    if exists {
        // Update existing vote
        sqlx::query!(
            r#"
            UPDATE votes
            SET rating = $3, timestamp = $4, updated_at = NOW()
            WHERE uuid = $1 AND sessionId = $2
            "#,
            user_id,
            session_id,
            score_value,
            timestamp_str
        )
        .execute(pool)
        .await?;
    } else {
        // Insert new vote
        sqlx::query!(
            r#"
            INSERT INTO votes (uuid, sessionId, rating, timestamp)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            session_id,
            score_value,
            timestamp_str
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

async fn delete_vote(pool: &PgPool, user_id: &str, session_id: &str) -> Result<(), ServiceError> {
    sqlx::query!(
        r#"
        DELETE FROM votes
        WHERE uuid = $1 AND sessionId = $2
        "#,
        user_id,
        session_id
    )
    .execute(pool)
    .await?;

    Ok(())
}