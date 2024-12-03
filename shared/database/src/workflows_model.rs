use diesel::prelude::*;
use chrono::NaiveDateTime;

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