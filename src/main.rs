use std::str::FromStr;

mod action;
use action::{Action, ActionKind};

mod game;
use game::Game;

mod io;
use io::read;

const HELP: [&str; 4] = [
    "Visit a field:                x y",
    "Toggle flag on a field:       !x y",
    "Visit all non-flagged fields: !!",
    "Abort:                        exit",
];

fn main() {
    match Game::from_args() {
        Ok(mut game) => {
            println!("{}", game);
            print_help();
            play_game(&mut game);
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}

fn play_game(game: &mut Game) {
    while !game.over() {
        match read::<String>("Enter action: ").trim() {
            "!!" => game.visit_unflagged_fields(),
            input => match Action::from_str(input) {
                Ok(action) => match action.kind() {
                    ActionKind::Visit => game.visit(&action.coordinate().unwrap()),
                    ActionKind::ToggleFlag => game.toggle_flag(&action.coordinate().unwrap()),
                    ActionKind::Exit => {
                        println!("Bye!");
                        break;
                    }
                },
                Err(msg) => eprintln!("Error: {}", msg),
            },
        }
    }
}

fn print_help() {
    for line in HELP {
        println!("{}", line);
    }
}
