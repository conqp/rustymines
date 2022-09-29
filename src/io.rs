use std::fmt::Debug;
use std::str::FromStr;

pub fn read<T>(prompt: impl Into<String>) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    println!("{}", prompt);
    let mut input = String::new();
    let result = std::io::stdin().read_line(&mut input);

    if result.is_err() {
        Err("no value read")
    } else {
        let result = input.trim().parse::<T>();

        if result.is_err() {
            Err("invalid value")
        } else {
            return Ok(result.unwrap());
        }
    }
}

pub fn read_repeat<T>(prompt: impl Into<String> + Copy) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut result = read::<T>(prompt);

    while result.is_err() {
        println!("{}", result.err().unwrap());
        result = read::<T>(prompt);
    }

    result.unwrap()
}
