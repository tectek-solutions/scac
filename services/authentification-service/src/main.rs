use query::{get_auth_service, get_auth_service_by_id, add_auth_service, update_auth_service, delete_auth_service};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;

mod query;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    get_auth_service();
    // Retrieve and validate environment variables
    let address = match env::var("BINDING_ADDRESS") {
        Ok(addr) => addr,
        Err(_) => {
            eprintln!("Error: BINDING_ADDRESS environment variable is not set.");
            std::process::exit(1);
        }
    };

    let port = match env::var("BINDING_PORT") {
        Ok(port_str) => port_str.parse::<u16>().unwrap_or_else(|_| {
            eprintln!("Error: BINDING_PORT must be a valid u16 integer.");
            std::process::exit(1);
        }),
        Err(_) => {
            eprintln!("Error: BINDING_PORT environment variable is not set.");
            std::process::exit(1);
        }
    };

    // Start the server
    HttpServer::new(|| App::new().service(health))
        .bind((address.as_str(), port))?
        .run()
        .await
}
