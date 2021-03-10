/* Diesel query builder */
use diesel::prelude::*;


/* Database macros */
use crate::{schema::*};

/* Database data structs (Apple, NewApple) */
use crate::models::*;
use crate::db;

use rocket::State;


use rocket_contrib::json::{Json};



/* List our inserted Apples */
#[get("/")]
pub fn list(db_conn: State<db::ConnectionPool>) -> Json<Vec<Apple>> {
    // let connection = crate::establish_connection();

    let conn = db_conn.get().expect("Could not establish database connection");
    /* Get all our Apples from database */
    let apples: Vec<Apple> = apples::table
        .select(apples::all_columns)
        .load::<Apple>(&*conn)
        .expect("Whoops, like this went bananas!");

    Json(apples)
}

#[post("/", format = "application/json", data = "<new_apple>")]
pub fn new(db_conn: State<db::ConnectionPool>, new_apple: Json<NewApple>) -> Json<Apple> {
    // let connection = crate::establish_connection();
    /* Get all our Apples from database */
    let conn = db_conn.get().expect("Could not establish database connection");
    let apples: Apple = diesel::insert_into(apples::table)
    .values(new_apple.into_inner())
    .get_result(&*conn)
    .expect("Error saving new Apple");

    Json(apples)
}