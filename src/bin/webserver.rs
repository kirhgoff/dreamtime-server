#![feature(proc_macro_hygiene, decl_macro)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use dotenv::dotenv;
use std::env;

use rocket::{catch, catchers, fairing::{Fairing, Info, Kind}, get, http::{Cookie, CookieJar}, launch, post, response::status::Created, routes, uri, Data, Request, State, Phase, Ignite};
use serde::Deserialize;
use rocket_contrib::json::Json;

use dreamtime_library::*;
use dreamtime_library::api_keys::*;
use dreamtime_library::models::*;
use dreamtime_library::user_repository::UserRepository;
use std::sync::RwLock;


type SyncUserRepository = RwLock<UserRepository>;

#[get("/")]
fn health(_auth: Auth) -> &'static str {
    return "Hello";
}

#[post("/users/connect", format = "application/json", data = "<new_user>")]
fn connect<'r>(_auth: Auth, new_user: Json<NewUser>, state: State<'r, SyncUserRepository>) -> &'static str {
    return "Hello";
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();
    let connection = establish_connection();

    rocket::build()
        .mount("/api/v1", routes![
            health
        ])
        .manage(RwLock::new( UserRepository { connection }))
        .manage(ApiKey(api_key))
}