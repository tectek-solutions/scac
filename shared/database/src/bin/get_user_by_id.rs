use self::model::User;
use diesel::prelude::*;
use database::*;
use std::env::args;

fn main () {
    use self::schema::users::dsl::users;

    let user_id = args()
        .nth(1)
        .expect("Expected an id to update")
        .parse::<i32>()
        .expect("Invalid ID");

    let connection = &mut etablish_connection();

    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .optional();

    match user {
        Ok(Some(user)) => {
            println!("Found user {:?}", user.username);
            println!("Found email {:?}", user.email);
        },
        Ok(None) => {
            println!("No user found with id {:?}", user_id);
        },
        Err(err) => {
            println!("Error: {:?} when fetching post {:?}", err, user_id);
        }
    }
}