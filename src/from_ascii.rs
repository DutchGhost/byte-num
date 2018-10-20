use std::error::Error;
use std::fmt;
use std::ops::Mul;
use std::str;

const ASCII_TO_INT_FACTOR: u8 = 48;

const POW10_U8: [u8; 3] = [100, 10, 1];

const POW10_U16: [u16; 5] = [10_000, 1_000, 100, 10, 1];

const POW10_U32: [u32; 10] = [
    1_000_000_000,
    100_000_000,
    10_000_000,
    1_000_000,
    100_000,
    10_000,
    1_000,
    100,
    10,
    1,
];

const POW10_U64: [u64; 20] = [
    10_000_000_000_000_000_000,
    1_000_000_000_000_000_000,
    100_000_000_000_000_000,
    10_000_000_000_000_000,
    1_000_000_000_000_000,
    100_000_000_000_000,
    10_000_000_000_000,
    1_000_000_000_000,
    100_000_000_000,
    10_000_000_000,
    1_000_000_000,
    100_000_000,
    10_000_000,
    1_000_000,
    100_000,
    10_000,
    1_000,
    100,
    10,
    1,
];

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseIntErr {
    /// Represents a character that could not be converted to a number.
    InvalidDigit([u8; 1]),

    /// Represents that parsing of the slice could not be started, the slice was too large.
    Overflow,
}

impl fmt::Display for ParseIntErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ParseIntErr::InvalidDigit([ref c]) => write!(f, "ParseIntErr::InvalidDigit({})", c),
            &ParseIntErr::Overflow => f.pad("ParseIntErr::Overflow"),
        }
    }
}

impl Error for ParseIntErr {
    fn description(&self) -> &str {
        match self {
            &ParseIntErr::InvalidDigit(ref c) => str::from_utf8(c).unwrap(),
            &ParseIntErr::Overflow => "number too large to fit in the target type",
        }
    }
}

impl ParseIntErr {
    pub fn with_byte(c: u8) -> Self {
        ParseIntErr::InvalidDigit([c])
    }
}

/// A trait that converts any sequence of bytes into a number.
pub trait FromAscii: Sized {
    /// The function performing the conversion from a byteslice to a number.
    /// It takes anything that can be transformed into a byte-slice.
    /// An empty slice returns the number 0.
    ///
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::from_ascii::{ParseIntErr, FromAscii};
    ///
    /// fn main() {
    ///     assert_eq!(u32::atoi("1928"), Ok(1928));
    ///     assert_eq!(u32::atoi("12e3"), Err(ParseIntErr::with_byte(b'e')));
    /// }
    /// ```
    /// # Safety
    /// It should be noted that trying to convert a slice that does not fit in the chosen integer type,
    /// wraps around.
    /// For example:
    /// ```
    /// extern crate byte_num;
    /// use byte_num::from_ascii::FromAscii;
    ///
    /// fn main () {
    ///     let n = u8::atoi("256");
    ///     assert_eq!(n, Ok(0));
    /// }
    /// ```
    #[inline]
    fn atoi<S: AsRef<[u8]>>(s: S) -> Result<Self, ParseIntErr> {
        Self::bytes_to_int(s.as_ref())
    }

    fn bytes_to_int(s: &[u8]) -> Result<Self, ParseIntErr>;
}

#[inline(always)]
fn parse_byte<N>(byte: &u8, pow10: N) -> Result<N, ParseIntErr>
where
    N: From<u8> + Mul<Output = N>,
{
    let d = byte.wrapping_sub(ASCII_TO_INT_FACTOR);

    if d > 9 {
        return Err(ParseIntErr::with_byte(d.wrapping_add(ASCII_TO_INT_FACTOR)));
    }

    Ok(N::from(d) * pow10)
}

macro_rules! unsigned_from_ascii {
    ($int:ty, $const_table:ident) => {
        impl FromAscii for $int {
            // 1) Start at correct position in pow10 table (const_table.len() - bytes.len() ).
            // 2) For each byte:
            //     - substract 48, wrapping
            //     - validate it's less than 9
            //     - multiply with some power of 10
            #[inline]
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
                if bytes.len() > $const_table.len() {
                    return Err(ParseIntErr::Overflow);
                }
        
                let mut result: Self = 0;
                let idx = $const_table.len().wrapping_sub(bytes.len());
        
                let mut chunks = bytes.chunks_exact(4);
                let mut table_chunks = $const_table[idx..].chunks_exact(4);
        
                for (chunk, table_chunk) in chunks.by_ref().zip(table_chunks.by_ref()) {
                    match (chunk, table_chunk) {
                        ([a, b, c, d], [p1, p2, p3, p4]) => {
                            let r1 = parse_byte(a, *p1)?;
                            let r2 = parse_byte(b, *p2)?;
                            let r3 = parse_byte(c, *p3)?;
                            let r4 = parse_byte(d, *p4)?;
        
                            result = result.wrapping_add(r1 + r2 + r3 + r4);
                        }
                        _ => unreachable!(),
                    }
                }
        
                for (byte, pow10) in chunks.remainder().iter().zip(table_chunks.remainder()) {
                    let r = parse_byte(byte, *pow10)?;
                    result = result.wrapping_add(r);
                }
        
                Ok(result)
            }
        }
    };

    // @NOTE: Specialize implementation for u8, since that's finished within 3 Iterations at max.
    (@u8, $const_table:ident) => {
        impl FromAscii for u8 {
            #[inline]
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
                if bytes.len() > $const_table.len() {
                    return Err(ParseIntErr::Overflow);
                }
        
                let mut result: Self = 0;
                let idx = $const_table.len() - bytes.len();
                let table_iter = $const_table[idx..].iter();
        
                for (byte, pow10) in bytes.iter().zip(table_iter) {
                    let r = parse_byte(byte, *pow10)?;
                    result = result.wrapping_add(r);
                }
        
                Ok(result)
            }
        }
    };
}

macro_rules! signed_from_ascii {
    ($int:ty, $unsigned_version:ty) => {
        impl FromAscii for $int {
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
                if bytes.starts_with(b"-") {
                    Ok(-(<$unsigned_version>::bytes_to_int(&bytes[1..])? as Self))
                } else {
                    Ok(<$unsigned_version>::bytes_to_int(bytes)? as Self)
                }
            }
        }
    };
}

unsigned_from_ascii!(@u8, POW10_U8);
unsigned_from_ascii!(u16, POW10_U16);
unsigned_from_ascii!(u32, POW10_U32);
unsigned_from_ascii!(u64, POW10_U64);

signed_from_ascii!(i8, u8);
signed_from_ascii!(i16, u16);
signed_from_ascii!(i32, u32);
signed_from_ascii!(i64, u64);
signed_from_ascii!(isize, usize);

#[cfg(target_pointer_width = "32")]
impl FromAscii for usize {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
        Ok(u32::bytes_to_int(bytes)? as Self)
    }
}

#[cfg(target_pointer_width = "64")]
impl FromAscii for usize {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
        Ok(u64::bytes_to_int(bytes)? as Self)
    }
}

#[cfg(test)]
mod tests {
    use super::{FromAscii, ParseIntErr};

    #[test]
    fn to_u8() {
        assert_eq!(u8::atoi("123"), Ok(123));
        assert_eq!(u8::atoi("256"), Ok(0));

        // Wraps around
        assert_eq!(u8::atoi("257"), Ok(1));

        // Error: InvalidDigit
        assert_eq!(u8::atoi("!23"), Err(ParseIntErr::with_byte(b'!')));

        // Error: Overflow
        assert_eq!(u8::atoi("1000"), Err(ParseIntErr::Overflow));
    }
}
