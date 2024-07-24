use diesel::prelude::*;
use rocket::serde::{Serialize, Deserialize};
use crate::schema::{games, attributes};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub note: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewGame<'a> {
    pub title: &'a str,
    pub note: Option<&'a str>,
}

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = attributes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Attribute {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = attributes)]
pub struct NewAttribute<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub parent_id: Option<i32>,
}