// src/routes/feedback.rs
use actix_web::{get, post, web, HttpResponse};
use chrono::Utc;

use crate::auth::{validate_admin_secret, validate_user, KotlinConfPrincipal};
use crate::config::AppConfig;
use crate::db::feedback;
use crate::error::ServiceError;
use crate::models::feedback::FeedbackInfo;

#[post("/feedback")]
async fn post_feedback(
    principal: KotlinConfPrincipal,
    feedback_info: web::Json<FeedbackInfo>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let result = feedback::set_feedback(
        &pool,
        &principal.token,
        &feedback_info.session_id,
        &feedback_info.value,
        Utc::now(),
    )
    .await?;
    
    if result {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::Forbidden().finish())
    }
}

#[get("/feedback/summary")]
async fn get_feedback_summary(
    principal: KotlinConfPrincipal,
    config: web::Data<AppConfig>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    validate_admin_secret(&principal, &config.service.secret).await?;
    
    let summary = feedback::get_feedback_summary(&pool).await?;
    Ok(HttpResponse::Ok().json(summary))
}