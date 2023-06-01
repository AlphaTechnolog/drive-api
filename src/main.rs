#[macro_use] extern crate rocket;

mod path;
mod config;
mod reader;
mod error;
mod routes;

use config::Config;
use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/core", routes![routes::core::listing_by_dirname])
        .attach(AdHoc::config::<Config>())
}