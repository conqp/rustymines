mod game;
use game::Game;

fn main() {
    let mut game = Game::new(3, 4, 5);
    game.visit(2, 2);
}
