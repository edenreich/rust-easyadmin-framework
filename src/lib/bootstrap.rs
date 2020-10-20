#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod controllers;
use controllers::admin;
use controllers::frontend;

fn main() {
    rocket::ignite()
        .mount("/admin", routes![admin::home_controller::index])
        .mount("/", routes![frontend::home_controller::index])
        .launch();
}
