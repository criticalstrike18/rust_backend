// src/db/sessions.rs
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::ServiceError;
use crate::models::conference::{Conference, Session, Speaker};
use crate::models::session::{ConferenceSessionRequest, SessionInfo};

pub async fn get_conference_data(pool: &PgPool) -> Result<Conference, ServiceError> {
    // Query sessions
    let sessions = sqlx::query!(
        r#"
        SELECT 
            cs.id, cs.title, cs.description, cs.starts_at, cs.ends_at,
            cr.name as room_name
        FROM conference_sessions cs
        LEFT JOIN conference_rooms cr ON cs.room_id = cr.id
        "#
    )
    .fetch_all(pool)
    .await?;
    
    // Map sessions and fetch related data
    let mut result_sessions = Vec::new();
    for session_row in sessions {
        // Get speaker IDs for this session
        let speaker_ids = sqlx::query!(
            r#"
            SELECT speaker_id
            FROM session_speakers
            WHERE session_id = $1
            "#,
            session_row.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|row| row.speaker_id)
        .collect::<Vec<String>>();
        
        // Get tags (categories) for this session
        let tags = sqlx::query!(
            r#"
            SELECT cc.title
            FROM session_categories sc
            JOIN conference_categories cc ON sc.category_item_id = cc.id
            WHERE sc.session_id = $1
            "#,
            session_row.id
        )
        .fetch_all(pool)
        .await?;
        
        // Debug log the category data
        log::info!("Session {}: Found {} categories", session_row.id, tags.len());
        if !tags.is_empty() {
            log::info!("First category title: {:?}", tags.first().map(|t| &t.title));
        }
        
        let tag_titles = tags
            .into_iter()
            .map(|row| row.title)
            .collect::<Vec<String>>();
        
        // Create session object with optional tags that may be null instead of empty
        result_sessions.push(Session {
            id: session_row.id,
            title: session_row.title,
            description: session_row.description.unwrap_or_default(),
            speaker_ids,
            location: session_row.room_name.unwrap_or_else(|| "Unknown Room".to_string()),
            starts_at: session_row.starts_at,
            ends_at: session_row.ends_at,
            tags: if tag_titles.is_empty() { None } else { Some(tag_titles) },
        });
    }
    
    // Query speakers
    let speakers = sqlx::query!(
        r#"
        SELECT 
            id, first_name, last_name, bio, tag_line, profile_picture
        FROM conference_speakers
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| Speaker {
        id: row.id,
        name: format!("{} {}", row.first_name, row.last_name),
        position: row.tag_line.unwrap_or_default(),
        description: row.bio.unwrap_or_default(),
        photo_url: row.profile_picture.unwrap_or_default(),
    })
    .collect();
    
    Ok(Conference {
        sessions: result_sessions,
        speakers,
    })
}

pub async fn add_session(
    pool: &PgPool,
    session: &ConferenceSessionRequest,
) -> Result<String, ServiceError> {
    // Check for duplicate session
    let duplicate = sqlx::query!(
        r#"
        SELECT id FROM conference_sessions 
        WHERE title = $1 AND description = $2
        "#,
        session.title,
        session.description
    )
    .fetch_optional(pool)
    .await?;

    if duplicate.is_some() {
        return Err(ServiceError::BadRequest(
            "A session with the same title and description already exists".to_string()
        ));
    }

    // Generate a new UUID for the session
    let generated_id = Uuid::new_v4().to_string();

    // Insert the session
    sqlx::query!(
        r#"
        INSERT INTO conference_sessions 
        (id, title, description, starts_at, ends_at, room_id, is_service_session, is_plenum_session, status)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        generated_id,
        session.title,
        session.description,
        session.starts_at,
        session.ends_at,
        session.room_id,
        session.is_service_session,
        session.is_plenum_session,
        session.status
    )
    .execute(pool)
    .await?;

    // Insert speaker associations
    for speaker_id in &session.speaker_ids {
        sqlx::query!(
            r#"
            INSERT INTO session_speakers (session_id, speaker_id)
            VALUES ($1, $2)
            "#,
            generated_id,
            speaker_id
        )
        .execute(pool)
        .await?;
    }

    // Insert category associations
    for category_id in &session.category_ids {
        sqlx::query!(
            r#"
            INSERT INTO session_categories (session_id, category_item_id)
            VALUES ($1, $2)
            "#,
            generated_id,
            category_id
        )
        .execute(pool)
        .await?;
    }

    Ok(generated_id)
}

pub async fn get_session_by_id(
    pool: &PgPool,
    session_id: &str,
) -> Result<Option<SessionInfo>, ServiceError> {
    let session = sqlx::query!(
        r#"
        SELECT 
            id, title, description, starts_at, ends_at, room_id, 
            is_service_session, is_plenum_session, status
        FROM conference_sessions
        WHERE id = $1
        "#,
        session_id
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = session {
        // Get speaker IDs for this session
        let speaker_ids = sqlx::query!(
            r#"
            SELECT speaker_id
            FROM session_speakers
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.speaker_id)
        .collect();

        // Get category IDs for this session
        let category_ids = sqlx::query!(
            r#"
            SELECT category_item_id
            FROM session_categories
            WHERE session_id = $1
            "#,
            session_id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.category_item_id)
        .collect();

        Ok(Some(SessionInfo {
            id: row.id,
            title: row.title,
            description: row.description,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            room_id: row.room_id,
            is_service_session: row.is_service_session.unwrap_or(false),
            is_plenum_session: row.is_plenum_session.unwrap_or(false),
            status: row.status.unwrap_or_else(|| "draft".to_string()),
            speaker_ids,
            category_ids,
        }))
    } else {
        Ok(None)
    }
}

pub async fn get_all_sessions(pool: &PgPool) -> Result<Vec<SessionInfo>, ServiceError> {
    let sessions = sqlx::query!(
        r#"
        SELECT 
            id, title, description, starts_at, ends_at, room_id, 
            is_service_session, is_plenum_session, status
        FROM conference_sessions
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    for row in sessions {
        // Get speaker IDs
        let speaker_ids = sqlx::query!(
            r#"
            SELECT speaker_id
            FROM session_speakers
            WHERE session_id = $1
            "#,
            row.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.speaker_id)
        .collect();

        // Get category IDs
        let category_ids = sqlx::query!(
            r#"
            SELECT category_item_id
            FROM session_categories
            WHERE session_id = $1
            "#,
            row.id
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.category_item_id)
        .collect();

        result.push(SessionInfo {
            id: row.id,
            title: row.title,
            description: row.description,
            starts_at: row.starts_at,
            ends_at: row.ends_at,
            room_id: row.room_id,
            is_service_session: row.is_service_session.unwrap_or(false),
            is_plenum_session: row.is_plenum_session.unwrap_or(false),
            status: row.status.unwrap_or_else(|| "draft".to_string()),
            speaker_ids,
            category_ids,
        });
    }

    Ok(result)
}

pub async fn add_session_speaker(
    pool: &PgPool,
    session_id: &str,
    speaker_id: &str,
) -> Result<(), ServiceError> {
    // Check if relationship already exists
    let exists = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM session_speakers
        WHERE session_id = $1 AND speaker_id = $2
        "#,
        session_id,
        speaker_id
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0)
        > 0;

    if !exists {
        sqlx::query!(
            r#"
            INSERT INTO session_speakers (session_id, speaker_id)
            VALUES ($1, $2)
            "#,
            session_id,
            speaker_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn add_session_category(
    pool: &PgPool,
    session_id: &str,
    category_id: i32,
) -> Result<(), ServiceError> {
    // Check if relationship already exists
    let exists = sqlx::query!(
        r#"
        SELECT COUNT(*) as count
        FROM session_categories
        WHERE session_id = $1 AND category_item_id = $2
        "#,
        session_id,
        category_id
    )
    .fetch_one(pool)
    .await?
    .count
    .unwrap_or(0)
        > 0;

    if !exists {
        sqlx::query!(
            r#"
            INSERT INTO session_categories (session_id, category_item_id)
            VALUES ($1, $2)
            "#,
            session_id,
            category_id
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}