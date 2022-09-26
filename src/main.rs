mod game;
use game::Game;

fn main() {
    let result = Game::new(5, 4, 8);

    if result.is_err() {
        println!("Error: {}", result.err().unwrap());
    } else {
        let mut game = result.unwrap();
        game.toggle_flag(1, 2);
        game.visit(2, 2);
        println!("Game: {:#?}", game);
    }
}
