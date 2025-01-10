use std::{fmt, ops::Index, u8};

use super::{file::File, rank::Rank};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Square<const WIDTH: usize, const HEIGHT: usize>(pub File<WIDTH>, pub Rank<HEIGHT>);

impl<const WIDTH: usize, const HEIGHT: usize> Square<WIDTH, HEIGHT> {
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
        self.1.get_index() * WIDTH + self.0.get_index()
    }

    #[must_use]
    pub const fn from_coords(x: i32, y: i32) -> Self {
        Self(File(x as u8), Rank(y as u8))
    }

    #[must_use]
    pub const fn from_index(idx: i32) -> Self {
        Self(
            File((idx as usize % WIDTH) as u8),
            Rank((idx as usize / WIDTH) as u8),
        )
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
                Ok(Square(f, r))
            } else {
                Err("Uh oh")
            }
        }
    }
}

impl<Any, const WIDTH: usize, const HEIGHT: usize, const T: usize> Index<Square<WIDTH, HEIGHT>>
    for [Any; T]
{
    type Output = Any;

    fn index(&self, sq: Square<WIDTH, HEIGHT>) -> &Self::Output {
        &self[sq.get_index()]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> fmt::Display for Square<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.get_file(), self.get_rank())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexing() {
        assert_eq!(Square::<7, 7>::from_coords(0, 0).get_index(), 0);
        assert_eq!(Square::<7, 7>::from_coords(1, 0).get_index(), 1);
        assert_eq!(Square::<7, 7>::from_coords(2, 0).get_index(), 2);
        assert_eq!(Square::<7, 7>::from_coords(3, 0).get_index(), 3);
        assert_eq!(Square::<7, 7>::from_coords(0, 1).get_index(), 7);
        assert_eq!(Square::<7, 7>::from_coords(6, 6).get_index(), 48);
    }

    #[test]
    fn failure_from_string() {
        let tests = ["", "a", "1", "aa", "a11", "a1a1", "a0", "a9", "i1", "i9"];
        for test in tests {
            assert!(Square::<8, 8>::from_string(test).is_err());
        }
    }

    #[test]
    fn from_string() {
        let tests = [
            ("a1", Square::<26, 26>::from_coords(0, 0)),
            ("b1", Square::<26, 26>::from_coords(1, 0)),
            ("c1", Square::<26, 26>::from_coords(2, 0)),
            ("d1", Square::<26, 26>::from_coords(3, 0)),
            ("a2", Square::<26, 26>::from_coords(0, 1)),
            ("b2", Square::<26, 26>::from_coords(1, 1)),
            ("c2", Square::<26, 26>::from_coords(2, 1)),
            ("d2", Square::<26, 26>::from_coords(3, 1)),
            ("h8", Square::<26, 26>::from_coords(7, 7)),
            ("i9", Square::<26, 26>::from_coords(8, 8)),
            ("a10", Square::<26, 26>::from_coords(0, 9)),
            ("a20", Square::<26, 26>::from_coords(0, 19)),
            ("j1", Square::<26, 26>::from_coords(9, 0)),
            ("t1", Square::<26, 26>::from_coords(19, 0)),
            ("z26", Square::<26, 26>::from_coords(25, 25)),
        ];

        for (movestr, sq) in tests {
            match Square::<26, 26>::from_string(movestr) {
                Ok(got) => {
                    assert_eq!(got, sq);
                    assert_eq!(movestr, got.to_string());
                }
                Err(e) => panic!("Failed {}", e),
            }
        }
    }
}
