use std::ops::Mul;

use crate::{constants::*, error::ParseIntErr};

/// This trait converts bytes to integers,
/// and is implemented on all integer types, except u128 and i128.
///
/// The most important method on this trait is [`FromAscii::atoi`], which can be called in a function-like style.
/// As argument, it takes anything that implements `AsRef<[u8]>`.
/// The return type is a [`Result`], indicating whether the convertion succeeded or failed
pub trait FromAscii: Sized {
    /// The function performing the conversion from a byteslice to a number.
    /// It takes anything that can be transformed into a byte-slice.
    /// An empty slice returns the number 0.
    ///
    /// # Examples
    /// ```
    /// use byte_num::{
    ///     from_ascii::FromAscii,
    ///     error::ParseIntErr,
    /// };
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
    /// use byte_num::from_ascii::FromAscii;
    ///
    /// fn main () {
    ///     let n = u8::atoi("256");
    ///     assert_eq!(n, Ok(0));
    /// }
    /// ```
    #[inline]
    fn atoi(s: impl AsRef<[u8]>) -> Result<Self, ParseIntErr> {
        Self::bytes_to_int(s.as_ref())
    }

    fn bytes_to_int(s: &[u8]) -> Result<Self, ParseIntErr>;
}

#[inline(always)]
fn parse_byte<N>(byte: u8, pow10: N) -> Result<N, ParseIntErr>
where
    N: From<u8> + Mul<Output = N>,
{
    let d = byte.wrapping_sub(ASCII_TO_INT_FACTOR);

    if d > 9 {
        return Err(ParseIntErr::with_byte(byte));
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
            fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, ParseIntErr> {
                if bytes.len() > $const_table.len() {
                    return Err(ParseIntErr::Overflow);
                }
        
                let mut result: Self = 0;
        
                let mut len = bytes.len();
                let mut idx = $const_table.len().wrapping_sub(len);
        
                // @NOTE: This is safe, we never overshoot the buffers.
                // First we checked of the length of `bytes` is NOT longer than the length of the corresponding table of powers of 10,
                // so there is no bounds check needed to access the table of powers of 10.
                // Second, we loop while the length of the bytes is larger than or equal to 4, but only accessing the first 4 elements.
                // No boundschecks is needed for that as well.
                unsafe {
                    while len >= 4 {
                        match (
                            bytes.get_unchecked(..4),
                            $const_table.get_unchecked(idx..idx + 4),
                        ) {
                            ([a, b, c, d], [p1, p2, p3, p4]) => {
                                let r1 = parse_byte(*a, *p1)?;
                                let r2 = parse_byte(*b, *p2)?;
                                let r3 = parse_byte(*c, *p3)?;
                                let r4 = parse_byte(*d, *p4)?;
        
                                result = result.wrapping_add(r1 + r2 + r3 + r4);
                            }
                            _ => unreachable!(),
                        }
        
                        len -= 4;
                        idx += 4;
                        bytes = bytes.get_unchecked(4..);
                    }
        
                    // Fixuploop
                    for offset in 0..len {
                        let a = bytes.get_unchecked(offset);
                        let p = $const_table.get_unchecked(idx + offset);
                        let r = parse_byte(*a, *p)?;
                        result = result.wrapping_add(r);
                    }
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
                let len = bytes.len();
                let idx = $const_table.len().wrapping_sub(len);
        
                unsafe {
                    for offset in 0..len {
                        let a = bytes.get_unchecked(offset);
                        let p = $const_table.get_unchecked(idx + offset);
                        let r = parse_byte(*a, *p)?;
                        result = result.wrapping_add(r);
                    }
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
                    // .wrapping_neg() wraps around.
                    Ok((<$unsigned_version>::bytes_to_int(&bytes[1..])? as Self).wrapping_neg())
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
unsigned_from_ascii!(usize, POW10_USIZE);

signed_from_ascii!(i8, u8);
signed_from_ascii!(i16, u16);
signed_from_ascii!(i32, u32);
signed_from_ascii!(i64, u64);
signed_from_ascii!(isize, usize);

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

    #[test]
    fn overflow_isize() {
        // overflows minimum value of the isize by 1, but it wraps arroo
        assert_eq!(isize::atoi("-9223372036854775809"), Ok(9223372036854775807));

        // overflows maximum value of the isize by 1, but it wraps aroo
        assert_eq!(isize::atoi("9223372036854775809"), Ok(-9223372036854775807));
    }
}
