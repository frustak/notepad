#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod auth;
mod config;
mod db;
mod lib;
mod models;
mod routes;
mod schema;

use lib::DbConn;
use rocket_contrib::{serve::StaticFiles, templates::Template};
use routes::*;

fn main() {
    rocket::ignite()
        .attach(Template::fairing())
        .attach(DbConn::fairing())
        .mount("/", StaticFiles::from("static"))
        .mount(
            "/",
            routes![
                index::get,
                index::get_unauth,
                sign_up::get,
                sign_up::post,
                sign_in::get,
                sign_in::post,
                sign_out::get,
                notepad::post,
                error::get,
            ],
        )
        .launch();
}
