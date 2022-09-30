use std::fmt::Debug;
use std::io::Write;
use std::str::FromStr;

pub fn read<T>(prompt: impl Into<String>) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{}", prompt.into());
    _ = std::io::stdout().flush();
    let mut input = String::new();
    let result = std::io::stdin().read_line(&mut input);

    if result.is_err() {
        Err("no value read")
    } else {
        let result = input.trim().parse::<T>();

        if result.is_err() {
            Err("invalid value")
        } else {
            Ok(result.unwrap())
        }
    }
}

pub fn read_repeat<T>(prompt: impl Into<String> + Copy) -> T
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
