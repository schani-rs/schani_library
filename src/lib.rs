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

mod database;
mod models;
mod service;
mod web;

pub use web::webservice::LibraryWebService;
