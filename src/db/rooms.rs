// src/db/rooms.rs
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::room::{ConferenceRoomRequest, RoomTable};

pub async fn get_all_rooms(pool: &PgPool) -> Result<Vec<RoomTable>, ServiceError> {
    let rooms = sqlx::query!(
        r#"
        SELECT id, name, sort
        FROM conference_rooms
        "#
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

pub async fn add_room(
    pool: &PgPool,
    room: &ConferenceRoomRequest,
) -> Result<i32, ServiceError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO conference_rooms (name, sort)
        VALUES ($1, $2)
        RETURNING id
        "#,
        room.name,
        room.sort
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}