// src/db/categories.rs
use sqlx::PgPool;

use crate::error::ServiceError;
use crate::models::category::{CategoriesTable, ConferenceCategoriesRequest};

pub async fn get_all_categories(pool: &PgPool) -> Result<Vec<CategoriesTable>, ServiceError> {
    let categories = sqlx::query!(
        r#"
        SELECT id, title, sort, type as "type_name"
        FROM conference_categories
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(categories
        .into_iter()
        .map(|row| CategoriesTable {
            id: row.id as i64,
            title: row.title,
            sort: row.sort,
            type_name: row.type_name,
        })
        .collect())
}

pub async fn get_category_by_id(
    pool: &PgPool,
    category_id: i32,
) -> Result<Option<CategoriesTable>, ServiceError> {
    let category = sqlx::query!(
        r#"
        SELECT id, title, sort, type as "type_name"
        FROM conference_categories
        WHERE id = $1
        "#,
        category_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(category.map(|row| CategoriesTable {
        id: row.id as i64,
        title: row.title,
        sort: row.sort,
        type_name: row.type_name,
    }))
}

pub async fn add_category(
    pool: &PgPool,
    category: &ConferenceCategoriesRequest,
) -> Result<i32, ServiceError> {
    let row = sqlx::query!(
        r#"
        INSERT INTO conference_categories (title, sort, type)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        category.title,
        category.sort,
        category.type_name
    )
    .fetch_one(pool)
    .await?;

    Ok(row.id)
}