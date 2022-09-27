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
    while game.running() {
        let x = read_repeat::<usize>("Enter x coordinate: ");
        println!("You entered: x = {}", x);
    }
}
