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
use routes::core::*;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/core", routes![listing_by_dirname, read_file, remove, upload])
        .attach(AdHoc::config::<Config>())
}