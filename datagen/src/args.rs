use games::Games;
use std::{env, fmt::Display, num::ParseIntError};

pub struct Args {
    pub fens_path: String,
    pub nodes: u64,
    pub threads: usize,
    pub num_games: usize,
    pub verbose: bool,
    pub game: Option<games::Games>,
}

#[derive(Debug)]
pub struct ArgsError {
    msg: String,
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl Default for Args {
    fn default() -> Self {
        Self {
            fens_path: Default::default(),
            nodes: 5_000,
            threads: 1,
            num_games: 1_000,
            verbose: false,
            game: None,
        }
    }
}

impl From<ArgsError> for std::io::Error {
    fn from(e: ArgsError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, e.msg)
    }
}

impl From<ParseIntError> for ArgsError {
    fn from(value: ParseIntError) -> Self {
        ArgsError {
            msg: format!("Bad value: {}", value).to_string(),
        }
    }
}

#[must_use]
pub fn parse_commandline() -> Result<Args, ArgsError> {
    let args: Vec<String> = env::args().collect();
    parse_string(&args[1..].join(" "))
}

#[must_use]
pub fn parse_string(line: &str) -> Result<Args, ArgsError> {
    parse(&line.split(" ").collect::<Vec<&str>>())
}

#[must_use]
pub fn parse(words: &[&str]) -> Result<Args, ArgsError> {
    let mut parsed = Args::default();
    let mut iter = words.iter().peekable();

    while let Some(word) = iter.next() {
        match (word, iter.peek()) {
            // fens
            (&"--fens", Some(_)) => parsed.fens_path = iter.next().unwrap().to_string(),
            (&"--fens", _) => {
                return Err(ArgsError {
                    msg: "Missing fen path".to_string(),
                });
            }
            // Game
            (&"--game", Some(name)) => {
                parsed.game = match name {
                    &&"ataxx" => Some(Games::Ataxx),
                    &&"chess" => Some(Games::Chess),
                    &&"connect4" => Some(Games::Connect4),
                    &&"droptaxx" => Some(Games::Droptaxx),
                    &&"gomoku" => Some(Games::Gomoku),
                    &&"isolation" => Some(Games::Isolation),
                    &&"pijersi" => Some(Games::Pijersi),
                    &&"tak" => Some(Games::Tak),
                    _ => {
                        return Err(ArgsError {
                            msg: "Unrecognised game name".to_string(),
                        });
                    }
                };
                iter.next();
            }

            // Nodes
            (&"--nodes", Some(_)) => parsed.nodes = iter.next().unwrap().parse::<u64>()?,
            (&"--nodes", _) => {
                return Err(ArgsError {
                    msg: "Missing node count".to_string(),
                });
            }
            // Games
            (&"--games", Some(_)) => parsed.num_games = iter.next().unwrap().parse::<usize>()?,
            (&"--games", _) => {
                return Err(ArgsError {
                    msg: "Missing number of games".to_string(),
                });
            }
            // Threads
            (&"--threads", Some(_)) => parsed.threads = iter.next().unwrap().parse::<usize>()?,
            (&"--threads", _) => {
                return Err(ArgsError {
                    msg: "Missing thread count".to_string(),
                });
            }
            // Flags
            (&"--verbose", _) => parsed.verbose = true,
            // Other
            _ => {
                return Err(ArgsError {
                    msg: format!("Unrecognised word: '{}'", word),
                });
            }
        }
    }

    if parsed.nodes < 1 {
        Err(ArgsError {
            msg: "nodes must be >= 1".to_string(),
        })
    } else if parsed.threads < 1 {
        Err(ArgsError {
            msg: "threads must be >= 1".to_string(),
        })
    } else if parsed.fens_path.is_empty() {
        Err(ArgsError {
            msg: "fen path must not be empty".to_string(),
        })
    } else if parsed.game.is_none() {
        Err(ArgsError {
            msg: "game must be specified".to_string(),
        })
    } else {
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        assert_eq!(Args::default().threads, 1);
    }

    #[test]
    fn example() {
        let args =
            parse_string("--fens path --game ataxx --nodes 1000 --games 5000 --threads 2").unwrap();
        assert_eq!(args.fens_path, "path");
        assert_eq!(args.game, Some(Games::Ataxx));
        assert_eq!(args.nodes, 1000);
        assert_eq!(args.num_games, 5000);
        assert_eq!(args.threads, 2);
    }

    #[test]
    fn success() {
        let tests = [
            "--fens path --game ataxx",
            "--fens path --game chess",
            "--fens path --game ataxx --verbose",
            "--fens path --game ataxx --nodes 1",
            "--fens path --game ataxx --nodes 100000",
            "--fens path --game ataxx --threads 1",
            "--fens path --game ataxx --threads 100000",
            "--fens path --game ataxx --nodes 1000 --threads 2",
        ];

        for line in tests {
            let res = parse_string(line);
            assert!(res.is_ok(), "{}", res.err().unwrap());
        }
    }

    #[test]
    fn errors() {
        let tests = [
            "",
            " ",
            "     ",
            "--fens",
            "--game",
            "--fens path",
            "--game ataxx",
            "--nodes",
            "--nodes 0",
            "--fens --game ataxx",
            "--fens path --game",
            "--fens path --game ataxx test",
            "--fens path --game unknown",
            "--fens path --game ataxx --nodes",
            "--fens path --game ataxx --nodes 0",
            "--fens path --game ataxx --nodes -1",
            "--fens path --game ataxx --nodes test",
            "--fens path --game ataxx --threads",
            "--fens path --game ataxx --threads 0",
            "--fens path --game ataxx --threads -1",
            "--fens path --game ataxx --threads test",
            "   --fens   path    --game    ataxx   ",
        ];

        for line in tests {
            assert!(parse_string(line).is_err(), "{}", line);
        }
    }
}
