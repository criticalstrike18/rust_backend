// src/models/conference.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::session::serialize_datetime_as_gmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conference {
    pub sessions: Vec<Session>,
    pub speakers: Vec<Speaker>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub description: String,
    
    #[serde(rename = "speakerIds")]
    pub speaker_ids: Vec<String>,
    
    pub location: String,
    
    #[serde(rename = "startsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub starts_at: DateTime<Utc>,
    
    #[serde(rename = "endsAt", serialize_with = "serialize_datetime_as_gmt")]
    pub ends_at: DateTime<Utc>,
    
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speaker {
    pub id: String,
    pub name: String,
    pub position: String,
    pub description: String,
    
    #[serde(rename = "photoUrl")]
    pub photo_url: String,
}