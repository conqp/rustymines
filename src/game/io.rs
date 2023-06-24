use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        match try_read::<T>(prompt) {
            Err(msg) => eprintln!("Error: {msg}"),
            Ok(value) => return value,
        }
    }
}

fn try_read<T>(prompt: &str) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print_prompt(prompt);
    let mut input = String::new();

    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input
            .trim()
            .parse::<T>()
            .map_or_else(|_| Err("invalid value"), |value| Ok(value)),
        Err(_) => Err("no value read"),
    }
}

fn print_prompt(prompt: &str) -> bool {
    print!("{prompt}");
    std::io::stdout().flush().is_ok()
}
