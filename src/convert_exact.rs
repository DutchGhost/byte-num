use std::ops::Mul;

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

pub trait FromAscii: Sized {
    /// The function performing the conversion from a byteslice to a number.
    /// It takes anything that can be transformed into a byte-slice.
    /// An empty slice returns the number 0.
    ///
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert_exact::{ParseIntErr, FromAscii};
    ///
    /// fn main() {
    ///     assert_eq!(u32::atoi("1928"), Ok(1928));
    ///     assert_eq!(u32::atoi("12e3"), Err(ParseIntErr::InvalidDigit('e')));
    /// }
    /// ```
    /// # Safety
    /// It should be noted that trying to convert a slice that does not fit in the chosen integer type,
    /// wraps around.
    /// For example:
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert_exact::FromAscii;
    ///
    /// fn main () {
    ///     let n = u8::atoi("257");
    ///     assert_eq!(n, Ok(1));
    /// }
    /// ```
    #[inline]
    fn atoi<S: AsRef<[u8]>>(s: S) -> Result<Self, ParseIntErr> {
        Self::bytes_to_int(s.as_ref())
    }

    fn bytes_to_int(s: &[u8]) -> Result<Self, ParseIntErr>;
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ParseIntErr {
    /// Represents a character that could not be converted to a number.
    InvalidDigit(char),

    /// Represents that parsing of the slice could not be started, the slice was too large.
    Overflow,
}

#[inline(always)]
fn parse_byte<N>(byte: &u8, pow10: N) -> Result<N, ParseIntErr>
where
    N: From<u8> + Mul<Output = N>,
{
    let d = byte.wrapping_sub(ASCII_TO_INT_FACTOR);

    if d > 9 {
        return Err(ParseIntErr::InvalidDigit(
            d.wrapping_add(ASCII_TO_INT_FACTOR) as char,
        ));
    }

    Ok(N::from(d) * pow10)
}

macro_rules! impl_unsigned_conversions {
    ($int:ty, $const_table:ident) => {
        impl FromAscii for $int {
            /*
                        1) Start at correct position in pow10 table (const_table.len() - bytes.len() ).
                        2) For each byte:
                            - substract 48, wrapping
                            - validate it's less than 9
                            - multiply with some power of 10
                    */
            #[inline(always)]
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ParseIntErr> {
                if bytes.len() > $const_table.len() {
                    return Err(ParseIntErr::Overflow);
                }
        
                let mut result: Self = 0;
                let mut chunks = bytes.exact_chunks(4);
        
                let idx = $const_table.len().wrapping_sub(bytes.len());
        
                let mut table_chunks = $const_table[idx..].exact_chunks(4);
        
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

    // @NOTE: Specialize implementation for u8
    (@u8, $const_table:ident) => {
        impl FromAscii for u8 {
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

macro_rules! impl_signed_conversions {
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

impl_unsigned_conversions!(@u8, POW10_U8);
impl_unsigned_conversions!(u16, POW10_U16);
impl_unsigned_conversions!(u32, POW10_U32);
impl_unsigned_conversions!(u64, POW10_U64);

impl_signed_conversions!(i8, u8);
impl_signed_conversions!(i16, u16);
impl_signed_conversions!(i32, u32);
impl_signed_conversions!(i64, u64);
impl_signed_conversions!(isize, usize);

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
