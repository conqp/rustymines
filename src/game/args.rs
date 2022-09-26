use clap::Parser;

#[derive(Parser)]
#[clap(name = "rustymines")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "0.1.0")]
#[clap(about = "A mine sweeping game written in Rust", long_about = None)]
pub struct Args {
    /// Optional name to operate on
    #[clap(short, long, value_parser)]
    width: usize,

    /// Sets a custom config file
    #[clap(short, long, value_parser)]
    height: usize,

    /// Turn debugging information on
    #[clap(short, long, value_parser)]
    mines: u8,
}
