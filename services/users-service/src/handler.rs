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
use redis::Commands;

use cache;
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

fn signing_jwt(cache: &web::Data<cache::Cache>, user_id: i32) -> Result<String, String> {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "HMAC creation failed".to_string())?;
    
    let mut claims = BTreeMap::new();
    claims.insert("id".to_string(), user_id.to_string());
    claims.insert(
        "expiration".to_string(),
        (Utc::now() + Duration::days(1)).timestamp().to_string(),
    );

    let token = claims
        .sign_with_key(&key)
        .map_err(|_| "Failed to sign claims".to_string())?;

    // Store the token in Redis
    let _: () = cache_connection
        .set_ex(format!("token:{}", user_id), &token, 86400)
        .map_err(|_| "Failed to store token in Redis".to_string())?;

    Ok(token)
}


fn verify_jwt(cache: &web::Data<cache::Cache>, token: &str) -> bool {
    let mut cache_connection = cache.get_connection();

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
            if expiration_ts < Utc::now().timestamp() {
                return false; // Token has expired
            }
        } else {
            return false; // Invalid expiration timestamp
        }
    }

    // Check if token exists in Redis
    let redis_key = format!("token:{}", claims.get("id").unwrap_or(&"".to_string()));
    match cache_connection.exists(&redis_key) {
        Ok(true) => true,
        _ => false,
    }
}

fn get_user_id_by_jwt(cache: &web::Data<cache::Cache>, token: &str) -> Result<Option<i32>, String> {
    let mut cache_connection = cache.get_connection();

    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set".to_string())?;
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "HMAC creation failed".to_string())?;

    let claims: BTreeMap<String, String> = token
        .verify_with_key(&key)
        .map_err(|_| "Failed to verify token".to_string())?;

    // Check if token exists in Redis
    let redis_key = format!("token:{}", claims.get("id").unwrap_or(&"".to_string()));
    if !cache_connection.exists(&redis_key).map_err(|_| "Redis check failed".to_string())? {
        return Ok(None);
    }

    // Extract user ID
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
async fn sign_up(
    database: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    user: web::Json<UserSignUp>,
) -> impl Responder {
    match query::get_user_by_email(&database, &user.email) {
        Ok(Some(_)) => {}
        Ok(None) => {
            return ErrorResponse::InternalServerError(
                "Failed to check for existing user".to_string(),
            )
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        Err(err) => {
            eprintln!("Error checking for existing user: {:?}", err);
            return ErrorResponse::InternalServerError(
                "Failed to check for existing user".to_string(),
            )
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
    match query::add_user(
        &database,
        user.name.clone(),
        user.email.clone(),
        password_hash,
    ) {
        Ok(Some(new_user)) => {
            let token = signing_jwt(&cache, new_user.id);
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
async fn sign_in(
    database: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    user: web::Json<UserSignIn>,
) -> impl Responder {
    let existing_user = match query::get_user_by_email(&database, &user.email) {
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

    match signing_jwt(&cache, existing_user.id) {
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
async fn sign_out(cache: web::Data<cache::Cache>, token: web::ReqData<String>) -> impl Responder {
    if !verify_jwt(&cache, &token.into_inner()) {
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
async fn me(
    database: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    token: web::ReqData<String>,
) -> impl Responder {
    let user_id = match get_user_id_by_jwt(&cache, &token.into_inner()) {
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

    match query::get_user_by_id(&database, user_id) {
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
            .service(me),
    );
}
