use actix_web::{get, post, delete, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use database;

use crate::query;

// ----------------------------
// Struct Definitions
// ---------------------------

// pub user_id: i32,
// pub name: &'a str,
// pub description: Option<&'a str>,
// pub action_id: i32,
// pub reaction_id: i32,
// pub data_transformation: Option<serde_json::Value>,
// pub created_at: Option<NaiveDateTime>,
// pub updated_at: Option<NaiveDateTime>,

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
    path = "/users/{id}",
    tag = "workflows",
    responses(
        (status = 200, description = "List of workflows retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/users/{id}")]
async fn list_workflows_by_users_id(db: web::Data<database::Database>, id: web::Path<i32>) -> impl Responder {
    match query::list_workflows_by_user_id_query(&db, id.into_inner()) {
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
        (status = 200, description = "Authentication details retrieved"),
        (status = 404, description = "Authentication ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_workflow_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_workflow_by_id_query(&db, id.into_inner()) {
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
    request_body = query::CreateWorkflow,
    responses(
        (status = 200, description = "Workflow created"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[post("/")]
async fn create_workflow(
    db: web::Data<database::Database>,
    workflow: web::Json<query::CreateWorkflow>,
) -> impl Responder {
    match query::create_workflow_query(&db, workflow.into_inner()) {
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
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::delete_workflow_by_id_query(&db, id.into_inner()) {
        Ok(workflow) => HttpResponse::Ok().json(workflow),
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
            .service(list_workflows_by_users_id)
            .service(get_workflow_by_id)
            .service(create_workflow)
            .service(delete_workflow_by_id),
    );
}
