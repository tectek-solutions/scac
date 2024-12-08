use actix_web::web;
use database;
use database::model::Authentification;
use diesel::prelude::*;

pub fn list_authentifications_query(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Authentification>>, diesel::result::Error> {
    use database::schema::authentifications::dsl::*;

    let mut connection = db.get_connection();
    let result = authentifications.load::<Authentification>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting authentifications: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_authentification_by_id_query(
    db: &web::Data<database::Database>,
    authentification_id: i32,
) -> Result<Option<Authentification>, diesel::result::Error> {
    use database::schema::authentifications::dsl::*;

    let mut connection = db.get_connection();

    match authentifications
        .find(authentification_id)
        .select(Authentification::as_select())
        .first::<Authentification>(&mut connection)
        .optional()
    {
        Ok(Some(authentification)) => Ok(Some(authentification)),
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

// pub fn create_authentification(
//     db: &web::Data<database::Database>,
//     name: String,
//     auth_url: String,
//     token_url: String,
//     client_id: String,
//     client_secret: String,
// ) -> Result<Option<Authentification>, diesel::result::Error> {
//     use database::schema::authentifications;

//     let mut connection = db.get_connection();

//     let new_authentification = NewAuthentification {
//         name: &name,
//         auth_url: &auth_url,
//         token_url: &token_url,
//         client_id: &client_id,
//         client_secret: &client_secret,
//     };

//     match diesel::insert_into(authentifications::table)
//         .values(&new_authentification)
//         .returning(Authentification::as_returning())
//         .get_result::<Authentification>(&mut connection)
//     {
//         Ok(authentification) => Ok(Some(authentification)),
//         Err(err) => {
//             eprintln!("Error adding authentification: {:?}", err);
//             Err(err)
//         }
//     }
// }

// pub fn update_authentification(
//     db: &web::Data<database::Database>,
//     authentification_id: i32,
//     new_name: String,
//     new_auth_url: String,
//     new_token_url: String,
//     new_client_id: String,
//     new_client_secret: String,
// ) -> Result<Option<Authentification>, diesel::result::Error> {
//     use database::schema::authentifications::dsl::*;

//     let mut connection = db.get_connection();
//     match diesel::update(authentifications.find(authentification_id))
//         .set((
//             name.eq(new_name.clone()),
//             auth_url.eq(new_auth_url.clone()),
//             token_url.eq(new_token_url.clone()),
//             client_id.eq(new_client_id.clone()),
//             client_secret.eq(new_client_secret.clone()),
//         ))
//         .returning(Authentification::as_returning())
//         .get_result::<Authentification>(&mut connection)
//     {
//         Ok(authentification) => Ok(Some(authentification)),
//         Err(err) => {
//             eprintln!(
//                 "Error updating authentification with ID {:?}: {:?}",
//                 authentification_id, err
//             );
//             Ok(None)
//         }
//     }
// }

// pub fn delete_authentification(
//     db: &web::Data<database::Database>,
//     authentification_id: i32,
// ) -> Result<Option<Authentification>, diesel::result::Error> {
//     use database::schema::authentifications::dsl::*;

//     let mut connection = db.get_connection();
//     match diesel::delete(authentifications.find(authentification_id))
//         .get_result::<Authentification>(&mut connection)
//     {
//         Ok(authentification) => Ok(Some(authentification)),
//         Err(err) => {
//             eprintln!(
//                 "Error deleting authentification with ID {:?}: {:?}",
//                 authentification_id, err
//             );
//             Err(err)
//         }
//     }
// }
