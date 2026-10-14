//! `#[tunable]` on a type of integers, and an array of them

use tunable::{Parts, tunable};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pair(i32, i32);

impl Parts<2> for Pair {
    fn from_parts([a, b]: [i32; 2]) -> Self {
        Self(a, b)
    }

    fn to_parts(self) -> [i32; 2] {
        [self.0, self.1]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Triple(i32, i32, i32);

impl Parts<3> for Triple {
    fn from_parts([a, b, c]: [i32; 3]) -> Self {
        Self(a, b, c)
    }

    fn to_parts(self) -> [i32; 3] {
        [self.0, self.1, self.2]
    }
}

#[tunable(min = -50, max = 100, step = 4)]
const SINGLE: Pair = Pair(10, 20);

#[tunable(min = -50, max = 100, step = 4, skip = [1])]
const TABLE: [Pair; 3] = [Pair(1, 2), Pair(3, -4), Pair(-5, 6)];

#[tunable(min = 0, max = 10, step = 1)]
const TRIPLES: [Triple; 2] = [Triple(1, 2, 3), Triple(4, 5, 6)];

#[test]
fn a_tunable_of_integers_reads_back_as_its_type() {
    assert_eq!(SINGLE.get(), Pair(10, 20));
    assert_eq!(TABLE[0], Pair(1, 2));
    assert_eq!(TABLE[1], Pair(3, -4));
    assert_eq!(TABLE[2], Pair(-5, 6));
    assert_eq!(TABLE.len(), 3);
    assert_eq!(TRIPLES[1], Triple(4, 5, 6));
}
