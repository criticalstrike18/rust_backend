// src/models/conference.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub description: String,
    pub speaker_ids: Vec<String>,
    pub location: String,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Speaker {
    pub id: String,
    pub name: String,
    pub position: String,
    pub description: String,
    pub photo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Conference {
    pub sessions: Vec<Session>,
    pub speakers: Vec<Speaker>,
}