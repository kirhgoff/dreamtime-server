extern crate dreamland_library;
extern crate diesel;

use self::dreamland_library::*;
use std::io::*;

fn read_value(comment: &str) -> String {
    println!("{}", comment);
    let mut value = String::new();
    stdin().read_line(&mut value).unwrap();
    return String::from(&value[..(value.len() - 1)]);
}

fn main() {
    let connection = establish_connection();

    println!("New user?");
    let name = read_value("Name: ");
    let email = read_value("Email: ");
    let password = read_value("Password: ");

    let user = create_user(&connection, &name, &email, &password);
    println!("\nCreated user {} with id {}", name, user.id);
}