use crate::prng;
use games::{chess::ChessPosition, gamerules::GameRules};
use protocols::ugi::options::{Check, Combo};

pub struct ChessState {
    pub pos: ChessPosition,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for ChessState {
    fn default() -> Self {
        Self {
            pos: ChessPosition::startpos(),
            prng: prng::XorshiftGenerator::new(0xe50076937a9e5b1c),
            // Options
            debug: Check {
                name: "debug".to_owned(),
                value: cfg!(debug_assertions),
            },
            search: Combo {
                name: "search".to_owned(),
                value: "primary".to_owned(),
                options: vec![
                    "primary".to_owned(),
                    "random".to_owned(),
                    "minimax".to_owned(),
                    "alphabeta".to_owned(),
                    "flatmc".to_owned(),
                ],
            },
        }
    }
}
