extern crate diesel;
extern crate dotenv;
extern crate dreamtime_library;

use diesel::prelude::*;

use models::*;
use dreamtime_library::*;

fn main() {
    use dreamtime_library::schema::users::dsl::*;

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
