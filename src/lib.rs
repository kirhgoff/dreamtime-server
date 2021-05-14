#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod api_keys;
pub mod user_repository;

use self::models::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Cannot create a db pool")
}

pub fn create_user<'a>(conn: &PgConnection, name: &'a str, email: &'a str, password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser { name, email, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Error saving new user")
}