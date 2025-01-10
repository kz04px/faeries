#[cfg(test)]
mod go {
    use uci::go::{self, GoSettings};

    #[test]
    fn test_input() {
        let tests = vec![
            ("go depth 123", GoSettings::from_depth(123)),
            ("go movetime 123", GoSettings::from_movetime(123)),
            ("go nodes 123", GoSettings::from_nodes(123)),
            (
                "go wtime 123",
                GoSettings::from_time(Some(123), None, None, None),
            ),
            (
                "go btime 123",
                GoSettings::from_time(None, Some(123), None, None),
            ),
            (
                "go wtime 123 btime 456",
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
            "go wtime",
            "go btime",
            "go winc",
            "go binc",
            "go nodes",
            "go movetime",
            "go movestogo",
            // Illegal value given
            "go wtime -1",
            "go btime -1",
            "go winc -1",
            "go binc -1",
            "go depth 0",
            "go depth -1",
            "go nodes 0",
            "go nodes -1",
            "go movetime -1",
            "go movestogo -1",
            // Increment without time
            "go winc 123",
            "go binc 123",
            "go winc 123 binc 123",
            // Not a number
            "go wtime test",
            "go btime test",
            "go winc test",
            "go binc test",
            "go depth test",
            "go nodes test",
            "go movetime test",
            "go movestogo test",
            // Repeats
            "go wtime 1 wtime 1",
            "go btime 1 btime 1",
            "go wtime 1 btime 1 winc 1 winc 1",
            "go wtime 1 btime 1 binc 1 binc 1",
            "go depth 1 depth 1",
            "go nodes 1 nodes 1",
            "go movestogo 1 movestogo 1",
            // Only search type
            "go search",
            "go perft",
            "go split",
            "go fastperft",
            // Others
            "go wtime 123 btime",
            "go wtime btime 123",
            "go 123",
        ];

        for input in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = go::parse(&mut stream);
            assert!(parsed.is_err(), "{}", input);
        }
    }
}
