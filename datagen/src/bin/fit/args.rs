use games::Games;
use std::{env, fmt::Display};

pub struct Args {
    pub data_path: String,
    pub game: Games,
    /// How much of the target is the search score rather than the game result, from 0 to 1.
    pub eval_weight: f64,
    /// The sigmoid scale in centipawns. Fitted to the game results when not given.
    pub k: Option<f64>,
    pub epochs: usize,
    /// Adam's step in parameter units.
    pub lr: f64,
    pub threads: usize,
    pub output_path: Option<String>,
}

#[derive(Debug)]
pub struct ArgsError {
    msg: String,
}

impl Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl From<ArgsError> for std::io::Error {
    fn from(e: ArgsError) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, e.msg)
    }
}

fn error<T>(msg: impl Into<String>) -> Result<T, ArgsError> {
    Err(ArgsError { msg: msg.into() })
}

/// The next word, parsed as the value of `flag`.
fn value<T: std::str::FromStr>(
    iter: &mut std::slice::Iter<&str>,
    flag: &str,
) -> Result<T, ArgsError> {
    let Some(word) = iter.next() else {
        return error(format!("Missing value for {}", flag));
    };
    match word.parse() {
        Ok(v) => Ok(v),
        Err(_) => error(format!("Bad value for {}: '{}'", flag, word)),
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
    let mut data_path = String::new();
    let mut game = None;
    let mut eval_weight = 0.5;
    let mut k = None;
    let mut epochs = 500;
    let mut lr = 1.0;
    let mut threads = 1;
    let mut output_path = None;
    let mut iter = words.iter();

    while let Some(word) = iter.next() {
        match *word {
            "--data" => data_path = value(&mut iter, word)?,
            "--game" => {
                let name: String = value(&mut iter, word)?;
                game = match games::parse_game_name(&name) {
                    Some(g) => Some(g),
                    None => return error("Unrecognised game name"),
                };
            }
            "--eval-weight" => eval_weight = value(&mut iter, word)?,
            "--k" => k = Some(value(&mut iter, word)?),
            "--epochs" => epochs = value(&mut iter, word)?,
            "--lr" => lr = value(&mut iter, word)?,
            "--threads" => threads = value(&mut iter, word)?,
            "--output" => output_path = Some(value(&mut iter, word)?),
            _ => return error(format!("Unrecognised word: '{}'", word)),
        }
    }

    if data_path.is_empty() {
        return error("data path must not be empty");
    }
    let Some(game) = game else {
        return error("game must be specified");
    };
    if !(0.0..=1.0).contains(&eval_weight) {
        return error("eval-weight must be between 0 and 1");
    }
    if k.is_some_and(|k| k <= 0.0 || !f64::is_finite(k)) {
        return error("k must be > 0");
    }
    if lr <= 0.0 || !f64::is_finite(lr) {
        return error("lr must be > 0");
    }
    if epochs < 1 {
        return error("epochs must be >= 1");
    }
    if threads < 1 {
        return error("threads must be >= 1");
    }

    Ok(Args {
        data_path,
        game,
        eval_weight,
        k,
        epochs,
        lr,
        threads,
        output_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let args = parse_string("--data d --game ataxx").unwrap();
        assert_eq!(args.eval_weight, 0.5);
        assert_eq!(args.k, None);
        assert_eq!(args.threads, 1);
    }

    #[test]
    fn example() {
        let args = parse_string(
            "--data d --game ataxx --eval-weight 0.25 --k 120 --epochs 10 --lr 2 --threads 4 --output o",
        )
        .unwrap();
        assert_eq!(args.game, Games::Ataxx);
        assert_eq!(args.eval_weight, 0.25);
        assert_eq!(args.k, Some(120.0));
        assert_eq!(args.epochs, 10);
        assert_eq!(args.lr, 2.0);
        assert_eq!(args.threads, 4);
        assert_eq!(args.output_path.as_deref(), Some("o"));
    }

    #[test]
    fn errors() {
        let tests = [
            "",
            "--data d",
            "--game ataxx",
            "--data --game ataxx",
            "--data d --game",
            "--data d --game unknown",
            "--data d --game ataxx --eval-weight 1.5",
            "--data d --game ataxx --eval-weight -0.1",
            "--data d --game ataxx --eval-weight x",
            "--data d --game ataxx --k 0",
            "--data d --game ataxx --lr 0",
            "--data d --game ataxx --epochs 0",
            "--data d --game ataxx --threads 0",
            "--data d --game ataxx extra",
        ];

        for line in tests {
            assert!(parse_string(line).is_err(), "{}", line);
        }
    }
}
