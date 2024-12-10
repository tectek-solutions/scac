use actix_web::web;
use database;
use database::model::{NewReactions, Reactions};
use diesel::prelude::*;

pub fn get_reactions(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Reactions>>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut connection = db.get_connection();
    let result = reactions.load::<Reactions>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting reactions: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_reactions_by_id(
    db: &web::Data<database::Database>,
    reaction_id: i32,
) -> Result<Option<Reactions>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut connection = db.get_connection();

    match reactions
        .find(reaction_id)
        .select(Reactions::as_select())
        .first::<Reactions>(&mut connection)
        .optional()
    {
        Ok(Some(reaction)) => Ok(Some(reaction)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting reaction with ID {:?}: {:?}",
                reaction_id, err
            );
            Err(err)
        }
    }
}

pub fn create_reaction(
    db: &web::Data<database::Database>,
    _api_service_id: i32,
    _name: String,
    _description: Option<String>,
    _endpoint: String,
    _method: String,
    _headers: Option<serde_json::Value>,
    _params: Option<serde_json::Value>,
    _json_path: Option<String>,
    _updated_at: Option<chrono::NaiveDateTime>,
) -> Result<Option<Reactions>, diesel::result::Error> {
    use database::schema::reactions;

    let new_reaction = NewReactions {
        api_service_id: _api_service_id,
        name: &_name,
        description: _description.as_deref(),
        endpoint: &_endpoint,
        method: _method,
        headers: _headers.as_ref(),
        params: _params.as_ref(),
        json_path: _json_path.as_deref(),
        created_at: Some(chrono::Utc::now().naive_utc()),
        updated_at: _updated_at,
    };

    let mut connection = db.get_connection();

    match diesel::insert_into(reactions::table)
        .values(&new_reaction)
        .get_result::<Reactions>(&mut connection)
    {
        Ok(reaction) => Ok(Some(reaction)),
        Err(err) => {
            eprintln!("Error creating reaction: {:?}", err);
            Err(err)
        }
    }
}

pub fn delete_reaction(
    db: &web::Data<database::Database>,
    reaction_id: i32,
) -> Result<Option<Reactions>, diesel::result::Error> {
    use database::schema::reactions::dsl::*;

    let mut connection = db.get_connection();

    match diesel::delete(reactions.filter(id.eq(reaction_id)))
        .get_result::<Reactions>(&mut connection)
        .optional()
    {
        Ok(reaction) => Ok(reaction),
        Err(err) => {
            eprintln!(
                "Error deleting reaction with ID {:?}: {:?}",
                reaction_id, err
            );
            Err(err)
        }
    }
}



