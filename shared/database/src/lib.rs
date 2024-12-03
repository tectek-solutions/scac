pub mod schema;
pub mod model;
pub mod user_tokens_model;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::model::{NewUser, User};
use self::user_tokens_model::{NewUserTokens, UserTokens};

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

pub fn create_user_tokens(conn: &mut PgConnection, user_id: i32, auth_service_id: i32, access_token: &str, refresh_token: Option<&str>, expires_at: chrono::NaiveDateTime, created_at: Option<chrono::NaiveDateTime>, updated_at: Option<chrono::NaiveDateTime>) -> UserTokens {
    use crate::schema::user_tokens;

    let new_user_token = NewUserTokens {
        user_id,
        auth_service_id,
        access_token,
        refresh_token,
        expires_at,
        created_at,
        updated_at,
    };

    diesel::insert_into(user_tokens::table)
        .values(&new_user_token)
        .returning(UserTokens::as_returning())
        .get_result(conn)
        .expect("Error saving new user token")
}