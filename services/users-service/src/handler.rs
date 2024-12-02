use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use bcrypt::{hash, verify, DEFAULT_COST};
use jwt::{Claims, DecodingKey, EncodingKey, Header, Validation};
use regex::Regex;

mod query;

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

// Helper Functions
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

fn generate_jwt(user_id: i32) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
    };
    jwt::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

fn get_user_id_by_jwt(token: &str) -> Option<i32> {
    jwt::decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )
    .ok()?
    .claims
    .sub
    .parse()
    .ok()
}

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
    if query::get_user_by_email(&user.email).is_some() {
        return ErrorResponse::Conflict("User already exists".to_string())
            .to_response(actix_web::http::StatusCode::CONFLICT);
    }

    if !is_valid_email(&user.email) {
        return ErrorResponse::InternalServerError("Invalid email format".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    if user.password != user.password_confirmation {
        return ErrorResponse::InternalServerError("Passwords do not match".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    if !is_valid_password(&user.password) {
        return ErrorResponse::InternalServerError(
            "Password must be at least 8 characters long and contain at least one letter and one number".to_string(),
        )
        .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let new_user = query::insert_user(user.email.clone(), hashed_password);

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
    let existing_user = match query::get_user_by_email(&user.email) {
        Some(user) => user,
        None => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED)
        }
    };

    if !verify(&user.password, &existing_user.password_hash).unwrap() {
        return ErrorResponse::Unauthorized("Incorrect password".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let token = generate_jwt(existing_user.id);
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
async fn user_profile(token: web::Header<String>) -> impl Responder {
    let user_id = match get_user_id_by_jwt(&token.into_inner()) {
        Some(id) => id,
        None => {
            return ErrorResponse::Unauthorized("Invalid token".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED)
        }
    };

    match query::get_user_by_id(user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
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
    match query::get_user_by_id(id.into_inner()) {
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
    if query::get_user_by_id(id.into_inner()).is_none() {
        return ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND);
    }

    if !is_valid_email(&user.email) || !is_valid_password(&user.password) {
        return ErrorResponse::InternalServerError("Invalid input".to_string())
            .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();
    let updated_user = query::update_user(id.into_inner(), user.email.clone(), hashed_password);

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
    if query::get_user_by_id(id.into_inner()).is_none() {
        return ErrorResponse::NotFound("User not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND);
    }

    query::delete_user(id.into_inner());
    HttpResponse::NoContent().finish()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(register)
            .service(login)
            .service(user_profile)
            .service(user_by_id)
            .service(update_user_handler)
            .service(delete_user_handler),
    );
}
