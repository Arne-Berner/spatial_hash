use std::fmt;

#[derive(Debug)]
pub enum Error {
    NumCellsEqualZero,
    OutOfBounds,
    // add other variants
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NumCellsEqualZero => write!(f, "number of cells must be greater than zero"),
            Error::OutOfBounds => write!(f, "Placed Entity out of Bounds!"),
        }
    }
}

impl std::error::Error for Error {}
