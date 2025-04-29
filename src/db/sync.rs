use chrono::DateTime;
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::category::CategoriesTable;
use crate::models::podcast::{EpisodeData,ChannelFullData};
use crate::models::room::RoomTable;
use crate::models::session::{SessionInfo, SpeakerInfo};

pub async fn get_sessions_changed_since(
    pool: &PgPool,
    timestamp: i64,
) -> Result<Vec<SessionInfo>, ServiceError> {
    let since_time = DateTime::from_timestamp(timestamp / 1000, 0)
        .ok_or_else(|| ServiceError::BadRequest("Invalid timestamp".to_string()))?;

    let sessions = sqlx::query!(
        r#"
        SELECT 
            id, title, description, starts_at, ends_at, 
            room_id, is_service_session, is_plenum_session, status
        FROM conference_sessions
        WHERE updated_at >= $1
        "#,
        since_time
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

pub async fn get_speakers_changed_since(
    pool: &PgPool,
    timestamp: i64,
) -> Result<Vec<SpeakerInfo>, ServiceError> {
    let since_time = DateTime::from_timestamp(timestamp / 1000, 0)
        .ok_or_else(|| ServiceError::BadRequest("Invalid timestamp".to_string()))?;

    let speakers = sqlx::query!(
        r#"
        SELECT 
            id, first_name, last_name, bio, tag_line, profile_picture, is_top_speaker
        FROM conference_speakers
        WHERE updated_at >= $1
        "#,
        since_time
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

pub async fn get_rooms_changed_since(
    pool: &PgPool,
    timestamp: i64,
) -> Result<Vec<RoomTable>, ServiceError> {
    let since_time = DateTime::from_timestamp(timestamp / 1000, 0)
        .ok_or_else(|| ServiceError::BadRequest("Invalid timestamp".to_string()))?;

    let rooms = sqlx::query!(
        r#"
        SELECT id, name, sort
        FROM conference_rooms
        WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?;

    Ok(rooms
        .into_iter()
        .map(|row| RoomTable {
            id: Some(row.id as i64),
            name: row.name,
            sort: row.sort,
        })
        .collect())
}

pub async fn get_categories_changed_since(
    pool: &PgPool,
    timestamp: i64,
) -> Result<Vec<CategoriesTable>, ServiceError> {
    let since_time = DateTime::from_timestamp(timestamp / 1000, 0)
        .ok_or_else(|| ServiceError::BadRequest("Invalid timestamp".to_string()))?;

    let categories = sqlx::query!(
        r#"
        SELECT id, title, sort, type as "type_name"
        FROM conference_categories
        WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?;

    Ok(categories
        .into_iter()
        .map(|row| CategoriesTable {
            id: row.id as i64,
            title: row.title,
            sort: row.sort,
            type_name: row.type_name,
        })
        .collect())
}

pub async fn get_podcasts_changed_since(
    pool: &PgPool,
    timestamp: i64,
) -> Result<Vec<ChannelFullData>, ServiceError> {
    let since_time = DateTime::from_timestamp(timestamp / 1000, 0)
        .ok_or_else(|| ServiceError::BadRequest("Invalid timestamp".to_string()))?;

    // Find channels updated since timestamp
    let updated_channels = sqlx::query!(
        r#"
        SELECT id FROM podcast_channels WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| r.id)
    .collect::<Vec<i32>>();

    // Find channels with episodes updated since timestamp
    let channels_with_updated_episodes = sqlx::query!(
        r#"
        SELECT DISTINCT channel_id 
        FROM podcast_episodes 
        WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| r.channel_id)
    .collect::<Vec<i32>>();

    // Find channels with category mapping updates
    let channels_with_category_updates = sqlx::query!(
        r#"
        SELECT DISTINCT channel_id
        FROM channel_category_map
        WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| r.channel_id)
    .collect::<Vec<i32>>();

    // Find episodes with category updates
    let episodes_with_category_updates = sqlx::query!(
        r#"
        SELECT DISTINCT episode_id
        FROM episode_category_map
        WHERE updated_at >= $1
        "#,
        since_time
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|r| r.episode_id)
    .collect::<Vec<i32>>();

    // Get channels for those episodes with category updates
    let mut channels_with_episode_category_updates: Vec<i32> = Vec::new();
    if !episodes_with_category_updates.is_empty() {
        channels_with_episode_category_updates = sqlx::query!(
            r#"
            SELECT DISTINCT channel_id
            FROM podcast_episodes
            WHERE id = ANY($1)
            "#,
            &episodes_with_category_updates
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|r| r.channel_id)
        .collect();
    }

    // Combine all affected channel IDs
    let mut all_affected_channels = Vec::new();
    all_affected_channels.extend(updated_channels);
    all_affected_channels.extend(channels_with_updated_episodes);
    all_affected_channels.extend(channels_with_category_updates);
    all_affected_channels.extend(channels_with_episode_category_updates);
    
    // Remove duplicates
    all_affected_channels.sort();
    all_affected_channels.dedup();
    
    if all_affected_channels.is_empty() {
        return Ok(Vec::new());
    }

    // 1. Get all channel categories for lookup
    let all_channel_categories = sqlx::query!(
        r#"
        SELECT id, name
        FROM podcast_channel_categories
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| (row.id, row.name))
    .collect::<std::collections::HashMap<i32, String>>();

    // 2. Get all episode categories for lookup
    let all_episode_categories = sqlx::query!(
        r#"
        SELECT id, name
        FROM podcast_episode_categories
        "#
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|row| (row.id, row.name))
    .collect::<std::collections::HashMap<i32, String>>();

    // 3. Get channel-category mappings for affected channels
    let channel_categories = if !all_affected_channels.is_empty() {
        sqlx::query!(
            r#"
            SELECT channel_id, category_id
            FROM channel_category_map
            WHERE channel_id = ANY($1)
            "#,
            &all_affected_channels
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, row| {
            acc.entry(row.channel_id)
                .or_insert_with(Vec::new)
                .push(all_channel_categories.get(&row.category_id).cloned().unwrap_or_default());
            acc
        })
    } else {
        std::collections::HashMap::new()
    };

    // 4. Get all episodes for affected channels
    let all_episodes_rows = if !all_affected_channels.is_empty() {
        sqlx::query!(
            r#"
            SELECT 
                id, channel_id, guid, title, description, link, pub_date,
                duration, explicit, image_url, media_url, media_type, media_length
            FROM podcast_episodes
            WHERE channel_id = ANY($1)
            ORDER BY pub_date DESC
            "#,
            &all_affected_channels
        )
        .fetch_all(pool)
        .await?
    } else {
        Vec::new()
    };

    // Get all episode IDs for episode-category mapping
    let all_episode_ids: Vec<i32> = all_episodes_rows.iter().map(|row| row.id).collect();

    // 5. Get episode-category mappings
    let episode_categories = if !all_episode_ids.is_empty() {
        sqlx::query!(
            r#"
            SELECT episode_id, category_id
            FROM episode_category_map
            WHERE episode_id = ANY($1)
            "#,
            &all_episode_ids
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .fold(std::collections::HashMap::new(), |mut acc, row| {
            acc.entry(row.episode_id)
                .or_insert_with(Vec::new)
                .push(all_episode_categories.get(&row.category_id).cloned().unwrap_or_default());
            acc
        })
    } else {
        std::collections::HashMap::new()
    };

    // 6. Group episodes by channel_id
    let episodes_by_channel = all_episodes_rows.into_iter().fold(
        std::collections::HashMap::<i32, Vec<_>>::new(),
        |mut acc, row| {
            acc.entry(row.channel_id).or_default().push(row);
            acc
        },
    );

    // 7. Get the channel details and build the final result
    let channels = sqlx::query!(
        r#"
        SELECT 
            id, title, link, description, copyright, language,
            author, owner_email, owner_name, image_url, last_build_date
        FROM podcast_channels
        WHERE id = ANY($1)
        ORDER BY id ASC
        "#,
        &all_affected_channels
    )
    .fetch_all(pool)
    .await?;

    // 8. Build the final ChannelFullData objects
    let result = channels
        .into_iter()
        .map(|channel_row| {
            let channel_id = channel_row.id;
            let episodes = episodes_by_channel
                .get(&channel_id)
                .map(|eps| {
                    eps.iter()
                        .map(|ep| {
                            let episode_id = ep.id;
                            EpisodeData {
                                id: Some(episode_id),
                                guid: ep.guid.clone(),
                                title: ep.title.clone(),
                                description: ep.description.clone(),
                                link: ep.link.clone(),
                                pub_date: ep.pub_date,
                                duration: Some(ep.duration),
                                explicit: ep.explicit,
                                image_url: ep.image_url.clone(),
                                media_url: Some(ep.media_url.clone()),
                                media_type: Some(ep.media_type.clone()),
                                media_length: Some(ep.media_length),
                                episode_category: episode_categories
                                    .get(&episode_id)
                                    .cloned()
                                    .unwrap_or_default(),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();

            ChannelFullData {
                id: channel_id,
                title: channel_row.title,
                link: channel_row.link,
                description: channel_row.description,
                copyright: channel_row.copyright,
                language: Some(channel_row.language),
                author: Some(channel_row.author),
                owner_email: Some(channel_row.owner_email),
                owner_name: Some(channel_row.owner_name),
                image_url: Some(channel_row.image_url),
                last_build_date: Some(channel_row.last_build_date.to_rfc3339()),
                categories: channel_categories.get(&channel_id).cloned().unwrap_or_default(),
                episodes,
            }
        })
        .collect();

    Ok(result)
}