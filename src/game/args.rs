use clap::Parser;

pub trait GameArgs {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn mines(&self) -> u8;
}

#[derive(Parser)]
#[clap(name = "rustymines")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "0.1.0")]
#[clap(about = "A mine sweeping game written in Rust.", long_about = None)]
struct GameArgsParser {
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

pub fn parse() -> impl GameArgs {
    GameArgsParser::parse()
}

impl GameArgs for GameArgsParser {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn mines(&self) -> u8 {
        self.mines
    }
}
