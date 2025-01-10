#[cfg(test)]
mod test {
    use games::{general::square::Square, gomoku::GomokuMove};

    #[test]
    fn move_parsing() {
        let tests = [
            ("a1", GomokuMove(Square::<15, 15>::from_coords(0, 0))),
            ("a15", GomokuMove(Square::<15, 15>::from_coords(0, 14))),
            ("o1", GomokuMove(Square::<15, 15>::from_coords(14, 0))),
            ("o15", GomokuMove(Square::<15, 15>::from_coords(14, 14))),
        ];

        for (movestr, expected) in tests {
            if let Ok(mv) = GomokuMove::from_string(&movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move {}", movestr);
            }
        }
    }
}
