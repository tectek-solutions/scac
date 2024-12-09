use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
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

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: i32,
    expiration: i64,
}

#[derive(ToSchema, Serialize, Deserialize)]
struct UserRegister {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

#[derive(ToSchema, Serialize, Deserialize)]
struct UserLogin {
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

type HmacSha256 = Hmac<Sha256>;

fn signing_jwt(user_id: i32) -> Result<String, String> {
    let jwt_secret = env::var("JWT_SECRET").map_err(|_| "JWT_SECRET not set")?;
    let key: HmacSha256 = Hmac::new_from_slice(jwt_secret.as_ref()).expect("HMAC creation failed");

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
    
    let key: HmacSha256 = match HmacSha256::new_from_slice(jwt_secret.as_ref()) {
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

    let key: HmacSha256 = match HmacSha256::new_from_slice(jwt_secret.as_ref()) {
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
    path = "/users/register",
    request_body = UserRegister,
    tag = "users",
    responses(
        (status = 201, description = "User registered successfully"),
        (status = 400, description = "Invalid request", body = ErrorResponse),
        (status = 409, description = "User already exists", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/register")]
async fn register(db: web::Data<database::Database>, user: web::Json<UserRegister>) -> impl Responder {
    // if query::get_user_by_email(&db, &user.email).is_some() {
    //     return ErrorResponse::Conflict("User already exists".to_string())
    //         .to_response(actix_web::http::StatusCode::CONFLICT);
    // }

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
    path = "/users/login",
    request_body = UserLogin,
    tag = "users",
    responses(
        (status = 200, description = "User logged in successfully"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/login")]
async fn login(db: web::Data<database::Database>, user: web::Json<UserLogin>) -> impl Responder {
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
async fn get_user_profile_handler(db: web::Data<database::Database>, token: web::ReqData<String>) -> impl Responder {
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

#[utoipa::path(
    get,
    path = "/users/{id}",
    params(("id" = i32, Path, description = "User ID")),
    tag = "users",
    responses(
        (status = 200, description = "User found"),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_user_handler(db: web::Data<database::Database>, id: web::Path<i32>) -> impl Responder {
    match query::get_user_by_id(&db, id.clone()) {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    put,
    path = "/users/{id}",
    params(("id" = i32, Path, description = "User ID")),
    tag = "users",
    responses(
        (status = 200, description = "User updated"),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[put("/{id}")]
async fn update_user_handler(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
    user: web::Json<UserRegister>,
) -> impl Responder {
    match query::get_user_by_id(&db, id.clone()) {
        Ok(Some(_)) => {}
        Ok(None) => {
            return ErrorResponse::NotFound("User not found".to_string())
                .to_response(actix_web::http::StatusCode::NOT_FOUND);
        }
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    if !is_valid_email(&user.email) || !is_valid_password(&user.password) {
        return ErrorResponse::InternalServerError("Invalid input".to_string())
            .to_response(actix_web::http::StatusCode::BAD_REQUEST);
    }

    let password_hash = hash(&user.password, DEFAULT_COST).expect("Password hashing failed");
    match query::update_user(&db, id.clone(), user.name.clone(), user.email.clone(), password_hash) {
        Ok(Some(updated_user)) => HttpResponse::Ok().json(updated_user),
        Ok(None) => ErrorResponse::InternalServerError("Failed to update user".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR),
        Err(err) => {
            eprintln!("Error updating user: {:?}", err);
            ErrorResponse::InternalServerError("Failed to update user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    delete,
    path = "/users/{id}",
    params(("id" = i32, Path, description = "User ID")),
    tag = "users",
    responses(
        (status = 204, description = "User deleted"),
        (status = 404, description = "User not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[delete("/{id}")]
async fn delete_user_handler(db: web::Data<database::Database>, id: web::Path<i32>) -> impl Responder {
    match query::get_user_by_id(&db, id.clone()) {
        Ok(Some(_)) => {}
        Ok(None) => {
            return ErrorResponse::NotFound("User not found".to_string())
                .to_response(actix_web::http::StatusCode::NOT_FOUND);
        }
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    match query::delete_user(&db, id.clone()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(err) => {
            eprintln!("Error deleting user: {:?}", err);
            ErrorResponse::InternalServerError("Failed to delete user".to_string())
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
            .service(register)
            .service(login)
            .service(get_user_handler)
            .service(get_user_profile_handler)
            .service(update_user_handler)
            .service(delete_user_handler),
    );
}
