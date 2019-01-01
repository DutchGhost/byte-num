use std::{
    fmt,
    str,
    error::Error
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseIntErr {
    /// Represents a character that could not be converted to a number.
    InvalidDigit([u8; 1]),

    /// Represents that parsing of the slice could not be started, the slice was too large.
    Overflow,
}

impl fmt::Display for ParseIntErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseIntErr::InvalidDigit([ref c]) => write!(f, "ParseIntErr::InvalidDigit({})", c),
            ParseIntErr::Overflow => f.pad("ParseIntErr::Overflow"),
        }
    }
}

impl Error for ParseIntErr {
    fn description(&self) -> &str {
        match *self {
            ParseIntErr::InvalidDigit(ref c) => str::from_utf8(c).unwrap(),
            ParseIntErr::Overflow => "number too large to fit in the target type",
        }
    }
}

impl ParseIntErr {
    pub fn with_byte(c: u8) -> Self {
        ParseIntErr::InvalidDigit([c])
    }
}