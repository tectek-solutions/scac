use actix_web::web;
use database;
use database::model::{NewUser, User};
use diesel::prelude::*;

pub fn get_users(
    database: &web::Data<database::Database>,
) -> Result<Option<Vec<User>>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();
    let result = users.load::<User>(&mut database_connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting users: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_user_by_id(
    database: &web::Data<database::Database>,
    user_id: i32,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();

    match users
        .find(user_id)
        .select(User::as_select())
        .first::<User>(&mut database_connection)
        .optional()
    {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting user with ID {:?}: {:?}", user_id, err);
            Err(err)
        }
    }
}

pub fn get_user_by_email(
    database: &web::Data<database::Database>,
    user_email: &String,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();
    match users
        .filter(email.eq(user_email.clone()))
        .select(User::as_select())
        .first::<User>(&mut database_connection)
        .optional()
    {
        Ok(Some(user)) => Ok(Some(user)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting user with email {:?}: {:?}", user_email, err);
            Ok(None)
        }
    }
}

pub fn add_user(
    database: &web::Data<database::Database>,
    name: String,
    email: String,
    password_hash: String,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users;

    let mut database_connection = database.get_connection();

    let new_user = NewUser {
        username: &name,
        email: &email,
        password_hash: &password_hash,
    };

    match diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result::<User>(&mut database_connection)
    {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error adding user: {:?}", err);
            Err(err)
        }
    }
}

pub fn update_user(
    database: &web::Data<database::Database>,
    user_id: i32,
    new_name: String,
    new_email: String,
    new_password_hash: String,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();
    match diesel::update(users.find(user_id))
        .set((
            username.eq(new_name.clone()),
            email.eq(new_email.clone()),
            password_hash.eq(new_password_hash.clone()),
        ))
        .returning(User::as_returning())
        .get_result::<User>(&mut database_connection)
    {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error updating user with ID {:?}: {:?}", user_id, err);
            Ok(None)
        }
    }
}

pub fn delete_user(
    database: &web::Data<database::Database>,
    user_id: i32,
) -> Result<Option<User>, diesel::result::Error> {
    use database::schema::users::dsl::*;

    let mut database_connection = database.get_connection();
    match diesel::delete(users.find(user_id)).get_result::<User>(&mut database_connection) {
        Ok(user) => Ok(Some(user)),
        Err(err) => {
            eprintln!("Error deleting user with ID {:?}: {:?}", user_id, err);
            Err(err)
        }
    }
}
