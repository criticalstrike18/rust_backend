// src/routes/users.rs
use actix_web::{web, post, HttpResponse};
use chrono::Utc;

use crate::db::users;

#[post("/sign")]
async fn sign(
    user_uuid: String,
    pool: web::Data<sqlx::PgPool>,
) -> HttpResponse {
    let timestamp = Utc::now().to_rfc3339();
    
    match users::create_user(&pool, &user_uuid, &timestamp).await {
        Ok(true) => HttpResponse::Created().finish(),
        Ok(false) => HttpResponse::Conflict().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}