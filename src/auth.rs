// src/auth.rs
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http::header, web, Error, FromRequest, HttpRequest,
};
use futures::future::{ready, Ready};
use sqlx::PgPool;
use std::future::Future;
use std::pin::Pin;

use crate::db::users;
use crate::error::ServiceError;

pub struct KotlinConfPrincipal {
    pub token: String,
}

impl FromRequest for KotlinConfPrincipal {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Extract the Authorization header
        let auth_header = match req.headers().get(header::AUTHORIZATION) {
            Some(header) => header,
            None => return ready(Err(ErrorUnauthorized("No authorization header"))),
        };
        
        // Parse the Bearer token
        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid authorization header"))),
        };
        
        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Invalid authorization scheme")));
        }
        
        let token = auth_str.trim_start_matches("Bearer ").trim().to_string();
        ready(Ok(KotlinConfPrincipal { token }))
    }
}

pub async fn validate_user(
    principal: &KotlinConfPrincipal, 
    pool: &PgPool
) -> Result<bool, ServiceError> {
    Ok(users::validate_user(pool, &principal.token).await?)
}

pub async fn validate_admin_secret(
    principal: &KotlinConfPrincipal,
    admin_secret: &str,
) -> Result<(), ServiceError> {
    if principal.token != admin_secret {
        return Err(ServiceError::Unauthorized);
    }
    Ok(())
}