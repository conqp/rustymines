use std::error::Error;
use std::fmt::Display;
use std::io::{Write, stdin, stdout};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ReadError<T> {
    InvalidInput,
    ParseError(T),
}

impl<T> Display for ReadError<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput => write!(f, "Invalid input"),
            Self::ParseError(error) => write!(f, "{error}"),
        }
    }
}

impl<T> Error for ReadError<T>
where
    T: Error + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseError(error) => Some(error),
            Self::InvalidInput => None,
        }
    }
}

pub fn try_read<T>(prompt: &str) -> Result<T, ReadError<T::Err>>
where
    T: FromStr,
{
    print!("{prompt}");
    stdout().flush().expect("Stdout should be able to flush.");

    let Some(value) = stdin().lines().find_map(Result::ok) else {
        return Err(ReadError::InvalidInput);
    };

    value.trim().parse().map_err(ReadError::ParseError)
}

pub fn read_until_valid<T>(prompt: &str) -> T
where
    T: FromStr<Err: Display>,
{
    loop {
        match try_read(prompt) {
            Ok(value) => return value,
            Err(error) => eprintln!("{error}"),
        }
    }
}
