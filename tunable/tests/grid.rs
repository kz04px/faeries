//! `#[tunable]` on an array of arrays

use tunable::{Parts, tunable};

const BIG: i32 = 1_000;

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

#[tunable(min = 0, max = 100, step = 5, skip = [0..2, 5])]
const GRID: [[i32; 3]; 2] = [[0, 0, 12], [13, 14, BIG]];

#[tunable(min = -50, max = 50, step = 4)]
const PAIRS: [[Pair; 2]; 2] = [[Pair(1, 2), Pair(3, 4)], [Pair(5, 6), Pair(7, 8)]];

#[test]
fn a_grid_is_read_by_row_then_column() {
    assert_eq!(GRID[0][2], 12);
    assert_eq!(GRID[1][0], 13);
    assert_eq!(GRID[1][2], BIG);
    assert_eq!(GRID.len(), 2);
    assert_eq!(GRID[1].len(), 3);
    assert_eq!(PAIRS[0][1], Pair(3, 4));
    assert_eq!(PAIRS[1][0], Pair(5, 6));
}

#[cfg(feature = "tune")]
#[test]
fn a_grid_is_listed_flat_and_set_by_element() {
    let prefix = module_path!();
    let names = |filter: &str| -> Vec<String> {
        tunable::params(prefix)
            .into_iter()
            .map(|p| p.name)
            .filter(|n| n.starts_with(filter))
            .collect()
    };

    // Skipped elements aren't listed, and the rest keep their row-major index
    assert_eq!(names("grid"), ["grid_2", "grid_3", "grid_4"]);
    assert_eq!(names("pairs").len(), 8);

    assert!(tunable::set(prefix, "grid_3", "50"));
    assert_eq!(GRID[1][0], 50);
    assert_eq!(GRID[0][2], 12);

    // The second integer of the pair at row 1, column 0
    assert!(tunable::set(prefix, "pairs_5", "-7"));
    assert_eq!(PAIRS[1][0], Pair(5, -7));
    assert_eq!(PAIRS[1][1], Pair(7, 8));

    assert!(!tunable::set(prefix, "grid_5", "1"));
    tunable::reset(prefix);
    assert_eq!(GRID[1][0], 13);
    assert_eq!(PAIRS[1][0], Pair(5, 6));
}
