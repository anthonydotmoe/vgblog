use vgblog::*;
use std::io::stdin;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();

    println!("Enter game title:");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("Attempting to write new game: {}", title);

    let game = create_game(connection, title, None);
    println!("\nSaved game {} with id {}", title, game.id);
}