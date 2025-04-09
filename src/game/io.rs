use std::fmt::Debug;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;

pub fn read_until_valid<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        print!("{prompt}");
        stdout().flush().expect("Could not flush stdout.");

        let value = stdin().lines().find_map(Result::ok);

        if let Some(value) = value.and_then(|line| line.trim().parse::<T>().ok()) {
            return value;
        }
    }
}
