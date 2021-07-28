use crate::schema::*;
use serde::*;
#[derive(Debug, Queryable, Serialize, Deserialize, AsChangeset)]
pub struct Bike {
    pub id: i32,
    pub title: std::string::String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name = "bikes"]
pub struct NewBike {
    pub title: String,
    pub body: String,
    pub published: bool,
}
