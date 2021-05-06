#![feature(proc_macro_hygiene, decl_macro)]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use dotenv::dotenv;
use std::env;

use rocket::{catch, catchers, fairing::{Fairing, Info, Kind}, get, http::{Cookie, CookieJar}, launch, post, response::status::Created, routes, uri, Data, Request, State, Phase, Ignite};

pub mod api_keys;

use dreamtime_library::establish_connection;
use api_keys::ApiKey;
use crate::api_keys::Auth;

#[get("/")]
fn index(_auth: Auth) -> &'static str {
    return "Hello";
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let api_key = env::var("API_KEY").unwrap();

    rocket::build()
        .mount("/", routes![
            index
        ])
        .manage(ApiKey(api_key))
}