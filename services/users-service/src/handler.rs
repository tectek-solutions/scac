use actix_web::{get, post, web, HttpResponse, Responder};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use cache;
use database;
use jwt;

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
    let _ = email;
    true
}

fn is_valid_password(password: &str) -> bool {
    let _ = password;
    true
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
        Ok(Some(_)) => {
            return ErrorResponse::InternalServerError(
                "Failed to check for existing user".to_string(),
            )
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(None) => {}
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
    match query::create_user(
        &database,
        database::model::CreateUser {
            name: user.name.clone(),
            email: user.email.clone(),
            password_hash,
        },
    ) {
        Ok(Some(new_user)) => {
            let token = jwt::signing_jwt(&cache, new_user.id);
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

    match jwt::signing_jwt(&cache, existing_user.id) {
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
    if !jwt::verify_jwt(&cache, &token.into_inner()) {
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
    let user_id = match jwt::get_user_id_by_jwt(&cache, &token.into_inner()) {
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
