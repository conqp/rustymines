mod game;
use game::Game;

const HELP: [&str; 4] = [
    "Visit a field:                x y",
    "Toggle flag on a field:       !x y",
    "Visit all non-flagged fields: !!",
    "Abort:                        exit",
];

fn main() {
    match Game::from_args() {
        Ok(mut game) => {
            println!("{}", game);
            print_help();
            game.play();
        }
        Err(msg) => eprintln!("Error: {}", msg),
    }
}

fn print_help() {
    for line in HELP {
        println!("{}", line);
    }
}
