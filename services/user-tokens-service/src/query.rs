use actix_web::web;
use chrono::NaiveDateTime;
use database::model::{NewUserTokens, UpdateUserTokens, UserTokens};
use diesel::prelude::*;

pub fn list_user_tokens_by_user_id_query(
    db: &web::Data<database::Database>,
    find_user_id: i32,
) -> Result<Option<Vec<UserTokens>>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    let result = user_tokens
        .filter(user_id.eq(find_user_id))
        .select(UserTokens::as_select())
        .load::<UserTokens>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting users: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_user_token_by_id_query(
    db: &web::Data<database::Database>,
    find_token_id: i32,
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    let result: Result<Option<UserTokens>, diesel::result::Error> = user_tokens
        .find(find_token_id)
        .select(UserTokens::as_select())
        .first::<UserTokens>(&mut connection)
        .optional();
    
    match result {
        Ok(result) => Ok(result),
        Err(err) => {
            eprintln!("Error getting users: {:?}", err);
            Err(err)
        }
    }
}

pub fn create_user_token(
    new_user_id: i32,
    new_auth_service_id: i32,
    new_access_token: &str,
    new_refresh_token: Option<&str>,
    new_expires_at: NaiveDateTime,
    new_created_at: Option<NaiveDateTime>,
    new_updated_at: Option<NaiveDateTime>,
    db: &web::Data<database::Database>,
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();

    let new_user_tokens = NewUserTokens {
        user_id: new_user_id,
        auth_service_id: new_auth_service_id,
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_at: new_expires_at,
        created_at: new_created_at,
        updated_at: new_updated_at,
    };

    let result = diesel::insert_into(user_tokens)
        .values(&new_user_tokens)
        .get_result::<UserTokens>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error adding user token: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_user_token(
    token_id: i32,
    update_user_id: Option<i32>,
    update_auth_service_id: Option<i32>,
    update_access_token: Option<&str>,
    update_refresh_token: Option<&str>,
    update_expires_at: Option<NaiveDateTime>,
    update_created_at: Option<NaiveDateTime>,
    update_updated_at: Option<NaiveDateTime>,
    db: &web::Data<database::Database>,
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();

    let update_user_tokens = UpdateUserTokens {
        user_id: update_user_id,
        auth_service_id: update_auth_service_id,
        access_token: update_access_token,
        refresh_token: update_refresh_token,
        expires_at: update_expires_at,
        created_at: update_created_at,
        updated_at: update_updated_at,
    };

    diesel::update(user_tokens.find(token_id))
        .set(&update_user_tokens)
        .get_result::<UserTokens>(&mut connection)
        .map(Some)
        .or_else(|err| {
            eprintln!("Error updating user token: {:?}", err);
            Err(err)
        })
}

pub fn delete_user_token(
    token_id: i32,
    db: &web::Data<database::Database>,
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    diesel::delete(user_tokens.find(token_id))
        .get_result::<UserTokens>(&mut connection)
        .map(Some)
        .or_else(|err| {
            eprintln!(
                "Error deleting user token with ID {:?}: {:?}",
                token_id, err
            );
            Err(err)
        })
}
