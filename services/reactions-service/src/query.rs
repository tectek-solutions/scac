use actix_web::web;
use database;
use database::model::{Reactions};
use diesel::prelude::*;

pub fn list_reactions_by_api_service_id_query(
    db: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Reactions>>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut connection = db.get_connection();
    let result = reactions
        .filter(api_service_id.eq(search_id))
        .select(Reactions::as_select())
        .load::<Reactions>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting reactions: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_reaction_by_id_query(
    db: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Reactions>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut connection = db.get_connection();

    match reactions
        .find(search_id)
        .select(Reactions::as_select())
        .first::<Reactions>(&mut connection)
        .optional()
    {
        Ok(Some(action)) => Ok(Some(action)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting action with ID {:?}: {:?}", search_id, err);
            Err(err)
        }
    }
}

// pub fn create_reactions(
//     db: &web::Data<database::Database>,
//     api_service_id: i32,
//     name: &str,
//     description: Option<&str>,
//     endpoint: &str,
//     method: String,
//     headers: Option<&serde_json::Value>,
//     params: Option<&serde_json::Value>,
//     json_path: Option<&str>,
//     created_at: Option<NaiveDateTime>,
//     updated_at: Option<NaiveDateTime>,
// ) -> Result<Option<Reactions>, diesel::result::Error> {
//     use database::schema::reactions;

//     let new_action = NewActions {
//         api_service_id: api_service_id,
//         name: name,
//         description: description,
//         endpoint: endpoint,
//         method: method,
//         headers: headers,
//         params: params,
//         json_path: json_path,
//         created_at: created_at,
//         updated_at: updated_at,
//     };

//     let mut connection = db.get_connection();

//     match diesel::insert_into(reactions::table)
//         .values(&new_action)
//         .returning(Reactions::as_select())
//         .get_result::<Reactions>(&mut connection)
//         .optional()
//     {
//         Ok(action) => Ok(action),
//         Err(err) => {
//             eprintln!("Error creating action: {:?}", err);
//             Err(err)
//         }
//     }
// }

// pub fn update_reactions(
//     db: &web::Data<database::Database>,
//     action_id: i32,
//     _api_service_id: i32,
//     _name: &str,
//     _description: Option<&str>,
//     _endpoint: &str,
//     _method: String,
//     _headers: Option<&serde_json::Value>,
//     _params: Option<&serde_json::Value>,
//     _json_path: Option<&str>,
//     _created_at: Option<NaiveDateTime>,
//     _updated_at: Option<NaiveDateTime>,
// ) -> Result<Option<Reactions>, diesel::result::Error> {
//     use database::schema::reactions::dsl::*;

//     let mut connection = db.get_connection();
//     diesel::update(reactions.find(action_id))
//         .set((
//             api_service_id.eq(_api_service_id),
//             name.eq(_name),
//             description.eq(_description),
//             endpoint.eq(_endpoint),
//             method.eq(_method),
//             headers.eq(_headers),
//             params.eq(_params),
//             json_path.eq(_json_path),
//             created_at.eq(_created_at),
//             updated_at.eq(_updated_at),
//         ))
//         .execute(&mut connection)?;

//     match reactions
//         .find(action_id)
//         .select(Reactions::as_select())
//         .first::<Reactions>(&mut connection)
//         .optional()
//     {
//         Ok(Some(action)) => Ok(Some(action)),
//         Ok(None) => Ok(None),
//         Err(err) => {
//             eprintln!(
//                 "Error updating action with ID {:?}: {:?}",
//                 action_id, err
//             );
//             Err(err)
//         }
//     }
// }

// pub fn delete_reactions(
//     db: &web::Data<database::Database>,
//     action_id: i32,
// ) -> Result<Option<Reactions>, diesel::result::Error> {
//     use database::schema::reactions::dsl::*;

//     let mut connection = db.get_connection();
//     let deleted_action = diesel::delete(reactions.find(action_id))
//         .get_result::<Reactions>(&mut connection)
//         .optional();

//     match deleted_action {
//         Ok(deleted_action) => Ok(deleted_action),
//         Err(err) => {
//             eprintln!(
//                 "Error deleting action with ID {:?}: {:?}",
//                 action_id, err
//             );
//             Err(err)
//         }
//     }
// }
