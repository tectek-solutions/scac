use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use std::env;
use tinytemplate::TinyTemplate;
use reqwest;

use database;
use cache;
use jwt;

use crate::query;

// ----------------------------
// Struct Definitions
// ----------------------------

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

#[derive(Serialize, Deserialize)]
struct UserTokenAuthenticationContext {
    client_id: String,
    state: String,
    redirect_uri: String,
}

#[derive(Deserialize, Serialize)]
struct UserTokenAuthenticationQuery {
    code: String,
    state: String,
}

#[derive(Deserialize, Serialize)]
struct UserTokenRefreshContext {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

// ----------------------------
// Helper Functions
// ----------------------------

fn get_authentication_url(authentication: database::model::Authentication, user_id: i32) -> String {
    let api_url = env::var("API_URL").expect("API_URL must be set");

    let redirect_uri = format!("{}/user-tokens/token/new", api_url);

    let state = format!("authentication_id={} user_id={}", authentication.id, user_id);

    let context = UserTokenAuthenticationContext {
        client_id: authentication.client_id,
        state: state,
        redirect_uri: redirect_uri,
    };

    let mut tt = TinyTemplate::new();
    match tt.add_template("url", &authentication.authentication_url) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Error adding template: {:?}", err);
            return "".to_string();
        }
    }
    
    let result = match tt.render("url", &context) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Error rendering template: {:?}", err);
            return "".to_string();
        }
    };

    result
}

// ----------------------------
// Handlers
// ----------------------------

#[utoipa::path(
    get,
    path = "/users/{user_id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "List of user tokens retrieved"),
        (status = 403, description = "Unauthorized", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/users/{user_id}")]
async fn list_user_tokens_by_user_id(
    db: web::Data<database::Database>,
    user_id: web::Path<i32>,
) -> impl Responder {
    match query::list_user_tokens_by_user_id_query(&db, user_id.into_inner()) {
        Ok(Some(user_tokens)) => HttpResponse::Ok().json(user_tokens),
        Ok(None) => ErrorResponse::NotFound("User tokens not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting user tokens: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user tokens".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/{id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "User token details retrieved"),
        (status = 404, description = "User token ID not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/{id}")]
async fn get_user_token_by_id(
    db: web::Data<database::Database>,
    id: web::Path<i32>,
) -> impl Responder {
    match query::get_user_token_by_id_query(&db, id.into_inner()) {
        Ok(Some(user_token)) => HttpResponse::Ok().json(user_token),
        Ok(None) => ErrorResponse::NotFound("User token not found".to_string())
            .to_response(actix_web::http::StatusCode::NOT_FOUND),
        Err(err) => {
            eprintln!("Error getting user token: {:?}", err);
            ErrorResponse::InternalServerError("Failed to get user token".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/url/authentications/{authentication_id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "User token URL retrieved"),
        (status = 404, description = "Authentification not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/url/authentications/{authentication_id}")]
async fn get_user_token_authentication_url_by_authentication_id(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    authentication_id: web::Path<i32>,
    request: HttpRequest,
) -> impl Responder {
    let jwt_token = match request.headers().get("Authorization") {
        Some(value) => value.to_str().unwrap_or("").to_string(),
        None => {
            return ErrorResponse::Unauthorized("No token provided".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let jwt_token = jwt_token.replace("Bearer ", "");

    if !jwt::verify_jwt(&cache, &jwt_token) {
        return ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let user_id = match jwt::get_user_id_by_jwt(&cache, &jwt_token) {
        Ok(Some(id)) => id,
        Ok(None) => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            return ErrorResponse::Unauthorized("Invalid token".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let authentication = match database::model::Authentication::read(&mut db.get_connection(), authentication_id.into_inner()) {
        Ok(authentication) => authentication,
        Err(err) => {
            eprintln!("Error getting authentification: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get authentification".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let url = get_authentication_url(authentication, user_id);

    HttpResponse::Ok().json(url)
}

#[utoipa::path(
    get,
    path = "/token/new",
    tag = "user-tokens",
    responses(
        (status = 200, description = "User token created"),
        (status = 404, description = "Authentification not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/token/new")]
async fn create_user_token(
    db: web::Data<database::Database>,
    query: web::Query<UserTokenAuthenticationQuery>,
) -> impl Responder {
    let code = &query.code;
    let state = &query.state;

    println!("Code: {:?}", code);
    println!("State: {:?}", state);

    let authentication_id = match state.split(" ").collect::<Vec<&str>>().get(0) {
        Some(value) => match value.split("=").collect::<Vec<&str>>().get(1) {
            Some(value) => match value.parse::<i32>() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error parsing authentication ID: {:?}", err);
                    return ErrorResponse::Unauthorized("Can't parse authentication ID".to_string())
                        .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
                }
            },
            None => {
                return ErrorResponse::Unauthorized("Can't split authentication ID".to_string())
                    .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
            }
        },
        None => {
            return ErrorResponse::Unauthorized("Can't split state".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let authentication = match database::model::Authentication::read(&mut db.get_connection(), authentication_id) {
        Ok(authentication) => authentication,
        Err(err) => {
            eprintln!("Error getting authentification: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get authentification".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user_id = match state.split(";").collect::<Vec<&str>>().get(1) {
        Some(value) => match value.split("=").collect::<Vec<&str>>().get(1) {
            Some(value) => match value.parse::<i32>() {
                Ok(value) => value,
                Err(err) => {
                    eprintln!("Error parsing user ID: {:?}", err);
                    return ErrorResponse::Unauthorized("Invalid state".to_string())
                        .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
                }
            },
            None => {
                return ErrorResponse::Unauthorized("Invalid state".to_string())
                    .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
            }
        },
        None => {
            return ErrorResponse::Unauthorized("Invalid state".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    match database::model::User::read(&mut db.get_connection(), user_id) {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Error getting user: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get user".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Code: {:?}", code);
    println!("State: {:?}", state);
    println!("Authentication ID: {:?}", authentication_id);
    println!("User ID: {:?}", user_id);

    let api_url = env::var("API_URL").expect("API_URL must be set");

    let redirect_uri = format!("{}/user-tokens/token/new", api_url);

    let url = authentication.refresh_token_url.clone();

    let params = [
        ("code", code.as_str()),
        ("client_id", authentication.client_id.as_str()),
        ("client_secret", authentication.client_secret.as_str()),
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_uri.as_str()),
        ("access_type", "offline"),
        ("prompt", "consent"),
        ("code_verifier", "challenge")
    ];

    let client = reqwest::Client::new();

    let authorization_header = format!("Basic {}", base64::encode(format!("{}:{}", authentication.client_id, authentication.client_secret)));

    let request = client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", authorization_header)
        .form(&params);

    println!("Request: {:?}", request);
    
    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("Error sending request: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to send request".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let text = match response.text().await   {
        Ok(text) => text,
        Err(err) => {
            eprintln!("Error getting text: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get text".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Text: {:?}", text);

    let json = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("Error parsing JSON: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to parse JSON".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let refresh_token = match json["refresh_token"].as_str() {
        Some(value) => value,
        None => "",
    };

    let user_token = database::model::CreateUserToken {
        users_id: user_id,
        authentications_id: authentication_id,
        access_token: json["access_token"].as_str().unwrap().to_string(),
        refresh_token: Some(refresh_token.to_string()),
        expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::seconds(json["expires_in"].as_i64().unwrap()),
    };

    match database::model::UserToken::create(&mut db.get_connection(), user_token) {
        Ok(user_token) => {
            let update_user_token = database::model::UpdateUserToken {
                access_token: Some(json["access_token"].as_str().unwrap().to_string()),
                refresh_token: Some(refresh_token.to_string()),
                expires_at: Some(chrono::Utc::now().naive_utc() + chrono::Duration::seconds(json["expires_in"].as_i64().unwrap())),
            };

            match database::model::UserToken::update(&mut db.get_connection(), user_token.id, update_user_token) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error updating user token: {:?}", err);
                    return ErrorResponse::InternalServerError("Failed to update user token".to_string())
                        .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
        Err(_) => {
            let user_token = database::model::CreateUserToken {
                users_id: user_id,
                authentications_id: authentication_id,
                access_token: json["access_token"].as_str().unwrap().to_string(),
                refresh_token: Some(refresh_token.to_string()),
                expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::seconds(json["expires_in"].as_i64().unwrap()),
            };

            match database::model::UserToken::create(&mut db.get_connection(), user_token) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("Error creating user token: {:?}", err);
                    return ErrorResponse::InternalServerError("Failed to create user token".to_string())
                        .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
                }
            }
        }
    }

    HttpResponse::Created().finish()
}

#[utoipa::path(
    get,
    path = "/authentications/{authentication_id}",
    tag = "user-tokens",
    responses(
        (status = 200, description = "User token URL retrieved"),
        (status = 404, description = "Authentification not found", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
#[get("/authentications/{authentication_id}")]
async fn get_user_token_by_authentication_id(
    db: web::Data<database::Database>,
    cache: web::Data<cache::Cache>,
    authentication_id: web::Path<i32>,
    request: HttpRequest,
) -> impl Responder {
    let jwt_token = match request.headers().get("Authorization") {
        Some(value) => value.to_str().unwrap_or("").to_string(),
        None => {
            return ErrorResponse::Unauthorized("No token provided".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let jwt_token = jwt_token.replace("Bearer ", "");

    if !jwt::verify_jwt(&cache, &jwt_token) {
        return ErrorResponse::Unauthorized("Invalid token".to_string())
            .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
    }

    let user_id = match jwt::get_user_id_by_jwt(&cache, &jwt_token) {
        Ok(Some(id)) => id,
        Ok(None) => {
            return ErrorResponse::Unauthorized("User not found".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
        Err(_) => {
            return ErrorResponse::Unauthorized("Invalid token".to_string())
                .to_response(actix_web::http::StatusCode::UNAUTHORIZED);
        }
    };

    let authentication_id = authentication_id.into_inner();

    match database::model::Authentication::read(&mut db.get_connection(), authentication_id) {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Error getting authentification: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get authentification".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user_token = match query::get_user_token_by_authentication_id_query(&db, authentication_id, user_id) {
        Ok(Some(user_token)) => user_token,
        Ok(None) => {
            return ErrorResponse::NotFound("User token not found".to_string())
                .to_response(actix_web::http::StatusCode::NOT_FOUND);
        }
        Err(err) => {
            eprintln!("Error getting user token: {:?}", err);
            return ErrorResponse::InternalServerError("Failed to get user token".to_string())
                .to_response(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    HttpResponse::Ok().json(user_token)
}

// ----------------------------
// Service Configuration
// ----------------------------

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-tokens")
            .service(list_user_tokens_by_user_id)
            .service(get_user_token_by_id)
            .service(get_user_token_authentication_url_by_authentication_id)
            .service(create_user_token)
            .service(get_user_token_by_authentication_id)
    );
}
