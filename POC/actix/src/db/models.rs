use super::schema::todos;

#[derive(serde_derive::Serialize, Queryable)]
pub struct Todo {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub description: &'a str,
}