use database::*;
use std::io::{stdin};

fn main () {
    let connection = &mut etablish_connection();

    let mut username = String::new();
    let mut email = String::new();

    println!("Username: ");
    stdin().read_line(&mut username).unwrap();
    let username = username.trim_end();

    println!("Email: ");
    stdin().read_line(&mut email).unwrap();
    let email = email.trim_end();

    let password_hash = "password";

    let user = create_user(connection, username, email, password_hash);

    println!("Saved user {:?}", user.username);
    println!("Saved email {:?}", user.email);
    println!("Saved password_hash {:?}", user.password_hash);
}

#[cfg(test)]
const EOF: &str = "CTRL+D";