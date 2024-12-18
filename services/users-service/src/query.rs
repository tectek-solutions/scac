use actix_web::web;
use database;
use database::model::{User, CreateUser, UpdateUser};
use diesel::prelude::*;
use log;

pub fn get_user_by_id(
    database: &web::Data<database::Database>,
    user_id: i32,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();

    match users
        .find(user_id)
        .select(User::as_select())
        .first::<User>(&mut database_connection)
        .optional()
    {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(err) => {
            log::error!("Error getting user with ID {:?}: {:?}", user_id, err);
            Err(err)
        }
    }
}

pub fn get_user_by_email(
    database: &web::Data<database::Database>,
    user_email: &String,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();
    match users
        .filter(email.eq(user_email.clone()))
        .select(User::as_select())
        .first::<User>(&mut database_connection)
        .optional()
    {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(err) => {
            log::error!("Error getting user with email {:?}: {:?}", user_email, err);
            Err(err)
        }
    }
}

pub fn create_user(
    database: &web::Data<database::Database>,
    new_user: CreateUser,
) -> Result<Option<User>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match User::create(&mut database_connection, new_user) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            log::error!("Error creating user: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_user(
    database: &web::Data<database::Database>,
    update_id: i32,
    update_user: UpdateUser,
) -> Result<Option<User>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match User::update(&mut database_connection, update_id, update_user) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            log::error!("Error updating user: {:?}", err);
            Err(err)
        }
    }
}

pub fn delete_user(
    database: &web::Data<database::Database>,
    user_id: i32,
) -> Result<Option<()>, diesel::result::Error> {
    let mut database_connection = database.get_connection();
   
    match User::delete(&mut database_connection, user_id) {
        Ok(size) => {
            if size == 0 {
                Ok(None)
            } else {
                Ok(Some(()))
            }
        }
        Err(err) => {
            log::error!("Error deleting user: {:?}", err);
            Err(err)
        }
    }
}
