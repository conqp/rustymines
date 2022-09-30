mod coordinate;
use coordinate::Coordinate;

mod game;
use game::Game;

mod io;
use io::read_repeat;

fn main() {
    let result = Game::parse();

    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    } else {
        run_game(&mut result.unwrap())
    }
}

fn run_game(game: &mut Game) {
    println!("{}", game);

    while game.running() {
        let coordinate = read_repeat::<Coordinate>("Enter coordinate: ");
        game.visit(coordinate.x(), coordinate.y());
    }

    println!("{}", game);
}
