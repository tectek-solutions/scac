use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;
use utoipa::{Modify, OpenApi};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging and environment variables
    setup_logging_and_env();

    // Retrieve server configuration
    let (address, port) = get_server_config();

    // Define OpenAPI documentation
    #[derive(OpenApi)]
    #[openapi(
        tags((name = "users-service", description = "Users service")),
        modifiers(&SecurityAddon),
        paths(handler::login, handler::register, handler::get_user_handler, handler::get_user_profile_handler, handler::update_user_handler, handler::delete_user_handler),
    )]
    struct ApiDoc;

    println!("🚀 Server started successfully on {}:{}", address, port);

    // Start the server
    HttpServer::new(move || {
        let cors = configure_cors();

        App::new()
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(health)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}

fn setup_logging_and_env() {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
}

fn get_server_config() -> (String, u16) {
    let address = env::var("BINDING_ADDRESS").unwrap_or_else(|_| {
        eprintln!("Error: BINDING_ADDRESS environment variable is not set.");
        std::process::exit(1);
    });

    let port = env::var("BINDING_PORT")
        .unwrap_or_else(|_| {
            eprintln!("Error: BINDING_PORT environment variable is not set.");
            std::process::exit(1);
        })
        .parse::<u16>()
        .unwrap_or_else(|_| {
            eprintln!("Error: BINDING_PORT must be a valid u16 integer.");
            std::process::exit(1);
        });

    (address, port)
}

fn configure_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:8000")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
        ])
        .supports_credentials()
}

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Header(
                        utoipa::openapi::security::ApiKeyValue::new("todo_apikey"),
                    ),
                ),
            );
        }
    }
}

#[actix_web::get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("OK")
}
