#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

mod controllers;
use controllers::admin;
use controllers::frontend;

mod utils;

fn main() {
    rocket::ignite()
        .mount("/admin", routes![admin::home_controller::index])
        .mount("/", routes![frontend::home_controller::index])
        .launch();
}
