use serde::{Deserialize, Serialize};

use super::schema::users;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}