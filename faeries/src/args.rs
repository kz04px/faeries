use std::env;

pub struct Args {
    pub game: String,
}

impl Default for Args {
    fn default() -> Self {
        Self {
            game: String::default(),
        }
    }
}

#[must_use]
pub fn parse_args() -> Args {
    let args: Vec<String> = env::args().collect();
    let mut parsed = Args::default();

    let mut i = 0;
    while i < args.len() {
        // Pairs
        if i + 1 < args.len() {
            match (args[i].as_str(), args[i + 1].as_str()) {
                ("--game", value) => {
                    parsed.game = value.to_owned();
                }
                _ => {}
            }
        }

        // Singles

        i += 1;
    }

    parsed
}
