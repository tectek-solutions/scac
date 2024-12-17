use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use database;

use crate::query;

// ----------------------------
// Struct Definitions
// ----------------------------

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

// ----------------------------
// Handlers
// ----------------------------

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "List of user tokens retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/users/{user_id}")]
async fn list_user_tokens_by_user_id(
    db: web::Data<database::Database>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match query::list_user_tokens_by_user_id_query(&db, user_id.into_inner()) {
        Ok(Some(user_tokens)) => HttpResponse::Ok().json(user_tokens),
        Ok(None) => ErrorResponse::NotFound("User tokens not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting user tokens: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user tokens".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "User token details retrieved"),
        (status = 404, description = "User token ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_user_token_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_user_token_by_id_query(&db, id.into_inner()) {
        Ok(Some(user_token)) => HttpResponse::Ok().json(user_token),
        Ok(None) => ErrorResponse::NotFound("User token not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting user token: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user token".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-tokens")
            .service(list_user_tokens_by_user_id)
            .service(get_user_token_by_id)
    );
}
