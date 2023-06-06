use std::{error, fmt::Display, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WrongEntriesLength {
        rows: usize,
        cols: usize,
        len: usize,
    },
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Error::WrongEntriesLength { rows, cols, len } => {
                write!(
                    f,
                    "entry array of size `{}` does not match matrix of size `{}x{}`",
                    len, rows, cols,
                )
            }
        }
    }
}
