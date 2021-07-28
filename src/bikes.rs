/* Diesel query builder */
use diesel::prelude::*;

/* Database macros */
use crate::schema::bikes;

/* Database data structs (Bike, NewBike) */
use crate::db;
use crate::models::*;

use rocket::State;
use rocket_contrib::json::Json;

#[get("/")]
pub fn list(db_conn: State<db::ConnectionPool>) -> Json<Vec<Bike>> {
    let conn = db_conn
        .get()
        .expect("Could not establish database connection");
    /* Get all our Bikes from database */
    let bikes: Vec<Bike> = bikes::table
        .select(bikes::all_columns)
        .load::<Bike>(&*conn)
        .expect("Whoops, like this went bananas!");

    Json(bikes)
}

#[post("/", format = "application/json", data = "<new_bike>")]
pub fn new(db_conn: State<db::ConnectionPool>, new_bike: Json<NewBike>) -> Json<Bike> {
    let conn = db_conn
        .get()
        .expect("Could not establish database connection");
    let bikes: Bike = diesel::insert_into(bikes::table)
        .values(new_bike.into_inner())
        .get_result(&*conn)
        .expect("Error saving new Bike");

    Json(bikes)
}

#[put("/<rid>", format = "application/json", data = "<bike>")]
pub fn update(rid: i32, bike: Json<NewBike>, db_conn: State<db::ConnectionPool>) -> Json<Bike> {
    let conn = db_conn
        .get()
        .expect("Could not establish database connection");
    let bike = diesel::update(bikes::table.find(rid))
        .set(bike.into_inner())
        .get_result::<Bike>(&*conn)
        .expect(&format!("Unable to find bike {:?}", rid));
    Json(bike)
}

#[delete("/<rid>", format = "application/json")]
pub fn delete(rid: i32, db_conn: State<db::ConnectionPool>) -> Json<Bike> {
    let conn = db_conn
        .get()
        .expect("Could not establish database connection");
    let bike = diesel::delete(bikes::table.find(rid))
        .get_result::<Bike>(&*conn)
        .expect(&format!("Unable to find bike {:?}", rid));
    Json(bike)
}
