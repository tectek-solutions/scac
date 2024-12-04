use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::env;
use chrono::{NaiveDateTime, Utc};
use query::{get_user_tokens, get_user_tokens_by_id, get_user_tokens_by_user_id, get_user_tokens_by_auth_service_id, add_user_tokens, update_user_tokens};
mod query;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let now = Utc::now().naive_utc();
    // add_user_tokens(1, 3, "access_token", Some("refresh_token"), now, Some(now), Some(now));

    update_user_tokens(3, Some(3), None, Some("new_access_token"), None, None, None, None);
    get_user_tokens();
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
