pub mod models;
pub mod schema;

use models::{Attribute, Game, NewAttribute, NewGame};

use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_game(conn: &mut SqliteConnection, title: &str, note: Option<&str>) -> Game {
    use crate::schema::games;

    let new_game = NewGame{title, note};
    
    diesel::insert_into(games::table)
        .values(&new_game)
        .returning(Game::as_returning())
        .get_result(conn)
        .expect("Error saving new game")
}

pub fn create_attribute(conn: &mut SqliteConnection, name: &str, description: Option<&str>, parent_id: Option<i32>) -> Attribute {
    use crate::schema::attributes;

    let new_attribute = NewAttribute{ name, description, parent_id };

    diesel::insert_into(attributes::table)
        .values(&new_attribute)
        .returning(Attribute::as_returning())
        .get_result(conn)
        .expect("Error saving new attribute")
}