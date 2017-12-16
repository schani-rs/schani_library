#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate fern;
extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate gotham_middleware_diesel;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate mime;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

mod database;
mod models;
mod service;
mod web;

pub use web::webservice::LibraryWebService;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
