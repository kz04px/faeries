use games::Games;
use std::{env, fmt::Display};

pub struct Args {
    pub data_path: String,
    /// Positions the fit doesn't train on, to report the loss on.
    pub test_path: Option<String>,
    pub game: Games,
    /// How much of the target is the search score rather than the game result, from 0 to 1.
    pub eval_weight: f64,
    /// The sigmoid scale in centipawns. Fitted to the game results when not given.
    pub k: Option<f64>,
    pub epochs: usize,
    /// Adam's step in parameter units.
    pub lr: f64,
    pub threads: usize,
    /// Start every value that isn't held at zero instead of at its default.
    pub zero_init: bool,
    /// Positions whose search score is further from zero than this are left out.
    pub max_score: Option<f64>,
    /// Tunables left at their defaults, by name: `hole_penalties` holds every element of it.
    pub hold: Vec<String>,
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
    let mut test_path = None;
    let mut game = None;
    let mut eval_weight = 0.5;
    let mut k = None;
    let mut epochs = 500;
    let mut lr = 1.0;
    let mut threads = 1;
    let mut output_path = None;
    let mut hold = Vec::new();
    let mut zero_init = false;
    let mut max_score = None;
    let mut iter = words.iter();

    while let Some(word) = iter.next() {
        match *word {
            "--data" => data_path = value(&mut iter, word)?,
            "--test" => test_path = Some(value(&mut iter, word)?),
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
            "--zero-init" => zero_init = true,
            "--max-score" => max_score = Some(value(&mut iter, word)?),
            "--hold" => {
                let names: String = value(&mut iter, word)?;
                hold = names.split(',').map(str::to_owned).collect();
                if hold.iter().any(|name| name.is_empty()) {
                    return error("hold takes names separated by commas");
                }
            }
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
    if max_score.is_some_and(|m| m <= 0.0 || !f64::is_finite(m)) {
        return error("max-score must be > 0");
    }
    if threads < 1 {
        return error("threads must be >= 1");
    }

    Ok(Args {
        data_path,
        test_path,
        game,
        eval_weight,
        k,
        epochs,
        lr,
        threads,
        zero_init,
        max_score,
        hold,
        output_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults() {
        let args = parse_string("--data d --game ataxx").unwrap();
        assert_eq!(args.test_path, None);
        assert_eq!(args.eval_weight, 0.5);
        assert_eq!(args.k, None);
        assert!(!args.zero_init);
        assert_eq!(args.threads, 1);
    }

    #[test]
    fn example() {
        let args = parse_string(
            "--data d --test t --game ataxx --eval-weight 0.25 --k 120 --epochs 10 --lr 2 --threads 4 --hold a,b_c --max-score 1000 --zero-init --output o",
        )
        .unwrap();
        assert_eq!(args.game, Games::Ataxx);
        assert_eq!(args.test_path.as_deref(), Some("t"));
        assert_eq!(args.eval_weight, 0.25);
        assert_eq!(args.k, Some(120.0));
        assert_eq!(args.epochs, 10);
        assert_eq!(args.lr, 2.0);
        assert_eq!(args.threads, 4);
        assert_eq!(args.hold, ["a", "b_c"]);
        assert!(args.zero_init);
        assert_eq!(args.max_score, Some(1000.0));
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
            "--data d --game ataxx --test",
            "--data d --game ataxx --hold",
            "--data d --game ataxx --max-score",
            "--data d --game ataxx --max-score 0",
            "--data d --game ataxx --hold a,,b",
        ];

        for line in tests {
            assert!(parse_string(line).is_err(), "{}", line);
        }
    }
}
