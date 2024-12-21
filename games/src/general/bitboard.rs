use super::square::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bitboard<const WIDTH: u8, const HEIGHT: u8>(pub u64);

impl<const WIDTH: u8, const HEIGHT: u8> fmt::Display for Bitboard<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                let sq = WIDTH * y + x;
                if self.is_set(sq) {
                    write!(f, "1")?;
                } else {
                    write!(f, "0")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

#[must_use]
const fn all<const WIDTH: u8, const HEIGHT: u8>() -> Bitboard<WIDTH, HEIGHT> {
    Bitboard(0xffffffffffffffff >> (64 - WIDTH * HEIGHT))
}

#[must_use]
const fn get_leftmost<const WIDTH: u8, const HEIGHT: u8>() -> Bitboard<WIDTH, HEIGHT> {
    let mut mask = 0x1u64;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    Bitboard::<WIDTH, HEIGHT>(mask & all::<WIDTH, HEIGHT>().0)
}

#[must_use]
const fn get_rightmost<const WIDTH: u8, const HEIGHT: u8>() -> Bitboard<WIDTH, HEIGHT> {
    let mut mask = 0x1u64 << (WIDTH - 1);
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    mask |= mask << WIDTH;
    Bitboard::<WIDTH, HEIGHT>(mask & all::<WIDTH, HEIGHT>().0)
}

impl<const WIDTH: u8, const HEIGHT: u8> Bitboard<WIDTH, HEIGHT> {
    #[must_use]
    pub const fn from_index(idx: u8) -> Self {
        Self(1u64 << idx)
    }

    #[must_use]
    pub const fn from_coords(x: i32, y: i32) -> Self {
        Self::from_index(y as u8 * WIDTH + x as u8)
    }

    #[must_use]
    pub const fn from_square(sq: Square<WIDTH, HEIGHT>) -> Self {
        Self::from_index(sq.0)
    }

    #[must_use]
    pub fn file_of_index(idx: u8) -> Self {
        Self(get_leftmost::<WIDTH, HEIGHT>().0 << (idx % WIDTH))
    }

    #[must_use]
    pub fn north(&self) -> Self {
        Bitboard(self.0 << WIDTH) & all::<WIDTH, HEIGHT>()
    }

    #[must_use]
    pub fn south(&self) -> Self {
        Bitboard(self.0 >> WIDTH) & all::<WIDTH, HEIGHT>()
    }

    #[must_use]
    pub fn east(&self) -> Self {
        Bitboard(self.0 << 1) & !get_leftmost::<WIDTH, HEIGHT>() & all::<WIDTH, HEIGHT>()
    }

    #[must_use]
    pub fn west(&self) -> Self {
        Bitboard(self.0 >> 1) & !get_rightmost::<WIDTH, HEIGHT>() & all::<WIDTH, HEIGHT>()
    }

    #[must_use]
    pub fn ne(&self) -> Self {
        self.north().east()
    }

    #[must_use]
    pub fn nw(&self) -> Self {
        self.north().west()
    }

    #[must_use]
    pub fn se(&self) -> Self {
        self.south().east()
    }

    #[must_use]
    pub fn sw(&self) -> Self {
        self.south().west()
    }

    #[must_use]
    pub const fn is_set(&self, idx: u8) -> bool {
        (self.0 >> idx) & 1 == 1
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        *self == all::<WIDTH, HEIGHT>()
    }

    #[must_use]
    pub const fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub const fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    #[must_use]
    pub const fn lsb(&self) -> u8 {
        self.0.trailing_zeros() as u8
    }

    #[must_use]
    pub fn popped(&self) -> Self {
        *self ^ Self::from_index(self.lsb())
    }

    // const LEFTMOST: Bitboard<WIDTH, HEIGHT> = get_leftmost();
    // const RIGHTMOST: Bitboard<WIDTH, HEIGHT> = get_rightmost();
    // const NOTLEFTMOST: Bitboard<WIDTH, HEIGHT> = !get_leftmost();

    // const RIGHTMOST: Bitboard<WIDTH, HEIGHT> = Bitboard::<WIDTH, HEIGHT>(0x0 & 0x1);
    // const NOT_RIGHTMOST: Bitboard<WIDTH, HEIGHT> = all::<WIDTH, HEIGHT>() & !RIGHTMOST;

    #[must_use]
    pub fn adjacent(&self) -> Self {
        Bitboard(
            (self.0 << WIDTH)
                | (self.0 >> WIDTH)
                | ((self.0 >> 1) | (self.0 >> (WIDTH + 1)) | (self.0 << (WIDTH - 1)))
                    & all::<WIDTH, HEIGHT>().west().0
                | ((self.0 << 1) | (self.0 << (WIDTH + 1)) | (self.0 >> (WIDTH - 1)))
                    & all::<WIDTH, HEIGHT>().east().0,
        )
    }

    #[must_use]
    pub fn dist2(&self) -> Self {
        Bitboard(
            // Left 2
            ((self.0 >> 2) | (self.0 >> (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH - 2)) | (self.0 << (WIDTH + WIDTH - 2))) & all::<WIDTH, HEIGHT>().west().west().0
            // Left 1
            | ((self.0 >> (WIDTH + WIDTH + 1)) | (self.0 << (WIDTH + WIDTH - 1))) & all::<WIDTH, HEIGHT>().west().0
            // Centre
            | (self.0 << (WIDTH + WIDTH)) | (self.0 >> (WIDTH + WIDTH))
            // Right 1
            | ((self.0 << (WIDTH + WIDTH + 1)) | (self.0 >> (WIDTH + WIDTH - 1))) & all::<WIDTH, HEIGHT>().east().0
            // Right 2
            | ((self.0 << 2) | (self.0 << (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH - 2)) | (self.0 >> (WIDTH - 2))) & all::<WIDTH, HEIGHT>().east().east().0,
        )
    }

    #[must_use]
    pub fn reach2(&self) -> Self {
        Bitboard(
            // Left 2
            ((self.0 >> 2) | (self.0 >> (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH - 2)) | (self.0 << (WIDTH + WIDTH - 2))) & all::<WIDTH, HEIGHT>().west().west().0
            // Left 1
            | ((self.0 >> 1) | (self.0 >> (WIDTH + 1)) | (self.0 >> (WIDTH + WIDTH + 1)) | (self.0 << (WIDTH - 1)) | (self.0 << (WIDTH + WIDTH - 1))) & all::<WIDTH, HEIGHT>().west().0
            // Centre
            | ((self.0 << WIDTH)| (self.0 << (WIDTH + WIDTH)) | (self.0 >> WIDTH) | (self.0 >> (WIDTH + WIDTH))) & all::<WIDTH, HEIGHT>().0
            // Right 1
            | ((self.0 << 1) | (self.0 << (WIDTH + 1)) | (self.0 << (WIDTH + WIDTH + 1)) | (self.0 >> (WIDTH - 1)) | (self.0 >> (WIDTH + WIDTH - 1))) & all::<WIDTH, HEIGHT>().east().0
            // Right 2
            | ((self.0 << 2) | (self.0 << (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH - 2)) | (self.0 >> (WIDTH - 2))) & all::<WIDTH, HEIGHT>().east().east().0,
        )
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitAnd for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;

    #[must_use]
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> Not for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;

    #[must_use]
    fn not(self) -> Self {
        Self(!self.0) & all::<WIDTH, HEIGHT>()
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitAndAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitand_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 &= rhs.0;
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitOrAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitor_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 |= rhs.0;
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitXorAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitxor_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 ^= rhs.0;
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitXor for Bitboard<WIDTH, HEIGHT> {
    type Output = Bitboard<WIDTH, HEIGHT>;

    #[must_use]
    fn bitxor(self, rhs: Bitboard<WIDTH, HEIGHT>) -> Bitboard<WIDTH, HEIGHT> {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> BitOr for Bitboard<WIDTH, HEIGHT> {
    type Output = Bitboard<WIDTH, HEIGHT>;

    #[must_use]
    fn bitor(self, rhs: Bitboard<WIDTH, HEIGHT>) -> Bitboard<WIDTH, HEIGHT> {
        Bitboard(self.0 | rhs.0)
    }
}

// #[must_use]
// pub const fn get_file<const WIDTH: u8, const HEIGHT: u8>(sq: u8) -> Bitboard<WIDTH, HEIGHT> {
//     Bitboard(0x0101010101010101 << (sq % 8))
// }

// #[must_use]
// pub const fn get_rank<const WIDTH: u8, const HEIGHT: u8>(sq: u8) -> Bitboard<WIDTH, HEIGHT> {
//     Bitboard(0xFFu64 << (8 * (sq / 8)))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(all::<8, 8>(), Bitboard::<8, 8>(0xffffffffffffffff));
        assert_eq!(all::<7, 8>(), Bitboard::<7, 8>(0xffffffffffffff));
        assert_eq!(all::<7, 7>(), Bitboard::<7, 7>(0x1ffffffffffff));
        assert_eq!(all::<1, 5>(), Bitboard::<1, 5>(0x1f));
        assert_eq!(all::<2, 2>(), Bitboard::<2, 2>(0xf));
        assert_eq!(all::<1, 1>(), Bitboard::<1, 1>(0x1));
    }

    #[test]
    fn test_north() {
        assert_eq!(Bitboard::<8, 8>(0xff).north(), Bitboard::<8, 8>(0xff00));
        assert_eq!(Bitboard::<7, 7>(0x7f).north(), Bitboard::<7, 7>(0x3f80));
    }

    #[test]
    fn test_south() {}

    #[test]
    fn test_east() {
        assert_eq!(Bitboard::<8, 8>(0x0).east(), Bitboard::<8, 8>(0x0));
        assert_eq!(
            Bitboard::<8, 8>(0x8080808080808080).east(),
            Bitboard::<8, 8>(0x0)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x4040404040404040).east(),
            Bitboard::<8, 8>(0x8080808080808080)
        );
    }

    #[test]
    fn test_west() {
        assert_eq!(Bitboard::<8, 8>(0x0).west(), Bitboard::<8, 8>(0x0));
    }

    #[test]
    fn test_file_of_index() {
        assert_eq!(
            Bitboard::<8, 8>::file_of_index(0),
            Bitboard::<8, 8>(0x101010101010101)
        );
        assert_eq!(
            Bitboard::<8, 8>::file_of_index(1),
            Bitboard::<8, 8>(0x202020202020202)
        );
        assert_eq!(
            Bitboard::<8, 8>::file_of_index(7),
            Bitboard::<8, 8>(0x8080808080808080)
        );
        assert_eq!(
            Bitboard::<8, 8>::file_of_index(28),
            Bitboard::<8, 8>(0x1010101010101010)
        );
        assert_eq!(
            Bitboard::<7, 7>::file_of_index(0),
            Bitboard::<7, 7>(0x40810204081)
        );
        assert_eq!(
            Bitboard::<7, 7>::file_of_index(1),
            Bitboard::<7, 7>(0x81020408102)
        );
    }

    // #[test]
    // fn test_leftmost() {
    //     assert_eq!(
    //         Bitboard::<8, 8>::LEFTMOST,
    //         Bitboard::<8, 8>(0x101010101010101)
    //     );
    // }

    // #[test]
    // fn test_rightmost() {
    //     assert_eq!(
    //         Bitboard::<8, 8>::RIGHTMOST,
    //         Bitboard::<8, 8>(0x8080808080808080)
    //     );
    // }

    #[test]
    fn test_adjacent() {
        assert_eq!(Bitboard::<8, 8>(0x0).adjacent(), Bitboard::<8, 8>(0x0));
        assert_eq!(Bitboard::<8, 8>(0x1).adjacent(), Bitboard::<8, 8>(0x302));
        assert_eq!(
            Bitboard::<8, 8>(0x2442810000814224).adjacent(),
            Bitboard::<8, 8>(0xffffe7c3c3e7ffff)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x2400810000810024).adjacent(),
            Bitboard::<8, 8>(0x5aff42c3c342ff5a)
        );
        assert_eq!(Bitboard::<7, 7>(0x0).adjacent(), Bitboard::<7, 7>(0x0));
        assert_eq!(Bitboard::<7, 7>(0x1).adjacent(), Bitboard::<7, 7>(0x182));
        assert_eq!(Bitboard::<2, 2>(0x0).adjacent(), Bitboard::<2, 2>(0x0));
        assert_eq!(Bitboard::<2, 2>(0x1).adjacent(), Bitboard::<2, 2>(0xe));
    }

    #[test]
    fn test_dist2() {
        assert_eq!(Bitboard::<8, 8>(0x0).dist2(), Bitboard::<8, 8>(0x0));
        assert_eq!(Bitboard::<8, 8>(0x1).dist2(), Bitboard::<8, 8>(0x70404));
        assert_eq!(
            Bitboard::<8, 8>(0x10000000).dist2(),
            Bitboard::<8, 8>(0x7c4444447c00)
        );
    }

    #[test]
    fn test_reach2() {
        assert_eq!(Bitboard::<8, 8>(0x0).reach2(), Bitboard::<8, 8>(0x0));
        assert_eq!(Bitboard::<8, 8>(0x1).reach2(), Bitboard::<8, 8>(0x70706));
        assert_eq!(
            Bitboard::<8, 8>(0x10000000).reach2(),
            Bitboard::<8, 8>(0x7c7c6c7c7c00)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x8000000000000000).reach2(),
            Bitboard::<8, 8>(0x60e0e00000000000)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x100000000000000).reach2(),
            Bitboard::<8, 8>(0x607070000000000)
        );
        assert_eq!(Bitboard::<8, 8>(0x80).reach2(), Bitboard::<8, 8>(0xe0e060));
        assert_eq!(all::<7, 7>().reach2(), all::<7, 7>());
    }

    #[test]
    fn test_count() {
        assert_eq!(Bitboard::<8, 8>(0x0).count(), 0);
        assert_eq!(Bitboard::<8, 8>(0x1).count(), 1);
        assert_eq!(Bitboard::<8, 8>(0x2).count(), 1);
        assert_eq!(Bitboard::<8, 8>(0x3).count(), 2);
        assert_eq!(all::<8, 8>().count(), 64);
        assert_eq!(all::<7, 8>().count(), 56);
        assert_eq!(all::<7, 7>().count(), 49);
        assert_eq!(all::<1, 5>().count(), 5);
        assert_eq!(all::<2, 2>().count(), 4);
        assert_eq!(all::<1, 1>().count(), 1);
    }
}
