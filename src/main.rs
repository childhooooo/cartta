#![feature(proc_macro_hygiene, decl_macro, custom_attribute, extern_crate_item_prelude)]
#![allow(proc_macro_derive_resolution_fallback, dead_code)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate validator_derive;
#[macro_use] extern crate tera;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate validator;
extern crate bcrypt;
extern crate chrono;
extern crate parking_lot;
extern crate time;
extern crate ammonia;
extern crate pulldown_cmark;
extern crate htmlescape;
extern crate xss;

#[macro_use] mod helpers;
mod schema;
mod contexts;
mod web;

#[cfg(test)] mod tests;

use self::web::app;

fn main() {
    app::rocket().0.launch();
}
