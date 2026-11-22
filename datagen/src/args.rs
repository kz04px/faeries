use std::{env, fmt::Display, num::ParseIntError};

pub struct Args {
    pub fens_path: String,
    pub output_path: String,
    pub search: Option<String>,
    pub random_plies: usize,
    pub max_plies: usize,
    pub seed: u64,
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
            output_path: Default::default(),
            search: None,
            random_plies: 0,
            max_plies: 500,
            seed: 0x9e3779b97f4a7c15,
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
            // Output
            (&"--output", Some(_)) => parsed.output_path = iter.next().unwrap().to_string(),
            (&"--output", _) => {
                return Err(ArgsError {
                    msg: "Missing output path".to_string(),
                });
            }
            // Search
            (&"--search", Some(_)) => parsed.search = Some(iter.next().unwrap().to_string()),
            (&"--search", _) => {
                return Err(ArgsError {
                    msg: "Missing search name".to_string(),
                });
            }
            // Random plies
            (&"--random-plies", Some(_)) => {
                parsed.random_plies = iter.next().unwrap().parse::<usize>()?
            }
            (&"--random-plies", _) => {
                return Err(ArgsError {
                    msg: "Missing random ply count".to_string(),
                });
            }
            // Max plies
            (&"--max-plies", Some(_)) => {
                parsed.max_plies = iter.next().unwrap().parse::<usize>()?
            }
            (&"--max-plies", _) => {
                return Err(ArgsError {
                    msg: "Missing max ply count".to_string(),
                });
            }
            // Seed
            (&"--seed", Some(_)) => parsed.seed = iter.next().unwrap().parse::<u64>()?,
            (&"--seed", _) => {
                return Err(ArgsError {
                    msg: "Missing seed".to_string(),
                });
            }
            // Game
            (&"--game", Some(name)) => {
                parsed.game = match games::parse_game_name(name) {
                    Some(game) => Some(game),
                    None => {
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
    } else if parsed.output_path.is_empty() {
        Err(ArgsError {
            msg: "output path must not be empty".to_string(),
        })
    } else if parsed.max_plies < 1 {
        Err(ArgsError {
            msg: "max-plies must be >= 1".to_string(),
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
    use games::Games;

    #[test]
    fn default() {
        assert_eq!(Args::default().threads, 1);
    }

    #[test]
    fn example() {
        let args = parse_string(
            "--fens path --output out --game ataxx --nodes 1000 --games 5000 --threads 2",
        )
        .unwrap();
        assert_eq!(args.fens_path, "path");
        assert_eq!(args.game, Some(Games::Ataxx));
        assert_eq!(args.nodes, 1000);
        assert_eq!(args.num_games, 5000);
        assert_eq!(args.threads, 2);
        assert_eq!(args.output_path, "out");
    }

    #[test]
    fn options() {
        let args = parse_string(
            "--fens path --output out --game ataxx --search random --random-plies 4 --max-plies 80 --seed 7",
        )
        .unwrap();
        assert_eq!(args.search.as_deref(), Some("random"));
        assert_eq!(args.random_plies, 4);
        assert_eq!(args.max_plies, 80);
        assert_eq!(args.seed, 7);
    }

    #[test]
    fn success() {
        let tests = [
            "--fens path --output out --game ataxx",
            "--fens path --output out --game chess",
            "--fens path --output out --game ataxx --verbose",
            "--fens path --output out --game ataxx --nodes 1",
            "--fens path --output out --game ataxx --nodes 100000",
            "--fens path --output out --game ataxx --threads 1",
            "--fens path --output out --game ataxx --threads 100000",
            "--fens path --output out --game ataxx --nodes 1000 --threads 2",
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
            "--fens path --game ataxx",
            "--output",
            "--search",
            "--seed x",
            "--fens path --output out --game ataxx --max-plies 0",
            "--game ataxx",
            "--nodes",
            "--nodes 0",
            "--fens --game ataxx",
            "--fens path --output out --game",
            "--fens path --output out --game ataxx test",
            "--fens path --output out --game unknown",
            "--fens path --output out --game ataxx --nodes",
            "--fens path --output out --game ataxx --nodes 0",
            "--fens path --output out --game ataxx --nodes -1",
            "--fens path --output out --game ataxx --nodes test",
            "--fens path --output out --game ataxx --threads",
            "--fens path --output out --game ataxx --threads 0",
            "--fens path --output out --game ataxx --threads -1",
            "--fens path --output out --game ataxx --threads test",
            "   --fens   path    --game    ataxx   ",
        ];

        for line in tests {
            assert!(parse_string(line).is_err(), "{}", line);
        }
    }
}
