// src/models/conference.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde::ser::Serializer;

#[derive(Debug, Serialize, Deserialize)]
pub struct Conference {
    pub sessions: Vec<Session>,
    pub speakers: Vec<Speaker>,
}

fn serialize_datetime_no_z<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(dt.format("%Y-%m-%dT%H:%M:%S").to_string().as_str())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: String,
    pub description: String,
    
    #[serde(rename = "speakerIds")]
    pub speaker_ids: Vec<String>,
    
    pub location: String,
    
    #[serde(rename = "startsAt", serialize_with = "serialize_datetime_no_z")]
    pub starts_at: DateTime<Utc>,
    #[serde(rename = "endsAt", serialize_with = "serialize_datetime_no_z")]
    pub ends_at: DateTime<Utc>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>
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