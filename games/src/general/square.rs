use std::fmt;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Square<const WIDTH: u8, const HEIGHT: u8>(pub u8);

impl<const WIDTH: u8, const HEIGHT: u8> Square<WIDTH, HEIGHT> {
    #[must_use]
    pub fn get_file(&self) -> u8 {
        self.0 % WIDTH
    }

    #[must_use]
    pub fn get_rank(&self) -> u8 {
        self.0 / WIDTH
    }

    #[must_use]
    pub fn from_coords(x: i32, y: i32) -> Self {
        Self((y * WIDTH as i32 + x) as u8)
    }

    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if word.len() != 2 {
            Err("Uh oh")
        } else {
            let c1 = word.chars().nth(0).unwrap();
            let c2 = word.chars().nth(1).unwrap();

            if c1 < 'a' || c1 >= (b'a' + WIDTH) as char {
                return Err("Uh oh");
            }

            if c2 < '1' || c2 >= (b'1' + WIDTH) as char {
                return Err("Uh oh");
            }

            let file = c1 as u8 - b'a';
            let rank = c2 as u8 - b'1';
            let idx = WIDTH * rank + file;

            if idx >= WIDTH * HEIGHT {
                Err("Uh oh")
            } else {
                Ok(Square(idx))
            }
        }
    }
}

impl<const WIDTH: u8, const HEIGHT: u8> fmt::Display for Square<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (b'a' + self.get_file()) as char,
            (b'1' + self.get_rank()) as char
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failure_from_string() {
        let tests = ["", "a", "1", "a11", "a1a1", "a0", "a9", "i1", "i9"];
        for test in tests {
            println!("{}", test);
            assert!(Square::<8, 8>::from_string(test).is_err());
        }
    }

    #[test]
    fn from_string() {
        let tests = [
            ("a1", Square::<8, 8>::from_coords(0, 0)),
            ("b1", Square::<8, 8>::from_coords(1, 0)),
            ("c1", Square::<8, 8>::from_coords(2, 0)),
            ("d1", Square::<8, 8>::from_coords(3, 0)),
            ("a2", Square::<8, 8>::from_coords(0, 1)),
            ("b2", Square::<8, 8>::from_coords(1, 1)),
            ("c2", Square::<8, 8>::from_coords(2, 1)),
            ("d2", Square::<8, 8>::from_coords(3, 1)),
            ("h8", Square::<8, 8>::from_coords(7, 7)),
        ];

        for (movestr, sq) in tests {
            if let Ok(got) = Square::<8, 8>::from_string(movestr) {
                assert_eq!(got, sq);
                assert_eq!(movestr, got.to_string());
            } else {
                panic!("Failed");
            }
        }
    }
}
