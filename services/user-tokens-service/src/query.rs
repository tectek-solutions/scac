use actix_web::web;
use database::model::{UserToken, CreateUserToken, UpdateUserToken};
use diesel::prelude::*;

pub fn list_user_tokens_by_user_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<UserToken>>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut database_connection = database.get_connection();
    let result = user_tokens
        .filter(users_id.eq(search_id))
        .select(UserToken::as_select())
        .load::<UserToken>(&mut database_connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting users: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_user_token_by_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<UserToken>, diesel::result::Error> {
    let mut database_connection = database.get_connection();
    match UserToken::read(&mut database_connection, search_id) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error getting user token: {:?}", err);
            Err(err)
        }
    }
}

pub fn create_user_token(
    database: &web::Data<database::Database>,
    new_user: CreateUserToken,
) -> Result<Option<UserToken>, diesel::result::Error> {

    let mut database_connection = database.get_connection();

    match UserToken::create(&mut database_connection, new_user) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error creating user token: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_user_token(
    database: &web::Data<database::Database>,
    update_id: i32,
    update_user_token: UpdateUserToken,
) -> Result<Option<UserToken>, diesel::result::Error> {
    let mut database_connection = database.get_connection();
    match UserToken::update(&mut database_connection, update_id, update_user_token) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error updating user token: {:?}", err);
            Err(err)
        }
    }
}

pub fn delete_user_token(
    database: &web::Data<database::Database>,
    delete_id: i32,
) -> Result<Option<UserToken>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match UserToken::delete(&mut database_connection, delete_id) {
        Ok(size) => {
            if size > 0 {
                Ok(None)
            } else {
                Ok(None)
            }
        }
        Err(err) => {
            eprintln!("Error deleting user token: {:?}", err);
            Err(err)
        }
    }
}
