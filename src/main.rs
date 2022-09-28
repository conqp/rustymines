mod game;
use game::Game;

mod io;
use io::read_repeat;

mod coordinate;
use coordinate::Coordinate;

fn main() {
    let result = Game::parse();

    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    } else {
        run_game(&mut result.unwrap())
    }
}

fn run_game(game: &mut Game) {
    while game.running() {
        let coordinate = read_repeat::<Coordinate>("Enter x coordinate: ");
        println!("You entered: {}x{}", coordinate.x(), coordinate.y());
        game.visit(coordinate.x(), coordinate.y());
    }

    println!("{}", game);
}
