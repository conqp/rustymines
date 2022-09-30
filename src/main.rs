mod coordinate;
use coordinate::Coordinate;

mod game;
use game::Game;

mod io;
use io::read_repeat;

fn main() {
    match Game::parse() {
        Ok(mut game) => run_game(&mut game),
        Err(msg) => eprintln!("Error: {}", msg),
    }
}

fn run_game(game: &mut Game) {
    println!("{}", game);

    while !game.over() {
        let coordinate = read_repeat::<Coordinate>("Enter coordinate: ");
        game.visit(coordinate.x(), coordinate.y());
    }
}
