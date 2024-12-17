use actix_web::web;
use database;
use database::model::Reaction;
use diesel::prelude::*;

pub fn list_reactions_by_api_service_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Reaction>>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut database_connection = database.get_connection();
    let result = reactions
        .filter(api_id.eq(search_id))
        .select(Reaction::as_select())
        .load::<Reaction>(&mut database_connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting reactions: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_reaction_by_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Reaction>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match Reaction::read(&mut database_connection, search_id) {
        Ok(reaction) => Ok(Some(reaction)),
        Err(err) => {
            eprintln!("Error getting reaction: {:?}", err);
            Err(err)
        }
    }
}
