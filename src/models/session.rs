// src/models/session.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub room_id: Option<i32>,
    pub is_service_session: bool,
    pub is_plenum_session: bool,
    pub status: String,
    pub speaker_ids: Vec<String>,
    pub category_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceSessionRequest {
    pub title: String,
    pub description: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub room_id: Option<i32>,
    pub is_service_session: bool,
    pub is_plenum_session: bool,
    pub status: String,
    pub speaker_ids: Vec<String>,
    pub category_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub success: bool,
    pub session_id: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSpeakerRequest {
    pub session_id: String,
    pub speaker_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCategoriesRequest {
    pub session_id: String,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpeakerInfo {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub tag_line: Option<String>,
    pub profile_picture: Option<String>,
    pub is_top_speaker: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceSpeakerRequest {
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub tag_line: Option<String>,
    pub profile_picture: Option<String>,
    pub is_top_speaker: bool,
}