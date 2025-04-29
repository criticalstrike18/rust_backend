// src/db/speakers.rs
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ServiceError;
use crate::models::session::{ConferenceSpeakerRequest, SpeakerInfo};

pub async fn get_speaker_by_id(
    pool: &PgPool,
    speaker_id: &str,
) -> Result<Option<SpeakerInfo>, ServiceError> {
    let speaker = sqlx::query!(
        r#"
        SELECT id, first_name, last_name, bio, tag_line, profile_picture, is_top_speaker
        FROM conference_speakers
        WHERE id = $1
        "#,
        speaker_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(speaker.map(|row| SpeakerInfo {
        id: row.id,
        first_name: row.first_name,
        last_name: row.last_name,
        bio: row.bio,
        tag_line: row.tag_line,
        profile_picture: row.profile_picture,
        is_top_speaker: row.is_top_speaker,
    }))
}

pub async fn get_all_speakers(pool: &PgPool) -> Result<Vec<SpeakerInfo>, ServiceError> {
    let speakers = sqlx::query!(
        r#"
        SELECT id, first_name, last_name, bio, tag_line, profile_picture, is_top_speaker
        FROM conference_speakers
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(speakers
        .into_iter()
        .map(|row| SpeakerInfo {
            id: row.id,
            first_name: row.first_name,
            last_name: row.last_name,
            bio: row.bio,
            tag_line: row.tag_line,
            profile_picture: row.profile_picture,
            is_top_speaker: row.is_top_speaker,
        })
        .collect())
}

pub async fn add_speaker(
    pool: &PgPool,
    speaker: &ConferenceSpeakerRequest,
) -> Result<String, ServiceError> {
    let generated_id = Uuid::new_v4().to_string();

    sqlx::query!(
        r#"
        INSERT INTO conference_speakers 
        (id, first_name, last_name, bio, tag_line, profile_picture, is_top_speaker)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        generated_id,
        speaker.first_name,
        speaker.last_name,
        speaker.bio,
        speaker.tag_line,
        speaker.profile_picture,
        speaker.is_top_speaker
    )
    .execute(pool)
    .await?;

    Ok(generated_id)
}