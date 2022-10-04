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
#[clap(version = "1.0.0")]
#[clap(about = "A mine sweeping game written in Rust.", long_about = None)]
struct GameArgsParser {
    #[clap(short, long, value_parser, default_value_t = 5)]
    width: usize,

    #[clap(short = 'H', long, value_parser, default_value_t = 5)]
    height: usize,

    #[clap(short, long, value_parser, default_value_t = 8)]
    mines: u8,

    #[clap(short, long, value_parser, default_value_t = 0)]
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
