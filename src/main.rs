extern crate dotenv;
extern crate schani_library;

use std::env;

use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL required");
    let web_service = schani_library::LibraryWebService::new(database_url.as_str());
    web_service.run();
}
