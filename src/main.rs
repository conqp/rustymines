mod action;
use action::Action;
use action::ActionKind;

mod game;
use game::Game;

mod io;
use io::read;

fn main() {
    match Game::from_args() {
        Ok(mut game) => {
            println!("{}", game);

            while !game.over() {
                let action: Action = read("Enter action ([!]x y): ");

                match action.kind() {
                    ActionKind::Visit => game.visit(&action.coordinate()),
                    ActionKind::ToggleFlag => game.toggle_flag(&action.coordinate()),
                }
            }
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
