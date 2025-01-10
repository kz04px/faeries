#[cfg(test)]
mod test {
    use games::{ataxx::AtaxxMove, general::square::Square};

    #[test]
    fn move_parsing() {
        let tests = [
            ("0000", AtaxxMove::Pass),
            ("a1", AtaxxMove::Single(Square::<7, 7>::from_coords(0, 0))),
            ("a7", AtaxxMove::Single(Square::<7, 7>::from_coords(0, 6))),
            ("g1", AtaxxMove::Single(Square::<7, 7>::from_coords(6, 0))),
            ("g7", AtaxxMove::Single(Square::<7, 7>::from_coords(6, 6))),
            (
                "a1c3",
                AtaxxMove::Double(
                    Square::<7, 7>::from_coords(0, 0),
                    Square::<7, 7>::from_coords(2, 2),
                ),
            ),
            (
                "g7e5",
                AtaxxMove::Double(
                    Square::<7, 7>::from_coords(6, 6),
                    Square::<7, 7>::from_coords(4, 4),
                ),
            ),
        ];

        for (movestr, expected) in tests {
            if let Ok(mv) = AtaxxMove::from_string(&movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }
}
