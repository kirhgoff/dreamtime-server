extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::models::{User, NewUser};

pub struct UserRepository {
    pub connection: PgConnection
}

impl UserRepository {
    // pub fn find_user(&self, email: &str) -> User {
    //     let new_user = NewUser { name, email, password };
    //     diesel::find(users::table)
    //         .values(&new_user)
    //         .get_result(&self.connection)
    //         .expect("Error saving new user")
    // }

    pub fn create_user<'a>(&self, name: &'a str, email: &'a str, password: &'a str) -> User {
        use crate::schema::users;

        let new_user = NewUser { name, email, password };
        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&self.connection)
            .expect("Error saving new user")
    }

    pub fn get_users(&self) -> Vec<User> {
        use crate::schema::users::dsl::*;

        // TODO: how to select all limit xx
        users
            .load::<User>(&self.connection)
            .expect("Error loading users")
    }
}