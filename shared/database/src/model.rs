use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use crate::schema::*;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime, Utc};
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
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_user: CreateUser) -> Result<User, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(users::table)
            .values((
                &new_user,
                (users::created_at.eq(now), users::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, user_id: i32) -> Result<User, Error> {
        users::table.find(user_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, user_id: i32, changes: UpdateUser) -> Result<User, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(users::table.find(user_id))
            .set((
                &changes,
                users::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, user_id: i32) -> Result<usize, Error> {
        diesel::delete(users::table.find(user_id)).execute(database_connection)
    }
}
// AUTHENTICATIONS
#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = authentications)]
pub struct Authentication {
    pub id: i32,
    pub name: String,
    pub authentication_url: String,
    pub authentication_url_json_path: String,
    pub refresh_token_url: String,
    pub refresh_token_url_json_path: String,
    pub access_token_expires_at_json_path: String,
    pub refresh_token_expires_at_json_path: String,
    pub is_expires_at_relative: Bool,
    pub client_id: String,
    pub client_secret: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = authentications)]
pub struct CreateAuthentication {
    pub name: String,
    pub authentication_url: String,
    pub authentication_url_json_path: String,
    pub refresh_token_url: String,
    pub refresh_token_url_json_path: String,
    pub access_token_expires_at_json_path: String,
    pub refresh_token_expires_at_json_path: String,
    pub is_expires_at_relative: Bool,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = authentications)]
pub struct UpdateAuthentication {
    pub name: String,
    pub authentication_url: String,
    pub authentication_url_json_path: String,
    pub refresh_token_url: String,
    pub refresh_token_url_json_path: String,
    pub access_token_expires_at_json_path: String,
    pub refresh_token_expires_at_json_path: String,
    pub is_expires_at_relative: Bool,
    pub client_id: String,
    pub client_secret: String,
}

impl Authentication {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_auth: CreateAuthentication) -> Result<Authentication, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(authentications::table)
            .values((
                &new_auth,
                (authentications::created_at.eq(now), authentications::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, auth_id: i32) -> Result<Authentication, Error> {
        authentications::table.find(auth_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, auth_id: i32, changes: UpdateAuthentication) -> Result<Authentication, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(authentications::table.find(auth_id))
            .set((
                &changes,
                authentications::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, auth_id: i32) -> Result<usize, Error> {
        diesel::delete(authentications::table.find(auth_id)).execute(database_connection)
    }
}

// USER TOKENS
#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = user_tokens)]
pub struct UserToken {
    pub id: i32,
    pub users_id: i32,
    pub authentication_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = user_tokens)]
pub struct CreateUserToken {
    pub users_id: i32,
    pub authentication_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<NaiveDateTime>,
    pub expires_at: NaiveDateTime,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = user_tokens)]
pub struct UpdateUserToken {
    pub users_id: i32,
    pub access_token: String,
    pub access_token_expires_at: NaiveDateTime,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_at: Option<NaiveDateTime>,
    pub expires_at: Option<NaiveDateTime>,
}

impl UserToken {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_token: CreateUserToken) -> Result<UserToken, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(user_tokens::table)
            .values((
                &new_token,
                (user_tokens::created_at.eq(now), user_tokens::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, token_id: i32) -> Result<UserToken, Error> {
        user_tokens::table.find(token_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, token_id: i32, changes: UpdateUserToken) -> Result<UserToken, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(user_tokens::table.find(token_id))
            .set((
                &changes,
                user_tokens::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, token_id: i32) -> Result<usize, Error> {
        diesel::delete(user_tokens::table.find(token_id)).execute(database_connection)
    }
}

// APIS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = apis)]
pub struct Api {
    pub id: i32,
    pub authentication_id: i32,
    pub name: String,
    pub base_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = apis)]
pub struct CreateApi {
    pub authentication_id: i32,
    pub name: String,
    pub base_url: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = apis)]
pub struct UpdateApi {
    pub authentication_id: Option<i32>,
    pub name: Option<String>,
    pub base_url: Option<String>,
}

impl Api {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_api: CreateApi) -> Result<Api, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(apis::table)
            .values((
                &new_api,
                (apis::created_at.eq(now), apis::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, api_id: i32) -> Result<Api, Error> {
        apis::table.find(api_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, api_id: i32, changes: UpdateApi) -> Result<Api, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(apis::table.find(api_id))
            .set((
                &changes,
                apis::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, api_id: i32) -> Result<usize, Error> {
        diesel::delete(apis::table.find(api_id)).execute(database_connection)
    }
}

// ACTIONS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = actions)]
pub struct Action {
    pub id: i32,
    pub api_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub trigger_data_json_path: String,
    pub trigger_data_conversion: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = actions)]
pub struct CreateAction {
    pub api_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub trigger_data_json_path: String,
    pub trigger_data_conversion: String,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = actions)]
pub struct UpdateAction {
    pub api_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub http_method: Option<String>,
    pub http_endpoint: Option<String>,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub trigger_data_json_path: String,
    pub trigger_data_conversion: String,
}

impl Action {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_action: CreateAction) -> Result<Action, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(actions::table)
            .values((
                &new_action,
                (actions::created_at.eq(now), actions::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, action_id: i32) -> Result<Action, Error> {
        actions::table.find(action_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, action_id: i32, changes: UpdateAction) -> Result<Action, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(actions::table.find(action_id))
            .set((
                &changes,
                actions::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, action_id: i32) -> Result<usize, Error> {
        diesel::delete(actions::table.find(action_id)).execute(database_connection)
    }
}

// REACTIONS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = reactions)]

pub struct Reaction {
    pub id: i32,
    pub api_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = reactions)]
pub struct CreateReaction {
    pub api_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub http_method: String,
    pub http_endpoint: String,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = reactions)]
pub struct UpdateReaction {
    pub api_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub http_method: Option<String>,
    pub http_endpoint: Option<String>,
    pub http_parameters: Option<serde_json::Value>,
    pub http_headers: Option<serde_json::Value>,
    pub http_body: Option<serde_json::Value>,
}

impl Reaction {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_reaction: CreateReaction) -> Result<Reaction, Error> {
        let now = Utc::now().naive_utc();
        diesel::insert_into(reactions::table)
            .values((
                &new_reaction,
                (reactions::created_at.eq(now), reactions::updated_at.eq(now))
            ))
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, reaction_id: i32) -> Result<Reaction, Error> {
        reactions::table.find(reaction_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, reaction_id: i32, changes: UpdateReaction) -> Result<Reaction, Error> {
        let now = Utc::now().naive_utc();
        diesel::update(reactions::table.find(reaction_id))
            .set((
                &changes,
                reactions::updated_at.eq(now)
            ))
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, reaction_id: i32) -> Result<usize, Error> {
        diesel::delete(reactions::table.find(reaction_id)).execute(database_connection)
    }
}

// WORKFLOWS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = workflows)]
pub struct Workflow {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub action_id: i32,
    pub reaction_id: i32,
    pub data_transformation: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = workflows)]
pub struct CreateWorkflow {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub action_id: i32,
    pub reaction_id: i32,
    pub data_transformation: Option<serde_json::Value>,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = workflows)]
pub struct UpdateWorkflow {
    pub user_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub action_id: Option<i32>,
    pub reaction_id: Option<i32>,
    pub data_transformation: Option<serde_json::Value>,
}

impl Workflow {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_workflow: CreateWorkflow) -> Result<Workflow, Error> {
        diesel::insert_into(workflows::table)
            .values(&new_workflow)
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, workflow_id: i32) -> Result<Workflow, Error> {
        workflows::table.find(workflow_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, workflow_id: i32, changes: UpdateWorkflow) -> Result<Workflow, Error> {
        diesel::update(workflows::table.find(workflow_id))
            .set(&changes)
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, workflow_id: i32) -> Result<usize, Error> {
        diesel::delete(workflows::table.find(workflow_id)).execute(database_connection)
    }
}

// TRIGGERS

#[derive(Queryable, Identifiable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = triggers)]
pub struct Trigger {
    pub id: i32,
    pub workflow_id: i32,
    pub data: Option<serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, ToSchema)]
#[diesel(table_name = triggers)]
pub struct CreateTrigger {
    pub workflow_id: i32,
    pub data: Option<serde_json::Value>,
}

#[derive(AsChangeset, Deserialize, ToSchema)]
#[diesel(table_name = triggers)]
pub struct UpdateTrigger {
    pub workflow_id: Option<i32>,
    pub data: Option<serde_json::Value>,
}

impl Trigger {
    pub fn create(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, new_trigger: CreateTrigger) -> Result<Trigger, Error> {
        diesel::insert_into(triggers::table)
            .values(&new_trigger)
            .get_result(database_connection)
    }

    pub fn read(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, trigger_id: i32) -> Result<Trigger, Error> {
        triggers::table.find(trigger_id).get_result(database_connection)
    }

    pub fn update(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, trigger_id: i32, changes: UpdateTrigger) -> Result<Trigger, Error> {
        diesel::update(triggers::table.find(trigger_id))
            .set(&changes)
            .get_result(database_connection)
    }

    pub fn delete(database_connection: &mut PooledConnection<ConnectionManager<PgConnection>>, trigger_id: i32) -> Result<usize, Error> {
        diesel::delete(triggers::table.find(trigger_id)).execute(database_connection)
    }
}
