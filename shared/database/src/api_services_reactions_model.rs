use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::api_services_reactions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ApiServicesReactions {
    pub id: i32,
    api_service_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub endpoint: String,
    pub method: HttpMethodEnum,
    pub headers: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub json_path: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::api_services_reactions)]
pub struct NewApiServicesReactions<'a> {
    api_service_id: i32,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub endpoint: &'a str,
    pub method: HttpMethodEnum,
    pub headers: Option<&'a serde_json::Value>,
    pub params: Option<&'a serde_json::Value>,
    pub json_path: Option<&'a str>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}