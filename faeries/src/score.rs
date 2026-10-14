//! A midgame and an endgame value that are tuned together.

use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use tunable::Parts;

/// A value with a midgame part and an endgame part
///
/// A tunable `Score` is two tunables, the midgame one first: element `i` of a `[Score; N]`
/// named `pst` is listed as `pst_<2i>` (midgame) and `pst_<2i+1>` (endgame).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Score {
    pub mg: i32,
    pub eg: i32,
}

impl Score {
    #[must_use]
    pub const fn new(mg: i32, eg: i32) -> Self {
        Self { mg, eg }
    }

    /// The value `phase` of the way from the midgame (0) to the endgame (`max_phase`)
    #[must_use]
    pub const fn taper(self, phase: i32, max_phase: i32) -> i32 {
        (self.mg * (max_phase - phase) + self.eg * phase) / max_phase
    }
}

/// Shorthand for [`Score::new`]
#[must_use]
#[allow(non_snake_case)]
pub const fn S(mg: i32, eg: i32) -> Score {
    Score::new(mg, eg)
}

impl Add for Score {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.mg + rhs.mg, self.eg + rhs.eg)
    }
}

impl Sub for Score {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.mg - rhs.mg, self.eg - rhs.eg)
    }
}

impl Neg for Score {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.mg, -self.eg)
    }
}

impl Mul<i32> for Score {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        Self::new(self.mg * rhs, self.eg * rhs)
    }
}

impl AddAssign for Score {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Score {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Parts<2> for Score {
    fn from_parts([mg, eg]: [i32; 2]) -> Self {
        Self::new(mg, eg)
    }

    fn to_parts(self) -> [i32; 2] {
        [self.mg, self.eg]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scores_do_arithmetic() {
        let mut total = S(1, 2) + S(10, 20) * 3 - S(1, 1);
        total += -S(0, 5);
        assert_eq!(total, S(30, 56));
    }

    #[test]
    fn taper_runs_from_midgame_to_endgame() {
        assert_eq!(S(100, 0).taper(0, 4), 100);
        assert_eq!(S(100, 0).taper(1, 4), 75);
        assert_eq!(S(100, 40).taper(4, 4), 40);
    }
}
