const ASCII_TO_INT_FACTOR: u8 = 48;

const POW10_U8_LEN: usize = 3;
const POW10_U16_LEN: usize = 5;
const POW10_U32_LEN: usize = 10;
const POW10_U64_LEN: usize = 20;

//all powers of 10 that fit in a u8
const POW10_U8: [u8; 3] = [
    100,
    10,
    1
];

const POW10_U16: [u16; 5] = [
    10000,
    1000,
    100,
    10,
    1
];

const POW10_U32: [u32; 10] = [
    1000000000,
    100000000,
    10000000,
    1000000,
    100000,
    10000,
    1000,
    100,
    10,
    1,
];

const POW10_U64: [u64; 20] = [
        10000000000000000000,
        1000000000000000000,
        100000000000000000,
        10000000000000000,
        1000000000000000,
        100000000000000,
        10000000000000,
        1000000000000,
        100000000000,
        10000000000,
        1000000000,
        100000000,
        10000000,
        1000000,
        100000,
        10000,
        1000,
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
    /// It should be noted that trying trying to convert a string that does not fit in the chosen integer,
    /// causes undifined behaviour, and gives a panic in debug builds.
    /// For example:
    /// ```
    /// extern crate byte_num;
    /// use byte_num::convert::FromAscii;
    ///
    /// fn main () {
    ///     let n = std::panic::catch_unwind(|| u8::atoi("257"));
    ///     assert!(n.is_err(), true);
    /// }
    ///```
    #[inline]
    fn atoi<S: AsRef<[u8]>>(s: S) -> Result<Self, ()>
    {
        Self::bytes_to_int(s.as_ref())
    }

    /// Performs the actual conversion from a byteslice to an unsigned integer.
    fn bytes_to_int(s: &[u8]) -> Result<Self, ()>;
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
        Self: Copy
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
    ($d:ident, $r:ident, $bytes:expr, $idx:expr, $offset:expr, $const_table:ident) => (
        let $d = ($bytes.get_unchecked($offset).wrapping_sub(ASCII_TO_INT_FACTOR)) as Self;

        //if the digit is greater than 9, something went terribly horribly wrong.
        //return an Err(())
        if $d > 9 {
            return Err(())
        }
        let $r = $d * $const_table.get_unchecked($idx + $offset);
    )
}

macro_rules! impl_unsigned_conversions {
    ($int:ty, $const_table:ident, $const_table_len:ident) => (
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

                        result += r1 + r2 + r3 + r4;
                        len -= 4;
                        idx += 4;
                        bytes = bytes.get_unchecked(4..);
                    }

                    for offset in 0..len {
                        atoi_unroll!(d, r, bytes, idx, offset, $const_table);
                        result += r;
                    }
                    return Ok(result)
                }
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
            fn int_to_bytes(mut self, buff: &mut [u8])  {
                let mut q;
                let mut r;

                let mut q1;
                let mut r1;

                let mut q2;
                let mut r2;

                let mut r3;

                let mut len = buff.len();

                while self >= 10_000 {
                    q  = self / 10;
                    q1 = self / 100;
                    q2 = self / 1000;

                    r  = (self % 10)  as u8 + ASCII_TO_INT_FACTOR;
                    r1 =    (q % 10)  as u8 + ASCII_TO_INT_FACTOR;
                    r2 =    (q1 % 10) as u8 + ASCII_TO_INT_FACTOR;
                    r3 =    (q2 % 10) as u8 + ASCII_TO_INT_FACTOR;

                    unsafe {
                        // last index
                        *buff.get_unchecked_mut(len - 1) = r;
                        // second last
                        *buff.get_unchecked_mut(len - 2) = r1;
                        // third last
                        *buff.get_unchecked_mut(len - 3) = r2;
                        // fourth last
                        *buff.get_unchecked_mut(len - 4) = r3;
                    }

                    len -= 4;
                    self /= 10_000;
                }
                
                //fixup loop. This might not be run if self was a multiple of 10_000
                for byte in unsafe {buff.get_unchecked_mut(..len) }.iter_mut().rev() {
                    q = self / 10;
                    r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    *byte = r;

                    //there's nothing more to do.
                    if q == 0 { return }
                    self = q;
                }
            }

            // #[inline]
            // fn int_to_bytes(mut self, buff: &mut [u8]) {
            //     let mut q;
            //     let mut r;

            //     for byte in buff.iter_mut().rev() {
            //         q = self / 10;
            //         r = (self % 10) as u8;
            //         *byte = r + ASCII_TO_INT_FACTOR;
            //         self = q;
            //         if self == 0 { break }
            //     }
            // }
        }
    );
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
                result += r;
            }
            Ok(result)
        }
    }
}

impl IntoAscii for u8 {
    #[inline]
    fn digits10(self) -> usize {

        if self < 10 { return 1 }
        if self < 100 { return 2}
        return 3
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

            if self == 0 { break }
        }
    }
}


impl_unsigned_conversions!(u16, POW10_U16, POW10_U16_LEN);
impl_unsigned_conversions!(u32, POW10_U32, POW10_U32_LEN);
impl_unsigned_conversions!(u64, POW10_U64, POW10_U64_LEN);

macro_rules! impl_signed_conversions {
    ($int:ty, $unsigned_version:ty) => {
        impl FromAscii for $int {
            fn bytes_to_int(bytes: &[u8]) -> Result<Self, ()> {
                if bytes.starts_with(b"-") {
                    unsafe {
                        Ok(<$unsigned_version>::bytes_to_int(bytes.get_unchecked(1..))? as Self * -1)
                    }
                }
                else {
                    Ok(<$unsigned_version>::bytes_to_int(bytes)? as Self)
                }
            }
        }

        impl IntoAscii for $int {
            fn itoa(&self) -> Vec<u8>
            where
                Self: Copy
            {
                
                if self < &0 {
                    let n = self * -1;
                    let size = Self::digits10(n) + 1;

                    let mut buff = vec![0; size];
                    unsafe { *buff.get_unchecked_mut(0) = b'-';}
                    n.int_to_bytes(&mut buff);
                    buff
                }
                else {
                    let size = Self::digits10(*self);
                    let mut buff = vec![0; size];
                    self.int_to_bytes(&mut buff);
                    buff
                }

            }

            #[inline]
            fn digits10(self) -> usize {
                (self as $unsigned_version).digits10()
            }

            #[inline]
            fn int_to_bytes(self, buff: &mut [u8]) {
                (self as $unsigned_version).int_to_bytes(buff);
            }
        }
    }
}

impl_signed_conversions!(i8, u8);
impl_signed_conversions!(i16, u16);
impl_signed_conversions!(i32, u32);
impl_signed_conversions!(i64, u64);

#[cfg(test)]
mod test_itoa {
    use super::*;

    #[test]
    fn negative_parse() {
        assert_eq!(i32::atoi(b"-123"), Ok(-123));
        assert_eq!(i32::atoi(b"123"), Ok(123));

        assert_eq!(i32::atoi(b"123e"), Err(()));
    }
    #[test]
    fn test_itoa() {
        let n = 9987u32;

        assert_eq!(n.itoa(), [b'9', b'9', b'8', b'7']);
    }
}