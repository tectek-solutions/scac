use actix_web::web;
use database::model::{NewApiServices, ApiServices};
use database;
use diesel::prelude::*;

pub fn get_api_services(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<ApiServices>>, diesel::result::Error> {
    use database::schema::api_services::dsl::*;

    let mut connection = db.get_connection();
    let result = api_services.load::<ApiServices>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting api services: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_api_services_by_id(
    db: &web::Data<database::Database>,
    api_service_id: i32,
) -> Result<Option<ApiServices>, diesel::result::Error> {
    use database::schema::api_services::dsl::*;

    let mut connection = db.get_connection();

    match api_services
        .find(api_service_id)
        .select(ApiServices::as_select())
        .first::<ApiServices>(&mut connection)
        .optional()
    {
        Ok(Some(api_service)) => Ok(Some(api_service)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting api service with ID {:?}: {:?}",
                api_service_id, err
            );
            Err(err)
        }
    }
}

pub fn create_api_service(
    db: &web::Data<database::Database>,
    authserviceid: i32,
    apiname: String,
    apibase_url: String,
) -> Result<Option<ApiServices>, diesel::result::Error> {
    use database::schema::api_services;

    let new_api_service = NewApiServices {
        auth_service_id: authserviceid,
        name: &apiname,
        base_url: &apibase_url,
        created_at: Some(chrono::Utc::now().naive_utc()),
        updated_at: Some(chrono::Utc::now().naive_utc()),
    };

    let mut connection = db.get_connection();
    match diesel::insert_into(api_services::table)
        .values(&new_api_service)
        .get_result::<ApiServices>(&mut connection)
        .optional()
    {
        Ok(new_api_service) => Ok(new_api_service),
        Err(err) => {
            eprintln!("Error creating api service: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_api_service(
    db: &web::Data<database::Database>,
    api_service_id: i32,
    _name: String,
    _base_url: String,
) -> Result<Option<ApiServices>, diesel::result::Error> {
    use database::schema::api_services::dsl::*;

    let mut connection = db.get_connection();
    match diesel::update(api_services.find(api_service_id))
        .set((
            name.eq(_name),
            base_url.eq(_base_url),
            updated_at.eq(Some(chrono::Utc::now().naive_utc())),
        ))
        .get_result::<ApiServices>(&mut connection)
        .optional()
    {
        Ok(updated_api_service) => Ok(updated_api_service),
        Err(err) => {
            eprintln!(
                "Error updating api service with ID {:?}: {:?}",
                api_service_id, err
            );
            Err(err)
        }
    }
}

pub fn delete_api_service(
    db: &web::Data<database::Database>,
    api_service_id: i32,
) -> Result<Option<ApiServices>, diesel::result::Error> {
    use database::schema::api_services::dsl::*;

    let mut connection = db.get_connection();
    let deleted_api_service = diesel::delete(api_services.find(api_service_id))
        .get_result::<ApiServices>(&mut connection)
        .optional();

    match deleted_api_service {
        Ok(deleted_api_service) => Ok(deleted_api_service),
        Err(err) => {
            eprintln!(
                "Error deleting api service with ID {:?}: {:?}",
                api_service_id, err
            );
            Err(err)
        }
    }
}