#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

mod utils;
mod controllers;

use controllers::{admin, frontend};

fn main() {
    rocket::ignite()
        .mount("/admin", routes![admin::home_controller::index])
        .mount("/", routes![frontend::home_controller::index])
        .launch();
}
