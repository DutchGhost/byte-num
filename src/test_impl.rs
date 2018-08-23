const ASCII_TO_INT_FACTOR: u8 = 48;

const POW10_U8_LEN: usize = 3;
const POW10_U16_LEN: usize = 5;
const POW10_U32_LEN: usize = 10;
const POW10_U64_LEN: usize = 20;

//all powers of 10 that fit in a u8
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

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct InvalidDigit(char);

/// This trait allows conversion from bytes to integers.
pub trait FromAscii: Sized {
    /// The function performing the conversion from a byteslice to a number.
    /// It takes anything that can be transformed into a byte-slice.
    ///
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::FromAscii;
    ///
    /// fn main() {
    ///     assert_eq!(u32::atoi("1928"), Ok(1928));
    ///     assert_eq!(u32::atoi("12e3"), Err(()));
    /// }
    /// ```
    /// # Safety
    /// It should be noted that trying to convert a string that does not fit in the chosen integer type,
    /// wraps around.
    /// For example:
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::FromAscii;
    ///
    /// fn main () {
    ///     let n = u8::atoi("257");
    ///     assert_eq!(n, Ok(1));
    /// }
    ///```
    #[inline]
    fn atoi2<S: AsRef<[u8]>>(s: S) -> Result<Self, InvalidDigit> {
        Self::bytes_to_int(s.as_ref())
    }

    /// Performs the actual conversion from a byteslice to an unsigned integer.
    fn bytes_to_int(s: &[u8]) -> Result<Self, InvalidDigit>;
}

/// This trait converts integers to bytes.
pub trait IntoAscii {
    /// The function performing the convertion from a number to a Vec<u8>, containing the digits of the number.
    ///
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::IntoAscii;
    ///
    /// fn main() {
    ///     assert_eq!(12345u32.itoa(), [b'1', b'2', b'3', b'4', b'5']);
    ///     assert_eq!((-12345i32).itoa(), [b'-', b'1', b'2', b'3', b'4', b'5']);
    /// }
    /// ```
    #[inline]
    fn itoa(&self) -> Vec<u8>
    where
        Self: Copy,
    {
        let size = Self::digits10(*self);
        let mut buff = vec![0; size];

        self.int_to_bytes(&mut buff);
        buff
    }

    /// Returns the size of an integer. This is how many digits the integer has.
    fn digits10(self) -> usize;

    /// Performs the actual convertion. It fills the given buff with bytes.
    /// Due to how the algorithm works, the last elements of the buffer get written to first.
    /// The buffer should have a size such that it can hold all the bytes.
    /// To get the size of an integer would take, use [`digits10()`](trait.IntoAscii.html#tymethod.digits10).
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::IntoAscii;
    ///
    /// fn main() {
    ///     let mut v = vec![0, 0, 0, 0, 0];
    ///
    ///     12345u32.int_to_bytes(&mut v);
    ///     assert_eq!(v, [b'1', b'2', b'3', b'4', b'5']);
    ///
    ///     54321u64.int_to_bytes(&mut v);
    ///     assert_eq!(v, [b'5', b'4', b'3', b'2', b'1']);
    ///
    ///     // if the buffer is larger than the number of digits, it fills with 0.
    ///     123u8.int_to_bytes(&mut v);
    ///     assert_eq!(v, [b'5', b'4', b'1', b'2', b'3']);
    ///
    ///     // use slicing to collect 2 numbers into the buffer:
    ///
    ///     12u8.int_to_bytes(&mut v[..2]);
    ///     648u32.int_to_bytes(&mut v[2..]);
    ///     assert_eq!(v, [b'1', b'2', b'6', b'4', b'8']);
    ///
    /// }
    fn int_to_bytes(self, buff: &mut [u8]);
}

macro_rules! atoi_unroll {
    ($d:ident, $r:ident, $bytes:expr, $idx:expr, $offset:expr, $const_table:ident) => {
        let $d = 
            $bytes
                .get_unchecked($offset)
                .wrapping_sub(ASCII_TO_INT_FACTOR);

        if $d > 9 {
            return Err(InvalidDigit(($d.wrapping_add(48)) as u8 as char));
        }
        let $r = Self::from($d) * $const_table.get_unchecked($idx + $offset);
    };
}

macro_rules! impl_unsigned_fromascii {
    ($int:ty, $const_table:ident, $const_table_len: ident) => {
        impl FromAscii for $int {
            #[inline]
            fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, InvalidDigit> {
                let mut result: Self = 0;
                let mut len: usize = bytes.len();
                let mut idx: usize = $const_table_len - len;

                unsafe {
                    while len >= 4 {
                        atoi_unroll!(d1, r1, bytes, idx, 0, $const_table);
                        atoi_unroll!(d2, r2, bytes, idx, 1, $const_table);
                        atoi_unroll!(d3, r3, bytes, idx, 2, $const_table);
                        atoi_unroll!(d4, r4, bytes, idx, 3, $const_table);

                        result = result.wrapping_add(r1 + r2 + r3 + r4);
                        len -= 4;
                        idx += 4;
                        bytes = bytes.get_unchecked(4..);
                    }

                    for offset in 0..len {
                        atoi_unroll!(d, r, bytes, idx, offset, $const_table);
                        result = result.wrapping_add(r);
                    }
                }

                Ok(result)
            }
        }
    }
}

impl_unsigned_fromascii!(u64, POW10_U64, POW10_U64_LEN);
impl_unsigned_fromascii!(u32, POW10_U32, POW10_U32_LEN);
impl_unsigned_fromascii!(u16, POW10_U16, POW10_U16_LEN);

// impl FromAscii for u64 {
//     fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, InvalidDigit> {
//         let mut result: Self = 0;
//         let mut len: usize = bytes.len();
//         let mut idx: usize = POW10_U64_LEN - len;

//         unsafe {
//             while len >= 4 {
//                 atoi_unroll!(d1, r1, bytes, idx, 0, POW10_U64);
//                 atoi_unroll!(d2, r2, bytes, idx, 1, POW10_U64);
//                 atoi_unroll!(d3, r3, bytes, idx, 2, POW10_U64);
//                 atoi_unroll!(d4, r4, bytes, idx, 3, POW10_U64);

//                 result = result.wrapping_add(r1 + r2 + r3 + r4);

//                 len -= 4;
//                 idx += 4;
//                 bytes = bytes.get_unchecked(4..);
//             }

//             for offset in 0..len {
//                 atoi_unroll!(d, r, bytes, idx, offset, POW10_U64);
//                 result += r;
//             }
//             return Ok(result);
//         }
//     }
// }

// impl FromAscii for u32 {
//     fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, InvalidDigit> {
//         let mut result: Self = 0;
//         let mut len: usize = bytes.len();
//         let mut idx: usize = POW10_U32_LEN - len;

//         unsafe {
//             while len >= 4 {
//                 atoi_unroll!(d1, r1, bytes, idx, 0, POW10_U32);
//                 atoi_unroll!(d2, r2, bytes, idx, 1, POW10_U32);
//                 atoi_unroll!(d3, r3, bytes, idx, 2, POW10_U32);
//                 atoi_unroll!(d4, r4, bytes, idx, 3, POW10_U32);

//                 result = result.wrapping_add(r1 + r2 + r3 + r4);

//                 len -= 4;
//                 idx += 4;
//                 bytes = bytes.get_unchecked(4..);
//             }

//             for offset in 0..len {
//                 atoi_unroll!(d, r, bytes, idx, offset, POW10_U32);
//                 result += r;
//             }
//             return Ok(result);
//         }
//     }
// }

// impl FromAscii for u16 {
//     fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, InvalidDigit> {
//         let mut result: Self = 0;
//         let mut len: usize = bytes.len();
//         let mut idx: usize = POW10_U16_LEN - len;

//         unsafe {
//             while len >= 4 {
//                 atoi_unroll!(d1, r1, bytes, idx, 0, POW10_U16);
//                 atoi_unroll!(d2, r2, bytes, idx, 1, POW10_U16);
//                 atoi_unroll!(d3, r3, bytes, idx, 2, POW10_U16);
//                 atoi_unroll!(d4, r4, bytes, idx, 3, POW10_U16);

//                 result = result.wrapping_add(r1 + r2 + r3 + r4);

//                 len -= 4;
//                 idx += 4;
//                 bytes = bytes.get_unchecked(4..);
//             }

//             for offset in 0..len {
//                 atoi_unroll!(d, r, bytes, idx, offset, POW10_U16);
//                 result += r;
//             }
//             return Ok(result);
//         }
//     }
// }

impl FromAscii for u8 {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, InvalidDigit> {
        let mut result: Self = 0;
        let len: usize = bytes.len();
        let idx: usize = POW10_U8_LEN - len;

        unsafe {
            for offset in 0..len {
                atoi_unroll!(d, r, bytes, idx, offset, POW10_U8);
                result = result.wrapping_add(r);
            }
            return Ok(result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atoi() {
        let n = u64::atoi2("100");

        assert_eq!(n, Ok(100));

        let errs = u64::atoi2("10000o1");

        assert_eq!(errs, Err(InvalidDigit('o')));

        let fault = u64::atoi2("!2345");

        assert_eq!(fault, Err(InvalidDigit('!')));
    }
}
