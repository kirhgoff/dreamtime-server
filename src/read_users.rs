extern crate diesel;
extern crate dotenv;
extern crate dreamland_library;

use diesel::prelude::*;

use self::dreamland_library::*;
use self::models::*;

fn main() {
    use dreamland_library::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    println!("\n----------");
    for user in results {
        println!("{} <{}>", user.name, user.email);
    }
}