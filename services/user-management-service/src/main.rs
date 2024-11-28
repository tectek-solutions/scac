mod handler;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, App, HttpServer};
use dotenv::dotenv;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_actix_web::AppExt;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
    #[openapi(paths(handler::login, handler::register, handler::user_by_id))]
    struct ApiDoc;

    println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());

    struct SecurityAddon;

    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
            let components = openapi.components.as_mut().unwrap(); // we can unwrap safely since there already is components registered.
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .openapi_service(|api| Redoc::with_url("/redoc", api))
        .openapi_service(|api| {
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", api)
        })
        .map(|app| app.service(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc")))
        .openapi_service(|api| {
            Scalar::with_url("/scalar", api)
        })
        .into_app()
        .configure(handler::config)
        .wrap(cors)
        .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}