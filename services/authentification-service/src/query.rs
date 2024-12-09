use actix_web::web;
use database;
use database::model::{Authentification, NewAuthentification};
use diesel::prelude::*;

pub fn get_authentifications(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Authentification>>, diesel::result::Error> {
    use database::schema::authentification::dsl::*;

    let mut connection = db.get_connection();
    let result = authentification.load::<Authentification>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting authentifications: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_authentification_by_id(
    db: &web::Data<database::Database>,
    authentification_id: i32,
) -> Result<Option<Authentification>, diesel::result::Error> {
    use database::schema::authentification::dsl::*;

    let mut connection = db.get_connection();

    match authentification
        .find(authentification_id)
        .select(Authentification::as_select())
        .first::<Authentification>(&mut connection)
        .optional()
    {
        Ok(Some(Authentification)) => Ok(Some(Authentification)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting authentification with ID {:?}: {:?}",
                authentification_id, err
            );
            Err(err)
        }
    }
}

pub fn add_authentification(
    db: &web::Data<database::Database>,
    name: String,
    auth_url: String,
    token_url: String,
    client_id: String,
    client_secret: String,
) -> Result<Option<Authentification>, diesel::result::Error> {
    use database::schema::authentification;

    let mut connection = db.get_connection();

    let new_authentification = NewAuthentification {
        name: &name,
        auth_url: &auth_url,
        token_url: &token_url,
        client_id: &client_id,
        client_secret: &client_secret,
    };

    match diesel::insert_into(authentification::table)
        .values(&new_authentification)
        .returning(Authentification::as_returning())
        .get_result::<Authentification>(&mut connection)
    {
        Ok(authentification) => Ok(Some(authentification)),
        Err(err) => {
            eprintln!("Error adding authentification: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_authentification(
    db: &web::Data<database::Database>,
    authentification_id: i32,
    new_name: String,
    new_auth_url: String,
    new_token_url: String,
    new_client_id: String,
    new_client_secret: String,
) -> Result<Option<Authentification>, diesel::result::Error> {
    use database::schema::authentification::dsl::*;

    let mut connection = db.get_connection();
    match diesel::update(authentification.find(authentification_id))
        .set((
            name.eq(new_name.clone()),
            auth_url.eq(new_auth_url.clone()),
            token_url.eq(new_token_url.clone()),
            client_id.eq(new_client_id.clone()),
            client_secret.eq(new_client_secret.clone()),
        ))
        .returning(Authentification::as_returning())
        .get_result::<Authentification>(&mut connection)
    {
        Ok(Authentification) => Ok(Some(Authentification)),
        Err(err) => {
            eprintln!(
                "Error updating authentification with ID {:?}: {:?}",
                authentification_id, err
            );
            Ok(None)
        }
    }
}

pub fn delete_authentification(
    db: &web::Data<database::Database>,
    authentification_id: i32,
) -> Result<Option<Authentification>, diesel::result::Error> {
    use database::schema::authentification::dsl::*;

    let mut connection = db.get_connection();
    match diesel::delete(authentification.find(authentification_id))
        .get_result::<Authentification>(&mut connection)
    {
        Ok(Authentification) => Ok(Some(Authentification)),
        Err(err) => {
            eprintln!(
                "Error deleting authentification with ID {:?}: {:?}",
                authentification_id, err
            );
            Err(err)
        }
    }
}
