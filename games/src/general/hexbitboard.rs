use crate::general::hex::Hex;
use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HexBitboard<const WIDTH: usize, const HEIGHT: usize>(pub u64);

impl<const WIDTH: usize, const HEIGHT: usize> HexBitboard<WIDTH, HEIGHT> {
    #[must_use]
    pub fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(0xffffffffffffffff >> (64 - Self::size()))
    }

    #[must_use]
    pub const fn size() -> usize {
        WIDTH * HEIGHT + HEIGHT / 2
    }

    #[must_use]
    pub fn count(&self) -> i32 {
        self.0.count_ones() as i32
    }

    #[must_use]
    pub fn lsb(&self) -> Hex<WIDTH, HEIGHT> {
        Hex::from_index(self.0.trailing_zeros() as i32)
    }

    #[must_use]
    pub fn from_hex(hex: &Hex<WIDTH, HEIGHT>) -> Self {
        Self::from_index(hex.get_index() as i32)
    }

    #[must_use]
    pub fn from_index(idx: i32) -> Self {
        Self(1u64 << idx)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn is_occupied(&self) -> bool {
        self.0 != 0
    }

    #[must_use]
    pub fn is_set(&self, hex: &Hex<WIDTH, HEIGHT>) -> bool {
        ((self.0 >> hex.get_index()) & 1) == 1
    }

    #[must_use]
    pub fn adjacent(&self) -> Self {
        self.left()
            | self.right()
            | self.up_left()
            | self.up_right()
            | self.down_left()
            | self.down_right()
    }

    #[must_use]
    pub const fn get_left_edge() -> Self {
        let mut mask = 0u64;

        let mut i = 0;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        let mut i = WIDTH;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        Self(mask)
    }

    #[must_use]
    pub const fn get_right_edge() -> Self {
        let mut mask = 0u64;

        let mut i = WIDTH - 1;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        let mut i = 2 * WIDTH;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        Self(mask)
    }

    #[must_use]
    pub const fn get_bottom_edge() -> Self {
        let mut mask = 0x1u64;
        let mut i = 1;
        while i < WIDTH {
            mask |= mask << 1;
            i += 1;
        }
        Self(mask)
    }

    #[must_use]
    pub const fn get_top_edge() -> Self {
        let mut mask = 0x1u64 << Self::size() - 1;
        let mut i = 1;
        while i < WIDTH {
            mask |= mask >> 1;
            i += 1;
        }
        Self(mask)
    }

    #[must_use]
    pub const fn get_border() -> Self {
        Self(
            Self::get_left_edge().0
                | Self::get_right_edge().0
                | Self::get_bottom_edge().0
                | Self::get_top_edge().0,
        )
    }

    #[must_use]
    pub const fn get_left_doodads() -> Self {
        let mut mask = 0u64;

        let mut i = WIDTH;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        Self(mask)
    }

    #[must_use]
    pub const fn get_right_doodads() -> Self {
        let mut mask = 0u64;

        let mut i = 2 * WIDTH;
        while i < Self::size() {
            mask |= 0x1u64 << i;
            i += 2 * WIDTH + 1;
        }

        Self(mask)
    }

    #[must_use]
    pub fn doubles(&self, blockers: Self) -> Self {
        (self.left() & !blockers).left()
            | (self.right() & !blockers).right()
            | (self.up_left() & !blockers).up_left()
            | (self.up_right() & !blockers).up_right()
            | (self.down_left() & !blockers).down_left()
            | (self.down_right() & !blockers).down_right()
    }

    #[must_use]
    pub fn left(&self) -> Self {
        Self(self.0 >> 1) & !Self::get_right_edge()
    }

    #[must_use]
    pub fn right(&self) -> Self {
        Self(self.0 << 1) & !Self::get_left_edge()
    }

    #[must_use]
    pub fn up_left(&self) -> Self {
        Self(self.0 << WIDTH) & !Self::get_right_doodads()
    }

    #[must_use]
    pub fn up_right(&self) -> Self {
        Self(self.0 << WIDTH << 1) & !Self::get_left_doodads()
    }

    #[must_use]
    pub fn down_left(&self) -> Self {
        Self(self.0 >> WIDTH >> 1) & !Self::get_right_doodads()
    }

    #[must_use]
    pub fn down_right(&self) -> Self {
        Self(self.0 >> WIDTH) & !Self::get_left_doodads()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Not for HexBitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0) & Self::all()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitAnd for HexBitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitOr for HexBitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitXor for HexBitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitOrAssign for HexBitboard<WIDTH, HEIGHT> {
    fn bitor_assign(&mut self, rhs: Self) -> () {
        *self = *self | rhs;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitXorAssign for HexBitboard<WIDTH, HEIGHT> {
    fn bitxor_assign(&mut self, rhs: Self) -> () {
        *self = *self ^ rhs;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitAndAssign for HexBitboard<WIDTH, HEIGHT> {
    fn bitand_assign(&mut self, rhs: Self) -> () {
        *self = *self & rhs;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for HexBitboard<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in (0..HEIGHT).rev() {
            let is_long = y % 2 == 1;

            if !is_long {
                write!(f, " ")?;
            }

            for x in 0..(WIDTH + is_long as usize) {
                let hex = Hex::from_coords(x as i32, y as i32);

                if self.is_set(&hex) {
                    write!(f, " 1")?
                } else {
                    write!(f, " 0")?
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(HexBitboard::<1, 1>::size(), 1);
        assert_eq!(HexBitboard::<2, 2>::size(), 5);
        assert_eq!(HexBitboard::<3, 1>::size(), 3);
        assert_eq!(HexBitboard::<3, 2>::size(), 7);
        assert_eq!(HexBitboard::<3, 3>::size(), 10);
        assert_eq!(HexBitboard::<3, 4>::size(), 14);
        assert_eq!(HexBitboard::<4, 1>::size(), 4);
        assert_eq!(HexBitboard::<4, 2>::size(), 9);
        assert_eq!(HexBitboard::<4, 3>::size(), 13);
        assert_eq!(HexBitboard::<4, 4>::size(), 18);
        assert_eq!(HexBitboard::<4, 5>::size(), 22);
    }

    #[test]
    fn all() {
        assert_eq!(HexBitboard::<1, 1>::all().0, 0x1);
        assert_eq!(HexBitboard::<4, 1>::all().0, 0xf);
        assert_eq!(HexBitboard::<4, 5>::all().0, 0x3fffff);
    }

    #[test]
    fn bitor() {
        assert_eq!(
            HexBitboard::<6, 7>(0x0) | HexBitboard::<6, 7>(0x0),
            HexBitboard::<6, 7>(0x0)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x0) | HexBitboard::<6, 7>(0x1),
            HexBitboard::<6, 7>(0x1)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x1) | HexBitboard::<6, 7>(0x2),
            HexBitboard::<6, 7>(0x3)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x3) | HexBitboard::<6, 7>(0x2),
            HexBitboard::<6, 7>(0x3)
        );
    }

    #[test]
    fn bitxor() {
        assert_eq!(
            HexBitboard::<6, 7>(0x0) ^ HexBitboard::<6, 7>(0x0),
            HexBitboard::<6, 7>(0x0)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x0) ^ HexBitboard::<6, 7>(0x1),
            HexBitboard::<6, 7>(0x1)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x1) ^ HexBitboard::<6, 7>(0x2),
            HexBitboard::<6, 7>(0x3)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x3) ^ HexBitboard::<6, 7>(0x2),
            HexBitboard::<6, 7>(0x1)
        );
    }

    #[test]
    fn left() {
        assert_eq!(
            HexBitboard::<6, 7>(1u64 << 22).left(),
            HexBitboard::<6, 7>(1u64 << 21)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x1fffffffffffu64).left(),
            HexBitboard::<6, 7>(0xfbf7dfbefdfu64)
        );

        // Left edge wrap
        assert!(HexBitboard::<6, 7>(1u64 << 0).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 6).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 13).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 19).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 26).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 32).left().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 39).left().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 0).left().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 4).left().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 9).left().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 13).left().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 18).left().is_empty());
    }

    #[test]
    fn right() {
        assert_eq!(
            HexBitboard::<6, 7>(1u64 << 21).right(),
            HexBitboard::<6, 7>(1u64 << 22)
        );
        assert_eq!(
            HexBitboard::<6, 7>(0x1fffffffffffu64).right(),
            HexBitboard::<6, 7>(0x1f7efbf7dfbeu64)
        );

        // Right edge wrap
        assert!(HexBitboard::<6, 7>(1u64 << 5).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 12).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 18).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 25).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 31).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 38).right().is_empty());
        assert!(HexBitboard::<6, 7>(1u64 << 44).right().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 3).right().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 8).right().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 12).right().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 17).right().is_empty());
        assert!(HexBitboard::<4, 5>(1u64 << 21).right().is_empty());
    }

    #[test]
    fn up_left() {
        let tests = [
            (0x0, 0x0),
            (1u64 << 22, 1u64 << 28),
            (0x1fffffffffffu64, 0x1fbffdffefc0u64),
            // Left edge wrap
            (1u64 << 0, 1u64 << 6),
            (1u64 << 6, 0x0),
            (1u64 << 13, 1u64 << 19),
            (1u64 << 19, 0x0),
            (1u64 << 26, 1u64 << 32),
            (1u64 << 32, 0x0),
            (1u64 << 39, 0x0),
        ];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<6, 7>(mask_a);
            let b = HexBitboard::<6, 7>(mask_b);
            assert_eq!(a.up_left(), b);
        }

        let tests = [
            (0x0, 0x0),
            (0xf, 0xf0),
            (0x119, 0x1090),
            (0x42211, 0x2010),
            (0x3c0000, 0x0),
        ];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<4, 5>(mask_a);
            let b = HexBitboard::<4, 5>(mask_b);
            assert_eq!(a.up_left(), b);
        }
    }

    #[test]
    fn down_left() {
        let tests = [
            (0x0, 0x0),
            (1u64 << 22, 1u64 << 15),
            (0x1fffffffffffu64, 0x3ffdffefffu64),
            // Left edge wrap
            (1u64 << 0, 0x0),
            (1u64 << 6, 0x0),
            (1u64 << 13, 1u64 << 6),
            (1u64 << 19, 0x0),
            (1u64 << 26, 1u64 << 19),
            (1u64 << 32, 0x0),
            (1u64 << 39, 1u64 << 32),
        ];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<6, 7>(mask_a);
            let b = HexBitboard::<6, 7>(mask_b);
            assert_eq!(a.down_left(), b);
        }
    }

    #[test]
    fn up_right() {
        let tests = [
            (0x0, 0x0),
            (1u64 << 22, 1u64 << 29),
            (0x1fffffffffffu64, 0x1ffefff7ff80u64),
            // Right edge wrap
            (1u64 << 5, 1u64 << 12),
            (1u64 << 12, 0x0),
            (1u64 << 18, 1u64 << 25),
            (1u64 << 25, 0x0),
            (1u64 << 31, 1u64 << 38),
            (1u64 << 38, 0x0),
            (1u64 << 44, 0x0),
        ];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<6, 7>(mask_a);
            let b = HexBitboard::<6, 7>(mask_b);
            assert_eq!(a.up_right(), b);
        }
    }

    #[test]
    fn down_right() {
        let tests = [
            (0x0, 0x0),
            (1u64 << 22, 1u64 << 16),
            (0x1fffffffffffu64, 0x7efff7ffbfu64),
            // Right edge wrap
            (1u64 << 5, 0x0),
            (1u64 << 12, 0x0),
            (1u64 << 18, 1u64 << 12),
            (1u64 << 25, 0x0),
            (1u64 << 31, 1u64 << 25),
            (1u64 << 38, 0x0),
            (1u64 << 44, 1u64 << 38),
        ];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<6, 7>(mask_a);
            let b = HexBitboard::<6, 7>(mask_b);
            assert_eq!(a.down_right(), b);
        }
    }

    #[test]
    fn adjacent() {
        let tests = [(0x0, 0x0), (1u64 << 22, 0x30a18000u64)];

        for (mask_a, mask_b) in tests {
            let a = HexBitboard::<6, 7>(mask_a);
            let b = HexBitboard::<6, 7>(mask_b);
            assert_eq!(a.adjacent(), b);
        }
    }

    #[test]
    fn test_left_edge() {
        assert_eq!(HexBitboard::<1, 1>::get_left_edge().0, 0x1);
        assert_eq!(HexBitboard::<2, 3>::get_left_edge().0, 0x25);
        assert_eq!(HexBitboard::<4, 5>::get_left_edge().0, 0x42211);
    }

    #[test]
    fn test_right_edge() {
        assert_eq!(HexBitboard::<1, 1>::get_right_edge().0, 0x1);
        assert_eq!(HexBitboard::<2, 3>::get_right_edge().0, 0x52);
        assert_eq!(HexBitboard::<4, 5>::get_right_edge().0, 0x221108);
    }

    #[test]
    fn test_top_edge() {
        assert_eq!(HexBitboard::<1, 1>::get_top_edge().0, 0x1);
        assert_eq!(HexBitboard::<2, 3>::get_top_edge().0, 0x60);
        assert_eq!(HexBitboard::<4, 5>::get_top_edge().0, 0x3c0000);
    }

    #[test]
    fn test_bottom_edge() {
        assert_eq!(HexBitboard::<1, 1>::get_bottom_edge().0, 0x1);
        assert_eq!(HexBitboard::<2, 3>::get_bottom_edge().0, 0x3);
        assert_eq!(HexBitboard::<4, 5>::get_bottom_edge().0, 0xf);
    }
}
