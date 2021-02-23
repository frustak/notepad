#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

mod auth;
mod config;
mod db;
mod lib;
mod models;
mod routes;
mod schema;

use lib::DbConn;
use rocket_contrib::templates::Template;
use routes::*;

fn main() {
    // pretty_env_logger::init();

    rocket::ignite()
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .mount(
            "/",
            routes![
                index::get,
                sign_up::get,
                sign_up::post,
                sign_in::get,
                sign_in::post,
                notepad::get,
                notepad::get_unauth,
                notepad::post,
            ],
        )
        .launch();
}
