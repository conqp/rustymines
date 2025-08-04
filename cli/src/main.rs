//! A mine sweeping game with optional dud mines.

use std::process::ExitCode;

use action::Action;
use args::Args;
use clap::Parser;
use io::read_until_valid;
use rustymines::{Game, State};

mod action;
mod args;
mod io;

const BASE: u32 = 34;

const HELP: [&str; 4] = [
    "Visit a field:                x y",
    "Toggle flag on a field:       !x y",
    "Visit all non-flagged fields: !!",
    "Abort:                        exit",
];

fn main() -> ExitCode {
    match Game::try_from(Args::parse()) {
        Ok(mut game) => {
            println!("{game}\n");
            print_help();

            while let Some(state) = get_action().and_then(|action| game.next_round(action)) {
                match state {
                    State::GameOver { won } => {
                        println!("{game}\n");

                        if won {
                            println!("\nYou won the game.\nTime: {:?}", game.duration());
                            return ExitCode::SUCCESS;
                        }

                        println!("\nYou lost the game.");
                        return ExitCode::FAILURE;
                    }
                    State::InvalidMove => println!("Invalid move."),
                    State::Continue => println!("{game}\n"),
                }
            }
        }
        Err(msg) => eprintln!("Error: {msg}"),
    }

    ExitCode::FAILURE
}

fn print_help() {
    for line in HELP {
        println!("{line}");
    }
}

fn get_action() -> Option<rustymines::Action> {
    match read_until_valid("Enter action: ") {
        Action::Action(action) => Some(action),
        Action::Abort => {
            println!("Bye!");
            None
        }
    }
}
