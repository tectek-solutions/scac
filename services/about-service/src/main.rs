use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, get, Responder, HttpResponse};
use dotenv::dotenv;
use std::env;
use utoipa::OpenApi;
use utoipa_swagger_ui::{Config, SwaggerUi};
use serde_json;

use database::{self, model::{Authentication, Api, Action, Reaction}, schema::{apis::dsl::*, authentications::dsl::*, actions::dsl::*, reactions::dsl::*}};
use diesel::prelude::*;

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
        .allow_any_origin()
        .allowed_methods(vec!["POST", "GET", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![
            actix_web::http::header::CONTENT_TYPE,
            actix_web::http::header::AUTHORIZATION,
            actix_web::http::header::ACCEPT,
        ])
        .supports_credentials()
}


#[utoipa::path(
    get,
    path = "/about.json",
    tag = "about",
    responses(
        (status = 200, description = "Successful response"),
        (status = 500, description = "Internal server error")
    )
)]
#[get("/about.json")]
async fn about(
    db: web::Data<database::Database>,
) -> impl Responder {
    let mut database_connection = db.get_connection();

    let mut response_json = serde_json::Map::new();

    let client_url = env::var("CLIENT_URL").unwrap_or_else(|_| {
        eprintln!("Error: CLIENT_URL environment variable is not set.");
        std::process::exit(1);
    });

    // Adding client information
    response_json.insert(
        "client".to_string(),
        serde_json::json!({
            "host": client_url,
        }),
    );

    response_json.insert(
        "server".to_string(),
        serde_json::json!({
            "current_time": chrono::Utc::now().timestamp(),
        }),
    );

    let mut server_json = serde_json::Map::new();
    server_json.insert("current_time".to_string(), serde_json::json!(chrono::Utc::now().timestamp()));

    let authentications_list = match authentications.load::<Authentication>(&mut database_connection) {
        Ok(list) => list,
        Err(_) => {
            return HttpResponse::InternalServerError().finish();
        }
    };

    let mut apis_json = Vec::new();

    for authentication in authentications_list {
        let apis_list = match apis.filter(authentications_id.eq(authentication.id)).load::<Api>(&mut database_connection) {
            Ok(list) => list,
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        };

        for api in apis_list {
            let mut api_json = serde_json::Map::new();
            api_json.insert("name".to_string(), serde_json::json!(api.name));

            // Load actions
            let actions_list = match actions.filter(database::schema::actions::apis_id.eq(api.id)).load::<Action>(&mut database_connection) {
                Ok(list) => list,
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            };

            let actions_json: Vec<_> = actions_list
                .iter()
                .map(|action| {
                    serde_json::json!({
                        "name": action.name,
                        "description": action.description,
                    })
                })
                .collect();
            api_json.insert("actions".to_string(), serde_json::json!(actions_json));

            // Load reactions
            let reactions_list = match reactions.filter(database::schema::reactions::apis_id.eq(api.id)).load::<Reaction>(&mut database_connection) {
                Ok(list) => list,
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            };

            let reactions_json: Vec<_> = reactions_list
                .iter()
                .map(|reaction| {
                    serde_json::json!({
                        "name": reaction.name,
                        "description": reaction.description,
                    })
                })
                .collect();
            api_json.insert("reactions".to_string(), serde_json::json!(reactions_json));

            apis_json.push(api_json);
        }
    }

    server_json.insert("apis".to_string(), serde_json::json!(apis_json));
    response_json.insert("server".to_string(), serde_json::json!(server_json));

    HttpResponse::Ok().json(response_json)
}


#[derive(OpenApi)]
#[openapi(
    tags((name = "about", description = "Information about the API")),
    paths(
        about,
    ),
)]
struct ApiDoc;

fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/about")
            .service(about)
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logging_and_env();

    let (address, port) = get_server_config();

    let database_url = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let db = web::Data::new(database::Database::new(&database_url));

    HttpServer::new(move || {
        let cors = configure_cors();
        let config = Config::new(vec!["/actions/api-docs/openapi.json"]);
        let swagger = SwaggerUi::new("/actions/swagger-ui/{_:.*}")
            .url("/actions/api-docs/openapi.json", ApiDoc::openapi())
            .config(config);

        App::new()
            .app_data(db.clone())
            .configure(crate::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(swagger)
    })
    .bind((address.as_str(), port))?
    .run()
    .await
}
