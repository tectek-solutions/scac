use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
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

use query::{delete_user, get_user_by_email, get_user_by_id, insert_user, update_user};

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
    let key: HmacSha256 = HmacSha256::new_from_slice(jwt_secret.as_ref())
        .map_err(|_| "Failed to create HMAC key")?;
    
    let mut claims = BTreeMap::new();
    claims.insert("id".to_string(), user_id.to_string());
    claims.insert(
        "expiration".to_string(),
        (Utc::now() + Duration::days(1)).timestamp().to_string(),
    );

    claims.sign_with_key(&key).map_err(|_| "Failed to sign claims")
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

fn get_user_id_by_jwt(token: &str) -> Option<i32> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET not set");
    let key = Hmac::new_from_slice(jwt_secret.as_ref()).expect("HMAC creation failed");
    let claims: BTreeMap<String, String>  = token.verify_with_key(&key).unwrap();

    match claims.get("id") {
        Some(id) => Some(id.parse::<i32>().unwrap()),
        None => None,
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
async fn register(user: web::Json<UserRegister>) -> impl Responder {
    if get_user_by_email(&user.email).is_some() {
        return ErrorResponse::Conflict("User already exists".to_string())
            .to_response(actix_web::http::StatusCode::CONFLICT);
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

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Password hashing failed");
    let new_user = insert_user(user.email.clone(), hashed_password);
    HttpResponse::Created().json(new_user)
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
async fn login(user: web::Json<UserLogin>) -> impl Responder {
    let existing_user = match get_user_by_email(&user.email) {
        Some(user) => user,
        None => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    if !verify(&user.password, &existing_user.password_hash).unwrap_or(false) {
        return ErrorResponse::Unauthorized("Incorrect password".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let token = signing_jwt(existing_user.id);
    HttpResponse::Ok().json(token)
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
async fn get_user_profile_handler(token: web::Header<String>) -> impl Responder {
    match get_user_id_by_jwt(&token.into_inner()) {
        Some(user_id) => match get_user_by_id(user_id) {
            Some(user) => HttpResponse::Ok().json(user),
            None => ErrorResponse::NotFound("User not found".to_string())
                .to_response(actix_web::http::StatusCode::NOT_FOUND),
        },
        None => ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED),
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
async fn get_user_handler(id: web::Path<i32>) -> impl Responder {
    match get_user_by_id(id.into_inner()) {
        Some(user) => HttpResponse::Ok().json(user),
        None => ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
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
    id: web::Path<i32>,
    user: web::Json<UserRegister>,
) -> impl Responder {
    if get_user_by_id(id.into_inner()).is_none() {
        return ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND);
    }

    if !is_valid_email(&user.email) || !is_valid_password(&user.password) {
        return ErrorResponse::InternalServerError("Invalid input".to_string())
            .to_response(actix_web::http::StatusCode::BAD_REQUEST);
    }

    let hashed_password = hash(&user.password, DEFAULT_COST).expect("Password hashing failed");
    let updated_user = update_user(id.into_inner(), user.email.clone(), hashed_password);
    HttpResponse::Ok().json(updated_user)
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
async fn delete_user_handler(id: web::Path<i32>) -> impl Responder {
    if get_user_by_id(id.into_inner()).is_none() {
        return ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND);
    }

    delete_user(id.into_inner());
    HttpResponse::NoContent().finish()
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
