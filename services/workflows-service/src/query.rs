use actix_web::web;
use database;
use database::model::{NewWorkflows, Workflows};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_json::Value;
use chrono::Utc;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateWorkflow {
    user_id: i32,
    name: String,
    description: Option<String>,
    action_id: i32,
    reaction_id: i32,
    data_transformation: Option<Value>,
}

pub fn list_workflows_by_user_id_query(
    db: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Workflows>>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();
    let result = workflows
        .filter(user_id.eq(search_id))
        .select(Workflows::as_select())
        .load::<Workflows>(&mut connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting workflows for user with ID {:?}: {:?}",
                search_id, err
            );
            Err(err)
        }
    }
}

pub fn get_workflow_by_id_query(
    db: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Workflows>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();

    match workflows
        .find(search_id)
        .select(Workflows::as_select())
        .first::<Workflows>(&mut connection)
        .optional()
    {
        Ok(Some(workflow)) => Ok(Some(workflow)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting workflow with ID {:?}: {:?}", search_id, err);
            Err(err)
        }
    }
}

pub fn create_workflow_query(
    db: &web::Data<database::Database>,
    new_workflow: CreateWorkflow,
) -> Result<Option<Workflows>, diesel::result::Error> {
    use database::schema::workflows;

    let mut connection = db.get_connection();
    diesel::insert_into(workflows::table)
        .values(&NewWorkflows {
            user_id: new_workflow.user_id,
            name: &new_workflow.name,
            description: new_workflow.description.as_deref(),
            action_id: new_workflow.action_id,
            reaction_id: new_workflow.reaction_id,
            data_transformation: new_workflow.data_transformation,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
        .execute(&mut connection)?;

    let workflow = workflows::table
        .order(workflows::id.desc())
        .first::<Workflows>(&mut connection)?;

    Ok(Some(workflow))
}

pub fn delete_workflow_by_id_query(
    db: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Workflows>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut connection = db.get_connection();
    match diesel::delete(workflows.filter(id.eq(search_id)))
        .execute(&mut connection)
        .optional()
    {
        Ok(_) => Ok(None),
        Err(err) => {
            eprintln!("Error deleting workflow with ID {:?}: {:?}", search_id, err);
            Err(err)
        }
    }
}
