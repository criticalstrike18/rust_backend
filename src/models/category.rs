// src/models/category.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesTable {
    pub id: i64,
    pub title: String,
    pub sort: Option<i32>,
    pub type_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConferenceCategoriesRequest {
    pub title: String,
    pub sort: Option<i32>,
    pub type_name: Option<String>,
}