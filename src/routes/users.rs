// src/routes/users.rs
use actix_web::{post, web, HttpResponse};
use chrono::Utc;

use crate::db::users;
use crate::error::ServiceError;

#[post("/sign")]
async fn sign(
    user_uuid: String,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let timestamp = Utc::now().to_rfc3339();
    
    match users::create_user(&pool, &user_uuid, &timestamp).await {
        Ok(true) => Ok(HttpResponse::Created().finish()),
        Ok(false) => Ok(HttpResponse::Conflict().finish()),
        Err(e) => Err(e.into()),
    }
}