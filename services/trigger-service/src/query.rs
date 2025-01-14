use actix_web::web;
use database;
use database::model::{Trigger, UserToken, Workflow};
use diesel::prelude::*;

pub fn list_triggers_by_worflows_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Trigger>>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut database_connection = database.get_connection();
    let result = triggers
        .filter(workflows_id.eq(search_id))
        .select(Trigger::as_select())
        .load::<Trigger>(&mut database_connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting triggers: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_trigger_by_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Trigger>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match Trigger::read(&mut database_connection, search_id) {
        Ok(trigger) => Ok(Some(trigger)),
        Err(err) => {
            eprintln!("Error getting trigger: {:?}", err);
            Err(err)
        }
    }
}

pub fn list_workflows(
    database: &web::Data<database::Database>,
) -> Result<Option<Vec<Workflow>>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut database_connection = database.get_connection();
    let result: Result<Option<Vec<Workflow>>, diesel::result::Error> = workflows
        .select(Workflow::as_select())
        .load::<Workflow>(&mut database_connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting workflows: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_user_token_by_authentication_by_user_id_query(
    database: &web::Data<database::Database>,
    authentication_id: i32,
    user_id: i32,
) -> Result<Option<UserToken>, diesel::result::Error> {
    use database::schema::user_tokens::dsl::*;

    let mut database_connection: diesel::r2d2::PooledConnection<
        diesel::r2d2::ConnectionManager<PgConnection>,
    > = database.get_connection();
    let result = user_tokens
        .filter(users_id.eq(user_id))
        .filter(authentications_id.eq(authentication_id))
        .select(UserToken::as_select())
        .first::<UserToken>(&mut database_connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting user token: {:?}", err);
            Err(err)
        }
    }
}
