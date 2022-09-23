mod game;
use game::Game;

fn main() {
    let (optional_game, error) = Game::new(5, 4, 8);

    if optional_game.is_some() {
        let mut game = optional_game.unwrap();
        game.toggle_flag(1, 2);
        game.visit(2, 2);
        println!("Game: {:#?}", game);
    } else {
        println!("Got error: {}.", error);
    }
}
