//! A mine sweeping game with optional dud mines.

use action::Action;
use args::Args;
use clap::Parser;
use io::read_until_valid;
use rustymines::{Board, Game, State};

mod action;
mod args;
mod io;

const HELP: [&str; 4] = [
    "Visit a field:                x y",
    "Toggle flag on a field:       !x y",
    "Visit all non-flagged fields: !!",
    "Abort:                        exit",
];

fn main() {
    match Board::try_from(Args::parse()).map(Game::new) {
        Ok(mut game) => {
            println!("{game}\n");
            print_help();

            loop {
                let action = match read_until_valid("Enter action: ") {
                    Action::Action(action) => action,
                    Action::Abort => {
                        println!("Bye!");
                        break;
                    }
                };

                let Some(state) = game.next_round(action) else {
                    break;
                };

                match state {
                    State::GameOver { won } => {
                        println!("{game}\n");

                        if won {
                            println!("\nYou won the game.\nTime: {:?}", game.duration());
                        } else {
                            println!("\nYou lost the game.");
                        }

                        break;
                    }
                    State::InvalidMove => println!("Invalid move."),
                    State::Continue => println!("{game}\n"),
                }
            }
        }
        Err(msg) => eprintln!("Error: {msg}"),
    }
}

fn print_help() {
    for line in HELP {
        println!("{line}");
    }
}
