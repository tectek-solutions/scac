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
    tag = "authentifications",
    responses(
        (status = 200, description = "List of authentifications retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/")]
async fn list_authentifications(db: web::Data<database::Database>) -> impl Responder {
    match query::list_authentifications_query(&db) {
        Ok(Some(authentifications)) => HttpResponse::Ok().json(authentifications),
        Ok(None) => ErrorResponse::NotFound("No authentifications found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentifications: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentifications".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "authentifications",
    responses(
        (status = 200, description = "Authentification details retrieved"),
        (status = 404, description = "Authentification ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_authentification_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_authentification_by_id_query(&db, id.into_inner()) {
        Ok(Some(authentification)) => HttpResponse::Ok().json(authentification),
        Ok(None) => ErrorResponse::NotFound("Authentification not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting authentification: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get authentification".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/authentifications")
            .service(list_authentifications)
            .service(get_authentification_by_id)
    );
}
