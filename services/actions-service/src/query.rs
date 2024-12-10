use actix_web::web;
use database;
use database::model::{NewActions, Actions};
use diesel::prelude::*;
use chrono::NaiveDateTime;

pub fn get_actions(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Actions>>, diesel::result::Error> {
    use database::schema::actions::dsl::*;

    let mut connection = db.get_connection();
    let result = actions.load::<Actions>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting actions: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_actions_by_id(
    db: &web::Data<database::Database>,
    action_id: i32,
) -> Result<Option<Actions>, diesel::result::Error> {
    use database::schema::actions::dsl::*;

    let mut connection = db.get_connection();

    match actions
        .find(action_id)
        .select(Actions::as_select())
        .first::<Actions>(&mut connection)
        .optional()
    {
        Ok(Some(action)) => Ok(Some(action)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting action with ID {:?}: {:?}",
                action_id, err
            );
            Err(err)
        }
    }
}

pub fn create_actions(
    db: &web::Data<database::Database>,
    api_service_id: i32,
    name: &str,
    description: Option<&str>,
    endpoint: &str,
    method: String,
    headers: Option<&serde_json::Value>,
    params: Option<&serde_json::Value>,
    json_path: Option<&str>,
    created_at: Option<NaiveDateTime>,
    updated_at: Option<NaiveDateTime>,
) -> Result<Option<Actions>, diesel::result::Error> {
    use database::schema::actions;

    let new_action = NewActions {
        api_service_id: api_service_id,
        name: name,
        description: description,
        endpoint: endpoint,
        method: method,
        headers: headers,
        params: params,
        json_path: json_path,
        created_at: created_at,
        updated_at: updated_at,
    };

    let mut connection = db.get_connection();

    match diesel::insert_into(actions::table)
        .values(&new_action)
        .returning(Actions::as_select())
        .get_result::<Actions>(&mut connection)
        .optional()
    {
        Ok(action) => Ok(action),
        Err(err) => {
            eprintln!("Error creating action: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_actions(
    db: &web::Data<database::Database>,
    action_id: i32,
    _api_service_id: i32,
    _name: &str,
    _description: Option<&str>,
    _endpoint: &str,
    _method: String,
    _headers: Option<&serde_json::Value>,
    _params: Option<&serde_json::Value>,
    _json_path: Option<&str>,
    _created_at: Option<NaiveDateTime>,
    _updated_at: Option<NaiveDateTime>,
) -> Result<Option<Actions>, diesel::result::Error> {
    use database::schema::actions::dsl::*;

    let mut connection = db.get_connection();
    diesel::update(actions.find(action_id))
        .set((
            api_service_id.eq(_api_service_id),
            name.eq(_name),
            description.eq(_description),
            endpoint.eq(_endpoint),
            method.eq(_method),
            headers.eq(_headers),
            params.eq(_params),
            json_path.eq(_json_path),
            created_at.eq(_created_at),
            updated_at.eq(_updated_at),
        ))
        .execute(&mut connection)?;

    match actions
        .find(action_id)
        .select(Actions::as_select())
        .first::<Actions>(&mut connection)
        .optional()
    {
        Ok(Some(action)) => Ok(Some(action)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error updating action with ID {:?}: {:?}",
                action_id, err
            );
            Err(err)
        }
    }
}

pub fn delete_actions(
    db: &web::Data<database::Database>,
    action_id: i32,
) -> Result<Option<Actions>, diesel::result::Error> {
    use database::schema::actions::dsl::*;

    let mut connection = db.get_connection();
    let deleted_action = diesel::delete(actions.find(action_id))
        .get_result::<Actions>(&mut connection)
        .optional();

    match deleted_action {
        Ok(deleted_action) => Ok(deleted_action),
        Err(err) => {
            eprintln!(
                "Error deleting action with ID {:?}: {:?}",
                action_id, err
            );
            Err(err)
        }
    }
}