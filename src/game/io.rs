use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        print!("{prompt}");
        std::io::stdout().flush().expect("Could not flush stdout.");

        if let Some(value) = std::io::stdin()
            .lines()
            .find_map(Result::ok)
            .and_then(|line| line.trim().parse::<T>().ok())
        {
            return value;
        }
    }
}
