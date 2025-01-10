use uci::UCI;

struct FakeState {
    pub received: Vec<String>,
}

impl UCI for FakeState {
    fn init(&mut self) {
        self.received.push("init".to_string());
    }

    fn shutdown(&mut self) {
        self.received.push("shutdown".to_string());
    }

    fn name(&self) -> String {
        "".to_string()
    }

    fn author(&self) -> String {
        "".to_string()
    }

    fn ucinewgame(&mut self) {
        self.received.push("ucinewgame".to_string());
    }

    fn isready(&mut self) {
        self.received.push("isready".to_string());
    }

    fn position(&mut self, fen: &str) {
        self.received.push(format!("position fen {}", fen));
    }

    fn moves(&mut self, movestr: &str) {
        self.received.push(format!("moves {}", movestr));
    }

    fn go(&mut self, _settings: &uci::go::GoSettings) {
        self.received.push("go".to_string());
    }

    fn perft(&mut self, _settings: &uci::go::GoSettings) {
        self.received.push("perft".to_string());
    }

    fn split(&mut self, _settings: &uci::go::GoSettings) {
        self.received.push("split".to_string());
    }

    fn stop(&mut self) {
        self.received.push("stop".to_string());
    }

    fn print(&self) {
        todo!()
    }

    fn print_options(&self) {}

    fn set_option(&mut self, _name: &str, _value: &str) {
        self.received.push("setoption".to_string());
    }

    fn is_debug(&self) -> bool {
        false
    }

    fn query_p1turn(&self) -> bool {
        todo!()
    }

    fn query_result(&self) -> Option<uci::UCIGameResult> {
        todo!()
    }
}

#[cfg(test)]
mod listen {
    use crate::FakeState;

    #[test]
    fn test_inputs() {
        let tests = vec![
            // Quit immediately
            (vec!["quit"], vec!["shutdown"]),
            // Provide isready
            (vec!["isready", "quit"], vec!["init", "isready", "shutdown"]),
            // Only initialise once
            (vec!["isready", "quit"], vec!["init", "isready", "shutdown"]),
            (
                vec!["isready", "isready", "quit"],
                vec!["init", "isready", "isready", "shutdown"],
            ),
            (
                vec!["isready", "isready", "isready", "quit"],
                vec!["init", "isready", "isready", "isready", "shutdown"],
            ),
            // isready, go
            (
                vec!["isready", "go depth 1", "quit"],
                vec!["init", "isready", "go", "shutdown"],
            ),
            // Skip isready
            (vec!["go depth 1", "quit"], vec!["init", "go", "shutdown"]),
            (vec!["perft 1", "quit"], vec!["init", "perft", "shutdown"]),
            (vec!["split 1", "quit"], vec!["init", "split", "shutdown"]),
            (
                vec!["ucinewgame", "quit"],
                vec!["init", "ucinewgame", "shutdown"],
            ),
            (
                vec!["position startpos", "quit"],
                vec!["init", "position fen startpos", "shutdown"],
            ),
            // setoption before isready
            (
                vec!["setoption name test value test", "quit"],
                vec!["setoption", "shutdown"],
            ),
            // setoption, isready
            (
                vec!["setoption name test value test", "isready", "quit"],
                vec!["setoption", "init", "isready", "shutdown"],
            ),
            // setoption, isready, setoption
            (
                vec![
                    "setoption name test value test",
                    "isready",
                    "setoption name test value test",
                    "quit",
                ],
                vec!["setoption", "init", "isready", "setoption", "shutdown"],
            ),
            // perft variants
            (vec!["perft 1", "quit"], vec!["init", "perft", "shutdown"]),
            (
                vec!["perft depth 1", "quit"],
                vec!["init", "perft", "shutdown"],
            ),
            (
                vec!["go perft 1", "quit"],
                vec!["init", "perft", "shutdown"],
            ),
            (
                vec!["go perft depth 1", "quit"],
                vec!["init", "perft", "shutdown"],
            ),
            (vec!["split 1", "quit"], vec!["init", "split", "shutdown"]),
            (
                vec!["split depth 1", "quit"],
                vec!["init", "split", "shutdown"],
            ),
            (
                vec!["go split 1", "quit"],
                vec!["init", "split", "shutdown"],
            ),
            (
                vec!["go split depth 1", "quit"],
                vec!["init", "split", "shutdown"],
            ),
            // position, moves
            (
                vec!["isready", "position startpos", "quit"],
                vec!["init", "isready", "position fen startpos", "shutdown"],
            ),
            (
                vec!["isready", "position startpos moves", "quit"],
                vec!["init", "isready", "position fen startpos", "shutdown"],
            ),
            (
                vec!["isready", "position startpos moves 1 2 3", "quit"],
                vec![
                    "init",
                    "isready",
                    "position fen startpos",
                    "moves 1",
                    "moves 2",
                    "moves 3",
                    "shutdown",
                ],
            ),
            (
                vec!["isready", "position fen startpos", "quit"],
                vec!["init", "isready", "position fen startpos", "shutdown"],
            ),
            (
                vec!["isready", "position fen startpos moves", "quit"],
                vec!["init", "isready", "position fen startpos", "shutdown"],
            ),
            (
                vec!["isready", "position fen 1 2 3 4 5 6 moves", "quit"],
                vec!["init", "isready", "position fen 1 2 3 4 5 6", "shutdown"],
            ),
            (
                vec!["isready", "position fen 1 2 3 4 5 6 moves 7 8", "quit"],
                vec![
                    "init",
                    "isready",
                    "position fen 1 2 3 4 5 6",
                    "moves 7",
                    "moves 8",
                    "shutdown",
                ],
            ),
            // Spaces
            (
                vec!["  isready  ", "  go  depth  1  ", "quit"],
                vec!["init", "isready", "go", "shutdown"],
            ),
            // Junk input
            (vec!["", "quit"], vec!["shutdown"]),
            (vec!["test", "quit"], vec!["shutdown"]),
            (vec!["test", "test", "test", "quit"], vec!["shutdown"]),
            (
                vec!["test", "isready", "test", "quit"],
                vec!["init", "isready", "shutdown"],
            ),
            (
                vec!["", "", "isready", "", "", "quit"],
                vec!["init", "isready", "shutdown"],
            ),
            (
                vec!["", "", "go depth 1", "", "", "quit"],
                vec!["init", "go", "shutdown"],
            ),
            (
                vec!["", "", "go depth 3", "", "isready", "", "quit"],
                vec!["init", "go", "isready", "shutdown"],
            ),
        ];

        for (inputs, outputs) in tests {
            let mut state = FakeState { received: vec![] };
            let mut i = 0;
            let get_input = |input: &mut String| {
                if i < inputs.len() {
                    *input = inputs[i].to_string();
                    i += 1;
                    Ok(input.len())
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Uh oh",
                    ))
                }
            };
            let result: Result<(), std::io::Error> = uci::listen::listen(&mut state, get_input);

            assert!(result.is_ok(), "{:#?}", inputs);
            assert_eq!(state.received.len(), outputs.len(), "{:#?}", inputs);
            assert_eq!(state.received, outputs, "{:#?}", inputs);
        }
    }

    #[test]
    fn test_inputs_invalid() {
        let tests = vec![
            (vec!["go", "quit"], vec!["init", "shutdown"]),
            (vec!["go 1", "quit"], vec!["init", "shutdown"]),
            (vec!["go depth -3", "quit"], vec!["init", "shutdown"]),
        ];

        for (inputs, outputs) in tests {
            let mut state = FakeState { received: vec![] };
            let mut i = 0;
            let get_input = |input: &mut String| {
                if i < inputs.len() {
                    *input = inputs[i].to_string();
                    i += 1;
                    Ok(input.len())
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Uh oh",
                    ))
                }
            };
            let result: Result<(), std::io::Error> = uci::listen::listen(&mut state, get_input);

            assert!(result.is_ok(), "{:#?}", inputs);
            assert_eq!(state.received.len(), outputs.len(), "{:#?}", inputs);
            assert_eq!(state.received, outputs, "{:#?}", inputs);
        }
    }

    #[test]
    fn test_input_errors() {
        let tests = vec![
            (vec![], vec![]),
            (vec!["isready"], vec!["init", "isready"]),
            (vec!["go"], vec!["init"]),
        ];

        for (inputs, outputs) in tests {
            let mut state = FakeState { received: vec![] };
            let mut i = 0;
            let get_input = |input: &mut String| {
                if i < inputs.len() {
                    *input = inputs[i].to_string();
                    i += 1;
                    Ok(input.len())
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Uh oh",
                    ))
                }
            };
            let result: Result<(), std::io::Error> = uci::listen::listen(&mut state, get_input);

            assert!(result.is_err(), "{:#?}", inputs);
            assert_eq!(state.received.len(), outputs.len(), "{:#?}", inputs);
            assert_eq!(state.received, outputs, "{:#?}", inputs);
        }
    }
}
