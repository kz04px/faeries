#[cfg(test)]
mod position {
    use ugi::position;

    #[test]
    fn test_input_nomoves() {
        let inputs = [
            ("startpos", "startpos"),
            ("fen test", "test"),
            ("fen 1 2", "1 2"),
            ("fen 1 2 3 4 5 6 7", "1 2 3 4 5 6 7"),
        ];

        for (input, expected) in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = position::parse(&mut stream);
            assert!(parsed.is_ok(), "{}", input);
            assert_eq!(parsed.unwrap(), expected, "{}", input);
            assert_eq!(stream.next(), None, "{}", input);
        }
    }

    #[test]
    fn test_input_moves() {
        let inputs = [
            ("startpos moves", "startpos"),
            ("startpos moves a", "startpos"),
            ("startpos moves a b c d e", "startpos"),
            ("fen test moves", "test"),
            ("fen test moves 1 2", "test"),
            ("fen 1 2 moves", "1 2"),
            ("fen 1 2 3 4 5 6 7 moves", "1 2 3 4 5 6 7"),
            ("fen 1 2 3 4 5 6 7 moves a", "1 2 3 4 5 6 7"),
            ("fen 1 2 3 4 5 6 7 moves a b c d e", "1 2 3 4 5 6 7"),
        ];

        for (input, expected) in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = position::parse(&mut stream);
            assert!(parsed.is_ok(), "{}", input);
            assert_eq!(parsed.unwrap(), expected, "{}", input);
            assert_eq!(stream.next().unwrap(), "moves", "{}", input);
        }
    }

    #[test]
    fn test_input_invalid() {
        let inputs = [
            "test",
            "fen",
            "fen moves",
            "fen moves a",
            "fen moves a b c d e",
        ];

        for input in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = position::parse(&mut stream);
            assert!(parsed.is_err(), "{}", input);
        }
    }
}
