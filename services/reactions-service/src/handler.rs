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
    path = "/apis/{id}",
    tag = "reactions",
    responses(
        (status = 200, description = "List of reactions retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/")]
async fn list_reactions_by_api_service_id(db: web::Data<database::Database>, id: web::Path<i32>) -> impl Responder {
    match query::list_reactions_by_api_service_id_query(&db, id.into_inner()) {
        Ok(Some(reactions)) => HttpResponse::Ok().json(reactions),
        Ok(None) => ErrorResponse::NotFound("No reactions found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting reactions: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get reactions".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "reactions",
    responses(
        (status = 200, description = "Reactions details retrieved"),
        (status = 404, description = "Reactions ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_reaction_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_reaction_by_id_query(&db, id.into_inner()) {
        Ok(Some(action)) => HttpResponse::Ok().json(action),
        Ok(None) => ErrorResponse::NotFound("Authentication not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting action: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get action".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/reactions")
            .service(list_reactions_by_api_service_id)
            .service(get_reaction_by_id),
    );
}
