use clap::Parser;

pub trait GameArgs {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn mines(&self) -> u8;
    fn duds(&self) -> u8;
}

#[derive(Parser)]
#[clap(name = "rustymines")]
#[clap(author = "Richard Neumann <mail@richard-neumann.de>")]
#[clap(version = "0.1.0")]
#[clap(about = "A mine sweeping game written in Rust.", long_about = None)]
struct GameArgsParser {
    #[clap(short, long, value_parser)]
    width: usize,

    #[clap(short, long, value_parser)]
    height: usize,

    #[clap(short, long, value_parser)]
    mines: u8,

    #[clap(short, long, value_parser)]
    duds: u8,
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

    fn duds(&self) -> u8 {
        self.duds
    }
}
