//! A mine sweeping game with optional dud mines.

use args::Args;
use clap::Parser;
use rustymines::{Board, Game, State};

use crate::io::read_until_valid;

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
                let Some(state) = game.next_round(read_until_valid("Enter action: ")) else {
                    break;
                };

                println!("{game}\n");
                match state {
                    State::GameOver { won } => {
                        if won {
                            println!("\nYou won the game.\nTime: {:?}", game.duration());
                        } else {
                            println!("\nYou lost the game.");
                        }

                        break;
                    }
                    State::Aborted => println!("Bye!"),
                    State::InvalidMove => println!("Invalid move."),
                    State::Continue => (),
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
