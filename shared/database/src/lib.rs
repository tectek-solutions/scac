pub mod schema;
pub mod model;


use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use self::model::{NewUser, User};

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