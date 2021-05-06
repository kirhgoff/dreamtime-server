#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub fullName: String,
    pub email: String,
    pub password: String,
}