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
    /// use byte_num::convert::FromAscii;
    ///
    /// fn main() {
    ///     assert_eq!(u32::atoi("1928"), Ok(1928));
    ///     assert_eq!(u32::atoi("12e3"), Err(()));
    /// }
    /// ```
    /// # Safety
    /// It should be noted that trying to convert a slice that does not fit in the chosen integer type,
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
    /// ```
    /// If you try to convert a slice with a length longer than the maximum digits of the integer type, this function will panic.
    /// The maximum digits for:
    /// - 64 bit integers is 20
    /// - 32 bit integers is 10
    /// - 16 bit integers is 5
    /// - 8 bit integers is 3
    ///
    #[inline]
    fn atoi<S: AsRef<[u8]>>(s: S) -> Result<Self, ()> {
        Self::bytes_to_int(s.as_ref())
    }

    fn bytes_to_int(s: &[u8]) -> Result<Self, ()>;
}

macro_rules! try_parse_byte {
    ($byte:expr, $const_table:ident, $offset:expr) => {{
        let d = Self::from($byte.wrapping_sub(ASCII_TO_INT_FACTOR));

        //if the digit is greater than 9, something went terribly horribly wrong.
        //return an Err(())
        if d > 9 {
            return Err(());
        }

        d * unsafe { $const_table.get_unchecked($offset) }
    }};
}

macro_rules! impl_unsigned_conversions {
    ($int:ty, $const_table:ident) => {
        impl FromAscii for $int {
            #[inline(always)]
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
                let mut result: Self = 0;
                let mut chunks = bytes.exact_chunks(4);
                let mut idx: usize = $const_table.len() - bytes.len();
        
                for chunk in chunks.by_ref() {
                    match chunk {
                        [a, b, c, d] => {
                            let r1 = try_parse_byte!(a, $const_table, idx + 0);
                            let r2 = try_parse_byte!(b, $const_table, idx + 1);
                            let r3 = try_parse_byte!(c, $const_table, idx + 2);
                            let r4 = try_parse_byte!(d, $const_table, idx + 3);
        
                            idx += 4;
                            result = result.wrapping_add(r1 + r2 + r3 + r4);
                        }
                        _ => unreachable!(),
                    }
                }
        
                for (offset, byte) in chunks.remainder().iter().enumerate() {
                    let r = try_parse_byte!(byte, $const_table, idx + offset);
                    result = result.wrapping_add(r);
                }
        
                Ok(result)
            }
        }
    };

    (u8) => {
        impl FromAscii for u8 {
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
                let mut result: Self = 0;
                let idx = POW10_U8.len() - bytes.len();
        
                for (offset, byte) in bytes.iter().enumerate() {
                    let r = try_parse_byte!(byte, POW10_U8, idx + offset);
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
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
                if bytes.starts_with(b"-") {
                    unsafe {
                        Ok(-(<$unsigned_version>::bytes_to_int(bytes.get_unchecked(1..))? as Self))
                    }
                } else {
                    Ok(<$unsigned_version>::bytes_to_int(bytes)? as Self)
                }
            }
        }
    };
}

impl_unsigned_conversions!(u8);
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
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
        Ok(u32::bytes_to_int(bytes)? as Self)
    }
}

#[cfg(target_pointer_width = "64")]
impl FromAscii for usize {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
        Ok(u64::bytes_to_int(bytes)? as Self)
    }
}
