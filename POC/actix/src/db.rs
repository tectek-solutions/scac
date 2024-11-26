pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use models::*;
use schema::todos;
use schema::todos::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn _db_get_todos() -> Vec<Todo> {
    let mut connection = establish_connection();
    todos
        .limit(5)
        .load::<Todo>(&mut connection)
        .expect("Error loading todos")
}

pub fn _db_get_todo_by_id(key: String) -> Vec<Todo> {
    let mut connection = establish_connection();
    todos
        .find(key)
        .load::<Todo>(&mut connection)
        .expect("Error loading todo")
}

pub fn _db_create_todo(n: &str, d: &str) -> Todo {
    let mut connection = establish_connection();

    let uuid = Uuid::new_v4().to_string();

    let new_todo = NewTodo {
        id: &uuid,
        name: n,
        description: d,
    };

    diesel::insert_into(todos::table)
        .values(&new_todo)
        .execute(&mut connection)
        .expect("Error saving new todo");

    Todo {
        id: uuid,
        name: n.to_string(),
        description: d.to_string(),
    }
}

pub fn _db_update_todo(key: String, n: &str, d: &str) -> Todo {
    let mut connection = establish_connection();

    diesel::update(todos.find(key.clone()))
        .set((name.eq(n), description.eq(d)))
        .execute(&mut connection)
        .expect("Error updating todo");

    Todo {
        id: key,
        name: n.to_string(),
        description: d.to_string(),
    }
}

pub fn _db_delete_todo(key: String) -> usize {
    let mut connection = establish_connection();

    diesel::delete(todos.find(key))
        .execute(&mut connection)
        .expect("Error deleting todo")
}