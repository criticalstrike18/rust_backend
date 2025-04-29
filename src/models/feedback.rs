// src/models/feedback.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackInfo {
    pub session_id: String,
    pub value: String,
}