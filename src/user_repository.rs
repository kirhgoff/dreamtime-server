extern crate diesel;

use diesel::prelude::*;

use crate::models::{User, NewUser};
use crate::PgPool;

pub struct UserRepository {
    pub connection: PgPool
}

impl UserRepository {
    // pub fn find_user(&self, email: &str) -> User {
    //     let new_user = NewUser { name, email, password };
    //     diesel::find(users::table)
    //         .values(&new_user)
    //         .get_result(&self.connection)
    //         .expect("Error saving new user")
    // }

    pub fn create_user<'a>(&self, new_user: &'a NewUser) -> User {
        use crate::schema::users;

        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(&self.connection.get().unwrap())
            .expect("Error saving new user")
    }

    pub fn get_users(&self) -> Vec<User> {
        use crate::schema::users::dsl::*;

        // TODO: how to select all limit xx
        users
            .load::<User>(&self.connection.get().unwrap())
            .expect("Error loading users")
    }
}