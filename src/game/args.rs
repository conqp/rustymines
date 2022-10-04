use clap::Parser;

#[derive(Parser)]
#[clap(name = "rustymines")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "1.0.0")]
#[clap(about = "A mine sweeping game written in Rust.", long_about = None)]
pub struct Args {
    #[clap(short, long, name = "width", value_parser, default_value_t = 5)]
    pub width: usize,

    #[clap(short = 'H', long, name = "height", value_parser, default_value_t = 5)]
    pub height: usize,

    #[clap(short, long, name = "mines", value_parser, default_value_t = 8)]
    pub mines: u8,

    #[clap(short, long, name = "duds", value_parser, default_value_t = 0)]
    pub duds: u8,
}
