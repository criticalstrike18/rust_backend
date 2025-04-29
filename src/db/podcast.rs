// src/db/podcast.rs
use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::podcast::{ChannelFullData, EpisodeData, PodcastImportRequest, PodcastQueryInfo};

pub async fn store_podcast_query(
    pool: &PgPool,
    user_id: &str,
    title: &str,
    author: &str,
    rss_link: &str,
) -> Result<bool, ServiceError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO podcast_request_table (uuid, title, author, rssUrl)
        VALUES ($1, $2, $3, $4)
        "#,
        user_id,
        title,
        author,
        rss_link
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn store_podcast_data(
    pool: &PgPool,
    import_request: &PodcastImportRequest,
) -> Result<i32, ServiceError> {
    // Start a transaction
    let mut tx = pool.begin().await?;

    // 1. Insert the channel
    let last_build_date = match &import_request.channel.last_build_date {
        Some(date_str) => {
            DateTime::parse_from_rfc3339(date_str)
                .map_err(|_| ServiceError::BadRequest("Invalid date format".to_string()))?
                .with_timezone(&Utc)
        }
        None => Utc::now(),
    };

    let channel_row = sqlx::query!(
        r#"
        INSERT INTO podcast_channels 
        (title, link, description, copyright, language, author, owner_email, owner_name, image_url, last_build_date)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
        import_request.channel.title,
        import_request.channel.link,
        import_request.channel.description,
        import_request.channel.copyright,
        import_request.channel.language.clone().unwrap_or_else(|| "en".to_string()),
        import_request.channel.author.clone().unwrap_or_default(),
        import_request.channel.owner_email.clone().unwrap_or_default(),
        import_request.channel.owner_name.clone().unwrap_or_default(),
        import_request.channel.image_url.clone().unwrap_or_default(),
        last_build_date
    )
    .fetch_one(&mut *tx)
    .await?;

    let channel_id = channel_row.id;

    // 2. Insert channel categories
    for category_name in &import_request.categories {
        // Find or create channel category
        let category_row = sqlx::query!(
            r#"
            SELECT id FROM podcast_channel_categories
            WHERE name = $1
            "#,
            category_name
        )
        .fetch_optional(&mut *tx)
        .await?;

        let category_id = match category_row {
            Some(row) => row.id,
            None => {
                // Category doesn't exist, create it
                let new_category = sqlx::query!(
                    r#"
                    INSERT INTO podcast_channel_categories (name)
                    VALUES ($1)
                    RETURNING id
                    "#,
                    category_name
                )
                .fetch_one(&mut *tx)
                .await?;
                new_category.id
            }
        };

        // Create mapping between channel and category
        sqlx::query!(
            r#"
            INSERT INTO channel_category_map (channel_id, category_id)
            VALUES ($1, $2)
            ON CONFLICT DO NOTHING
            "#,
            channel_id,
            category_id
        )
        .execute(&mut *tx)
        .await?;
    }

    // 3. Insert episodes
    for episode in &import_request.episodes {
        let episode_row = sqlx::query!(
            r#"
            INSERT INTO podcast_episodes 
            (channel_id, guid, title, description, link, pub_date, duration, explicit, 
             image_url, media_url, media_type, media_length)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            RETURNING id
            "#,
            channel_id,
            episode.guid,
            episode.title,
            episode.description,
            episode.link,
            episode.pub_date,
            episode.duration.unwrap_or(0),
            episode.explicit,
            episode.image_url,
            episode.media_url.clone().unwrap_or_default(),
            episode.media_type.clone().unwrap_or_else(|| "audio/mpeg".to_string()),
            episode.media_length.unwrap_or(0)
        )
        .fetch_one(&mut *tx)
        .await?;

        let episode_id = episode_row.id;

        // Insert episode categories
        for category_name in &episode.episode_category {
            // Find or create episode category
            let category_row = sqlx::query!(
                r#"
                SELECT id FROM podcast_episode_categories
                WHERE name = $1
                "#,
                category_name
            )
            .fetch_optional(&mut *tx)
            .await?;

            let category_id = match category_row {
                Some(row) => row.id,
                None => {
                    // Category doesn't exist, create it
                    let new_category = sqlx::query!(
                        r#"
                        INSERT INTO podcast_episode_categories (name)
                        VALUES ($1)
                        RETURNING id
                        "#,
                        category_name
                    )
                    .fetch_one(&mut *tx)
                    .await?;
                    new_category.id
                }
            };

            // Create mapping between episode and category
            sqlx::query!(
                r#"
                INSERT INTO episode_category_map (episode_id, category_id)
                VALUES ($1, $2)
                ON CONFLICT DO NOTHING
                "#,
                episode_id,
                category_id
            )
            .execute(&mut *tx)
            .await?;
        }
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(channel_id)
}

pub async fn get_all_podcast_data(pool: &PgPool) -> Result<Vec<ChannelFullData>, ServiceError> {
    // Get all channel categories
    let channel_categories = sqlx::query!(
        r#"
        SELECT cc.id, cc.name, cm.channel_id
        FROM podcast_channel_categories cc
        JOIN channel_category_map cm ON cc.id = cm.category_id
        "#
    )
    .fetch_all(pool)
    .await?;

    // Group channel categories by channel_id
    let mut channel_categories_map: std::collections::HashMap<i32, Vec<String>> = std::collections::HashMap::new();
    for row in channel_categories {
        channel_categories_map
            .entry(row.channel_id)
            .or_default()
            .push(row.name);
    }

    // Get all episode categories
    let episode_categories = sqlx::query!(
        r#"
        SELECT ec.id, ec.name, em.episode_id
        FROM podcast_episode_categories ec
        JOIN episode_category_map em ON ec.id = em.category_id
        "#
    )
    .fetch_all(pool)
    .await?;

    // Group episode categories by episode_id
    let mut episode_categories_map: std::collections::HashMap<i32, Vec<String>> = std::collections::HashMap::new();
    for row in episode_categories {
        episode_categories_map
            .entry(row.episode_id)
            .or_default()
            .push(row.name);
    }

    // Get all channels
    let channels = sqlx::query!(
        r#"
        SELECT 
            id, title, link, description, copyright, language,
            author, owner_email, owner_name, image_url, last_build_date
        FROM podcast_channels
        ORDER BY id ASC
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    // Process each channel
    for channel in channels {
        // Get episodes for this channel
        let episodes = sqlx::query!(
            r#"
            SELECT 
                id, guid, title, description, link, pub_date, 
                duration, explicit, image_url, media_url, media_type, media_length
            FROM podcast_episodes
            WHERE channel_id = $1
            ORDER BY pub_date DESC
            "#,
            channel.id
        )
        .fetch_all(pool)
        .await?;

        // Map episodes
        let episode_data = episodes
            .into_iter()
            .map(|ep| EpisodeData {
                id: Some(ep.id),
                guid: ep.guid,
                title: ep.title,
                description: ep.description,
                link: ep.link,
                pub_date: ep.pub_date,
                duration: Some(ep.duration),
                explicit: ep.explicit,
                image_url: ep.image_url,
                media_url: Some(ep.media_url),
                media_type: Some(ep.media_type),
                media_length: Some(ep.media_length),
                episode_category: episode_categories_map.get(&ep.id).cloned().unwrap_or_default(),
            })
            .collect();

        // Add channel with its episodes
        result.push(ChannelFullData {
            id: channel.id,
            title: channel.title,
            link: channel.link,
            description: channel.description,
            copyright: channel.copyright,
            language: Some(channel.language),
            author: Some(channel.author),
            owner_email: Some(channel.owner_email),
            owner_name: Some(channel.owner_name),
            image_url: Some(channel.image_url),
            last_build_date: Some(channel.last_build_date.to_rfc3339()),
            categories: channel_categories_map.get(&channel.id).cloned().unwrap_or_default(),
            episodes: episode_data,
        });
    }

    Ok(result)
}