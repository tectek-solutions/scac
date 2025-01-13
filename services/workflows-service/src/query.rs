use actix_web::web;
use database;
use database::model::{Workflow, CreateWorkflow};
use diesel::prelude::*;

pub fn list_workflows_by_user_id_query(
    database: &web::Data<database::Database>,
    search_id: i32,
) -> Result<Option<Vec<Workflow>>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut database_connection = database.get_connection();
    let result = workflows
        .filter(users_id.eq(search_id))
        .select(Workflow::as_select())
        .load::<Workflow>(&mut database_connection)
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

pub fn get_workflow_by_id_by_user_id_query(
    database: &web::Data<database::Database>,
    search_workflow_id: i32,
    search_user_id: i32,
) -> Result<Option<Workflow>, diesel::result::Error> {
    use database::schema::workflows::dsl::*;

    let mut database_connection = database.get_connection();
    let result = workflows
        .filter(users_id.eq(search_user_id))
        .filter(id.eq(search_workflow_id))
        .select(Workflow::as_select()) 
        .first::<Workflow>(&mut database_connection)
        .optional();

    match result {
        Ok(Some(result)) => Ok(Some(result)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!(
                "Error getting workflow with ID {:?} for user with ID {:?}: {:?}",
                search_workflow_id, search_user_id, err
            );
            Err(err)
        }
    }
}

pub fn create_workflow_query(
    database: &web::Data<database::Database>,
    new_workflow: CreateWorkflow,
) -> Result<Option<Workflow>, diesel::result::Error> {
    let mut database_connection = database.get_connection();
    
    match Workflow::create(&mut database_connection, new_workflow) {
        Ok(workflow) => Ok(Some(workflow)),
        Err(err) => {
            eprintln!("Error creating workflow: {:?}", err);
            Err(err)
        }
    }
}

pub fn delete_workflow_by_id_query(
    database: &web::Data<database::Database>,
    delete_id: i32,
) -> Result<Option<()>, diesel::result::Error> {
    let mut database_connection = database.get_connection();

    match Workflow::delete(&mut database_connection, delete_id) {
        Ok(size) => {
            if size == 0 {
                Ok(None)
            } else {
                Ok(Some(()))
            }
        }
        Err(err) => {
            eprintln!("Error deleting workflow with ID {:?}: {:?}", delete_id, err);
            Err(err)
        }
    }
}
