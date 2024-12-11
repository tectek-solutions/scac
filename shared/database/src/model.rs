use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Queryable)]
pub struct YourModel {
    pub headers: Option<Value>,
    pub params: Option<Value>,
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::authentification)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Authentification {
    pub id: i32,
    pub name: String,
    pub auth_url: String,
    pub token_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::authentification)]
pub struct NewAuthentification<'a> {
    pub name: &'a str,
    pub auth_url: &'a str,
    pub token_url: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::actions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Actions {
    pub id: i32,
    pub api_service_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub json_path: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::actions)]
pub struct NewActions<'a> {
    pub api_service_id: i32,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub endpoint: &'a str,
    pub method: String,
    pub headers: Option<&'a serde_json::Value>,
    pub params: Option<&'a serde_json::Value>,
    pub json_path: Option<&'a str>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::api_services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApiServices {
    pub id: i32,
    pub auth_service_id: i32,
    pub name: String,
    pub base_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::api_services)]
pub struct NewApiServices<'a> {
    pub auth_service_id: i32,
    pub name: &'a str,
    pub base_url: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable,     Serialize, Deserialize)]
#[diesel(table_name = crate::schema::reactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Reactions {
    pub id: i32,
    pub api_service_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub method: String,
    pub headers: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub json_path: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::reactions)]
pub struct NewReactions<'a> {
    pub api_service_id: i32,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub endpoint: &'a str,
    pub method: String,
    pub headers: Option<&'a serde_json::Value>,
    pub params: Option<&'a serde_json::Value>,
    pub json_path: Option<&'a str>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::user_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserTokens {
    pub id: i32,
    pub user_id: i32,
    pub auth_service_id: i32,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::user_tokens)]
pub struct NewUserTokens<'a> {
    pub user_id: i32,
    pub auth_service_id: i32,
    pub access_token: &'a str,
    pub refresh_token: Option<&'a str>,
    pub expires_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, AsChangeset    )]
#[diesel(table_name = crate::schema::user_tokens)]
pub struct UpdateUserTokens<'a> {
    pub user_id: Option<i32>,
    pub auth_service_id: Option<i32>,
    pub access_token: Option<&'a str>,
    pub refresh_token: Option<&'a str>,
    pub expires_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::workflows)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Workflows {
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

#[derive(Insertable)]
#[diesel(table_name = crate::schema::workflows)]
pub struct NewWorkflows<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub action_id: i32,
    pub reaction_id: i32,
    pub data_transformation: Option<&'a serde_json::Value>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::triggers)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Triggers {
    pub id: i32,
    pub workflow_id: i32,
    pub data: serde_json::Value,
    pub status: bool,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::triggers)]
pub struct NewTriggers<'a> {
    pub workflow_id: i32,
    pub data: &'a serde_json::Value,
    pub status: bool,
    pub created_at: Option<NaiveDateTime>,
}