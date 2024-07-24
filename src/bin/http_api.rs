#[macro_use]
extern crate rocket;

use self::models::*;
use rocket::{fs::FileServer, http::hyper::server::conn, serde::json::Json};
use diesel::prelude::*;
use vgblog::*;

// Read Routes

#[get("/games")]
async fn all_games() -> Json<Vec<Game>> {
    use self::schema::games::dsl::*;

    let connection = &mut establish_connection();
    let results = games
        .select(Game::as_select())
        .load(connection);

    results.unwrap().into()
}

#[get("/attributes")]
async fn all_attributes() -> Json<Vec<Attribute>> {
    use self::schema::attributes::dsl::*;

    let connection = &mut establish_connection();
    let results = attributes
        .select(Attribute::as_select())
        .load(connection);

    results.unwrap().into()
}

// Create Routes

#[post("/attributes", format = "application/json", data = "<attr>")]
async fn add_attribute(attr: Json<NewAttribute<'_>>) -> Json<Attribute> {
    use self::schema::attributes::dsl::*;

    let connection = &mut establish_connection();
    let attribute = create_attribute(connection, attr.name, attr.description, attr.parent_id);

    attribute.into()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/api", routes![all_games, all_attributes, add_attribute])
}