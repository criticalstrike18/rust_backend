// src/models/room.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomTable {
    pub id: Option<i64>,
    pub name: String,
    pub sort: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceRoomRequest {
    pub name: String,
    pub sort: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomResponse {
    pub success: bool,
    #[serde(rename = "roomId")]
    pub room_id: Option<i32>,
    pub message: Option<String>,
}