#[cfg(test)]
mod go {
    use protocols::GoSettings;
    use protocols::tei;

    #[test]
    fn test_input() {
        let tests = vec![
            ("depth 123", GoSettings::from_depth(123)),
            ("movetime 123", GoSettings::from_movetime(123)),
            ("nodes 123", GoSettings::from_nodes(123)),
            (
                "wtime 123",
                GoSettings::from_time(Some(123), None, None, None),
            ),
            (
                "btime 123",
                GoSettings::from_time(None, Some(123), None, None),
            ),
            (
                "wtime 123 btime 456",
                GoSettings::from_time(Some(123), Some(456), None, None),
            ),
            ("  depth      123   ", GoSettings::from_depth(123)),
        ];

        for (input, expected) in tests {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = tei::go::parse(&mut stream);
            assert!(parsed.is_ok(), "{}", input);
            assert_eq!(expected, parsed.unwrap(), "{}", input);
        }
    }

    #[test]
    fn test_input_errors() {
        let inputs = [
            // No options at all
            "",
            // No value given
            "depth",
            "wtime",
            "btime",
            "winc",
            "binc",
            "nodes",
            "movetime",
            "movestogo",
            // Illegal value given
            "wtime -1",
            "btime -1",
            "winc -1",
            "binc -1",
            "depth 0",
            "depth -1",
            "nodes 0",
            "nodes -1",
            "movetime -1",
            "movestogo -1",
            // Increment without time
            "winc 123",
            "binc 123",
            "winc 123 binc 123",
            // Not a number
            "wtime test",
            "btime test",
            "winc test",
            "binc test",
            "depth test",
            "nodes test",
            "movetime test",
            "movestogo test",
            // Repeats
            "wtime 1 wtime 1",
            "btime 1 btime 1",
            "wtime 1 btime 1 winc 1 winc 1",
            "wtime 1 btime 1 binc 1 binc 1",
            "depth 1 depth 1",
            "nodes 1 nodes 1",
            "movestogo 1 movestogo 1",
            // Only search type
            "search",
            "perft",
            "split",
            "fastperft",
            // Others
            "wtime 123 btime",
            "wtime btime 123",
            "test",
            "123",
            "asd depth 123",
            "depth asd 123",
            "depth 123 asd",
        ];

        for input in inputs {
            let mut stream = input.split_ascii_whitespace().peekable();
            let parsed = tei::go::parse(&mut stream);
            assert!(parsed.is_err(), "{}", input);
        }
    }
}
