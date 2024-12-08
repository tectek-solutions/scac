use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use utoipa::ToSchema;
use chrono::{Utc, Duration};

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
    Conflict(String),
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

fn is_valid_email(email: &str) -> bool {
    Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$")
        .unwrap()
        .is_match(email)
}

fn is_valid_password(password: &str) -> bool {
    Regex::new(r"^(?=.*[A-Za-z])(?=.*\d)[A-Za-z\d]{8,}$")
        .unwrap()
        .is_match(password)
}

fn signing_jwt(user_id: i32) -> Result<String, String> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set")?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref()).expect("HMAC creation failed");
    
    let mut claims = BTreeMap::new();
    claims.insert("id".to_string(), user_id.to_string());
    claims.insert(
        "expiration".to_string(),
        (Utc::now() + Duration::days(1)).timestamp().to_string(),
    );

    claims.sign_with_key(&key).map_err(|_| "Failed to sign claims".to_string())
}

fn verify_jwt(token: &str) -> bool {
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return false,
    };
    
    let key: Hmac<Sha256> = match Hmac::new_from_slice(jwt_secret.as_ref()) {
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

fn get_user_id_by_jwt(token: &str) -> Result<Option<i32>, String> {
    let jwt_secret = match env::var("JWT_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err("JWT_SECRET not set".to_string()),
    };

    let key: Hmac<Sha256> = match Hmac::new_from_slice(jwt_secret.as_ref()) {
        Ok(k) => k,
        Err(_) => return Err("HMAC creation failed".to_string()),
    };

    let claims: BTreeMap<String, String> = match token.verify_with_key(&key) {
        Ok(c) => c,
        Err(_) => return Err("Failed to verify token".to_string()),
    };

    match claims.get("id") {
        Some(id) => match id.parse::<i32>() {
            Ok(id) => Ok(Some(id)),
            Err(_) => Err("Failed to parse user ID".to_string()),
        },
        None => Ok(None),
    }
}


// ----------------------------
// Handlers
// ----------------------------

#[utoipa::path(
    post,
    path = "/users/sign_up",
    request_body = UserSignUp,
    tag = "users",
    responses(
        (status = 201, description = "User sign up successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/sign_up")]
async fn sign_up(db: web::Data<database::Database>, user: web::Json<UserSignUp>) -> impl Responder {
    match query::get_user_by_email(&db, &user.email) {
        Ok(Some(_)) => {}
        Ok(None) => {
            return ErrorResponse::InternalServerError("Failed to check for existing user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(err) => {
            eprintln!("Error checking for existing user: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to check for existing user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    if !is_valid_email(&user.email) {
        return ErrorResponse::InternalServerError("Invalid email format".to_string())
            .to_response(actix_web::http::StatusCode::BAD_REQUEST);
    }

    if user.password != user.password_confirmation {
        return ErrorResponse::InternalServerError("Passwords do not match".to_string())
            .to_response(actix_web::http::StatusCode::BAD_REQUEST);
    }

    if !is_valid_password(&user.password) {
        return ErrorResponse::InternalServerError(
            "Password must be at least 8 characters long and contain at least one letter and one number".to_string(),
        )
        .to_response(actix_web::http::StatusCode::BAD_REQUEST);
    }

    let password_hash = hash(&user.password, DEFAULT_COST).expect("Password hashing failed");
    match query::add_user(&db, user.name.clone(), user.email.clone(), password_hash) {
        Ok(Some(new_user)) => {
            let token = signing_jwt(new_user.id);
            HttpResponse::Created().json(token)
        }
        Ok(None) => ErrorResponse::InternalServerError("Failed to add user".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        Err(err) => {
            eprintln!("Error adding user: {:?}", err);
            ErrorResponse::InternalServerError("Failed to add user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    post,
    path = "/users/sign_in",
    request_body = UserSignIn,
    tag = "users",
    responses(
        (status = 200, description = "User sign in in successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/sign_in")]
async fn sign_in(db: web::Data<database::Database>, user: web::Json<UserSignIn>) -> impl Responder {
    let existing_user = match query::get_user_by_email(&db, &user.email) {
        Ok(Some(user)) => user,
        Ok(None) => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !verify(&user.password, &existing_user.password_hash).unwrap_or(false) {
        return ErrorResponse::Unauthorized("Incorrect password".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    match signing_jwt(existing_user.id) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => ErrorResponse::InternalServerError("Failed to sign token".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[utoipa::path(
    post,
    path = "/users/sign_out",
    tag = "users",
    responses(
        (status = 200, description = "User sign out successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/sign_out")]
async fn sign_out(token: web::ReqData<String>) -> impl Responder {
    if !verify_jwt(&token.into_inner()) {
        return ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    HttpResponse::Ok().finish()
}

#[utoipa::path(
    get,
    path = "/users/me",
    tag = "users",
    responses(
        (status = 200, description = "User profile retrieved"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/me")]
async fn me(db: web::Data<database::Database>, token: web::ReqData<String>) -> impl Responder {
    let user_id = match get_user_id_by_jwt(&token.into_inner()) {
        Ok(Some(id)) => id,
        Ok(None) => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            return ErrorResponse::Unauthorized("Invalid token".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    match query::get_user_by_id(&db, user_id) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => ErrorResponse::Unauthorized("User not found".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED),
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(sign_up)
            .service(sign_in)
            .service(sign_out)
            .service(me)
    );
}
