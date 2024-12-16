use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use database;

use crate::query;

// ----------------------------
// Struct Definitions
// ---------------------------

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
    path = "/",
    tag = "authentications",
    responses(
        (status = 200, description = "List of authentications retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/")]
async fn list_authentications(db: web::Data<database::Database>) -> impl Responder {
    match query::list_authentications_query(&db) {
        Ok(Some(authentications)) => HttpResponse::Ok().json(authentications),
        Ok(None) => ErrorResponse::NotFound("No authentications found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentications: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentications".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "authentications",
    responses(
        (status = 200, description = "Authentication details retrieved"),
        (status = 404, description = "Authentication ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_authentication_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_authentication_by_id_query(&db, id.into_inner()) {
        Ok(Some(authentication)) => HttpResponse::Ok().json(authentication),
        Ok(None) => ErrorResponse::NotFound("Authentication not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentication: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentication".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authentications")
            .service(list_authentications)
            .service(get_authentication_by_id)
    );
}
