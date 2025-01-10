#[derive(Clone, Copy, PartialEq, Debug)]
pub struct File<const WIDTH: usize>(pub u8);

impl<const WIDTH: usize> File<WIDTH> {
    #[must_use]
    pub fn from_char(c: char) -> Result<Self, &'static str> {
        if c < 'a' || c > (b'a' + WIDTH as u8 - 1) as char {
            Err("char out of range")
        } else {
            Ok(Self(c as u8 - b'a'))
        }
    }

    #[must_use]
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if let Some(c) = word.chars().nth(0) {
            Self::from_char(c)
        } else {
            Err("Wrong string length")
        }
    }

    #[must_use]
    pub const fn get_index(&self) -> usize {
        self.0 as usize
    }
}

impl<Any, const WIDTH: usize> std::ops::Index<File<WIDTH>> for [Any; WIDTH] {
    type Output = Any;

    fn index(&self, f: File<WIDTH>) -> &Self::Output {
        &self[f.get_index()]
    }
}

impl<Any, const WIDTH: usize> std::ops::IndexMut<File<WIDTH>> for [Any; WIDTH] {
    fn index_mut(&mut self, f: File<WIDTH>) -> &mut Self::Output {
        &mut self[f.get_index()]
    }
}

impl<const WIDTH: usize> std::fmt::Display for File<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (b'a' + self.0) as char)
    }
}

#[cfg(test)]
mod test {
    use super::File;

    #[test]
    fn indexing() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(array[File::<8>(0)], 1);
        assert_eq!(array[File::<8>(1)], 2);
        assert_eq!(array[File::<8>(2)], 3);
        assert_eq!(array[File::<8>(3)], 4);
        assert_eq!(array[File::<8>(4)], 5);
        assert_eq!(array[File::<8>(5)], 6);
        assert_eq!(array[File::<8>(6)], 7);
        assert_eq!(array[File::<8>(7)], 8);
    }

    #[test]
    fn from_char() {
        assert!(File::<8>::from_char('a').is_ok());
        assert!(File::<8>::from_char('b').is_ok());
        assert!(File::<8>::from_char('h').is_ok());
        assert!(File::<8>::from_char('i').is_err());
        assert_eq!(File::<8>::from_char('a'), Ok(File(0)));
        assert_eq!(File::<8>::from_char('b'), Ok(File(1)));
        assert_eq!(File::<8>::from_char('h'), Ok(File(7)));
        assert_eq!(File::<8>::from_char('i'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('`'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('#'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('0'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('9'), Err("char out of range"));
        assert_eq!(File::<8>::from_char('!'), Err("char out of range"));
    }

    #[test]
    fn to_string() {
        for c in 'a'..='h' {
            if let Ok(f) = File::<8>::from_char(c) {
                assert_eq!(f.to_string(), c.to_string());
            } else {
                panic!("Uh oh")
            }
        }
        for c in 'i'..='z' {
            assert!(File::<8>::from_char(c).is_err());
        }
    }

    #[test]
    fn index() {
        assert_eq!(File::<8>::from_string("a").unwrap().get_index(), 0);
        assert_eq!(File::<8>::from_string("b").unwrap().get_index(), 1);
        assert_eq!(File::<8>::from_string("c").unwrap().get_index(), 2);
    }
}
