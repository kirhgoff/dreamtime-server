use serde::{Deserialize, Serialize};
use super::schema::users;
use postgis_diesel::sql_types::*;
use postgis_diesel::PointC;
use postgis::ewkb::Point;

#[derive(Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub geom: PointC<Point>,
}

#[derive(Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub geom: PointC<Point>,
}