use actix_web::{
    get, post, web, HttpResponse, Responder
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const USERS: &str = "users";


#[derive(ToSchema, Serialize)]
struct User {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) enum ErrorResponse {
    NotFound(String),
    Conflict(String),
    Unauthorized(String),
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = User,
    tag = USERS,
    responses(
        (status = 201, description = "User registered successfully", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 400, description = "Invalid request", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("Invalid email format")))),
        (status = 409, description = "User already exists", body = ErrorResponse, example = json!(ErrorResponse::Conflict(String::from("An account with this email already exists")))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Internal server error"))))
    )
)]
#[post("/register")]
async fn register() -> impl Responder {
    HttpResponse::Ok().body("Register")
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = User,
    tag = USERS,
    responses(
        (status = 200, description = "User logged in successfully", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 400, description = "Invalid request", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("Invalid email format")))),
        (status = 401, description = "Unauthorized", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Invalid email or password")))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Internal server error"))))
    )
)]
#[post("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Login")
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    tag = USERS,
    responses(
        (status = 200, description = "User found", body = User, example = json!(User { id: 1, name: "John Doe".to_string() })),
        (status = 404, description = "User not found", body = ErrorResponse, example = json!(ErrorResponse::NotFound(String::from("User not found")))),
        (status = 500, description = "Internal server error", body = ErrorResponse, example = json!(ErrorResponse::Unauthorized(String::from("Internal server error"))))
    )
)]
#[get("/user/{id}")]
async fn user_by_id(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("User: {}", id))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/users")
        .service(register)
        .service(login)
        .service(user_by_id);
    conf.service(scope);
}