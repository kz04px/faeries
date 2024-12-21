use super::bitboard::Bitboard;

#[derive(Debug, Clone)]
pub struct BitboardIter(u64);

impl Iterator for BitboardIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let index = self.0.trailing_zeros() as u8;
            self.0 &= self.0 - 1;
            Some(index)
        }
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> IntoIterator for Bitboard<WIDTH, HEIGHT> {
    type Item = u8;
    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter(self.0)
    }
}
