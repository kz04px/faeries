#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rank<const HEIGHT: usize>(pub u8);

impl<const HEIGHT: usize> Rank<HEIGHT> {
    #[must_use]
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        if let Ok(n) = word.parse::<u8>() {
            if n < 1 || n > HEIGHT as u8 {
                Err("char out of range")
            } else {
                Ok(Self(n - 1))
            }
        } else {
            Err("Uh oh")
        }
    }

    #[must_use]
    pub const fn get_index(&self) -> usize {
        self.0 as usize
    }

    #[must_use]
    pub const fn flipped(&self) -> Self {
        Self(HEIGHT as u8 - self.0 - 1)
    }
}

impl<Any, const HEIGHT: usize> std::ops::Index<Rank<HEIGHT>> for [Any; HEIGHT] {
    type Output = Any;

    fn index(&self, f: Rank<HEIGHT>) -> &Self::Output {
        &self[f.get_index()]
    }
}

impl<Any, const HEIGHT: usize> std::ops::IndexMut<Rank<HEIGHT>> for [Any; HEIGHT] {
    fn index_mut(&mut self, f: Rank<HEIGHT>) -> &mut Self::Output {
        &mut self[f.get_index()]
    }
}

impl<const HEIGHT: usize> std::fmt::Display for Rank<HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0 + 1)
    }
}

#[cfg(test)]
mod test {
    use super::Rank;

    #[test]
    fn indexing() {
        let array = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(array[Rank::<8>(0)], 1);
        assert_eq!(array[Rank::<8>(1)], 2);
        assert_eq!(array[Rank::<8>(2)], 3);
        assert_eq!(array[Rank::<8>(3)], 4);
        assert_eq!(array[Rank::<8>(4)], 5);
        assert_eq!(array[Rank::<8>(5)], 6);
        assert_eq!(array[Rank::<8>(6)], 7);
        assert_eq!(array[Rank::<8>(7)], 8);
    }

    #[test]
    fn from_string() {
        assert!(Rank::<8>::from_string("1").is_ok());
        assert!(Rank::<8>::from_string("2").is_ok());
        assert!(Rank::<8>::from_string("8").is_ok());
        assert_eq!(Rank::<8>::from_string("1"), Ok(Rank(0)));
        assert_eq!(Rank::<8>::from_string("2"), Ok(Rank(1)));
        assert_eq!(Rank::<8>::from_string("3"), Ok(Rank(2)));
    }

    #[test]
    fn to_string() {
        assert_eq!(Rank::<8>::from_string("1").unwrap().to_string(), "1");
        assert_eq!(Rank::<8>::from_string("4").unwrap().to_string(), "4");
        assert!(Rank::<8>::from_string("11").is_err());
    }

    #[test]
    fn index() {
        assert_eq!(Rank::<8>::from_string("1").unwrap().get_index(), 0);
        assert_eq!(Rank::<8>::from_string("2").unwrap().get_index(), 1);
        assert_eq!(Rank::<8>::from_string("3").unwrap().get_index(), 2);
        assert!(Rank::<8>::from_string("123").is_err());
    }

    #[test]
    fn flip() {
        assert_eq!(Rank::<8>(0).flipped(), Rank::<8>(7));
        assert_eq!(Rank::<8>(1).flipped(), Rank::<8>(6));
        assert_eq!(Rank::<8>(2).flipped(), Rank::<8>(5));
        assert_eq!(Rank::<8>(3).flipped(), Rank::<8>(4));
        assert_eq!(Rank::<8>(4).flipped(), Rank::<8>(3));
        assert_eq!(Rank::<8>(5).flipped(), Rank::<8>(2));
        assert_eq!(Rank::<8>(6).flipped(), Rank::<8>(1));
        assert_eq!(Rank::<8>(7).flipped(), Rank::<8>(0));
    }
}
