const ASCII_TO_INT_FACTOR: u8 = 48;

/// A trait that converts integers to a representation in bytes.
/// That is, `99usize` in bytes is `[b'9', b'9']`.
/// Negative values will also include a '-' in their byte representation.
pub trait IntoAscii {
    /// Converts `self` into it's representation in bytes.
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

    /// Writes `self` into `buff`.
    /// This function assumes `buff` has enough space to hold all digits of `self`. For the number of digits `self`has, see [`IntoAscii::digits10`].
    fn int_to_bytes(self, buff: &mut [u8]);
}

macro_rules! unsigned_into_ascii {
    ($int:ty) => {
        impl IntoAscii for $int {
            #[inline]
            #[clippy::skip]
            fn digits10(mut self) -> usize {
                let mut result = 1;
        
                loop {
                    if self < 10 { break result;}
                    if self < 100 { break result + 1; }
                    if self < 1000 { break result + 2; }
                    if self < 10000 { break result + 3; }
        
                    self /= 10_000;
                    result += 4;
                }
            }
        
            #[inline]
            fn int_to_bytes(mut self, buff: &mut [u8]) {
                let mut chunked = buff.rchunks_exact_mut(4);
                for mut chunk in chunked.by_ref() {
                    let q = self / 10;
                    let q1 = self / 100;
                    let q2 = self / 1000;
        
                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r1 = (q % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r2 = (q1 % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r3 = (q2 % 10) as u8 + ASCII_TO_INT_FACTOR;
        
                    // @NOTE: Make me nicer when NLL hits stable
                    match &mut chunk {
                        [b3, b2, b1, b] => {
                            *b = r;
                            *b1 = r1;
                            *b2 = r2;
                            *b3 = r3;
                        }
                        _ => unreachable!(),
                    }
        
                    self /= 10_000;
                }
        
                for byte in chunked.into_remainder().iter_mut().rev() {
                    let q = self / 10;
                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    *byte = r;
        
                    //there's nothing more to do.
                    if q == 0 {
                        break;
                    }
        
                    self = q;
                }
            }
        }
    };

    (@u8) => {
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
                for byte in buff.iter_mut().rev() {
                    let q = self / 10;
                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    *byte = r;
        
                    if self == 0 {
                        break;
                    }
        
                    self = q;
                }
            }
        }
    };
}

macro_rules! signed_into_ascii {
    ($int:ty, $unsigned_version:ty) => {
        impl IntoAscii for $int {
            #[inline]
            fn itoa(&self) -> Vec<u8>
            where
                Self: Copy,
            {
                let (n, size) = if self.is_negative() {
                    (self * -1, self.digits10() + 1)
                } else {
                    (*self, self.digits10())
                };
        
                let mut buff = vec![b'-'; size];
                (n as $unsigned_version).int_to_bytes(&mut buff);
                buff
            }
        
            #[inline]
            fn digits10(self) -> usize {
                (self.abs() as $unsigned_version).digits10()
            }
        
            #[inline]
            fn int_to_bytes(self, buff: &mut [u8]) {
                if self.is_negative() {
                    (self.abs() as $unsigned_version).int_to_bytes(buff);
                    buff[0] = b'-';
                } else {
                    (self as $unsigned_version).int_to_bytes(buff);
                }
            }
        }
    };
}

unsigned_into_ascii!(@u8);
unsigned_into_ascii!(u16);
unsigned_into_ascii!(u32);
unsigned_into_ascii!(u64);
unsigned_into_ascii!(usize);

signed_into_ascii!(i8, u8);
signed_into_ascii!(i16, u16);
signed_into_ascii!(i32, u32);
signed_into_ascii!(i64, u64);
signed_into_ascii!(isize, usize);

impl<'a, N: Copy> IntoAscii for &'a N
where
    N: IntoAscii,
{
    #[inline]
    fn digits10(self) -> usize {
        (*self).digits10()
    }

    #[inline]
    fn int_to_bytes(self, buff: &mut [u8]) {
        (*self).int_to_bytes(buff);
    }
}

impl<'a, N: Copy> IntoAscii for &'a mut N
where
    N: IntoAscii,
{
    #[inline]
    fn digits10(self) -> usize {
        (*self).digits10()
    }

    #[inline]
    fn int_to_bytes(self, buff: &mut [u8]) {
        (*self).int_to_bytes(buff);
    }
}

impl<N: Copy> IntoAscii for Box<N>
where
    N: IntoAscii,
{
    #[inline]
    fn digits10(self) -> usize {
        (*self).digits10()
    }

    #[inline]
    fn int_to_bytes(self, buff: &mut [u8]) {
        (*self).int_to_bytes(buff);
    }
}

#[cfg(test)]
mod tests {
    use super::IntoAscii;

    #[test]
    fn itoa_usize() {
        assert_eq!(
            123_456_789usize.itoa(),
            vec![b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']
        );
    }

    #[test]
    fn itoa_isize() {
        assert_eq!(
            (-123_456_789isize).itoa(),
            vec![b'-', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9']
        );
    }

    #[test]
    fn itoa_0usize() {
        assert_eq!(0usize.itoa(), vec![b'0']);
    }

    #[test]
    fn itoa_0isize() {
        assert_eq!((-0isize).itoa(), vec![b'0']);
    }

    #[test]
    fn digits10_usize() {
        assert_eq!(123456789usize.digits10(), 9);
    }

    #[test]
    fn digits10_isize() {
        assert_eq!((-123456789isize).digits10(), 9);
    }

    #[test]
    fn digits10_0usize() {
        assert_eq!(0usize.digits10(), 1);
    }

    #[test]
    fn digits10_0isize() {
        assert_eq!((-0isize).digits10(), 1);
    }
}
