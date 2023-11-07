use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input = String::new();

    loop {
        input.clear();
        print!("{prompt}");
        std::io::stdout().flush().expect("Could not flush stdout.");

        if let Some(value) = std::io::stdin()
            .read_line(&mut input)
            .ok()
            .and_then(|_| input.trim().parse::<T>().ok())
        {
            return value;
        }
    }
}
