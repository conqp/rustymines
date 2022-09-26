use clap::{App, Arg, ArgMatches};

pub struct Args {
    pub width: usize,
    pub height: usize,
    pub mines: u8,
}

impl Args {
    pub fn parse() -> Result<Self, &'static str> {
        let matches = get_matches();
        let mut args = Self {
            width: 0,
            height: 0,
            mines: 0,
        };

        match matches.value_of("width") {
            None => return Err("no width provided"),
            Some(s) => match s.parse::<usize>() {
                Ok(width) => args.width = width,
                Err(_) => return Err("invalid width"),
            },
        }

        match matches.value_of("height") {
            None => return Err("no height provided"),
            Some(s) => match s.parse::<usize>() {
                Ok(height) => args.height = height,
                Err(_) => return Err("invalid height"),
            },
        }

        match matches.value_of("mines") {
            None => return Err("no mines provided"),
            Some(s) => match s.parse::<u8>() {
                Ok(mines) => args.mines = mines,
                Err(_) => return Err("invalid amount of mines"),
            },
        }

        Ok(args)
    }
}

fn get_matches() -> ArgMatches {
    App::new("rustymines")
        .version("0.1.0")
        .author("Richard Neumann <mail@richard-neumann.de>")
        .about("A mine sweeping game written in Rust")
        .arg(
            Arg::with_name("width")
                .short('x')
                .long("width")
                .takes_value(true)
                .help("Width of the board"),
        )
        .arg(
            Arg::with_name("height")
                .short('y')
                .long("height")
                .takes_value(true)
                .help("Height of the board"),
        )
        .arg(
            Arg::with_name("amount")
                .short('m')
                .long("mines")
                .takes_value(true)
                .help("Amount of mines on the board"),
        )
        .get_matches()
}
