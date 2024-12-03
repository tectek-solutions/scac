pub mod handler;
pub mod query;

use actix_cors::Cors;
use actix_web::{web, middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use std::env;
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

use database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging_and_env();

    let (address, port) = get_server_config();

    let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db = web::Data::new(database::Database::new(&database_url));

    #[derive(OpenApi)]
    #[openapi(
        tags((name = "users-service", description = "Users service")),
        modifiers(&SecurityAddon),
        paths(
            handler::login,
            handler::register,
            handler::get_user_handler,
            handler::get_user_profile_handler,
            handler::update_user_handler,
            handler::delete_user_handler
        ),
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        let cors = configure_cors();

        App::new()
            .app_data(db.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
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
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
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
