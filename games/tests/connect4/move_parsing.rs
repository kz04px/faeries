#[cfg(test)]
mod test {
    use games::{connect4::Connect4Move, general::file::File};

    #[test]
    fn move_parsing() {
        let tests = [
            ("a", Connect4Move(File(0))),
            ("b", Connect4Move(File(1))),
            ("c", Connect4Move(File(2))),
            ("d", Connect4Move(File(3))),
            ("e", Connect4Move(File(4))),
            ("f", Connect4Move(File(5))),
            ("g", Connect4Move(File(6))),
        ];

        for (movestr, expected) in tests {
            if let Ok(mv) = Connect4Move::from_string(&movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }
}
