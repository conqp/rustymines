use std::str::FromStr;

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
            println!("Visit a field:                x y");
            println!("Toggle flag on a field:       !x y");
            println!("Visit all non-flagged fields: !!");

            while !game.over() {
                match read::<String>("Enter action: ").trim() {
                    "!!" => game.visit_unflagged_fields(),
                    input => match Action::from_str(input) {
                        Ok(action) => match action.kind() {
                            ActionKind::Visit => game.visit(&action.coordinate()),
                            ActionKind::ToggleFlag => game.toggle_flag(&action.coordinate()),
                        },
                        Err(msg) => eprintln!("Error: {}", msg),
                    },
                }
            }
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}
