use std::num::NonZero;

use clap::Parser;
use rustymines::{Error, Game};

const DEFAULT_SIZE: NonZero<usize> = NonZero::new(8).expect("Default size should be non-zero.");

/// Command line arguments.
#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    /// The width of the game board.
    #[clap(short, long, name = "width", value_parser, default_value_t = DEFAULT_SIZE)]
    pub width: NonZero<usize>,
    /// The height of the game board.
    #[clap(short = 'H', long, name = "height", value_parser, default_value_t = DEFAULT_SIZE)]
    pub height: NonZero<usize>,
    /// The amount of mines on the game board.
    #[clap(short, long, name = "mines", value_parser, default_value_t = 10)]
    pub mines: u8,
    /// The amount of duds among the mines.
    #[clap(short, long, name = "duds", value_parser, default_value_t = 0)]
    pub duds: u8,
}

impl TryFrom<Args> for Game {
    type Error = Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Self::new(args.width, args.height, args.mines, args.duds)
    }
}
