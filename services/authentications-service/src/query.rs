use actix_web::web;
use database;
use database::model::Authentication;
use diesel::prelude::*;

pub fn list_authentications_query(
    database: &web::Data<database::Database>,
) -> Result<Option<Vec<Authentication>>, diesel::result::Error> {
    use database::schema::authentications::dsl::*;

    let mut database_connection = database.get_connection();
    let result = authentications.load::<Authentication>(&mut database_connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting Authentications: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_authentication_by_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Authentication>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match Authentication::read(&mut database_connection, search_id) {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting Authentication: {:?}", err);
            Err(err)
        }
    }
}
