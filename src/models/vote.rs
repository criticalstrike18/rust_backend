// src/models/vote.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Score {
    GOOD = 1,
    OK = 0,
    BAD = -1,
}

impl Score {
    pub fn from_value(value: i32) -> Option<Score> {
        match value {
            1 => Some(Score::GOOD),
            0 => Some(Score::OK),
            -1 => Some(Score::BAD),
            _ => None,
        }
    }
    
    pub fn value(&self) -> i32 {
        *self as i32
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoteInfo {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub score: Option<Score>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Votes {
    pub votes: Vec<VoteInfo>,
}