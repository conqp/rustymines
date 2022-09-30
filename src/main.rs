mod game;
use game::Game;

mod io;
use io::read;

fn main() {
    match Game::parse() {
        Ok(mut game) => {
            println!("{}", game);

            while !game.over() {
                game.visit(&read("Enter coordinate: "));
            }
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
