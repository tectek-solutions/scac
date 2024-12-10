use actix_web::web;
use database;
use database::model::{NewWorkflows, Workflows};
use diesel::prelude::*;
use serde_json::Value;

pub fn get_workflows(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Workflows>>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();
    let result = workflows.load::<Workflows>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting workflows: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_workflow_by_id(
    db: &web::Data<database::Database>,
    workflow_id: i32,
) -> Result<Option<Workflows>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();

    match workflows
        .find(workflow_id)
        .select(Workflows::as_select())
        .first::<Workflows>(&mut connection)
        .optional()
    {
        Ok(Some(workflow)) => Ok(Some(workflow)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting workflow with ID {:?}: {:?}",
                workflow_id, err
            );
            Err(err)
        }
    }
}

pub fn create_workflow(
    db: &web::Data<database::Database>,
    _user_id: i32,
    _name: String,
    _description: String,
    _action_id: i32,
    _reaction_id: i32,
    _data_transformation: Option<Value>,
) -> Result<Option<Workflows>, diesel::result::Error> {
    use database::schema::workflows;

    let new_workflow = NewWorkflows {
        user_id: _user_id,
        name: &_name,
        description: Some(&_description),
        action_id: _action_id,
        reaction_id: _reaction_id,
        data_transformation: _data_transformation.as_ref(),
        created_at: Some(chrono::Local::now().naive_local()),
        updated_at: Some(chrono::Local::now().naive_local()),
    };

    let mut connection = db.get_connection();
    diesel::insert_into(workflows::table)
        .values(&new_workflow)
        .execute(&mut connection)?;

    let workflow = workflows::table
        .order(workflows::id.desc())
        .first::<Workflows>(&mut connection)?;

    Ok(Some(workflow))
}

pub fn delete_workflow(
    db: &web::Data<database::Database>,
    workflow_id: i32,
) -> Result<(), diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();
    match diesel::delete(workflows.filter(id.eq(workflow_id))).execute(&mut connection) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!(
                "Error deleting workflow with ID {:?}: {:?}",
                workflow_id, err
            );
            Err(err)
        }
    }
}