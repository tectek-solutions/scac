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

// - **GET** `/:id`
//   - **Access**: Public  
//   - **Responses**:  
//     - `200 OK`: API service retrieved.  
//     - `404 Not Found`: Service ID not found.  
//     - `500 Internal Server Error`: Error fetching service.

// - **GET** `/authentification/:id`
//   - **Access**: Public  
//   - **Responses**:  
//     - `200 OK`: List of authentication services retrieved.
//     - `404 Not Found`: Service ID not found.
//     - `500 Internal Server Error`: Error fetching services.

// handler::list_api_services_by_authentification_id,
// handler::get_api_service_by_id


#[utoipa::path(
    get,
    path = "/authentification/{id}",
    tag = "api-services",
    responses(
        (status = 200, description = "List of api-services retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/authentification/{id}")]
async fn list_api_services_by_authentification_id(db: web::Data<database::Database>, id: web::Path<i32>) -> impl Responder {
    match query::list_api_services_by_authentification_id_query(&db, id.into_inner()) {
        Ok(Some(api_services)) => HttpResponse::Ok().json(api_services),
        Ok(None) => ErrorResponse::NotFound("No api services found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting api services: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get api services".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "api-services",
    responses(
        (status = 200, description = "Authentification details retrieved"),
        (status = 404, description = "Authentification ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_api_service_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_api_service_by_id_query(&db, id.into_inner()) {
        Ok(Some(api_service)) => HttpResponse::Ok().json(api_service),
        Ok(None) => ErrorResponse::NotFound("Api service not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting api service: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get api service".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api-services")
            .service(list_api_services_by_authentification_id)
            .service(get_api_service_by_id),
    );
}