use actix_web::{get, web, HttpResponse, Responder};
use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::ToSchema;
use chrono::Utc;

use database;

use crate::query;

// ----------------------------
// Struct Definitions
// ----------------------------

#[derive(ToSchema, Serialize, Deserialize)]
struct UserSignUp {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(ToSchema, Serialize, Deserialize)]
struct UserSignIn {
    email: String,
    password: String,
}

#[derive(ToSchema, Serialize, Deserialize, Clone)]
struct UserId {
    id: i32,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
    NotFound(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl ErrorResponse {
    fn to_response(&self, status: actix_web::http::StatusCode) -> HttpResponse {
        HttpResponse::build(status).json(self)
    }
}

// ----------------------------
// Helper Functions
// ----------------------------

fn verify_jwt(token: &str) -> bool {
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };
    
    let key: Hmac<Sha256> = match Hmac::<Sha256>::new_from_slice(jwt_secret.as_bytes()) {
        Ok(k) => k,
        Err(_) => return false,
    };
    
    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(expiration) = claims.get("expiration") {
        if let Ok(expiration_ts) = expiration.parse::<i64>() {
            return expiration_ts >= Utc::now().timestamp();
        }
    }

    false
}

// ----------------------------
// Handlers
// ----------------------------

#[utoipa::path(
    get,
    path = "/",
    tag = "authentifications",
    responses(
        (status = 200, description = "List of authentifications retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/")]
async fn list_authentifications(db: web::Data<database::Database>) -> impl Responder {
    if !verify_jwt("token") {
        return ErrorResponse::Unauthorized("Unauthorized".to_string())
            .to_response(actix_web::http::StatusCode::FORBIDDEN);
    }

    match query::list_authentifications_query(&db) {
        Ok(Some(authentifications)) => HttpResponse::Ok().json(authentifications),
        Ok(None) => ErrorResponse::NotFound("No authentifications found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentifications: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentifications".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "authentifications",
    responses(
        (status = 200, description = "Authentification details retrieved"),
        (status = 404, description = "Authentification ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_authentification_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    if !verify_jwt("token") {
        return ErrorResponse::Unauthorized("Unauthorized".to_string())
            .to_response(actix_web::http::StatusCode::FORBIDDEN);
    }

    match query::get_authentification_by_id_query(&db, id.into_inner()) {
        Ok(Some(authentification)) => HttpResponse::Ok().json(authentification),
        Ok(None) => ErrorResponse::NotFound("Authentification not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentification: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentification".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authentifications")
            .service(list_authentifications)
            .service(get_authentification_by_id)
    );
}
