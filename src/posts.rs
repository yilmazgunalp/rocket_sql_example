/* Diesel query builder */
use diesel::prelude::*;


/* Database macros */
use crate::{schema::*};

/* Database data structs (Post, NewPost) */
use crate::models::*;
use crate::db;

use rocket::State;


use rocket_contrib::json::{Json};



/* List our inserted posts */
#[get("/")]
pub fn list(db_conn: State<db::ConnectionPool>) -> Json<Vec<Post>> {
    // let connection = crate::establish_connection();

    let conn = db_conn.get().expect("Could not establish database connection");
    /* Get all our posts from database */
    let posts: Vec<Post> = posts::table
        .select(posts::all_columns)
        .load::<Post>(&*conn)
        .expect("Whoops, like this went bananas!");

    Json(posts)
}

#[post("/", format = "application/json", data = "<new_post>")]
pub fn new(db_conn: State<db::ConnectionPool>, new_post: Json<NewPost>) -> Json<Post> {
    // let connection = crate::establish_connection();
    /* Get all our posts from database */
    let conn = db_conn.get().expect("Could not establish database connection");
    let posts: Post = diesel::insert_into(posts::table)
    .values(new_post.into_inner())
    .get_result(&*conn)
    .expect("Error saving new post");

    Json(posts)
}