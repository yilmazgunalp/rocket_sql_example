#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

pub mod models;
pub mod bikes;
pub mod schema;
pub mod db;

use rocket::config::{Config, Environment};

use std::env;

embed_migrations!();

fn main() {
    let connection = db::establish_connection();
    
    match embedded_migrations::run(&connection) {
        Ok(_) => println!("yahoo"),
        Err(e) => eprintln!("Oh noes, we don't know which era we're in! :( \n  {}", e),
    }

    let port = env::var("PORT").unwrap().parse::<u16>().expect("$PORT must be set");

    let config = Config::build(Environment::Production)
    .port(port)
    .finalize().expect("Configuration error");

    rocket::custom(config)
    .manage(db::establish_connection_pool())
    .mount("/", routes![
        bikes::list,
        bikes::new
        ]).launch();
}
