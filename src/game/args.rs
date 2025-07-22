use std::num::NonZero;

use clap::Parser;

const DEFAULT_SIZE: NonZero<usize> = NonZero::new(8).expect("Default size should be non-zero.");

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Args {
    #[clap(short, long, name = "width", value_parser, default_value_t = DEFAULT_SIZE)]
    pub width: NonZero<usize>,
    #[clap(short = 'H', long, name = "height", value_parser, default_value_t = DEFAULT_SIZE)]
    pub height: NonZero<usize>,
    #[clap(short, long, name = "mines", value_parser, default_value_t = 10)]
    pub mines: u8,
    #[clap(short, long, name = "duds", value_parser, default_value_t = 0)]
    pub duds: u8,
}
