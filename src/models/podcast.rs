// src/models/podcast.rs
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelData {
    pub title: String,
    pub link: String,
    pub description: String,
    pub copyright: Option<String>,
    pub language: Option<String>,
    pub author: Option<String>,
    
    #[serde(rename = "ownerEmail")]
    pub owner_email: Option<String>,
    
    #[serde(rename = "ownerName")]
    pub owner_name: Option<String>,
    
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodeData {
    pub id: Option<i32>,
    pub guid: String,
    pub title: String,
    pub description: String,
    pub link: String,
    
    #[serde(rename = "pubDate")]
    pub pub_date: DateTime<Utc>,
    
    pub duration: Option<i32>,
    pub explicit: bool,
    
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    
    #[serde(rename = "mediaUrl")]
    pub media_url: Option<String>,
    
    #[serde(rename = "mediaType")]
    pub media_type: Option<String>,
    
    #[serde(rename = "mediaLength")]
    pub media_length: Option<i64>,
    
    #[serde(rename = "episodeCategory")]
    pub episode_category: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodcastImportRequest {
    pub channel: ChannelData,
    pub categories: Vec<String>,
    pub episodes: Vec<EpisodeData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelFullData {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub description: String,
    pub copyright: Option<String>,
    pub language: Option<String>,
    pub author: Option<String>,
    
    #[serde(rename = "ownerEmail")]
    pub owner_email: Option<String>,
    
    #[serde(rename = "ownerName")]
    pub owner_name: Option<String>,
    
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    
    #[serde(rename = "lastBuildDate")]
    pub last_build_date: Option<String>,
    
    pub categories: Vec<String>,
    pub episodes: Vec<EpisodeData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodcastQueryInfo {
    pub title: String,
    pub author: String,
    
    #[serde(rename = "rssLink")]
    pub rss_link: String,
}