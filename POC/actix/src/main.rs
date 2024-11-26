#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde_derive;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod db;
use db::*;

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
struct CreateTodo {
    name: String,
    description: String,
}

async fn get_todos() -> impl Responder {
    let todos = _db_get_todos();
    HttpResponse::Ok().json(todos)
}

async fn get_todo_by_id(path: web::Path<String>) -> impl Responder {
    let todo = _db_get_todo_by_id(path.into_inner());
    HttpResponse::Ok().json(todo)
}

async fn create_todo(todo: web::Json<CreateTodo>) -> impl Responder {
    let result = _db_create_todo(todo.0.name.as_ref(), todo.0.description.as_ref());

    HttpResponse::Ok().json(result)
}

async fn update_todo(path: web::Path<String>, todo: web::Json<CreateTodo>) -> impl Responder {
    let result = _db_update_todo(path.into_inner(), todo.0.name.as_ref(), todo.0.description.as_ref());

    HttpResponse::Ok().json(result)
}

async fn delete_todo(path: web::Path<String>) -> impl Responder {
    let result = _db_delete_todo(path.into_inner());

    if result == 0 {
        return HttpResponse::NotFound().finish();
    }
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/todos", web::get().to(get_todos))
            .route("/todos/{id}", web::get().to(get_todo_by_id))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}