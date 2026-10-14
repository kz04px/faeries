#[cfg(test)]
mod position {
    use protocols::tei::position;

    #[test]
    fn test_input_nomoves() {
        let inputs = [
            ("startpos", "startpos"),
            ("tps test", "test"),
            ("tps 1 2", "1 2"),
            ("tps 1 2 3 4 5 6 7", "1 2 3 4 5 6 7"),
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
            ("tps test moves", "test"),
            ("tps test moves 1 2", "test"),
            ("tps 1 2 moves", "1 2"),
            ("tps 1 2 3 4 5 6 7 moves", "1 2 3 4 5 6 7"),
            ("tps 1 2 3 4 5 6 7 moves a", "1 2 3 4 5 6 7"),
            ("tps 1 2 3 4 5 6 7 moves a b c d e", "1 2 3 4 5 6 7"),
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
            "tps",
            "tps moves",
            "tps moves a",
            "tps moves a b c d e",
        ];

        for input in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = position::parse(&mut stream);
            assert!(parsed.is_err(), "{}", input);
        }
    }
}
