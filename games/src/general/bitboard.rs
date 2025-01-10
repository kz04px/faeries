use super::file::File;
use super::rank::Rank;
use super::square::Square;
use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bitboard<const WIDTH: usize, const HEIGHT: usize>(pub u64);

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Bitboard<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                let sq = Square::from_coords(x as i32, y as i32);
                if self.is_square_set(sq) {
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
pub fn floodfill<const WIDTH: usize, const HEIGHT: usize>(
    mut us: Bitboard<WIDTH, HEIGHT>,
    allowed: Bitboard<WIDTH, HEIGHT>,
) -> Bitboard<WIDTH, HEIGHT> {
    for _ in 0..15 {
        us = (us | us.north() | us.south() | us.east() | us.west()) & allowed;
    }
    us
}

impl Bitboard<8, 8> {
    #[must_use]
    pub fn swap(&self) -> Self {
        Self(self.0.swap_bytes())
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Bitboard<WIDTH, HEIGHT> {
    #[must_use]
    pub const fn from_index(idx: usize) -> Self {
        Self(1u64 << idx)
    }

    #[must_use]
    pub const fn from_coords(x: i32, y: i32) -> Self {
        Self::from_index(y as usize * WIDTH + x as usize)
    }

    #[must_use]
    pub const fn from_square(sq: Square<WIDTH, HEIGHT>) -> Self {
        Self::from_index(sq.get_index())
    }

    #[must_use]
    pub fn from_file(f: File<WIDTH>) -> Self {
        Self(Self::get_left_edge().0 << f.get_index())
    }

    #[must_use]
    pub fn from_rank(r: Rank<HEIGHT>) -> Self {
        Self(Self::get_bottom_edge().0 << r.get_index() * WIDTH)
    }

    #[must_use]
    pub fn north(&self) -> Self {
        Bitboard(self.0 << WIDTH) & Self::all()
    }

    #[must_use]
    pub fn south(&self) -> Self {
        Bitboard(self.0 >> WIDTH) & Self::all()
    }

    #[must_use]
    pub fn east(&self) -> Self {
        Bitboard(self.0 << 1) & !Self::get_left_edge() & Self::all()
    }

    #[must_use]
    pub fn west(&self) -> Self {
        Bitboard(self.0 >> 1) & !Self::get_right_edge() & Self::all()
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
    pub fn knights(&self) -> Self {
        self.north().north().east()
            | self.north().north().west()
            | self.south().south().east()
            | self.south().south().west()
            | self.east().east().north()
            | self.east().east().south()
            | self.west().west().north()
            | self.west().west().south()
    }

    #[must_use]
    pub const fn all() -> Self {
        Self(0xffffffffffffffff >> (64 - WIDTH * HEIGHT))
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(0x0)
    }

    #[must_use]
    pub const fn get_left_edge() -> Self {
        let mut mask = 0x1u64;
        let mut i = 1;
        while i < HEIGHT {
            mask |= mask << WIDTH;
            i += 1;
        }
        Self(mask)
    }

    #[must_use]
    pub const fn get_right_edge() -> Self {
        let mut mask = 0x1u64 << (WIDTH - 1);
        let mut i = 1;
        while i < HEIGHT {
            mask |= mask << WIDTH;
            i += 1;
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
        let mut mask = 0x1u64 << (WIDTH * HEIGHT - 1);
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
    pub const fn is_index_set(&self, idx: i32) -> bool {
        (self.0 >> idx) & 1 == 1
    }

    #[must_use]
    pub const fn is_square_set(&self, sq: Square<WIDTH, HEIGHT>) -> bool {
        (self.0 >> sq.get_index()) & 1 == 1
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[must_use]
    pub fn is_full(&self) -> bool {
        *self == Self::all()
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
    pub const fn lsb(&self) -> Square<WIDTH, HEIGHT> {
        Square::from_index(self.0.trailing_zeros() as i32)
    }

    #[must_use]
    pub const fn lsb_bb(&self) -> Self {
        Self(self.0 ^ (self.0 & (self.0 - 1)))
    }

    #[must_use]
    pub const fn msb(&self) -> Square<WIDTH, HEIGHT> {
        Square::from_index(63 - self.0.leading_zeros() as i32)
    }

    #[must_use]
    pub fn popped(&self) -> Self {
        *self ^ Self::from_square(self.lsb())
    }

    #[must_use]
    pub fn adjacent(&self) -> Self {
        Bitboard(
            (self.0 << WIDTH)
                | (self.0 >> WIDTH)
                | ((self.0 >> 1) | (self.0 >> (WIDTH + 1)) | (self.0 << (WIDTH - 1)))
                    & Self::all().west().0
                | ((self.0 << 1) | (self.0 << (WIDTH + 1)) | (self.0 >> (WIDTH - 1)))
                    & Self::all().east().0,
        )
    }

    #[must_use]
    pub fn dist2(&self) -> Self {
        Bitboard(
            // Left 2
            ((self.0 >> 2) | (self.0 >> (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH - 2)) | (self.0 << (WIDTH + WIDTH - 2))) & Self::all().west().west().0
            // Left 1
            | ((self.0 >> (WIDTH + WIDTH + 1)) | (self.0 << (WIDTH + WIDTH - 1))) & Self::all().west().0
            // Centre
            | (self.0 << (WIDTH + WIDTH)) | (self.0 >> (WIDTH + WIDTH))
            // Right 1
            | ((self.0 << (WIDTH + WIDTH + 1)) | (self.0 >> (WIDTH + WIDTH - 1))) & Self::all().east().0
            // Right 2
            | ((self.0 << 2) | (self.0 << (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH - 2)) | (self.0 >> (WIDTH - 2))) & Self::all().east().east().0,
        )
    }

    #[must_use]
    pub fn reach2(&self) -> Self {
        Bitboard(
            // Left 2
            ((self.0 >> 2) | (self.0 >> (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH - 2)) | (self.0 << (WIDTH + WIDTH - 2))) & Self::all().west().west().0
            // Left 1
            | ((self.0 >> 1) | (self.0 >> (WIDTH + 1)) | (self.0 >> (WIDTH + WIDTH + 1)) | (self.0 << (WIDTH - 1)) | (self.0 << (WIDTH + WIDTH - 1))) & Self::all().west().0
            // Centre
            | ((self.0 << WIDTH)| (self.0 << (WIDTH + WIDTH)) | (self.0 >> WIDTH) | (self.0 >> (WIDTH + WIDTH))) & Self::all().0
            // Right 1
            | ((self.0 << 1) | (self.0 << (WIDTH + 1)) | (self.0 << (WIDTH + WIDTH + 1)) | (self.0 >> (WIDTH - 1)) | (self.0 >> (WIDTH + WIDTH - 1))) & Self::all().east().0
            // Right 2
            | ((self.0 << 2) | (self.0 << (WIDTH + WIDTH + 2)) | (self.0 << (WIDTH + 2)) | (self.0 >> (WIDTH + WIDTH - 2)) | (self.0 >> (WIDTH - 2))) & Self::all().east().east().0,
        )
    }

    #[must_use]
    pub fn ray_north(&self, blockers: Self) -> Self {
        let mut mask = self.north();

        mask |= (mask & !blockers).north();
        mask |= (mask & !blockers).north();
        mask |= (mask & !blockers).north();
        mask |= (mask & !blockers).north();
        mask |= (mask & !blockers).north();
        mask |= (mask & !blockers).north();

        mask
    }

    #[must_use]
    pub fn ray_south(&self, blockers: Self) -> Self {
        let mut mask = self.south();

        mask |= (mask & !blockers).south();
        mask |= (mask & !blockers).south();
        mask |= (mask & !blockers).south();
        mask |= (mask & !blockers).south();
        mask |= (mask & !blockers).south();
        mask |= (mask & !blockers).south();

        mask
    }

    #[must_use]
    pub fn ray_east(&self, blockers: Self) -> Self {
        let mut mask = self.east();

        mask |= (mask & !blockers).east();
        mask |= (mask & !blockers).east();
        mask |= (mask & !blockers).east();
        mask |= (mask & !blockers).east();
        mask |= (mask & !blockers).east();
        mask |= (mask & !blockers).east();

        mask
    }

    #[must_use]
    pub fn ray_west(&self, blockers: Self) -> Self {
        let mut mask = self.west();

        mask |= (mask & !blockers).west();
        mask |= (mask & !blockers).west();
        mask |= (mask & !blockers).west();
        mask |= (mask & !blockers).west();
        mask |= (mask & !blockers).west();
        mask |= (mask & !blockers).west();

        mask
    }

    #[must_use]
    pub fn ray_ne(&self, blockers: Self) -> Self {
        let mut mask = self.ne();

        mask |= (mask & !blockers).ne();
        mask |= (mask & !blockers).ne();
        mask |= (mask & !blockers).ne();
        mask |= (mask & !blockers).ne();
        mask |= (mask & !blockers).ne();
        mask |= (mask & !blockers).ne();

        mask
    }

    #[must_use]
    pub fn ray_nw(&self, blockers: Self) -> Self {
        let mut mask = self.nw();

        mask |= (mask & !blockers).nw();
        mask |= (mask & !blockers).nw();
        mask |= (mask & !blockers).nw();
        mask |= (mask & !blockers).nw();
        mask |= (mask & !blockers).nw();
        mask |= (mask & !blockers).nw();

        mask
    }

    #[must_use]
    pub fn ray_se(&self, blockers: Self) -> Self {
        let mut mask = self.se();

        mask |= (mask & !blockers).se();
        mask |= (mask & !blockers).se();
        mask |= (mask & !blockers).se();
        mask |= (mask & !blockers).se();
        mask |= (mask & !blockers).se();
        mask |= (mask & !blockers).se();

        mask
    }

    #[must_use]
    pub fn ray_sw(&self, blockers: Self) -> Self {
        let mut mask = self.sw();

        mask |= (mask & !blockers).sw();
        mask |= (mask & !blockers).sw();
        mask |= (mask & !blockers).sw();
        mask |= (mask & !blockers).sw();
        mask |= (mask & !blockers).sw();
        mask |= (mask & !blockers).sw();

        mask
    }

    #[must_use]
    pub fn xray_north(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.north();
        ray |= (ray & !blockers).north();
        ray |= (ray & !blockers).north();
        ray |= (ray & !blockers).north();
        ray |= (ray & !blockers).north();
        ray |= (ray & !blockers).north();
        ray |= (ray & !blockers).north();

        let mut xray = (ray & blockers).north();
        xray |= (xray & !blockers).north();
        xray |= (xray & !blockers).north();
        xray |= (xray & !blockers).north();
        xray |= (xray & !blockers).north();
        xray |= (xray & !blockers).north();
        xray |= (xray & !blockers).north();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_south(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.south();
        ray |= (ray & !blockers).south();
        ray |= (ray & !blockers).south();
        ray |= (ray & !blockers).south();
        ray |= (ray & !blockers).south();
        ray |= (ray & !blockers).south();
        ray |= (ray & !blockers).south();

        let mut xray = (ray & blockers).south();
        xray |= (xray & !blockers).south();
        xray |= (xray & !blockers).south();
        xray |= (xray & !blockers).south();
        xray |= (xray & !blockers).south();
        xray |= (xray & !blockers).south();
        xray |= (xray & !blockers).south();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_east(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.east();
        ray |= (ray & !blockers).east();
        ray |= (ray & !blockers).east();
        ray |= (ray & !blockers).east();
        ray |= (ray & !blockers).east();
        ray |= (ray & !blockers).east();
        ray |= (ray & !blockers).east();

        let mut xray = (ray & blockers).east();
        xray |= (xray & !blockers).east();
        xray |= (xray & !blockers).east();
        xray |= (xray & !blockers).east();
        xray |= (xray & !blockers).east();
        xray |= (xray & !blockers).east();
        xray |= (xray & !blockers).east();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_west(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.west();
        ray |= (ray & !blockers).west();
        ray |= (ray & !blockers).west();
        ray |= (ray & !blockers).west();
        ray |= (ray & !blockers).west();
        ray |= (ray & !blockers).west();
        ray |= (ray & !blockers).west();

        let mut xray = (ray & blockers).west();
        xray |= (xray & !blockers).west();
        xray |= (xray & !blockers).west();
        xray |= (xray & !blockers).west();
        xray |= (xray & !blockers).west();
        xray |= (xray & !blockers).west();
        xray |= (xray & !blockers).west();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_ne(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.ne();
        ray |= (ray & !blockers).ne();
        ray |= (ray & !blockers).ne();
        ray |= (ray & !blockers).ne();
        ray |= (ray & !blockers).ne();
        ray |= (ray & !blockers).ne();
        ray |= (ray & !blockers).ne();

        let mut xray = (ray & blockers).ne();
        xray |= (xray & !blockers).ne();
        xray |= (xray & !blockers).ne();
        xray |= (xray & !blockers).ne();
        xray |= (xray & !blockers).ne();
        xray |= (xray & !blockers).ne();
        xray |= (xray & !blockers).ne();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_nw(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.nw();
        ray |= (ray & !blockers).nw();
        ray |= (ray & !blockers).nw();
        ray |= (ray & !blockers).nw();
        ray |= (ray & !blockers).nw();
        ray |= (ray & !blockers).nw();
        ray |= (ray & !blockers).nw();

        let mut xray = (ray & blockers).nw();
        xray |= (xray & !blockers).nw();
        xray |= (xray & !blockers).nw();
        xray |= (xray & !blockers).nw();
        xray |= (xray & !blockers).nw();
        xray |= (xray & !blockers).nw();
        xray |= (xray & !blockers).nw();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_se(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.se();
        ray |= (ray & !blockers).se();
        ray |= (ray & !blockers).se();
        ray |= (ray & !blockers).se();
        ray |= (ray & !blockers).se();
        ray |= (ray & !blockers).se();
        ray |= (ray & !blockers).se();

        let mut xray = (ray & blockers).se();
        xray |= (xray & !blockers).se();
        xray |= (xray & !blockers).se();
        xray |= (xray & !blockers).se();
        xray |= (xray & !blockers).se();
        xray |= (xray & !blockers).se();
        xray |= (xray & !blockers).se();

        (ray, xray)
    }

    #[must_use]
    pub fn xray_sw(&self, blockers: Self) -> (Self, Self) {
        let mut ray = self.sw();
        ray |= (ray & !blockers).sw();
        ray |= (ray & !blockers).sw();
        ray |= (ray & !blockers).sw();
        ray |= (ray & !blockers).sw();
        ray |= (ray & !blockers).sw();
        ray |= (ray & !blockers).sw();

        let mut xray = (ray & blockers).sw();
        xray |= (xray & !blockers).sw();
        xray |= (xray & !blockers).sw();
        xray |= (xray & !blockers).sw();
        xray |= (xray & !blockers).sw();
        xray |= (xray & !blockers).sw();
        xray |= (xray & !blockers).sw();

        (ray, xray)
    }

    #[must_use]
    pub fn rays_hor(&self, blockers: Self) -> Self {
        self.ray_east(blockers) | self.ray_west(blockers)
    }

    #[must_use]
    pub fn rays_ver(&self, blockers: Self) -> Self {
        self.ray_north(blockers) | self.ray_south(blockers)
    }

    #[must_use]
    pub fn rays_hor_ver(&self, blockers: Self) -> Self {
        self.rays_hor(blockers) | self.rays_ver(blockers)
    }

    #[must_use]
    pub fn rays_ver_hor(&self, blockers: Self) -> Self {
        self.rays_hor_ver(blockers)
    }

    #[must_use]
    pub fn rays_ne_sw(&self, blockers: Self) -> Self {
        self.ray_ne(blockers) | self.ray_sw(blockers)
    }

    #[must_use]
    pub fn rays_nw_se(&self, blockers: Self) -> Self {
        self.ray_nw(blockers) | self.ray_se(blockers)
    }

    #[must_use]
    pub fn rays_diagonal(&self, blockers: Self) -> Self {
        self.rays_ne_sw(blockers) | self.rays_nw_se(blockers)
    }

    #[must_use]
    pub fn rays_queen(&self, blockers: Self) -> Self {
        self.rays_diagonal(blockers) | self.rays_hor_ver(blockers)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitAnd for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Not for Bitboard<WIDTH, HEIGHT> {
    type Output = Self;

    fn not(self) -> Self {
        Self(!self.0) & Self::all()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitAndAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitand_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 &= rhs.0;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitOrAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitor_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 |= rhs.0;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitXorAssign for Bitboard<WIDTH, HEIGHT> {
    fn bitxor_assign(&mut self, rhs: Bitboard<WIDTH, HEIGHT>) {
        self.0 ^= rhs.0;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitXor for Bitboard<WIDTH, HEIGHT> {
    type Output = Bitboard<WIDTH, HEIGHT>;

    fn bitxor(self, rhs: Bitboard<WIDTH, HEIGHT>) -> Bitboard<WIDTH, HEIGHT> {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BitOr for Bitboard<WIDTH, HEIGHT> {
    type Output = Bitboard<WIDTH, HEIGHT>;

    fn bitor(self, rhs: Bitboard<WIDTH, HEIGHT>) -> Bitboard<WIDTH, HEIGHT> {
        Bitboard(self.0 | rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all() {
        assert_eq!(Bitboard::<8, 8>::all().0, 0xffffffffffffffff);
        assert_eq!(Bitboard::<7, 8>::all().0, 0xffffffffffffff);
        assert_eq!(Bitboard::<7, 7>::all().0, 0x1ffffffffffff);
        assert_eq!(Bitboard::<1, 5>::all().0, 0x1f);
        assert_eq!(Bitboard::<2, 2>::all().0, 0xf);
        assert_eq!(Bitboard::<1, 1>::all().0, 0x1);
    }

    #[test]
    fn test_north() {
        assert_eq!(Bitboard::<8, 8>(0xff).north().0, 0xff00);
        assert_eq!(Bitboard::<7, 7>(0x7f).north().0, 0x3f80);
    }

    #[test]
    fn test_south() {
        assert_eq!(Bitboard::<8, 8>(0xff00).south().0, 0xff);
        assert_eq!(Bitboard::<7, 7>(0x3f80).south().0, 0x7f);
    }

    #[test]
    fn test_east() {
        assert_eq!(Bitboard::<8, 8>(0x0).east().0, 0x0);
        assert_eq!(Bitboard::<8, 8>(0x8080808080808080).east().0, 0x0);
        assert_eq!(
            Bitboard::<8, 8>(0x4040404040404040).east().0,
            0x8080808080808080
        );
    }

    #[test]
    fn test_west() {
        assert_eq!(Bitboard::<8, 8>(0x0).west().0, 0x0);
    }

    #[test]
    fn test_from_file() {
        assert_eq!(Bitboard::<8, 8>::from_file(File(0)).0, 0x101010101010101);
        assert_eq!(Bitboard::<8, 8>::from_file(File(1)).0, 0x202020202020202);
        assert_eq!(Bitboard::<8, 8>::from_file(File(7)).0, 0x8080808080808080);
        assert_eq!(Bitboard::<7, 7>::from_file(File(0)).0, 0x40810204081);
        assert_eq!(Bitboard::<7, 7>::from_file(File(1)).0, 0x81020408102);
        assert_eq!(Bitboard::<8, 8>::from_file(File(4)).0, 0x1010101010101010);
    }

    #[test]
    fn test_from_rank() {
        assert_eq!(Bitboard::<8, 8>::from_rank(Rank(0)).0, 0xff);
        assert_eq!(Bitboard::<8, 8>::from_rank(Rank(1)).0, 0xff00);
        assert_eq!(Bitboard::<8, 8>::from_rank(Rank(7)).0, 0xff00000000000000);
        assert_eq!(Bitboard::<7, 7>::from_rank(Rank(0)).0, 0x7f);
        assert_eq!(Bitboard::<7, 7>::from_rank(Rank(1)).0, 0x3f80);
        assert_eq!(Bitboard::<8, 8>::from_rank(Rank(3)).0, 0xff000000);
    }

    #[test]
    fn test_left_edge() {
        assert_eq!(Bitboard::<64, 1>::get_left_edge().0, 0x1);
        assert_eq!(Bitboard::<32, 2>::get_left_edge().0, 0x100000001);
        assert_eq!(Bitboard::<16, 4>::get_left_edge().0, 0x1000100010001);
        assert_eq!(Bitboard::<8, 8>::get_left_edge().0, 0x101010101010101);
        assert_eq!(Bitboard::<4, 16>::get_left_edge().0, 0x1111111111111111);
        assert_eq!(Bitboard::<2, 32>::get_left_edge().0, 0x5555555555555555);
        assert_eq!(Bitboard::<1, 64>::get_left_edge().0, 0xffffffffffffffff);
        assert_eq!(Bitboard::<3, 6>::get_left_edge().0, 0x9249);
    }

    #[test]
    fn test_right_edge() {
        assert_eq!(Bitboard::<64, 1>::get_right_edge().0, 0x8000000000000000);
        assert_eq!(Bitboard::<32, 2>::get_right_edge().0, 0x8000000080000000);
        assert_eq!(Bitboard::<16, 4>::get_right_edge().0, 0x8000800080008000);
        assert_eq!(Bitboard::<8, 8>::get_right_edge().0, 0x8080808080808080);
        assert_eq!(Bitboard::<4, 16>::get_right_edge().0, 0x8888888888888888);
        assert_eq!(Bitboard::<2, 32>::get_right_edge().0, 0xaaaaaaaaaaaaaaaa);
        assert_eq!(Bitboard::<1, 64>::get_right_edge().0, 0xffffffffffffffff);
        assert_eq!(Bitboard::<3, 6>::get_right_edge().0, 0x24924);
    }

    #[test]
    fn test_bottom_edge() {
        assert_eq!(Bitboard::<64, 1>::get_bottom_edge().0, 0xffffffffffffffff);
        assert_eq!(Bitboard::<32, 2>::get_bottom_edge().0, 0xffffffff);
        assert_eq!(Bitboard::<16, 4>::get_bottom_edge().0, 0xffff);
        assert_eq!(Bitboard::<8, 8>::get_bottom_edge().0, 0xff);
        assert_eq!(Bitboard::<4, 16>::get_bottom_edge().0, 0xf);
        assert_eq!(Bitboard::<2, 32>::get_bottom_edge().0, 0x3);
        assert_eq!(Bitboard::<1, 64>::get_bottom_edge().0, 0x1);
        assert_eq!(Bitboard::<3, 6>::get_bottom_edge().0, 0x7);
    }

    #[test]
    fn test_top_edge() {
        assert_eq!(Bitboard::<64, 1>::get_top_edge().0, 0xffffffffffffffff);
        assert_eq!(Bitboard::<32, 2>::get_top_edge().0, 0xffffffff00000000);
        assert_eq!(Bitboard::<16, 4>::get_top_edge().0, 0xffff000000000000);
        assert_eq!(Bitboard::<8, 8>::get_top_edge().0, 0xff00000000000000);
        assert_eq!(Bitboard::<4, 16>::get_top_edge().0, 0xf000000000000000);
        assert_eq!(Bitboard::<2, 32>::get_top_edge().0, 0xc000000000000000);
        assert_eq!(Bitboard::<1, 64>::get_top_edge().0, 0x8000000000000000);
        assert_eq!(Bitboard::<3, 6>::get_top_edge().0, 0x38000);
    }

    #[test]
    fn test_border() {
        assert_eq!(Bitboard::<8, 8>::get_border().0, 0xff818181818181ff);
        assert_eq!(Bitboard::<4, 4>::get_border().0, 0xf99f);
        assert_eq!(Bitboard::<4, 8>::get_border().0, 0xf999999f);
        assert_eq!(Bitboard::<8, 4>::get_border().0, 0xff8181ff);
        assert_eq!(Bitboard::<2, 4>::get_border().0, 0xff);
        assert_eq!(Bitboard::<4, 2>::get_border().0, 0xff);
        assert_eq!(Bitboard::<3, 3>::get_border().0, 0x1ef);
    }

    #[test]
    fn test_swap() {
        assert_eq!(Bitboard::<8, 8>(0x0).swap(), Bitboard::<8, 8>(0x0));
        assert_eq!(
            Bitboard::<8, 8>(0xAA).swap(),
            Bitboard::<8, 8>(0xAA00000000000000)
        );
        assert_eq!(
            Bitboard::<8, 8>(0xAA00000000000000).swap(),
            Bitboard::<8, 8>(0xAA)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x102040810204080).swap(),
            Bitboard::<8, 8>(0x8040201008040201)
        );
    }

    #[test]
    fn test_adjacent() {
        assert_eq!(Bitboard::<8, 8>(0x0).adjacent().0, 0x0);
        assert_eq!(Bitboard::<8, 8>(0x1).adjacent().0, 0x302);
        assert_eq!(
            Bitboard::<8, 8>(0x2442810000814224).adjacent().0,
            0xffffe7c3c3e7ffff
        );
        assert_eq!(
            Bitboard::<8, 8>(0x2400810000810024).adjacent().0,
            0x5aff42c3c342ff5a
        );
        assert_eq!(Bitboard::<7, 7>(0x0).adjacent().0, 0x0);
        assert_eq!(Bitboard::<7, 7>(0x1).adjacent().0, 0x182);
        assert_eq!(Bitboard::<2, 2>(0x0).adjacent().0, 0x0);
        assert_eq!(Bitboard::<2, 2>(0x1).adjacent().0, 0xe);
    }

    #[test]
    fn test_dist2() {
        assert_eq!(Bitboard::<8, 8>(0x0).dist2().0, 0x0);
        assert_eq!(Bitboard::<8, 8>(0x1).dist2().0, 0x70404);
        assert_eq!(Bitboard::<8, 8>(0x10000000).dist2().0, 0x7c4444447c00);
    }

    #[test]
    fn test_ray_north() {
        let tests = [
            (0, Bitboard(0x1), 0x101010101010100),
            (28, Bitboard(0x0), 0x1010101000000000),
            (28, Bitboard(0x10000000), 0x1010101000000000),
            (28, Bitboard(0x100010000000), 0x101000000000),
        ];
        for (idx, blockers, ray) in tests {
            let bb = Bitboard::<8, 8>::from_index(idx);
            assert_eq!(bb.ray_north(blockers).0, ray);
        }
    }

    #[test]
    fn test_xray_north() {
        let tests = [
            (0, Bitboard(0x1), 0x101010101010100, 0x0),
            (0, Bitboard(0x101), 0x100, 0x101010101010000),
            (28, Bitboard(0x0), 0x1010101000000000, 0x0),
            (28, Bitboard(0x10000000), 0x1010101000000000, 0x0),
            (
                28,
                Bitboard(0x100010000000),
                0x101000000000,
                0x1010000000000000,
            ),
            (
                28,
                Bitboard(0x10100010000000),
                0x101000000000,
                0x10000000000000,
            ),
        ];
        for (idx, blockers, ray, xray) in tests {
            let bb = Bitboard::<8, 8>::from_index(idx);
            let (got_ray, got_xray) = bb.xray_north(blockers);
            assert_eq!(got_ray.0, ray);
            assert_eq!(got_xray.0, xray);
        }
    }

    #[test]
    fn test_reach2() {
        assert_eq!(Bitboard::<8, 8>(0x0).reach2().0, 0x0);
        assert_eq!(Bitboard::<8, 8>(0x1).reach2().0, 0x70706);
        assert_eq!(Bitboard::<8, 8>(0x10000000).reach2().0, 0x7c7c6c7c7c00);
        assert_eq!(
            Bitboard::<8, 8>(0x8000000000000000).reach2().0,
            0x60e0e00000000000
        );
        assert_eq!(
            Bitboard::<8, 8>(0x100000000000000).reach2().0,
            0x607070000000000
        );
        assert_eq!(Bitboard::<8, 8>(0x80).reach2().0, 0xe0e060);
        assert_eq!(Bitboard::<7, 7>::all().reach2().0, 0x1ffffffffffff);
    }

    #[test]
    fn test_count() {
        assert_eq!(Bitboard::<8, 8>(0x0).count(), 0);
        assert_eq!(Bitboard::<8, 8>(0x1).count(), 1);
        assert_eq!(Bitboard::<8, 8>(0x2).count(), 1);
        assert_eq!(Bitboard::<8, 8>(0x3).count(), 2);
        assert_eq!(Bitboard::<8, 8>::all().count(), 64);
        assert_eq!(Bitboard::<7, 8>::all().count(), 56);
        assert_eq!(Bitboard::<7, 7>::all().count(), 49);
        assert_eq!(Bitboard::<1, 5>::all().count(), 5);
        assert_eq!(Bitboard::<2, 2>::all().count(), 4);
        assert_eq!(Bitboard::<1, 1>::all().count(), 1);
    }

    #[test]
    fn test_lsb() {
        assert_eq!(
            Bitboard::<8, 8>(0x1).lsb(),
            Square::<8, 8>::from_coords(0, 0)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x3c0000).lsb(),
            Square::<8, 8>::from_coords(2, 2)
        );
        assert_eq!(
            Bitboard::<8, 8>(0x8000000000000000).lsb(),
            Square::<8, 8>::from_coords(7, 7)
        );
    }

    #[test]
    fn test_lsb_bb() {
        assert_eq!(Bitboard::<8, 8>(0x1).lsb_bb().0, 0x1);
        assert_eq!(Bitboard::<8, 8>(0x3c0000).lsb_bb().0, 0x40000);
        assert_eq!(
            Bitboard::<8, 8>(0x8000000000000000).lsb_bb().0,
            0x8000000000000000
        );
        assert_eq!(Bitboard::<4, 4>(0x6800).lsb_bb().0, 0x800);
    }
}
