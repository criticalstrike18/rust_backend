// src/routes/podcast.rs
use actix_web::{get, post, web, HttpResponse};
use actix_web::http::header::ContentType;

use crate::auth::{validate_user, KotlinConfPrincipal};
use crate::db::podcast;
use crate::error::ServiceError;
use crate::models::podcast::{PodcastImportRequest, PodcastQueryInfo};

#[post("/podcast/sendRequest")]
async fn send_podcast_request(
    principal: KotlinConfPrincipal,
    query: web::Json<PodcastQueryInfo>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let result = podcast::store_podcast_query(
        &pool,
        &principal.token,
        &query.title,
        &query.author,
        &query.rss_link,
    )
    .await?;
    
    if result {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[post("/podcast/import")]
async fn import_podcast(
    import_request: web::Json<PodcastImportRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let channel_id = podcast::store_podcast_data(&pool, &import_request).await?;
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "channel_id": channel_id
    })))
}

#[get("/podcast/all")]
async fn get_all_podcasts(
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    let data = podcast::get_all_podcast_data(&pool).await?;
    
    // In the Kotlin version, this uses ProtoBuf serialization
    // In this Rust version, we'll use JSON for simplicity
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(data))
}