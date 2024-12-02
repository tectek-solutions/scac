use diesel::prelude::*;
use database::model::*;
use database::*;

fn main() {
    get_user();
    get_user_by_id(1);
    // add_user("user_test", "user_mail");
    // update_user(2, "user_test_updated");
    // delete_user(2);
}

fn get_user() {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user.username);
        println!("{:?}", user.email);
    }
}

fn get_user_by_id(user_id: i32) {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let user = users
        .find(user_id)
        .select(User::as_select())
        .first(connection)
        .optional();
    match user {
        Ok(Some(user)) => {
            println!("Found user {:?}, with id {:?}", user.username, user_id);
            println!("Found email {:?} for user_id {:?}", user.email, user_id);
        },
        Ok(None) => {
            println!("No user found with id {:?}", user_id);
        },
        Err(err) => {
            println!("Error: {:?} when fetching post {:?}", err, user_id);
        }
    }
}

fn add_user(username: &str, email: &str) {
    let connection = &mut etablish_connection();
    let username = username;
    let email = email;

    let password_hash = "password_hash";

    let user = create_user(connection, username, email, password_hash);

    println!("Created user {:?}", user.username);
    println!("Created email {:?}", user.email);
    println!("Created password_hash {:?}", user.password_hash);
}

fn update_user(user_id: i32, _username: &str) {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let user = diesel::update(users.find(user_id))
        .set(username.eq(_username))
        .returning(User::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Updated user {:?}", user.username);
}

fn delete_user(user_id: i32) {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let user: User = diesel::delete(users.find(user_id))
        .get_result(connection)
        .expect("Error deleting user");

    println!("Deleted user {:?}", user.username);
}
