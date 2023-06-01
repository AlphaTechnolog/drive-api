#[macro_use] extern crate rocket;

mod path;
mod config;
mod reader;
mod error;
mod log;
mod routes;
mod controllers;

use config::Config;
use rocket::fairing::AdHoc;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/core", routes![routes::core::listing_by_dirname, routes::core::read_file, routes::core::remove])
        .attach(AdHoc::config::<Config>())
}