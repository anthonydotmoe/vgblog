use self::models::*;
use diesel::prelude::*;
use vgblog::*;

fn main() {
    use self::schema::games::dsl::*;

    let connection = &mut establish_connection();
    let results = games
        .select(Game::as_select())
        .load(connection)
        .expect("Error loading games");

    println!("Displaying {} games", results.len());

    for game in results {
        println!("{}", game.title);
        println!("{:?}", game.genre);
        println!("--------------\n");
    }
}