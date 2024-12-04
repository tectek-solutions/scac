use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::api_services)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApiServices {
    pub id: i32,
    auth_service_id: i32,
    pub name: String,
    pub base_url: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::api_services)]
pub struct NewApiServices<'a> {
    auth_service_id: i32,
    pub name: &'a str,
    pub base_url: &'a str,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}