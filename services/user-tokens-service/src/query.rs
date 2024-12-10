use diesel::prelude::*;
use database::model::{NewUserTokens, UserTokens, UpdateUserTokens};
use chrono::NaiveDateTime;
use actix_web::web;



pub fn get_user_tokens(db: &web::Data<database::Database>) -> Result<Option<Vec<UserTokens>>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    let result = user_tokens.load::<UserTokens>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting user tokens: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_user_tokens_by_id(
    token_id: i32,
    db: &web::Data<database::Database>
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    match user_tokens
        .find(token_id)
        .select(UserTokens::as_select())
        .first::<UserTokens>(&mut connection)
        .optional()
    {
        Ok(Some(user_token)) => Ok(Some(user_token)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting user token with ID {:?}: {:?}", token_id, err);
            Err(err)
        }
    }
}

pub fn get_user_tokens_by_user_id(
    user_id_param: i32,
    db: &web::Data<database::Database>
) -> Result<Option<Vec<UserTokens>>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    match user_tokens
        .filter(user_id.eq(user_id_param))
        .select(UserTokens::as_select())
        .load::<UserTokens>(&mut connection)
    {
        Ok(user_token) => Ok(Some(user_token)),
        Err(err) => {
            eprintln!("Error getting user tokens with user_id {:?}: {:?}", user_id_param, err);
            Err(err)
        }
    }
}

pub fn get_user_tokens_by_auth_service_id(
    auth_service_id_param: i32,
    db: &web::Data<database::Database>
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    match user_tokens
        .filter(auth_service_id.eq(auth_service_id_param))
        .select(UserTokens::as_select())
        .first::<UserTokens>(&mut connection)
        .optional()
    {
        Ok(Some(user_token)) => Ok(Some(user_token)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting user token with auth_service_id {:?}: {:?}", auth_service_id_param, err);
            Err(err)
        }
    }
}

pub fn add_user_tokens(userid: i32, auth_serviceid: i32, accesstoken: &str, refreshtoken: Option<&str>, expiresat: chrono::NaiveDateTime, createdat: Option<chrono::NaiveDateTime>, updatedat: Option<chrono::NaiveDateTime>, db: &web::Data<database::Database>) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;
    let mut connection = db.get_connection();

    let new_user_tokens = NewUserTokens {
        user_id: userid,
        auth_service_id: auth_serviceid,
        access_token: accesstoken,
        refresh_token: refreshtoken,
        expires_at: expiresat,
        created_at: createdat,
        updated_at: updatedat,
    };
    match diesel::insert_into(user_tokens)
        .values(&new_user_tokens)
        .get_result::<UserTokens>(&mut connection)
    {
        Ok(user_token) => Ok(Some(user_token)),
        Err(err) => {
            eprintln!("Error adding user token: {:?}", err);
            Err(err)
        }
    }

}

pub fn update_user_tokens(
    token_id: i32,
    new_user_id: Option<i32>,
    new_auth_service_id: Option<i32>,
    new_access_token: Option<&str>,
    new_refresh_token: Option<&str>,
    new_expires_at: Option<NaiveDateTime>,
    new_created_at: Option<NaiveDateTime>,
    new_updated_at: Option<NaiveDateTime>,
    db: &web::Data<database::Database>
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;
    let mut connection = db.get_connection();

    // Crée une structure avec les valeurs optionnelles
    let changes = UpdateUserTokens {
        user_id: new_user_id,
        auth_service_id: new_auth_service_id,
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        expires_at: new_expires_at,
        created_at: new_created_at,
        updated_at: new_updated_at,
    };

    match diesel::update(user_tokens.find(token_id))
        .set(&changes)
        .get_result::<UserTokens>(&mut connection)
    {
        Ok(user_token) => Ok(Some(user_token)),
        Err(err) => {
            eprintln!("Error updating user token: {:?}", err);
            Err(err)
        }
    }
}

pub fn delete_user_tokens(
    token_id: i32,
    db: &web::Data<database::Database>
) -> Result<Option<UserTokens>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut connection = db.get_connection();
    match diesel::delete(user_tokens.find(token_id))
        .get_result::<UserTokens>(&mut connection)
    {
        Ok(user_token) => Ok(Some(user_token)),
        Err(err) => {
            eprintln!("Error deleting user token with ID {:?}: {:?}", token_id, err);
            Err(err)
        }
    }
}


