// src/models/session.rs
use chrono::{DateTime, Datelike, Timelike, Utc};
use serde::{Deserialize, Serialize, Serializer};

pub fn serialize_datetime_as_gmt<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Format to match "yyyy-MM-dd'T'HH:mm:ss"
    let formatted = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", 
        date.year(), date.month(), date.day(),
        date.hour(), date.minute(), date.second()
    );
    serializer.serialize_str(&formatted)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    
    #[serde(rename = "startsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub starts_at: DateTime<Utc>,
    
    #[serde(rename = "endsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub ends_at: DateTime<Utc>,
    
    #[serde(rename = "roomId")]
    pub room_id: Option<i32>,
    
    #[serde(rename = "isServiceSession")]
    pub is_service_session: bool,
    
    #[serde(rename = "isPlenumSession")]
    pub is_plenum_session: bool,
    
    pub status: String,
    
    #[serde(rename = "speakerIds")]
    pub speaker_ids: Vec<String>,
    
    #[serde(rename = "categoryIds")]
    pub category_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceSessionRequest {
    pub title: String,
    pub description: Option<String>,
    
    #[serde(rename = "startsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub starts_at: DateTime<Utc>,
    
    #[serde(rename = "endsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub ends_at: DateTime<Utc>,
    
    #[serde(rename = "roomId")]
    pub room_id: Option<i32>,
    
    #[serde(rename = "isServiceSession")]
    pub is_service_session: bool,
    
    #[serde(rename = "isPlenumSession")]
    pub is_plenum_session: bool,
    
    pub status: String,
    
    #[serde(rename = "speakerIds")]
    pub speaker_ids: Vec<String>,
    
    #[serde(rename = "categoryIds")]
    pub category_ids: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponse {
    pub success: bool,
    
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionSpeakerRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "speakerId")]
    pub speaker_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionCategoriesRequest {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "categoryId")]
    pub category_id: i32,
}

// SpeakerInfo
#[derive(Debug, Serialize, Deserialize)]
pub struct SpeakerInfo {
    pub id: String,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub bio: Option<String>,
    #[serde(rename = "tagLine")]
    pub tag_line: Option<String>,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    #[serde(rename = "isTopSpeaker")]
    pub is_top_speaker: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceSpeakerRequest {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub bio: Option<String>,
    #[serde(rename = "tagLine")]
    pub tag_line: Option<String>,
    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<String>,
    #[serde(rename = "isTopSpeaker")]
    pub is_top_speaker: bool,
}