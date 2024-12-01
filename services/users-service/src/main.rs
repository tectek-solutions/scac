use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use utoipa::{Modify, OpenApi};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

mod handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging_and_env();

    #[derive(OpenApi)]
    #[openapi(
        tags((name = "users-service", description = "Users service")),
        modifiers(&SecurityAddon),
        paths(handler::login, handler::register, handler::user_by_id)
    )]
    struct ApiDoc;

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = configure_cors();

        App::new()
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .service(Redoc::with_url("/redoc", ApiDoc::openapi()))
            .service(
                RapiDoc::new("/api-docs/openapi.json")
                    .path("/rapidoc"),
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

fn setup_logging_and_env() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();
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
