use std::fmt::Debug;
use std::str::FromStr;

pub fn read<T>(prompt: impl Into<String>) -> Result<T, &'static str>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    print!("{}", prompt.into());

    for line in std::io::stdin().lines() {
        println!();
        let result = line.unwrap().trim().parse::<T>();

        if result.is_err() {
            return Err("invalid value");
        }

        return Ok(result.unwrap());
    }

    Err("no line read")
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
