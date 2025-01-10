#[cfg(test)]
mod go {
    use ugi::go::{self, GoSettings};

    #[test]
    fn test_input() {
        let tests = vec![
            ("go depth 123", GoSettings::from_depth(123)),
            ("go movetime 123", GoSettings::from_movetime(123)),
            ("go nodes 123", GoSettings::from_nodes(123)),
            (
                "go p1time 123",
                GoSettings::from_time(Some(123), None, None, None),
            ),
            (
                "go p2time 123",
                GoSettings::from_time(None, Some(123), None, None),
            ),
            (
                "go p1time 123 p2time 456",
                GoSettings::from_time(Some(123), Some(456), None, None),
            ),
            ("   go      depth      123   ", GoSettings::from_depth(123)),
        ];

        for (input, expected) in tests {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = go::parse(&mut stream);
            assert!(parsed.is_ok(), "{}", input);
            assert_eq!(expected, parsed.unwrap(), "{}", input);
        }
    }

    #[test]
    fn test_input_errors() {
        let inputs = [
            // No options at all
            "go",
            // No value given
            "go depth",
            "go p1time",
            "go p2time",
            "go p1inc",
            "go p2inc",
            "go nodes",
            "go movetime",
            "go movestogo",
            // Illegal value given
            "go p1time -1",
            "go p2time -1",
            "go p1inc -1",
            "go p2inc -1",
            "go depth 0",
            "go depth -1",
            "go nodes 0",
            "go nodes -1",
            "go movetime -1",
            "go movestogo -1",
            // Increment without time
            "go p1inc 123",
            "go p2inc 123",
            "go p1inc 123 p2inc 123",
            // Not a number
            "go p1time test",
            "go p2time test",
            "go p1inc test",
            "go p2inc test",
            "go depth test",
            "go nodes test",
            "go movetime test",
            "go movestogo test",
            // Repeats
            "go p1time 1 p1time 1",
            "go p2time 1 p2time 1",
            "go p1time 1 p2time 1 p1inc 1 p1inc 1",
            "go p1time 1 p2time 1 p2inc 1 p2inc 1",
            "go depth 1 depth 1",
            "go nodes 1 nodes 1",
            "go movestogo 1 movestogo 1",
            // Only search type
            "go search",
            "go perft",
            "go split",
            "go fastperft",
            // Others
            "go p1time 123 p2time",
            "go p1time p2time 123",
            "go 123",
        ];

        for input in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = go::parse(&mut stream);
            assert!(parsed.is_err(), "{}", input);
        }
    }
}
