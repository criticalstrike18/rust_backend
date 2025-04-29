// src/routes/sessions.rs
use actix_web::{get, post, web, HttpResponse};

use crate::auth::{validate_user, KotlinConfPrincipal};
use crate::db::{categories, rooms, sessions, speakers};
use crate::error::ServiceError;
use crate::models::room::{ConferenceRoomRequest, RoomResponse};
use crate::models::session::{ConferenceSessionRequest, SessionCategoriesRequest, SessionResponse, SessionSpeakerRequest};

#[get("/get/sessions")]
async fn get_sessions(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let sessions_data = sessions::get_all_sessions(&pool).await?;
    Ok(HttpResponse::Ok().json(sessions_data))
}

#[get("/get/categories")]
async fn get_categories(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let categories_data = categories::get_all_categories(&pool).await?;
    Ok(HttpResponse::Ok().json(categories_data))
}

#[get("/get/rooms")]
async fn get_rooms(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let rooms_data = rooms::get_all_rooms(&pool).await?;
    Ok(HttpResponse::Ok().json(rooms_data))
}

#[get("/get/speakers")]
async fn get_speakers(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let speakers_data = speakers::get_all_speakers(&pool).await?;
    Ok(HttpResponse::Ok().json(speakers_data))
}

#[get("/get/session-speakers")]
async fn get_session_speakers(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let sessions_data = sessions::get_all_sessions(&pool).await?;
    let mut session_speakers = std::collections::HashMap::new();
    
    for session in sessions_data {
        session_speakers.insert(session.id, session.speaker_ids);
    }
    
    Ok(HttpResponse::Ok().json(session_speakers))
}

#[get("/get/session-categories")]
async fn get_session_categories(
    principal: KotlinConfPrincipal,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    let sessions_data = sessions::get_all_sessions(&pool).await?;
    let mut session_categories = std::collections::HashMap::new();
    
    for session in sessions_data {
        session_categories.insert(session.id, session.category_ids);
    }
    
    Ok(HttpResponse::Ok().json(session_categories))
}

#[post("/send/sessions")]
async fn send_session(
    principal: KotlinConfPrincipal,
    session: web::Json<ConferenceSessionRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    match sessions::add_session(&pool, &session).await {
        Ok(session_id) => Ok(HttpResponse::Created().json(SessionResponse {
            success: true,
            session_id: Some(session_id),
            message: Some("Session added successfully".to_string()),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(SessionResponse {
            success: false,
            session_id: None,
            message: Some(format!("{}", e)),
        })),
    }
}

#[post("/send/rooms")]
async fn send_room(
    principal: KotlinConfPrincipal,
    room: web::Json<ConferenceRoomRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    match rooms::add_room(&pool, &room).await {
        Ok(room_id) => Ok(HttpResponse::Created().json(RoomResponse {
            success: true,
            room_id: Some(room_id),
            message: Some("Room added successfully".to_string()),
        })),
        Err(e) => Ok(HttpResponse::BadRequest().json(RoomResponse {
            success: false,
            room_id: None,
            message: Some(format!("{}", e)),
        })),
    }
}

#[post("/send/session-speaker")]
async fn send_session_speaker(
    principal: KotlinConfPrincipal,
    session_speaker: web::Json<SessionSpeakerRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    // Verify that both session and speaker exist
    let session = sessions::get_session_by_id(&pool, &session_speaker.session_id).await?;
    if session.is_none() {
        return Err(ServiceError::NotFound);
    }
    
    let speaker = speakers::get_speaker_by_id(&pool, &session_speaker.speaker_id).await?;
    if speaker.is_none() {
        return Err(ServiceError::NotFound);
    }
    
    // Add the relationship
    sessions::add_session_speaker(&pool, &session_speaker.session_id, &session_speaker.speaker_id).await?;
    
    Ok(HttpResponse::Created().json("Session-speaker relationship added successfully"))
}

#[post("/send/session-categories")]
async fn send_session_categories(
    principal: KotlinConfPrincipal,
    session_categories: web::Json<SessionCategoriesRequest>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ServiceError> {
    if !validate_user(&principal, &pool).await? {
        return Err(ServiceError::Unauthorized);
    }
    
    // Verify that both session and category exist
    let session = sessions::get_session_by_id(&pool, &session_categories.session_id).await?;
    if session.is_none() {
        return Err(ServiceError::NotFound);
    }
    
    let category = categories::get_category_by_id(&pool, session_categories.category_id).await?;
    if category.is_none() {
        return Err(ServiceError::NotFound);
    }
    
    // Add the relationship
    sessions::add_session_category(&pool, &session_categories.session_id, session_categories.category_id).await?;
    
    Ok(HttpResponse::Created().json("Session-category relationship added successfully"))
}