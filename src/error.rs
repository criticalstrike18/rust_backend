// src/error.rs
use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
    
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    
    #[display(fmt = "Not Found")]
    NotFound,
    
    #[display(fmt = "Service Unavailable")]
    ServiceUnavailable,
    
    #[display(fmt = "Forbidden: Invalid Secret")]
    SecretInvalid,
    
    #[display(fmt = "Come Back Later")]
    ComeBackLater,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {  // Remove the * dereference operator
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
            ServiceError::ServiceUnavailable => {
                HttpResponse::ServiceUnavailable().json("Service Unavailable")
            }
            ServiceError::SecretInvalid => HttpResponse::Forbidden().json("Invalid Secret"),
            ServiceError::ComeBackLater => {
                HttpResponse::build(self.status_code()).json("Come Back Later")
            }
        }
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServiceError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceError::NotFound => StatusCode::NOT_FOUND,
            ServiceError::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            ServiceError::SecretInvalid => StatusCode::FORBIDDEN,
            ServiceError::ComeBackLater => StatusCode::from_u16(477).unwrap_or(StatusCode::OK),
        }
    }
}

// Implement From for sqlx::Error
impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> ServiceError {
        log::error!("Database error: {:?}", err);
        ServiceError::InternalServerError
    }
}

// Implement From for reqwest::Error
impl From<reqwest::Error> for ServiceError {
    fn from(err: reqwest::Error) -> ServiceError {
        log::error!("Request error: {:?}", err);
        ServiceError::ServiceUnavailable
    }
}