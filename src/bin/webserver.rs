#![feature(proc_macro_hygiene, decl_macro)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use dotenv::dotenv;
use std::env;

use rocket::{catch, catchers, fairing::{Fairing, Info, Kind}, get, http::{Cookie, CookieJar}, launch, post, response::status::Created, routes, uri, Data, Request, State, Phase, Ignite, Rocket, Build};
use serde::Deserialize;
use rocket_contrib::json::Json;

use dreamtime_library::*;
use dreamtime_library::api_keys::*;
use dreamtime_library::models::*;
use dreamtime_library::user_repository::UserRepository;
use std::sync::{RwLock, Mutex};

#[get("/")]
fn health(_auth: Auth) -> &'static str {
    return "Hello";
}

type SyncUserRepository = Mutex<Box<UserRepository>>;

#[post("/users/connect", format = "application/json", data = "<new_user>")]
fn users_connect<'r>(_auth: Auth, new_user: Json<NewUser>, repository: State<'r, SyncUserRepository>) -> Json<User> {
    let locked_repo = repository.lock().unwrap();
    Json(locked_repo.create_user(&new_user.0))
}

#[get("/users")]
fn users_all<'r>(_auth: Auth, repository: State<'r, SyncUserRepository>) -> Json<Vec<User>> {
    let locked_repo = repository.lock().unwrap();
    Json(locked_repo.get_users())
}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();
    let connection = establish_connection();

    rocket::build()
        .mount("/api/v1", routes![
            health,
            users_connect,
            users_all
        ])
        .manage(Mutex::new( Box::new(UserRepository { connection })))
        .manage(ApiKey(api_key))
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, Header};

    #[test]
    fn health() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client
            .get("/api/v1")
            .header(Header::new("X-Api-Key", "kirill@ch"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Hello");
    }

}