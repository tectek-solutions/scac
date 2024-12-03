use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Selectable)]
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

#[derive(Serialize, Deserialize, AsChangeset)]
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