// src/models/feedback.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackInfo {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub value: String,
}