
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;


pub mod models;
pub mod posts;
pub mod schema;
pub mod db;




use rocket::config::{Config, Environment};

use std::env;

embed_migrations!("migrations");



fn main() {
    let port = env::var("PORT").unwrap().parse::<u16>().expect("$PORT must be set");

    let config = Config::build(Environment::Production)
    .address("127.0.0.1")
    .port(port)
    .finalize().expect("Configuration error");

    rocket::custom(config)
    .manage(db::establish_connection_pool())
    .mount("/", routes![
        posts::list,
        posts::new
        ]).launch();
}
