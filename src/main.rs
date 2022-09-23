mod game;
use game::Game;

fn main() {
    let mut game = Game::new(3, 4, 5);
    game.toggle_flag(1, 2);
    game.visit(2, 2);
    println!("Game: {:#?}", game);
}
