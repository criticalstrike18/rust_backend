// src/routes/admin.rs
use actix_web::{get, post, web, HttpResponse};

use crate::auth::{validate_admin_secret, KotlinConfPrincipal};
use crate::config::AppConfig;
use crate::db::{sessions, speakers, rooms, categories};
use crate::error::ServiceError;
use crate::models::category::ConferenceCategoriesRequest;
use crate::models::room::ConferenceRoomRequest;
use crate::models::session::ConferenceSpeakerRequest;
use crate::services::admin;
use crate::services::sync::synchronize_with_sessionize;

#[post("/sessionizeSync")]
async fn sessionize_sync(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    synchronize_with_sessionize(&pool, &config.sessionize.url).await?;
    Ok(HttpResponse::Ok().finish())
}

#[get("/time")]
async fn get_time() -> HttpResponse {
    HttpResponse::Ok().json(admin::now())
}

#[post("/time/{timestamp}")]
async fn set_time(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    path: web::Path<String>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let timestamp = path.into_inner();
    if timestamp == "null" {
        admin::update_time(None);
    } else {
        match timestamp.parse::<i64>() {
            Ok(time) => admin::update_time(Some(time)),
            Err(_) => return Err(ServiceError::BadRequest("Invalid timestamp".to_string())),
        }
    }
    
    Ok(HttpResponse::Ok().finish())
}

#[post("/admin/session")]
async fn add_admin_session(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    session: web::Json<ConferenceSessionRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let session_id = sessions::add_session(&pool, &session).await?;
    Ok(HttpResponse::Created().json(session_id))
}

#[post("/admin/speakers")]
async fn add_admin_speaker(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    speaker: web::Json<ConferenceSpeakerRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let speaker_id = speakers::add_speaker(&pool, &speaker).await?;
    Ok(HttpResponse::Created().json(speaker_id))
}

#[post("/admin/rooms")]
async fn add_admin_room(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    room: web::Json<ConferenceRoomRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let room_id = rooms::add_room(&pool, &room).await?;
    Ok(HttpResponse::Created().json(room_id))
}

#[post("/admin/categories")]
async fn add_admin_category(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    category: web::Json<ConferenceCategoriesRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let category_id = categories::add_category(&pool, &category).await?;
    Ok(HttpResponse::Created().json(category_id))
}