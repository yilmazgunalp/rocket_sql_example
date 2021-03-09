
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
// #[macro_use] extern crate figment;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate rocket_contrib;
// extern crate dotenv;
extern crate r2d2_diesel;


pub mod models;
pub mod posts;
pub mod schema;
pub mod db;




use figment::{Figment, providers::{Format, Toml, Env}};
use std::env;

embed_migrations!("migrations");


fn main() {
    let port = env::var("PORT").unwrap().parse::<u16>().expect("$PORT must be set");

    // let figment = Figment::new()
    // .merge(Toml::file("Rocket.toml"))
    // .merge(Env::prefixed("ROCKET_"));
    // let figment = Figment::from(rocket::Config::default())
    // .merge(("port", port));
    rocket::ignite()
    .manage(db::establish_connection_pool())
    .mount("/", routes![
        posts::list,
        posts::new
        ]).launch();
}
