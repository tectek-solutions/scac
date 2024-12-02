pub mod schema;
pub mod model;
pub mod auth_service_models;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::model::{NewUser, User};
use self::auth_service_models::{NewAuthService, AuthService};

pub fn etablish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, username: &str, email: &str, password_hash: &str) -> User {
    use crate::schema::users;

    let new_user = NewUser {
        username,
        email,
        password_hash,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new user")
}

pub fn create_auth_service(conn: &mut PgConnection, name: &str, auth_url: &str, token_url: &str, client_id: &str, client_secret: &str) -> AuthService {
    use crate::schema::auth_service;

    let new_auth_service = NewAuthService {
        name,
        auth_url,
        token_url,
        client_id,
        client_secret,
    };

    diesel::insert_into(auth_service::table)
        .values(&new_auth_service)
        .get_result(conn)
        .expect("Error saving new auth service")
}