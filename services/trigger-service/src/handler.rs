use actix_web::{get, web, HttpResponse, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use database;
use cache;

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
    path = "/workflows/{id}",
    tag = "triggers",
    responses(
        (status = 200, description = "List of triggers retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/workflows/{id}")]
async fn list_triggers_by_worflows_id(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    id: web::Path<i32>,
    request: HttpRequest,
) -> impl Responder {
    let jwt_token = match request.headers().get("Authorization") {
        Some(value) => value.to_str().unwrap_or("").to_string(),
        None => {
            return ErrorResponse::Unauthorized("No token provided".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let jwt_token = jwt_token.replace("Bearer ", "");

    if !jwt::verify_jwt(&cache, &jwt_token) {
        return ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let user_id = match jwt::get_user_id_by_jwt(&cache, &jwt_token) {
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

    let workflow_id = id.into_inner();

    let workflow = match database::model::Workflow::read(&mut db.get_connection(), workflow_id) {
        Ok(workflow) => workflow,
        Err(err) => {
            eprintln!("Error getting workflow: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get workflow".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if workflow.users_id != user_id {
        return ErrorResponse::Unauthorized("Unauthorized".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    match query::list_triggers_by_worflows_id_query(&db, workflow_id) {
        Ok(Some(triggers)) => HttpResponse::Ok().json(triggers),
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
    tag = "triggers",
    responses(
        (status = 200, description = "Triggers details retrieved"),
        (status = 404, description = "Triggers ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_trigger_by_id(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    id: web::Path<i32>,
    request: HttpRequest
) -> impl Responder {
    let jwt_token = match request.headers().get("Authorization") {
        Some(value) => value.to_str().unwrap_or("").to_string(),
        None => {
            return ErrorResponse::Unauthorized("No token provided".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let jwt_token = jwt_token.replace("Bearer ", "");

    if !jwt::verify_jwt(&cache, &jwt_token) {
        return ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let user_id = match jwt::get_user_id_by_jwt(&cache, &jwt_token) {
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

    let workflow_id = id.into_inner();

    match query::get_trigger_by_id_query(&db, workflow_id) {
        Ok(Some(trigger)) => {
            let workflow = database::model::Workflow::read(&mut db.get_connection(), trigger.workflows_id).unwrap();
            if workflow.users_id != user_id {
                return ErrorResponse::Unauthorized("Unauthorized".to_string())
                    .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
            }

            if workflow.users_id != user_id {
                return ErrorResponse::Unauthorized("Unauthorized".to_string())
                    .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
            }

            HttpResponse::Ok().json(trigger)
        }
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
        web::scope("/triggers")
            .service(list_triggers_by_worflows_id)
            .service(get_trigger_by_id)
    );
}
