use diesel::prelude::*;
use database::model::*;
use database::*;

/// Executes a query to retrieve a limited number of user records from the database.
/// 
/// This function performs the following steps:
/// 1. Selects the user records using the `User::as_select()` method.
/// 2. Loads the selected user records from the database using the provided connection.
///
/// # Parameters
/// - `connection`: A reference to the database connection.
///
/// # Returns
/// - A vector of user records.
///
/// # Panics
/// - This function will panic if there is an error loading the users from the database.
pub fn get_user() {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user.username);
        println!("{:?}", user.email);
    }
}

/// Queries the `users` table to find a user by `user_id`, selects the user data
/// using the `User::as_select()` method, and attempts to retrieve the first result
/// from the database connection. Returns an `Option` containing the user data if found,
/// or `None` if no user is found.
///
/// # Arguments
///
/// * `user_id` - The ID of the user to find.
/// * `connection` - The database connection to use for the query.
///
/// # Returns
///
/// * `Option<User>` - An optional user object if found, otherwise `None`.
pub fn get_user_by_id(user_id: i32) {
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


/// Creates a new user in the database.
///
/// # Arguments
///
/// * `connection` - A reference to the database connection.
/// * `username` - A string slice that holds the username of the new user.
/// * `email` - A string slice that holds the email of the new user.
/// * `password_hash` - A string slice that holds the hashed password of the new user.
///
/// # Returns
///
/// A `Result` containing the created user or an error if the operation fails.
pub fn add_user(username: &str, email: &str) {
    let connection = &mut etablish_connection();
    let username = username;
    let email = email;

    let password_hash = "password_hash";

    let user = create_user(connection, username, email, password_hash);

    println!("Created user {:?}", user.username);
    println!("Created email {:?}", user.email);
    println!("Created password_hash {:?}", user.password_hash);
}

/// Updates the username of a user in the database and returns the updated user.
///
/// This function uses Diesel to perform an update on the `users` table, setting the `username`
/// field to the provided `_username` value for the user with the specified `user_id`.
/// It then returns the updated `User` object.
///
/// # Arguments
///
/// * `user_id` - The ID of the user to update.
/// * `_username` - The new username to set for the user.
/// * `connection` - The database connection to use for the query.
///
/// # Returns
///
/// * `User` - The updated user object.
///
/// # Panics
///
/// This function will panic if the database query fails.
pub fn update_user(user_id: i32, _username: &str) {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let user = diesel::update(users.find(user_id))
        .set(username.eq(_username))
        .returning(User::as_returning())
        .get_result(connection)
        .unwrap();

    println!("Updated user {:?}", user.username);
}

/// Deletes a user from the database based on the provided user ID.
/// 
/// # Arguments
/// 
/// * `user_id` - The ID of the user to be deleted.
/// * `connection` - The database connection to be used for the operation.
/// 
/// # Returns
/// 
/// * `User` - The deleted user object.
/// 
/// # Panics
/// 
/// This function will panic if there is an error deleting the user.
pub fn delete_user(user_id: i32) {
    use database::schema::users::dsl::*;

    let connection = &mut etablish_connection();
    let user: User = diesel::delete(users.find(user_id))
        .get_result(connection)
        .expect("Error deleting user");

    println!("Deleted user {:?}", user.username);
}