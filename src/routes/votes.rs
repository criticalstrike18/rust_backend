// src/routes/votes.rs
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;

use crate::auth::{validate_admin_secret, validate_user, KotlinConfPrincipal};
use crate::config::AppConfig;
use crate::db::{sessions, votes};
use crate::error::ServiceError;
use crate::models::vote::{VoteInfo, Votes};
use crate::services::admin::now;

#[get("/vote")]
async fn get_votes(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let votes_list = votes::get_votes(&pool, &principal.token).await?;
    Ok(HttpResponse::Ok().json(Votes { votes: votes_list }))
}

#[post("/vote")]
async fn post_vote(
    principal: KotlinConfPrincipal,
    vote_info: web::Json<VoteInfo>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    // Get the session to check if voting is allowed
    let session = match sessions::get_session_by_id(&pool, &vote_info.session_id).await? {
        Some(s) => s,
        None => return Err(ServiceError::NotFound),
    };
    
    // Check if voting period has started
    let now_time = now();
    if now_time < session.starts_at.timestamp_millis() {
        return Err(ServiceError::ComeBackLater);  // Use ComeBackLater instead of building a response manually
    }
    
    // Change the vote
    votes::change_vote(
        &pool,
        &principal.token,
        &vote_info.session_id,
        vote_info.score,
        Utc::now(),
    )
    .await?;
    
    Ok(HttpResponse::Ok().finish())
}

#[get("/vote/all")]
async fn get_all_votes(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let votes_list = votes::get_all_votes(&pool).await?;
    Ok(HttpResponse::Ok().json(votes_list))
}