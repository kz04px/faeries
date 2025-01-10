use super::{bitboard::Bitboard, square::Square};

#[derive(Debug, Clone)]
pub struct BitboardIter<const WIDTH: usize, const HEIGHT: usize>(u64);

impl<const WIDTH: usize, const HEIGHT: usize> Iterator for BitboardIter<WIDTH, HEIGHT> {
    type Item = Square<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let idx = self.0.trailing_zeros() as i32;
            let sq = Square::<WIDTH, HEIGHT>::from_index(idx);
            self.0 &= self.0 - 1;
            Some(sq)
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IntoIterator for Bitboard<WIDTH, HEIGHT> {
    type Item = Square<WIDTH, HEIGHT>;
    type IntoIter = BitboardIter<WIDTH, HEIGHT>;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::general::{bitboard::Bitboard, square::Square};

    #[test]
    fn iter() {
        let bb = Bitboard::<8, 8>::default();
        let mut iter = bb.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter2() {
        let bb = Bitboard::<8, 8>(0xFF);
        let mut iter = bb.into_iter();
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(0)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(1)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(2)));
        assert_eq!(iter.next(), Some(Square::<8, 8>::from_index(3)));
    }
}
