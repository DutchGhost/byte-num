pub const ASCII_TO_INT_FACTOR: u8 = 48;

const POW10_U8_LEN: usize = 3;
const POW10_U16_LEN: usize = 5;
pub const POW10_U32_LEN: usize = 10;
pub const POW10_U64_LEN: usize = 20;

//all powers of 10 that fit in a u8
const POW10_U8: [u8; 3] = [100, 10, 1];

const POW10_U16: [u16; 5] = [10_000, 1_000, 100, 10, 1];

pub const POW10_U32: [u32; 10] = [
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

pub const POW10_U64: [u64; 20] = [
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

/// This trait allows convertion from bytes to integers.
pub trait FromAscii: Sized {
    /// The function performing the conversion. It takes anything that can be transformed into a byte-slice.
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
    fn atoi<S: AsRef<[u8]>>(s: S) -> Result<Self, ()> {
        Self::bytes_to_int(s.as_ref())
    }

    /// Performs the actual conversion from a byteslice to an unsigned integer.
    fn bytes_to_int(s: &[u8]) -> Result<Self, ()>;

    /// Converts bytes to integers, but does not check if the bytes are valid digits.
    #[inline]
    unsafe fn atoi_unchecked<S: AsRef<[u8]>>(s: S) -> Self {
        Self::bytes_to_int_unchecked(s.as_ref())
    }
    unsafe fn bytes_to_int_unchecked(&[u8]) -> Self;
}

/// This trait converts integers to bytes.
pub trait IntoAscii {
    /// The function performing the convertion.
    /// # Examples
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::IntoAscii;
    ///
    /// fn main() {
    ///     assert_eq!(12345u32.itoa(), [b'1', b'2', b'3', b'4', b'5']);
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
        let $d = Self::from(
            $bytes
                .get_unchecked($offset)
                .wrapping_sub(ASCII_TO_INT_FACTOR),
        );

        //if the digit is greater than 9, something went terribly horribly wrong.
        //return an Err(())
        if $d > 9 {
            return Err(());
        }
        let $r = $d * $const_table.get_unchecked($idx + $offset);
    };
}

#[inline]
pub fn atoi_structured(mut bytes: &[u8]) -> Result<u64, ()> {
    let mut result: u64 = 0;
    let mut len = bytes.len();
    let mut idx = POW10_U64_LEN - len;

    unsafe {
        while len >= 4 {
            let d1 = u64::from(bytes.get_unchecked(0).wrapping_sub(ASCII_TO_INT_FACTOR));
            let d2 = u64::from(bytes.get_unchecked(1).wrapping_sub(ASCII_TO_INT_FACTOR));
            let d3 = u64::from(bytes.get_unchecked(2).wrapping_sub(ASCII_TO_INT_FACTOR));
            let d4 = u64::from(bytes.get_unchecked(3).wrapping_sub(ASCII_TO_INT_FACTOR));

            if (d1 > 9) | (d3 > 9) || (d2 > 9) | (d4 > 9) {
                return Err(());
            }

            let r1 = d1 * POW10_U64.get_unchecked(idx);
            let r2 = d2 * POW10_U64.get_unchecked(idx + 1);
            let r3 = d3 * POW10_U64.get_unchecked(idx + 2);
            let r4 = d4 * POW10_U64.get_unchecked(idx + 3);

            result = result.wrapping_add(r1 + r3 + r2 + r4);
            len -= 4;
            idx += 4;
            bytes = bytes.get_unchecked(4..);
        }

        for offset in 0..len {
            let d = u64::from(bytes.get_unchecked(offset).wrapping_sub(ASCII_TO_INT_FACTOR));
            if d > 9 {
                return Err(());
            }
            let r = d * POW10_U64.get_unchecked(idx + offset);
            result = result.wrapping_add(r);
        }
        return Ok(result)
    }
}

// #[inline]
// pub fn atoi_transmute(mut bytes: &[u8]) -> Result<u64, ()> {
//     let mut result: u64 = 0;
//     let mut len = bytes.len();
//     let mut idx = POW10_U64_LEN - len;

//     let mut buff = [0u8; 4];

//     unsafe {
//         while len >= 4 {
//             buff.copy_from_slice(bytes.get_unchecked(..4));
//             let mut n: u32 = ::std::mem::transmute(buff);
//             n = n.wrapping_sub(0b00110000001100000011000000110000);
//             //0b00001001000010010000100100001001
//             if n > ::std::mem::transmute([9u8, 9, 9, 9]) {
//                 return Err(())
//             }

//             let (n1, n2, n3, n4): (u8, u8, u8, u8) = ::std::mem::transmute(n);
//             // if (n1 > 9) | (n2 > 9) || (n3 > 9) | (n4 > 9) {
//             //     return Err(())
//             // }
//             let r1 = n1 as u64 * POW10_U64.get_unchecked(idx);
//             let r2 = n2 as u64 * POW10_U64.get_unchecked(idx + 1);
//             let r3 = n3 as u64 * POW10_U64.get_unchecked(idx + 2);
//             let r4 = n4 as u64 * POW10_U64.get_unchecked(idx + 3);

//             result = result.wrapping_add(r1 + r2 + r3 + r4);
//             len -= 4;
//             idx += 4;
//             bytes = bytes.get_unchecked(4..);
//         }

//         for offset in 0..len {
//             let d = u64::from(bytes.get_unchecked(offset).wrapping_sub(ASCII_TO_INT_FACTOR));
//             if d > 9 {
//                 return Err(());
//             }
//             let r = d * POW10_U64.get_unchecked(idx + offset);
//             result = result.wrapping_add(r);
//         }
//         return Ok(result)
//     }
// }

macro_rules! impl_unsigned_conversions {
    ($int:ty, $const_table:ident, $const_table_len:ident) => {
        impl FromAscii for $int {
            #[inline]
            fn bytes_to_int(mut bytes: &[u8]) -> Result<Self, ()> {
                let mut result: Self = 0;
                let mut len: usize = bytes.len();
                let mut idx: usize = $const_table_len - len;

                unsafe {
                    while len >= 4 {
                        atoi_unroll!(d1, r1, bytes, idx, 0, $const_table);
                        atoi_unroll!(d2, r2, bytes, idx, 1, $const_table);
                        atoi_unroll!(d3, r3, bytes, idx, 2, $const_table);
                        atoi_unroll!(d4, r4, bytes, idx, 3, $const_table);

                        /*
                            @NOTE: changed `result += r1 + r2 + r3 + r4;` to the wrapping add version on 1 august 2018,
                            to prevent debug builds from panicing due to overflow.
                        */
                        result = result.wrapping_add(r1 + r2 + r3 + r4);

                        len -= 4;
                        idx += 4;
                        bytes = bytes.get_unchecked(4..);
                    }

                    for offset in 0..len {
                        atoi_unroll!(d, r, bytes, idx, offset, $const_table);
                        result += r;
                    }
                    return Ok(result);
                }
            }

            #[inline]
            unsafe fn bytes_to_int_unchecked(mut bytes: &[u8]) -> Self {
                let mut result: Self = 0;
                let mut len = bytes.len();
                let mut idx: usize = $const_table_len - len;

                while len >= 4 {

                    let r1 = Self::from(bytes.get_unchecked(0).wrapping_sub(ASCII_TO_INT_FACTOR));
                    let r2 = Self::from(bytes.get_unchecked(1).wrapping_sub(ASCII_TO_INT_FACTOR));
                    let r3 = Self::from(bytes.get_unchecked(2).wrapping_sub(ASCII_TO_INT_FACTOR));
                    let r4 = Self::from(bytes.get_unchecked(3).wrapping_sub(ASCII_TO_INT_FACTOR));

                    let d1 = r1 * $const_table.get_unchecked(idx);
                    let d2 = r2 * $const_table.get_unchecked(idx + 1);
                    let d3 = r3 * $const_table.get_unchecked(idx + 2);
                    let d4 = r4 * $const_table.get_unchecked(idx + 3);

                    result = result.wrapping_add(d1 + d2 + d3 + d4);

                    len -= 4;
                    idx += 4;

                    bytes = bytes.get_unchecked(4..);
                }

                for offset in 0..len {
                    let r1 = Self::from(bytes.get_unchecked(offset).wrapping_sub(ASCII_TO_INT_FACTOR));
                    let d1 = r1 * $const_table.get_unchecked(idx + offset);
                    result = result.wrapping_add(d1);
                }
            
            result

            }
        }

        impl IntoAscii for $int {
            #[inline]
            fn digits10(mut self) -> usize {
                let mut result = 1;

                loop {
                    if self < 10 { return result }
                    if self < 100 { return result + 1 }
                    if self < 1000 { return result + 2 }
                    if self < 10000 { return result + 3 }

                    self /= 10_000;
                    result += 4;
                }
            }

            #[inline]
            fn int_to_bytes(mut self, buff: &mut [u8]) {
                let mut len = buff.len();

                while self >= 10_000 {
                    let q = self / 10;
                    let q1 = self / 100;
                    let q2 = self / 1000;

                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r1 = (q % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r2 = (q1 % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r3 = (q2 % 10) as u8 + ASCII_TO_INT_FACTOR;

                    // update the last 4 indecies
                    unsafe {
                        *buff.get_unchecked_mut(len - 1) = r;
                        *buff.get_unchecked_mut(len - 2) = r1;
                        *buff.get_unchecked_mut(len - 3) = r2;
                        *buff.get_unchecked_mut(len - 4) = r3;
                    }

                    len -= 4;
                    self /= 10_000;
                }

                //fixup loop. This might not be run if self was a multiple of 10_000
                for byte in unsafe { buff.get_unchecked_mut(..len) }.iter_mut().rev() {
                    let q = self / 10;
                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    *byte = r;

                    //there's nothing more to do.
                    if q == 0 {
                        return;
                    }

                    self = q;
                }
            }
        }
    };
}

//this implementation is different than for all other unsigned integers. max u8 is 255, which only has a length of 3.
impl FromAscii for u8 {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
        let mut result: Self = 0;
        let len: usize = bytes.len();
        let idx = POW10_U8_LEN - len;
        unsafe {
            for offset in 0..len {
                atoi_unroll!(d, r, bytes, idx, offset, POW10_U8);

                // @NOTE: changed from `result += r` to this, due to overflow panics on debug builds.
                result = result.wrapping_add(r);
            }
            Ok(result)
        }
    }

    #[inline]
    unsafe fn bytes_to_int_unchecked(bytes: &[u8]) -> Self {
        let mut result: Self = 0;
        let len: usize = bytes.len();
        let idx = POW10_U8_LEN - len;

        for offset in 0..len {
            let r1 = Self::from(bytes.get_unchecked(offset).wrapping_sub(ASCII_TO_INT_FACTOR));
            let d1 = r1 * POW10_U8.get_unchecked(idx + offset);
            result = result.wrapping_add(d1);
        }

        result
    }
}

impl IntoAscii for u8 {
    #[inline]
    fn digits10(self) -> usize {
        if self < 10 {
            1
        } else if self < 100 {
            2
        } else {
            3
        }
    }

    #[inline]
    fn int_to_bytes(mut self, buff: &mut [u8]) {
        let mut q;
        let mut r;

        for byte in buff.iter_mut().rev() {
            q = self / 10;
            r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
            *byte = r;
            self = q;

            if self == 0 {
                break;
            }
        }
    }
}

impl_unsigned_conversions!(u16, POW10_U16, POW10_U16_LEN);
impl_unsigned_conversions!(u32, POW10_U32, POW10_U32_LEN);
impl_unsigned_conversions!(u64, POW10_U64, POW10_U64_LEN);

#[cfg(target_pointer_width = "32")]
impl FromAscii for usize {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
        Ok(u32::bytes_to_int(bytes)? as Self)
    }

    #[inline]
    unsafe fn bytes_to_int_unchecked(bytes: &[u8]) -> Self {
        u32::bytes_to_int_unchecked(bytes) as Self
    }
}

#[cfg(target_pointer_width = "64")]
impl FromAscii for usize {
    #[inline]
    fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
        Ok(u64::bytes_to_int(bytes)? as Self)
    }

    #[inline]
    unsafe fn bytes_to_int_unchecked(bytes: &[u8]) -> Self {
        u64::bytes_to_int_unchecked(bytes) as Self
    }
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

            unsafe fn bytes_to_int_unchecked(bytes: &[u8]) -> Self {
                if bytes.starts_with(b"-") {
                    -(<$unsigned_version>::bytes_to_int_unchecked(bytes.get_unchecked(1..)) as Self)
                } else {
                    <$unsigned_version>::bytes_to_int_unchecked(bytes) as Self
                }
            }
        }

        impl IntoAscii for $int {
            fn itoa(&self) -> Vec<u8>
            where
                Self: Copy,
            {
                if self < &0 {
                    let n = self * -1;
                    let size = Self::digits10(n) + 1;

                    let mut buff = vec![0; size];
                    unsafe {
                        *buff.get_unchecked_mut(0) = b'-';
                    }
                    n.int_to_bytes(&mut buff);
                    buff
                } else {
                    let size = Self::digits10(*self);
                    let mut buff = vec![0; size];
                    self.int_to_bytes(&mut buff);
                    buff
                }
            }

            #[inline]
            fn digits10(mut self) -> usize {
                /*
                    @NOTE: Verry important, some signed numbers get 'more digits' when casted to their unsigned version.
                */
                if self < 0 {
                    self *= -1;
                }
                (self as $unsigned_version).digits10()
            }

            #[inline]
            fn int_to_bytes(self, buff: &mut [u8]) {
                (self as $unsigned_version).int_to_bytes(buff);
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
impl IntoAscii for usize {
    #[inline]
    fn digits10(self) -> Self {
        u32::digits10(self as u32)
    }

    #[inline]
    fn int_to_bytes(self, buff: &mut [u8]) {
        u32::int_to_bytes(self as u32, buff);
    }
}

#[cfg(target_pointer_width = "64")]
impl IntoAscii for usize {
    #[inline]
    fn digits10(self) -> Self {
        u64::digits10(self as u64)
    }

    #[inline]
    fn int_to_bytes(self, buff: &mut [u8]) {
        u64::int_to_bytes(self as u64, buff);
    }
}

impl_signed_conversions!(i8, u8);
impl_signed_conversions!(i16, u16);
impl_signed_conversions!(i32, u32);
impl_signed_conversions!(i64, u64);
impl_signed_conversions!(isize, usize);

#[cfg(test)]
mod test_parsing {
    use super::*;

    #[test]
    fn bytes_to_int() {
        assert_eq!(i32::atoi(b"-123"), Ok(-123));
        assert_eq!(i32::atoi(b"123"), Ok(123));

        assert_eq!(i32::atoi(b"123e"), Err(()));

        assert_eq!(isize::atoi(b"9223372036854775807"), Ok(isize::max_value()));
        assert_eq!(isize::atoi(b"9223372036854775808"), Ok(isize::min_value()));

        assert_eq!(usize::atoi(b"18446744073709551615"), Ok(usize::max_value()));
        assert_eq!(usize::atoi(b"18446744073709551616"), Ok(usize::min_value()));

        assert_eq!(u64::atoi(b"12345"), Ok(12345));
    }
    #[test]
    fn test_itoa() {

        assert_eq!(9987u32.itoa(), [b'9', b'9', b'8', b'7']);

        assert_eq!(isize::max_value().itoa(), [b'9', b'2', b'2', b'3', b'3', b'7', b'2', b'0', b'3', b'6', b'8', b'5', b'4', b'7', b'7', b'5', b'8', b'0', b'7'])
    }
    
    #[test]
    fn test_digits10() {
        assert_eq!( (-99i8).digits10(), 2);
        assert_eq!( (-99i16).digits10(), 2);
        assert_eq!( (-99i32).digits10(), 2);
        assert_eq!( (-99i64).digits10(), 2);
        assert_eq!( (-99isize).digits10(), 2);
    }

    #[test]
    fn test_atoi_unchecked() {
        assert_eq!(unsafe { u64::atoi_unchecked("12345") }, 12345);
    }
    // #[test]
    // fn test_atoi_transmute() {
    //     assert_eq!(atoi_transmute(b"12345".as_ref()), Ok(12345));
    //     assert_eq!(atoi_transmute(b"123456".as_ref()), Ok(123456));
    //     assert_eq!(atoi_transmute(b"e12345".as_ref()), Err(()));
    // }
}
