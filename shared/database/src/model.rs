use crate::schema::*;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use serde_json;
use utoipa::ToSchema;

// USERS
#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password_hash: Option<String>,
}

impl User {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_user: CreateUser,
    ) -> Result<User, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(users::table)
            .values((
                &new_user,
                (users::created_at.eq(now), users::updated_at.eq(now)),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        user_id: i32,
    ) -> Result<User, Error> {
        users::table.find(user_id).get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        user_id: i32,
        changes: UpdateUser,
    ) -> Result<User, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(users::table.find(user_id))
            .set((&changes, users::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        user_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(users::table.find(user_id)).execute(database_connection)
    }
}

// AUTHENTICATIONS
#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = authentications)]
pub struct Authentication {
    pub id: i32,
    pub name: String,
    pub authorization_url: String,
    pub authorization_http_parameters: serde_json::Value,
    pub token_url: String,
    pub token_url_http_parameters: serde_json::Value,
    pub client_id: String,
    pub client_secret: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = authentications)]
pub struct CreateAuthentication {
    pub name: String,
    pub authorization_url: String,
    pub authorization_http_parameters: serde_json::Value,
    pub token_url: String,
    pub token_url_http_parameters: serde_json::Value,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = authentications)]
pub struct UpdateAuthentication {
    pub name: Option<String>,
    pub authorization_url: String,
    pub authorization_http_parameters: serde_json::Value,
    pub token_url: String,
    pub token_url_http_parameters: serde_json::Value,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

impl Authentication {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_auth: CreateAuthentication,
    ) -> Result<Authentication, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(authentications::table)
            .values((
                &new_auth,
                (
                    authentications::created_at.eq(now),
                    authentications::updated_at.eq(now),
                ),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        auth_id: i32,
    ) -> Result<Authentication, Error> {
        authentications::table
            .find(auth_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        auth_id: i32,
        changes: UpdateAuthentication,
    ) -> Result<Authentication, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(authentications::table.find(auth_id))
            .set((&changes, authentications::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        auth_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(authentications::table.find(auth_id)).execute(database_connection)
    }
}

// USER TOKENS
#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = user_tokens)]
pub struct UserToken {
    pub id: i32,
    pub users_id: i32,
    pub authentications_id: i32,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = user_tokens)]
pub struct CreateUserToken {
    pub users_id: i32,
    pub authentications_id: i32,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = user_tokens)]
pub struct UpdateUserToken {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub expires_at: Option<NaiveDateTime>,
}

impl UserToken {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_token: CreateUserToken,
    ) -> Result<UserToken, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(user_tokens::table)
            .values((
                &new_token,
                (
                    user_tokens::created_at.eq(now),
                    user_tokens::updated_at.eq(now),
                ),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        token_id: i32,
    ) -> Result<UserToken, Error> {
        user_tokens::table
            .find(token_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        token_id: i32,
        changes: UpdateUserToken,
    ) -> Result<UserToken, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(user_tokens::table.find(token_id))
            .set((&changes, user_tokens::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        token_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(user_tokens::table.find(token_id)).execute(database_connection)
    }
}

// APIS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = apis)]
pub struct Api {
    pub id: i32,
    pub authentications_id: i32,
    pub name: String,
    pub base_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = apis)]
pub struct CreateApi {
    pub authentications_id: i32,
    pub name: String,
    pub base_url: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = apis)]
pub struct UpdateApi {
    pub authentications_id: Option<i32>,
    pub name: Option<String>,
    pub base_url: Option<String>,
}

impl Api {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_api: CreateApi,
    ) -> Result<Api, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(apis::table)
            .values((
                &new_api,
                (apis::created_at.eq(now), apis::updated_at.eq(now)),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        api_id: i32,
    ) -> Result<Api, Error> {
        apis::table.find(api_id).get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        api_id: i32,
        changes: UpdateApi,
    ) -> Result<Api, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(apis::table.find(api_id))
            .set((&changes, apis::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        api_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(apis::table.find(api_id)).execute(database_connection)
    }
}

// ACTIONS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = actions)]
pub struct Action {
    pub id: i32,
    pub apis_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
    pub last_id_json_path: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = actions)]
pub struct CreateAction {
    pub apis_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
    pub last_id_json_path: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = actions)]
pub struct UpdateAction {
    pub apis_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub http_method: Option<String>,
    pub http_endpoint: Option<String>,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
    pub last_id_json_path: String,
}

impl Action {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_action: CreateAction,
    ) -> Result<Action, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(actions::table)
            .values((
                &new_action,
                (actions::created_at.eq(now), actions::updated_at.eq(now)),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        action_id: i32,
    ) -> Result<Action, Error> {
        actions::table
            .find(action_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        action_id: i32,
        changes: UpdateAction,
    ) -> Result<Action, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(actions::table.find(action_id))
            .set((&changes, actions::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        action_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(actions::table.find(action_id)).execute(database_connection)
    }
}

// REACTIONS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = reactions)]

pub struct Reaction {
    pub id: i32,
    pub apis_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = reactions)]
pub struct CreateReaction {
    pub apis_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = reactions)]
pub struct UpdateReaction {
    pub apis_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub http_method: Option<String>,
    pub http_endpoint: Option<String>,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub data_keys: Option<serde_json::Value>,
}

impl Reaction {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_reaction: CreateReaction,
    ) -> Result<Reaction, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(reactions::table)
            .values((
                &new_reaction,
                (reactions::created_at.eq(now), reactions::updated_at.eq(now)),
            ))
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        reaction_id: i32,
    ) -> Result<Reaction, Error> {
        reactions::table
            .find(reaction_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        reaction_id: i32,
        changes: UpdateReaction,
    ) -> Result<Reaction, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(reactions::table.find(reaction_id))
            .set((&changes, reactions::updated_at.eq(now)))
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        reaction_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(reactions::table.find(reaction_id)).execute(database_connection)
    }
}

// WORKFLOWS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = workflows)]
pub struct Workflow {
    pub id: i32,
    pub users_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub actions_id: i32,
    pub reactions_id: i32,
    pub action_data: Option<serde_json::Value>,
    pub reaction_data: Option<serde_json::Value>,
    pub last_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = workflows)]
pub struct CreateWorkflow {
    pub users_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub actions_id: i32,
    pub reactions_id: i32,
    pub action_data: Option<serde_json::Value>,
    pub reaction_data: Option<serde_json::Value>,
    pub last_id: Option<String>,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = workflows)]
pub struct UpdateWorkflow {
    pub users_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub actions_id: Option<i32>,
    pub reactions_id: Option<i32>,
    pub action_data: Option<serde_json::Value>,
    pub reaction_data: Option<serde_json::Value>,
    pub last_id: Option<String>,
}

impl Workflow {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_workflow: CreateWorkflow,
    ) -> Result<Workflow, Error> {
        diesel::insert_into(workflows::table)
            .values(&new_workflow)
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        workflow_id: i32,
    ) -> Result<Workflow, Error> {
        workflows::table
            .find(workflow_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        workflow_id: i32,
        changes: UpdateWorkflow,
    ) -> Result<Workflow, Error> {
        diesel::update(workflows::table.find(workflow_id))
            .set(&changes)
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        workflow_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(workflows::table.find(workflow_id)).execute(database_connection)
    }
}

// TRIGGERS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = triggers)]
pub struct Trigger {
    pub id: i32,
    pub workflows_id: i32,
    pub status: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = triggers)]
pub struct CreateTrigger {
    pub workflows_id: i32,
    pub status: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = triggers)]
pub struct UpdateTrigger {
    pub workflows_id: Option<i32>,
    pub status: String,
}

impl Trigger {
    pub fn create(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        new_trigger: CreateTrigger,
    ) -> Result<Trigger, Error> {
        diesel::insert_into(triggers::table)
            .values(&new_trigger)
            .get_result(database_connection)
    }

    pub fn read(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        trigger_id: i32,
    ) -> Result<Trigger, Error> {
        triggers::table
            .find(trigger_id)
            .get_result(database_connection)
    }

    pub fn update(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        trigger_id: i32,
        changes: UpdateTrigger,
    ) -> Result<Trigger, Error> {
        diesel::update(triggers::table.find(trigger_id))
            .set(&changes)
            .get_result(database_connection)
    }

    pub fn delete(
        database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>,
        trigger_id: i32,
    ) -> Result<usize, Error> {
        diesel::delete(triggers::table.find(trigger_id)).execute(database_connection)
    }
}
