use diesel::prelude::*;
use database::model::*;
use database::*;

pub fn get_users() {
    use database::schema::users::dsl::*;

    let connection = &mut establish_connection();
    match users.select(User::as_select()).load::<User>(connection) {
        Ok(results) => {
            println!("Displaying {} users", results.len());
            for user in results {
                println!("Username: {:?}, Email: {:?}", user.username, user.email);
            }
        }
        Err(err) => eprintln!("Error loading users: {:?}", err),
    }
}

pub fn get_user_by_id(user_id: i32) {
    use database::schema::users::dsl::*;

    let connection = &mut establish_connection();
    match users
        .find(user_id)
        .select(User::as_select())
        .first::<User>(connection)
        .optional()
    {
        Ok(Some(user)) => {
            println!("Found user {:?}, with ID {:?}", user.username, user_id);
            println!("Email: {:?}", user.email);
        }
        Ok(None) => println!("No user found with ID {:?}", user_id),
        Err(err) => eprintln!("Error fetching user with ID {:?}: {:?}", user_id, err),
    }
}

pub fn get_user_by_email(email: &str) {
    use database::schema::users::dsl::*;

    let connection = &mut establish_connection();
    match users
        .filter(email.eq(email))
        .select(User::as_select())
        .first::<User>(connection)
        .optional()
    {
        Ok(Some(user)) => {
            println!("Found user {:?}, with email {:?}", user.username, email);
            println!("ID: {:?}", user.id);
        }
        Ok(None) => println!("No user found with email {:?}", email),
        Err(err) => eprintln!("Error fetching user with email {:?}: {:?}", email, err),
    }
}

pub fn add_user(username: &str, email: &str) {
    let connection = &mut establish_connection();
    let password_hash = "password_hash"; // Placeholder for actual password hash generation

    match create_user(connection, username, email, password_hash) {
        Ok(user) => {
            println!("Created user: {:?}", user.username);
            println!("Email: {:?}", user.email);
            println!("Password Hash: {:?}", user.password_hash);
        }
        Err(err) => eprintln!("Error creating user: {:?}", err),
    }
}

pub fn update_user(user_id: i32, new_username: &str) {
    use database::schema::users::dsl::*;

    let connection = &mut establish_connection();
    match diesel::update(users.find(user_id))
        .set(username.eq(new_username))
        .returning(User::as_returning())
        .get_result::<User>(connection)
    {
        Ok(user) => println!("Updated user: {:?}", user.username),
        Err(err) => eprintln!("Error updating user with ID {:?}: {:?}", user_id, err),
    }
}

pub fn delete_user(user_id: i32) {
    use database::schema::users::dsl::*;

    let connection = &mut establish_connection();
    match diesel::delete(users.find(user_id))
        .get_result::<User>(connection)
    {
        Ok(user) => println!("Deleted user: {:?}", user.username),
        Err(err) => eprintln!("Error deleting user with ID {:?}: {:?}", user_id, err),
    }
}
