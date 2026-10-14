//! Drive the engine binary over its protocols, as a GUI would.

#[cfg(test)]
mod engine {
    use std::io::{Read, Write};
    use std::process::{Command, Stdio};
    use std::thread;
    use std::time::{Duration, Instant};

    const SEARCHES: [&str; 5] = ["primary", "random", "minimax", "alphabeta", "flatmc"];

    /// Longest a single engine run may take before the test fails.
    const TIMEOUT: Duration = Duration::from_secs(60);

    /// Run the engine with the given commands and return its stdout.
    /// Fails the test if the engine panics, exits with an error, or hangs.
    fn run(game: &str, commands: &[&str]) -> String {
        let mut child = Command::new(env!("CARGO_BIN_EXE_faeries"))
            .args(["--game", game])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to start engine");

        let mut input = commands.join("\n");
        input.push_str("\nquit\n");
        child
            .stdin
            .take()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();

        // Read output on other threads so a full pipe can't block the engine
        let mut out = child.stdout.take().unwrap();
        let mut err = child.stderr.take().unwrap();
        let stdout = thread::spawn(move || {
            let mut buf = String::new();
            out.read_to_string(&mut buf).map(|_| buf)
        });
        let stderr = thread::spawn(move || {
            let mut buf = String::new();
            err.read_to_string(&mut buf).map(|_| buf)
        });

        let start = Instant::now();
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if start.elapsed() > TIMEOUT {
                child.kill().unwrap();
                panic!("engine hung on {game} with {commands:?}");
            }
            thread::sleep(Duration::from_millis(10));
        };

        let stdout = stdout.join().unwrap().unwrap();
        let stderr = stderr.join().unwrap().unwrap();
        assert!(
            status.success(),
            "engine failed on {game} with {commands:?}\nstderr: {stderr}\nstdout: {stdout}",
        );
        stdout
    }

    /// The moves the engine chose, in order.
    fn bestmoves(output: &str) -> Vec<&str> {
        output
            .lines()
            .filter_map(|line| line.strip_prefix("bestmove "))
            .collect()
    }

    #[test]
    fn ugi_finished_games() {
        let positions = [
            (
                "ataxx",
                "fen xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 0 1",
                "p1win",
            ),
            ("chess", "fen 1R2k3/8/4K3/8/8/8/8/8 b - - 0 1", "p1win"),
            ("chess", "fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1", "draw"),
            ("connect4", "startpos moves d e d e d e d", "p1win"),
            (
                "droptaxx",
                "fen xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 1",
                "p1win",
            ),
            (
                "gomoku",
                "fen 15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxx10 o 1",
                "p1win",
            ),
            ("isolation", "fen 8/8/8/8/8/6Pp b", "p2win"),
            ("pijersi", "fen R-4r-/7/6/7/6/7/6 w 0 1", "p1win"),
            ("tak", "fen x6/x6/x6/x6/x6/1,1,1,1,1,1 1 1", "p1win"),
        ];

        for (game, position, result) in positions {
            for search in SEARCHES {
                let output = run(
                    game,
                    &[
                        "ugi",
                        &format!("setoption name search value {search}"),
                        &format!("position {position}"),
                        "query result",
                        "go depth 2",
                    ],
                );

                assert!(
                    output.contains(&format!("response {result}")),
                    "{game} {position}: expected {result}\n{output}"
                );
                assert_eq!(
                    bestmoves(&output),
                    ["0000"],
                    "{game} {search} {position}\n{output}"
                );
            }
        }
    }

    #[test]
    fn uci_finished_games() {
        for position in [
            "fen 1R2k3/8/4K3/8/8/8/8/8 b - - 0 1",
            "fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1",
            "startpos moves f2f3 e7e5 g2g4 d8h4",
        ] {
            for search in SEARCHES {
                let output = run(
                    "chess",
                    &[
                        "uci",
                        &format!("setoption name search value {search}"),
                        &format!("position {position}"),
                        "go depth 2",
                    ],
                );
                assert_eq!(bestmoves(&output), ["0000"], "{search} {position}");
            }
        }
    }

    #[test]
    fn tei_finished_games() {
        for (size, position) in [
            (6, "tps x6/x6/x6/x6/x6/1,1,1,1,1,1 1 1"),
            (3, "startpos moves a3 c3 c2 a1 c1"),
        ] {
            for search in SEARCHES {
                let output = run(
                    "tak",
                    &[
                        "tei",
                        &format!("teinewgame size {size}"),
                        &format!("setoption name search value {search}"),
                        &format!("position {position}"),
                        "go depth 2",
                    ],
                );
                assert_eq!(bestmoves(&output), ["0000"], "{search} {position}");
            }
        }
    }

    #[test]
    fn flatmc_without_a_limit_reports_an_error() {
        let protocols = [
            ("chess", "ugi"),
            ("chess", "uci"),
            ("connect4", "ugi"),
            ("tak", "tei"),
        ];

        for (game, protocol) in protocols {
            let output = run(
                game,
                &[
                    protocol,
                    "setoption name search value flatmc",
                    "go depth 3",
                    "isready",
                ],
            );
            assert!(
                output.contains("info string error"),
                "{game} {protocol}\n{output}"
            );
            assert_eq!(bestmoves(&output), ["0000"], "{game} {protocol}");
            assert!(output.contains("readyok"), "{game} {protocol}");
        }
    }

    #[test]
    fn flatmc_playouts_survive_stalemate() {
        let output = run(
            "chess",
            &[
                "ugi",
                "setoption name search value flatmc",
                "position startpos moves e2e4 e7e5 g1f3",
                "go nodes 500",
            ],
        );
        let moves = bestmoves(&output);
        assert_eq!(moves.len(), 1, "{output}");
        assert_ne!(moves[0], "0000", "{output}");
    }
}
