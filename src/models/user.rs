// src/models/user.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub timestamp: String,
}