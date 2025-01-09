use actix_web::{delete, get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use cache;
use database;
use jwt;

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
    tag = "workflows",
    responses(
        (status = 200, description = "List of workflows retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/")]
async fn list_workflows(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    request: HttpRequest,
) -> impl Responder {
    let jwt_token: String = match request.headers().get("Authorization") {
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

    match query::list_workflows_by_user_id_query(&db, user_id) {
        Ok(Some(workflows)) => HttpResponse::Ok().json(workflows),
        Ok(None) => ErrorResponse::NotFound("No workflows found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting workflows: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get workflows".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "workflows",
    responses(
        (status = 200, description = "Workflow details retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 404, description = "Workflow ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_workflow_by_id(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    request: HttpRequest,
    id: web::Path<i32>,
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

    match query::get_workflow_by_id_by_user_id_query(&db, id.into_inner(), user_id) {
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

#[utoipa::path(
    post,
    path = "/",
    tag = "workflows",
    request_body = database::model::CreateWorkflow,
    responses(
        (status = 200, description = "Workflow created"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/")]
async fn create_workflow(
    database: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    request: HttpRequest,
    workflow: web::Json<database::model::CreateWorkflow>,
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

    let mut workflow = workflow.into_inner();
    workflow.users_id = user_id;

    match query::create_workflow_query(&database, workflow) {
        Ok(workflow) => HttpResponse::Ok().json(workflow),
        Err(err) => {
            eprintln!("Error creating workflow: {:?}", err);
            ErrorResponse::InternalServerError("Failed to create workflow".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    delete,
    path = "/{id}",
    tag = "workflows",
    responses(
        (status = 200, description = "Workflow deleted"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[delete("/{id}")]
async fn delete_workflow_by_id(
    database: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    request: HttpRequest,
    id: web::Path<i32>,
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

    let workflow = match query::get_workflow_by_id_by_user_id_query(&database, workflow_id, user_id)
    {
        Ok(Some(workflow)) => workflow,
        Ok(None) => {
            return ErrorResponse::NotFound("Workflow not found".to_string())
                .to_response(actix_web::http::StatusCode::NOT_FOUND);
        }
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

    match query::delete_workflow_by_id_query(&database, workflow_id) {
        Ok(Some(())) => HttpResponse::Ok().finish(),
        Ok(None) => ErrorResponse::NotFound("Workflow not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error deleting workflow: {:?}", err);
            ErrorResponse::InternalServerError("Failed to delete workflow".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/workflows")
            .service(list_workflows)
            .service(get_workflow_by_id)
            .service(create_workflow)
            .service(delete_workflow_by_id),
    );
}
