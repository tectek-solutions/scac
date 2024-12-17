use actix_web::web;
use database;
use database::model::Trigger;
use diesel::prelude::*;

pub fn list_triggers_by_worflows_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Trigger>>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut database_connection = database.get_connection();
    let result = triggers
        .filter(workflow_id.eq(search_id))
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
