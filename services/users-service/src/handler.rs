use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(ToSchema, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub enum ErrorResponse {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}

#[utoipa::path(
    post,
    path = "/users/register",
    request_body = User,
    tag = "users",
    responses(
        (status = 201, description = "User registered successfully", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 400, description = "Invalid request", body = ErrorResponse, example = json!(ErrorResponse::NotFound("Invalid email format".to_string()))),
        (status = 409, description = "User already exists", body = ErrorResponse, example = json!(ErrorResponse::Conflict("An account with this email already exists".to_string()))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized("Internal server error".to_string())))
    )
)]
#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register")
}

#[utoipa::path(
    post,
    path = "/users/login",
    request_body = User,
    tag = "users",
    responses(
        (status = 200, description = "User logged in successfully", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 400, description = "Invalid request", body = ErrorResponse, example = json!(ErrorResponse::NotFound("Invalid email format".to_string()))),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized("Invalid email or password".to_string()))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized("Internal server error".to_string())))
    )
)]
#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[utoipa::path(
    get,
    path = "/users/user/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    tag = "users",
    responses(
        (status = 200, description = "User found", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound("User not found".to_string()))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized("Internal server error".to_string())))
    )
)]
#[get("/user/{id}")]
async fn user_by_id(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("User: {}", id))
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/users")
            .service(register)
            .service(login)
            .service(user_by_id),
    );
}
