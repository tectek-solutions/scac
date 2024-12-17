use actix_web::web;
use database;
use database::model::Action;
use diesel::prelude::*;

pub fn list_actions_by_api_service_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Action>>, diesel::result::Error> {
    use database::schema::actions::dsl::*;

    let mut database_connection = database.get_connection();
    let result = actions
        .filter(api_id.eq(search_id))
        .select(Action::as_select())
        .load::<Action>(&mut database_connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting actions: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_action_by_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Action>, diesel::result::Error> {
    let mut database_connection = database.get_connection();
    match Action::read(&mut database_connection, search_id) {
        Ok(action) => Ok(Some(action)),
        Err(err) => {
            eprintln!("Error getting action: {:?}", err);
            Err(err)
        }
    }
}
