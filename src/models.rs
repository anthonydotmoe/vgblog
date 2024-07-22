use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub release_date: Option<chrono::NaiveDate>,
    pub genre: Option<String>,
    pub platform: Option<String>,
}