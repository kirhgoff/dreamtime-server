extern crate dreamtime_library;
extern crate diesel;

use std::io::*;

use dreamtime_library::*;
use std::process;

use dreamtime_library::user_repository::UserRepository;

fn main() {
    let connection = establish_connection();
    let repository = UserRepository { connection: &connection };
    println!("Database console v.1");
    println!("=========================================");
    loop {
        let command = read_value("Enter command (create/find/all)>");
        match command {
            command if command.starts_with("create") => create_user(command, &repository),
            command if command.starts_with("all") => get_all_users(command, &repository),
            command if command.starts_with("quit") => process::exit(0x0100),
            _ => continue
        }
    }
}

fn create_user(_command: String, repository: &UserRepository) {
    println!(">>>>>>>> Creating a user, enter the details");
    let name = read_value("Name: ");
    let email = read_value("Email: ");
    let password = read_value("Password: ");

    let user = repository.create_user(&name, &email, &password);
    println!("\nCreated user {} with id {}", name, user.id);
}

fn get_all_users(_command: String, repository: &UserRepository) {
    println!(">>>>>>>> Listing all users");

    let user_list = repository.get_users();
    println!("Displaying {} users", user_list.len());
    println!("\n----------");
    for user in user_list {
        println!("{} <{}>", user.name, user.email);
    }
}

fn read_value(comment: &str) -> String {
    println!("{}", comment);
    let mut value = String::new();
    stdin().read_line(&mut value).unwrap();
    return String::from(&value[..(value.len() - 1)]);
}

