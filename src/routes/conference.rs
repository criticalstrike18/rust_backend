// src/routes/conference.rs
use actix_web::{get, web, HttpResponse};

use crate::db::sessions;
use crate::error::ServiceError;

#[get("/conference")]
async fn get_conference(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let conference = sessions::get_conference_data(&pool).await?;
    Ok(HttpResponse::Ok().json(conference))
}