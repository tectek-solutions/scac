use actix_web::web;
use database::model::Api;
use database;
use diesel::prelude::*;

pub fn list_api_services_by_authentication_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Api>>, diesel::result::Error> {
    use database::schema::apis::dsl::*;

    let mut connection = database.get_connection();
    let result = apis
        .filter(authentication_id.eq(search_id))
        .select(Api::as_select())
        .load::<Api>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting api services: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_api_service_by_id_query(
    database: &web::Data<database::Database>,
    api_service_id: i32,
) -> Result<Option<Api>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match Api::read(&mut database_connection, api_service_id) {
        Ok(api_service) => Ok(Some(api_service)),
        Err(err) => {
            eprintln!(
                "Error getting api service with ID {:?}: {:?}",
                api_service_id, err
            );
            Err(err)
        }
    }
}
