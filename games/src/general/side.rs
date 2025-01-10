use std::ops::Not;

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub enum Side {
    #[default]
    Player1,
    Player2,
}

impl Not for Side {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Side::Player1 => Side::Player2,
            Side::Player2 => Side::Player1,
        }
    }
}

impl<Any> std::ops::Index<Side> for [Any; 2] {
    type Output = Any;

    fn index(&self, index: Side) -> &Self::Output {
        &self[index as usize]
    }
}

impl<Any> std::ops::IndexMut<Side> for [Any; 2] {
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[cfg(test)]
mod test {
    use crate::general::side::Side;

    #[test]
    fn not() {
        assert_eq!(!Side::Player1, Side::Player2);
        assert_eq!(!Side::Player2, Side::Player1);
    }

    #[test]
    fn indexing() {
        let array = [1, 2];
        assert_eq!(array[Side::Player1], 1);
        assert_eq!(array[Side::Player2], 2);
        assert_eq!(array[!Side::Player1], 2);
        assert_eq!(array[!Side::Player2], 1);
    }
}
