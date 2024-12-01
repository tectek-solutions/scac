use self::models::*;
use diesel::prelude::*;
use diesel_demo::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(username.like("user%"))
        .limit(5)
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading users");
    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user.username);
        println!("----------\n");
        println!("{:?}", user.email);
    }

}