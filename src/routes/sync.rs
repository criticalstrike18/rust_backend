use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

use crate::auth::{validate_user, KotlinConfPrincipal};
use crate::db::sync;
use crate::error::ServiceError;

#[derive(Debug, Deserialize)]
pub struct SyncQuery {
    pub since: Option<i64>,
}

#[get("/sync/sessions")]
async fn sync_sessions(
    principal: KotlinConfPrincipal,
    query: web::Query<SyncQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let since_timestamp = query.since.unwrap_or(0);
    let sessions = sync::get_sessions_changed_since(&pool, since_timestamp).await?;
    
    Ok(HttpResponse::Ok().json(sessions))
}

#[get("/sync/speakers")]
async fn sync_speakers(
    principal: KotlinConfPrincipal,
    query: web::Query<SyncQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let since_timestamp = query.since.unwrap_or(0);
    let speakers = sync::get_speakers_changed_since(&pool, since_timestamp).await?;
    
    Ok(HttpResponse::Ok().json(speakers))
}

#[get("/sync/rooms")]
async fn sync_rooms(
    principal: KotlinConfPrincipal,
    query: web::Query<SyncQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let since_timestamp = query.since.unwrap_or(0);
    let rooms = sync::get_rooms_changed_since(&pool, since_timestamp).await?;
    
    Ok(HttpResponse::Ok().json(rooms))
}

#[get("/sync/categories")]
async fn sync_categories(
    principal: KotlinConfPrincipal,
    query: web::Query<SyncQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let since_timestamp = query.since.unwrap_or(0);
    let categories = sync::get_categories_changed_since(&pool, since_timestamp).await?;
    
    Ok(HttpResponse::Ok().json(categories))
}

#[get("/sync/podcasts")]
async fn sync_podcasts(
    principal: KotlinConfPrincipal,
    query: web::Query<SyncQuery>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let since_timestamp = query.since.unwrap_or(0);
    let podcasts = sync::get_podcasts_changed_since(&pool, since_timestamp).await?;
    
    // Return serialized data
    let serialized = serde_json::to_vec(&podcasts)?;
    
    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .body(serialized))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(sync_sessions)
       .service(sync_speakers)
       .service(sync_rooms)
       .service(sync_categories)
       .service(sync_podcasts);
}