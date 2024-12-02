use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::auth_service)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AuthService {
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
#[diesel(table_name = crate::schema::auth_service)]
pub struct NewAuthService<'a> {
    pub name: &'a str,
    pub auth_url: &'a str,
    pub token_url: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
}