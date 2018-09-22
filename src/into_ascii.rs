const ASCII_TO_INT_FACTOR: u8 = 48;

pub trait IntoAscii {
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

    fn int_to_bytes(self, buff: &mut [u8]);
}

macro_rules! impl_unsigned_conversions {
    ($int:ty) => {
        impl IntoAscii for $int {
            #[inline]
            fn digits10(mut self) -> usize {
                let mut result = 1;
        
                loop {
                    if self < 10 {
                        break result;
                    } else if self < 100 {
                        break result + 1;
                    } else if self < 1000 {
                        break result + 2;
                    } else if self < 10000 {
                        break result + 3;
                    }
        
                    self /= 10_000;
                    result += 4;
                }
            }
        
            fn int_to_bytes(mut self, buff: &mut [u8]) {
                // [1, 2, 3, 4, 5].exact_chunks(2).rev() gives [3, 4] and [2, 1],
                // while we wanted [4, 5] and [2, 3].
                // So we make the remainder ourselves.
                let rem = buff.len() % 4;
                let splitof = buff.len() - (buff.len() - rem);
                let (remainder, v) = buff.split_at_mut(splitof);
        
                for mut chunk in v.exact_chunks_mut(4).rev() {
                    let q = self / 10;
                    let q1 = self / 100;
                    let q2 = self / 1000;
        
                    let r = (self % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r1 = (q % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r2 = (q1 % 10) as u8 + ASCII_TO_INT_FACTOR;
                    let r3 = (q2 % 10) as u8 + ASCII_TO_INT_FACTOR;
        
                    // @NOTE: Make me nicer when NLL hits stable
                    match &mut chunk {
                        &mut [ref mut b3, ref mut b2, ref mut b1, ref mut b] => {
                            *b = r;
                            *b1 = r1;
                            *b2 = r2;
                            *b3 = r3;
                        }
                        _ => unreachable!(),
                    }
        
                    self /= 10_000;
                }
        
                for byte in remainder.iter_mut().rev() {
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
        
            fn int_to_bytes(self, buff: &mut [u8]) {
                unimplemented!()
            }
        }
    };
}

impl_unsigned_conversions!(@u8);
impl_unsigned_conversions!(u16);
impl_unsigned_conversions!(u32);
impl_unsigned_conversions!(u64);
impl_unsigned_conversions!(usize);

#[cfg(test)]
mod tests {
    use super::IntoAscii;

    #[test]
    fn int_to_bytes_usize() {
        assert_eq!(99_999_9999usize.itoa(), vec![b'9'; 9]);
    }
}
