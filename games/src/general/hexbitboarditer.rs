use super::{hex::Hex, hexbitboard::HexBitboard};

#[derive(Debug, Clone)]
pub struct HexBitboardIter<const WIDTH: usize, const HEIGHT: usize>(u64);

impl<const WIDTH: usize, const HEIGHT: usize> Iterator for HexBitboardIter<WIDTH, HEIGHT> {
    type Item = Hex<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let idx = self.0.trailing_zeros() as i32;
            let hex = Hex::<WIDTH, HEIGHT>::from_index(idx);
            self.0 &= self.0 - 1;
            Some(hex)
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> IntoIterator for HexBitboard<WIDTH, HEIGHT> {
    type Item = Hex<WIDTH, HEIGHT>;
    type IntoIter = HexBitboardIter<WIDTH, HEIGHT>;

    fn into_iter(self) -> Self::IntoIter {
        HexBitboardIter(self.0)
    }
}
