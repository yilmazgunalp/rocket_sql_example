/* For beeing able to serialize */
use crate::schema::*;
use serde::*;
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Apple {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Insertable, AsChangeset, Serialize, Deserialize)]
#[table_name="apples"]
pub struct NewApple {
    pub title: String,
    pub body: String,
    pub published: bool,
}