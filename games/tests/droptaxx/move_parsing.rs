#[cfg(test)]
mod test {
    use games::{droptaxx::DroptaxxMove, general::square::Square};

    #[test]
    fn move_parsing() {
        let tests = [
            ("a1", DroptaxxMove(Square::<7, 7>::from_coords(0, 0))),
            ("a7", DroptaxxMove(Square::<7, 7>::from_coords(0, 6))),
            ("g1", DroptaxxMove(Square::<7, 7>::from_coords(6, 0))),
            ("g7", DroptaxxMove(Square::<7, 7>::from_coords(6, 6))),
        ];

        for (movestr, expected) in tests {
            if let Ok(mv) = DroptaxxMove::from_string(&movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }
}
