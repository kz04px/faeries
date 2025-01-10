use super::{file::File, rank::Rank};
use std::{fmt, ops::Index, u8};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Hex<const WIDTH: usize, const HEIGHT: usize>(pub File<WIDTH>, pub Rank<HEIGHT>);

impl<const WIDTH: usize, const HEIGHT: usize> Hex<WIDTH, HEIGHT> {
    #[must_use]
    pub const fn get_file(&self) -> File<WIDTH> {
        self.0
    }

    #[must_use]
    pub const fn get_rank(&self) -> Rank<HEIGHT> {
        self.1
    }

    #[must_use]
    pub const fn get_index(&self) -> usize {
        let chunk_size = 2 * WIDTH as i32 + 1;
        let chunk = self.get_rank().get_index() as i32 / 2;
        let is_wide_rank = self.get_rank().get_index() % 2 == 1;
        (chunk * chunk_size
            + (is_wide_rank as i32 * WIDTH as i32)
            + self.get_file().get_index() as i32) as usize
    }

    #[must_use]
    pub const fn from_coords(x: i32, y: i32) -> Self {
        Self(File(x as u8), Rank(y as u8))
    }

    #[must_use]
    pub const fn from_index(idx: i32) -> Self {
        let chunk_size = 2 * WIDTH as i32 + 1;
        let chunk = idx / chunk_size;
        let chunk_idx = idx % chunk_size;
        let is_wide_row = chunk_idx >= WIDTH as i32;
        let r = 2 * chunk + is_wide_row as i32;
        let f = if is_wide_row {
            chunk_idx - WIDTH as i32
        } else {
            chunk_idx
        };
        Self(File(f as u8), Rank(r as u8))
    }

    #[must_use]
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if word.len() < 2 {
            Err("movestr too short")
        } else {
            if let (Ok(f), Ok(r)) = (
                File::from_string(&word[0..1]),
                Rank::from_string(&word[1..]),
            ) {
                Ok(Hex(f, r))
            } else {
                Err("Uh oh")
            }
        }
    }

    #[must_use]
    pub fn to_string_inverted(&self) -> String {
        format!(
            "{}{}",
            (b'a' + self.get_rank().get_index() as u8) as char,
            (b'1' + self.get_file().get_index() as u8) as char,
        )
    }
}

impl<Any, const WIDTH: usize, const HEIGHT: usize, const T: usize> Index<Hex<WIDTH, HEIGHT>>
    for [Any; T]
{
    type Output = Any;

    fn index(&self, sq: Hex<WIDTH, HEIGHT>) -> &Self::Output {
        &self[sq.get_index()]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Hex<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.get_file(), self.get_rank())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file() {
        assert_eq!(Hex::<4, 5>::from_index(0).get_file(), File(0));
        assert_eq!(Hex::<4, 5>::from_index(1).get_file(), File(1));
        assert_eq!(Hex::<4, 5>::from_index(2).get_file(), File(2));
        assert_eq!(Hex::<4, 5>::from_index(3).get_file(), File(3));
        assert_eq!(Hex::<4, 5>::from_index(4).get_file(), File(0));
        assert_eq!(Hex::<4, 5>::from_index(5).get_file(), File(1));
        assert_eq!(Hex::<4, 5>::from_index(6).get_file(), File(2));
        assert_eq!(Hex::<4, 5>::from_index(7).get_file(), File(3));
        assert_eq!(Hex::<4, 5>::from_index(8).get_file(), File(4));
        assert_eq!(Hex::<4, 5>::from_index(9).get_file(), File(0));
        assert_eq!(Hex::<4, 5>::from_index(10).get_file(), File(1));
        assert_eq!(Hex::<4, 5>::from_index(11).get_file(), File(2));
        assert_eq!(Hex::<4, 5>::from_index(12).get_file(), File(3));
        assert_eq!(Hex::<4, 5>::from_index(13).get_file(), File(0));
        assert_eq!(Hex::<4, 5>::from_index(14).get_file(), File(1));
        assert_eq!(Hex::<4, 5>::from_index(15).get_file(), File(2));
        assert_eq!(Hex::<4, 5>::from_index(16).get_file(), File(3));
        assert_eq!(Hex::<4, 5>::from_index(17).get_file(), File(4));
        assert_eq!(Hex::<4, 5>::from_index(18).get_file(), File(0));
        assert_eq!(Hex::<4, 5>::from_index(19).get_file(), File(1));
        assert_eq!(Hex::<4, 5>::from_index(20).get_file(), File(2));
        assert_eq!(Hex::<4, 5>::from_index(21).get_file(), File(3));
    }

    #[test]
    fn get_rank() {
        assert_eq!(Hex::<4, 5>::from_index(0).get_rank(), Rank(0));
        assert_eq!(Hex::<4, 5>::from_index(1).get_rank(), Rank(0));
        assert_eq!(Hex::<4, 5>::from_index(2).get_rank(), Rank(0));
        assert_eq!(Hex::<4, 5>::from_index(3).get_rank(), Rank(0));

        assert_eq!(Hex::<4, 5>::from_index(4).get_rank(), Rank(1));
        assert_eq!(Hex::<4, 5>::from_index(5).get_rank(), Rank(1));
        assert_eq!(Hex::<4, 5>::from_index(6).get_rank(), Rank(1));
        assert_eq!(Hex::<4, 5>::from_index(7).get_rank(), Rank(1));
        assert_eq!(Hex::<4, 5>::from_index(8).get_rank(), Rank(1));

        assert_eq!(Hex::<4, 5>::from_index(9).get_rank(), Rank(2));
        assert_eq!(Hex::<4, 5>::from_index(10).get_rank(), Rank(2));
        assert_eq!(Hex::<4, 5>::from_index(11).get_rank(), Rank(2));
        assert_eq!(Hex::<4, 5>::from_index(12).get_rank(), Rank(2));

        assert_eq!(Hex::<4, 5>::from_index(13).get_rank(), Rank(3));
        assert_eq!(Hex::<4, 5>::from_index(14).get_rank(), Rank(3));
        assert_eq!(Hex::<4, 5>::from_index(15).get_rank(), Rank(3));
        assert_eq!(Hex::<4, 5>::from_index(16).get_rank(), Rank(3));
        assert_eq!(Hex::<4, 5>::from_index(17).get_rank(), Rank(3));

        assert_eq!(Hex::<4, 5>::from_index(18).get_rank(), Rank(4));
        assert_eq!(Hex::<4, 5>::from_index(19).get_rank(), Rank(4));
        assert_eq!(Hex::<4, 5>::from_index(20).get_rank(), Rank(4));
        assert_eq!(Hex::<4, 5>::from_index(21).get_rank(), Rank(4));
    }

    #[test]
    fn indexing() {
        assert_eq!(Hex::<4, 5>::from_coords(0, 0).get_index(), 0);
        assert_eq!(Hex::<4, 5>::from_coords(1, 0).get_index(), 1);
        assert_eq!(Hex::<4, 5>::from_coords(2, 0).get_index(), 2);
        assert_eq!(Hex::<4, 5>::from_coords(3, 0).get_index(), 3);

        assert_eq!(Hex::<4, 5>::from_coords(0, 1).get_index(), 4);
        assert_eq!(Hex::<4, 5>::from_coords(1, 1).get_index(), 5);
        assert_eq!(Hex::<4, 5>::from_coords(2, 1).get_index(), 6);
        assert_eq!(Hex::<4, 5>::from_coords(3, 1).get_index(), 7);
        assert_eq!(Hex::<4, 5>::from_coords(4, 1).get_index(), 8);

        assert_eq!(Hex::<4, 5>::from_coords(0, 2).get_index(), 9);
        assert_eq!(Hex::<4, 5>::from_coords(1, 2).get_index(), 10);
        assert_eq!(Hex::<4, 5>::from_coords(2, 2).get_index(), 11);
        assert_eq!(Hex::<4, 5>::from_coords(3, 2).get_index(), 12);

        assert_eq!(Hex::<4, 5>::from_coords(0, 3).get_index(), 13);
        assert_eq!(Hex::<4, 5>::from_coords(1, 3).get_index(), 14);
        assert_eq!(Hex::<4, 5>::from_coords(2, 3).get_index(), 15);
        assert_eq!(Hex::<4, 5>::from_coords(3, 3).get_index(), 16);
        assert_eq!(Hex::<4, 5>::from_coords(4, 3).get_index(), 17);
    }

    #[test]
    fn failure_from_string() {
        let tests = ["", "a", "1", "aa", "a11", "a1a1", "a0", "a9", "i1", "i9"];
        for test in tests {
            assert!(Hex::<8, 8>::from_string(test).is_err());
        }
    }

    #[test]
    fn from_string() {
        let tests = [
            ("a1", Hex::<26, 26>::from_coords(0, 0)),
            ("b1", Hex::<26, 26>::from_coords(1, 0)),
            ("c1", Hex::<26, 26>::from_coords(2, 0)),
            ("d1", Hex::<26, 26>::from_coords(3, 0)),
            ("a2", Hex::<26, 26>::from_coords(0, 1)),
            ("b2", Hex::<26, 26>::from_coords(1, 1)),
            ("c2", Hex::<26, 26>::from_coords(2, 1)),
            ("d2", Hex::<26, 26>::from_coords(3, 1)),
            ("h8", Hex::<26, 26>::from_coords(7, 7)),
            ("i9", Hex::<26, 26>::from_coords(8, 8)),
            ("a10", Hex::<26, 26>::from_coords(0, 9)),
            ("a20", Hex::<26, 26>::from_coords(0, 19)),
            ("j1", Hex::<26, 26>::from_coords(9, 0)),
            ("t1", Hex::<26, 26>::from_coords(19, 0)),
            ("z26", Hex::<26, 26>::from_coords(25, 25)),
        ];

        for (movestr, sq) in tests {
            match Hex::<26, 26>::from_string(movestr) {
                Ok(got) => {
                    assert_eq!(got, sq);
                    assert_eq!(movestr, got.to_string());
                }
                Err(e) => panic!("Failed {}", e),
            }
        }
    }

    #[test]
    fn to_string() {
        assert_eq!(Hex::<4, 5>::from_coords(0, 0).to_string(), "a1");
        assert_eq!(Hex::<4, 5>::from_coords(1, 0).to_string(), "b1");
        assert_eq!(Hex::<4, 5>::from_coords(2, 0).to_string(), "c1");
        assert_eq!(Hex::<4, 5>::from_coords(3, 0).to_string(), "d1");
        assert_eq!(Hex::<4, 5>::from_coords(0, 1).to_string(), "a2");
        assert_eq!(Hex::<4, 5>::from_coords(1, 1).to_string(), "b2");
        assert_eq!(Hex::<4, 5>::from_coords(2, 1).to_string(), "c2");
        assert_eq!(Hex::<4, 5>::from_coords(3, 1).to_string(), "d2");
        assert_eq!(Hex::<4, 5>::from_coords(4, 1).to_string(), "e2");
        assert_eq!(Hex::<4, 5>::from_coords(0, 2).to_string(), "a3");

        assert_eq!(Hex::<4, 5>::from_coords(0, 0).to_string_inverted(), "a1");
        assert_eq!(Hex::<4, 5>::from_coords(1, 0).to_string_inverted(), "a2");
        assert_eq!(Hex::<4, 5>::from_coords(2, 0).to_string_inverted(), "a3");
        assert_eq!(Hex::<4, 5>::from_coords(3, 0).to_string_inverted(), "a4");
        assert_eq!(Hex::<4, 5>::from_coords(0, 1).to_string_inverted(), "b1");
        assert_eq!(Hex::<4, 5>::from_coords(1, 1).to_string_inverted(), "b2");
        assert_eq!(Hex::<4, 5>::from_coords(2, 1).to_string_inverted(), "b3");
        assert_eq!(Hex::<4, 5>::from_coords(3, 1).to_string_inverted(), "b4");
        assert_eq!(Hex::<4, 5>::from_coords(4, 1).to_string_inverted(), "b5");
        assert_eq!(Hex::<4, 5>::from_coords(0, 2).to_string_inverted(), "c1");
    }
}
