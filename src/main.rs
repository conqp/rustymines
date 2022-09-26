mod game;
use game::Game;

fn main() {
    let mut game = Game::new(5, 4, 8).unwrap();
    game.toggle_flag(1, 2);
    game.visit(2, 2);
    println!("Game: {:#?}", game);
}
