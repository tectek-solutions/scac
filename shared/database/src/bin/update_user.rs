use self::model::User;
use diesel::prelude::*;
use database::*;
use std::env::args;

fn main () {
    use self::schema::users::dsl::{users, username};

    let id = args()
        .nth(1)
        .expect("Expected an id to update")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut etablish_connection();

    let user = diesel::update(users.find(id))
        .set(username.eq("updated_username"))
        .returning(User::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Updated user {:?}", user.username);
}