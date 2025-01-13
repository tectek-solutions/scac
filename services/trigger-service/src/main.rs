pub mod handler;
pub mod query;
pub mod worker;

use actix_web::{web, middleware::Logger, App, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
use tokio::time::{interval, Duration};
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
use log::{info, warn, error};

use database;
use cache;

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
        .allowed_methods(vec!["POST", "GET", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .supports_credentials()
}

#[derive(OpenApi)]
#[openapi(
    tags((name = "triggers", description = "Triggers")),
    paths(
        handler::list_triggers_by_worflows_id,
        handler::get_trigger_by_id,
    ),
)]
struct ApiDoc;

async fn run_background_job(db: web::Data<database::Database>) {
    let mut interval = interval(Duration::from_secs(5));
    loop {
        interval.tick().await;

        let workflows = match query::list_workflows(&db) {
            Ok(Some(workflows)) => workflows,
            Ok(None) => {
                warn!("No workflows found");
                continue;
            }
            Err(err) => {
                error!("Error getting workflows: {:?}", err);
                continue;
            }
        };

        for workflow in workflows {
            let worker = worker::Worker::new(&db, workflow);
            match worker.run().await {
                Ok(_) => info!("Worker ran successfully"),
                Err(err) => error!("Error running worker: {:?}", err),
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging_and_env();

    let (address, port) = get_server_config();

    let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db = web::Data::new(database::Database::new(&database_url));

    let cache_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cache = web::Data::new(cache::Cache::new(&cache_url));

    // Spawn the background job
    let db_clone = db.clone();
    tokio::spawn(async move {
        run_background_job(db_clone).await;
    });

    HttpServer::new(move || {
        let cors = configure_cors();
        let config = Config::new(vec!["/triggers/api-docs/openapi.json"]);
        let swagger = SwaggerUi::new("/triggers/swagger-ui/{_:.*}")
            .url("/triggers/api-docs/openapi.json", ApiDoc::openapi())
            .config(config);

        App::new()
            .app_data(db.clone())
            .app_data(cache.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(swagger)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
