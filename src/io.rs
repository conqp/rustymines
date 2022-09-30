use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: &str) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print_prompt(prompt);
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<T>() {
            Ok(value) => Ok(value),
            Err(_) => Err("invalid value"),
        },
        Err(_) => Err("no value read"),
    }
}

pub fn read_repeat<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        match read::<T>(prompt) {
            Err(msg) => eprintln!("Error: {}", msg),
            Ok(value) => return value,
        }
    }
}

fn print_prompt(prompt: &str) {
    print!("{}", prompt);

    match std::io::stdout().flush() {
        Ok(_) => (),
        Err(_) => (),
    }
}
