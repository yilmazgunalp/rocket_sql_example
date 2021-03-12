/* Diesel query builder */
use diesel::prelude::*;


/* Database macros */
use crate::{schema::*};

/* Database data structs (Bike, NewBike) */
use crate::models::*;
use crate::db;

use rocket::State;


use rocket_contrib::json::{Json};



/* List our inserted Bikes */
#[get("/")]
pub fn list(db_conn: State<db::ConnectionPool>) -> Json<Vec<Bike>> {
    // let connection = crate::establish_connection();

    let conn = db_conn.get().expect("Could not establish database connection");
    /* Get all our Bikes from database */
    let bikes: Vec<Bike> = bikes::table
        .select(bikes::all_columns)
        .load::<Bike>(&*conn)
        .expect("Whoops, like this went bananas!");

    Json(bikes)
}

#[post("/", format = "application/json", data = "<new_bike>")]
pub fn new(db_conn: State<db::ConnectionPool>, new_bike: Json<NewBike>) -> Json<Bike> {
    // let connection = crate::establish_connection();
    /* Get all our Bikes from database */
    let conn = db_conn.get().expect("Could not establish database connection");
    let bikes: Bike = diesel::insert_into(bikes::table)
    .values(new_bike.into_inner())
    .get_result(&*conn)
    .expect("Error saving new Bike");

    Json(bikes)
}