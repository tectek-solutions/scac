use diesel::prelude::*;
use database::user_tokens_model::*;
use database::*;
use chrono::NaiveDateTime;


pub fn get_user_tokens() {
    use database::schema::user_tokens::dsl::*;

    let connection = &mut etablish_connection();
    let results = user_tokens
        .select(UserTokens::as_select())
        .load(connection)
        .expect("Error loading user tokens");
    println!("Displaying {} user tokens", results.len());
    for user_token in results {
        println!("{:?}", user_token.user_id);
        println!("{:?}", user_token.auth_service_id);
    }
}

pub fn get_user_tokens_by_id(token_id: i32) {
    use database::schema::user_tokens::dsl::*;

    let connection = &mut etablish_connection();
    let user_token = user_tokens
        .find(token_id)
        .select(UserTokens::as_select())
        .first(connection)
        .optional();
    match user_token {
        Ok(Some(user_token)) => {
            println!("Found user token {:?}, with id {:?}", user_token.user_id, token_id);
            println!("Found auth_service_id {:?} for token_id {:?}", user_token.auth_service_id, token_id);
        },
        Ok(None) => println!("No user token found with id {:?}", token_id),
        Err(err) => println!("Error finding user token: {:?}", err),
    }
}

pub fn get_user_tokens_by_user_id(user_id_param: i32) {
    use database::schema::user_tokens::dsl::*;

    let connection = &mut etablish_connection();
    let tokens = user_tokens
        .filter(user_id.eq(user_id_param))
        .select(UserTokens::as_select())
        .load(connection)
        .expect("Error loading user tokens");
    println!("Displaying {} user tokens for user_id {}", tokens.len(), user_id_param);
    for user_token in tokens {
        println!("{:?}", user_token.user_id);
        println!("{:?}", user_token.auth_service_id);
    }
}

pub fn get_user_tokens_by_auth_service_id(auth_service_id_param: i32) {
    use database::schema::user_tokens::dsl::*;

    let connection = &mut etablish_connection();
    let tokens = user_tokens
        .filter(auth_service_id.eq(auth_service_id))
        .select(UserTokens::as_select())
        .load(connection)
        .expect("Error loading user tokens");
    println!("Displaying {} user tokens for auth_service_id {}", tokens.len(), auth_service_id_param);
    for user_token in tokens {
        println!("{:?}", user_token.user_id);
        println!("{:?}", user_token.auth_service_id);
    }
}

pub fn add_user_tokens(user_id: i32, auth_service_id: i32, access_token: &str, refresh_token: Option<&str>, expires_at: chrono::NaiveDateTime, created_at: Option<chrono::NaiveDateTime>, updated_at: Option<chrono::NaiveDateTime>) {
    let connection = &mut etablish_connection();
    let user_token = create_user_tokens(connection, user_id, auth_service_id, access_token, refresh_token, expires_at, created_at, updated_at);
    println!("Saved user token {:?}", user_token.user_id);
    println!("Saved auth_service_id {:?}", user_token.auth_service_id);
    println!("Saved access_token {:?}", user_token.access_token);
    println!("Saved refresh_token {:?}", user_token.refresh_token);
    println!("Saved expires_at {:?}", user_token.expires_at);
    println!("Saved created_at {:?}", user_token.created_at);
    println!("Saved updated_at {:?}", user_token.updated_at);
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
) {
    use database::schema::user_tokens::dsl::*;
    let connection = &mut etablish_connection();

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

    // Appliquer les changements à la requête
    let updated_rows = diesel::update(user_tokens.find(token_id))
        .set(changes)
        .execute(connection)
        .expect("Error updating user token");

    println!(
        "Updated {} user token(s) with token_id {:?}",
        updated_rows, token_id
    );
}

pub fn delete_user_tokens(token_id: i32) {
    use database::schema::user_tokens::dsl::*;

    let connection = &mut etablish_connection();
    let deleted_user_token: UserTokens = diesel::delete(user_tokens.find(token_id))
        .get_result(connection)
        .expect("Error deleting user token");

    println!("Deleted user token {:?}", deleted_user_token.user_id);
    println!("Deleted auth_service_id {:?}", deleted_user_token.auth_service_id);
    println!("Deleted access_token {:?}", deleted_user_token.access_token);
    println!("Deleted refresh_token {:?}", deleted_user_token.refresh_token);
    println!("Deleted expires_at {:?}", deleted_user_token.expires_at);
    println!("Deleted created_at {:?}", deleted_user_token.created_at);
    println!("Deleted updated_at {:?}", deleted_user_token.updated_at);
}


