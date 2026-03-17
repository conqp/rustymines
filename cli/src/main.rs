//! A mine sweeping game with optional dud mines.

use std::process::ExitCode;

use clap::Parser;
use rustymines::{Game, Outcome, State};

use self::action::Action;
use self::args::Args;
use self::io::read_until_valid;

mod action;
mod args;
mod io;

const HELP: [&str; 4] = [
    "Visit a field:                x y",
    "Toggle flag on a field:       !x y",
    "Visit all non-flagged fields: !!",
    "Abort:                        exit | quit | q",
];

fn main() -> ExitCode {
    match Game::try_from(Args::parse()) {
        Ok(mut game) => {
            println!("{game}\n");
            print_help();

            while let Some(state) = get_action().and_then(|action| game.next_round(action)) {
                match state {
                    State::GameOver(outcome) => {
                        println!("{game}\n");
                        return match outcome {
                            Outcome::Won(end) => {
                                println!(
                                    "\nYou won the game.\nTime: {:?}",
                                    end.duration_since(game.start())
                                );
                                ExitCode::SUCCESS
                            }
                            Outcome::Lost(_) => {
                                println!("\nYou lost the game.");
                                ExitCode::FAILURE
                            }
                        };
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
