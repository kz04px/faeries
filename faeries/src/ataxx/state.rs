use crate::prng;
use games::{ataxx::AtaxxPosition, gamerules::GameRules};
use protocols::ugi::options::{Check, Combo};

pub struct AtaxxState {
    pub pos: AtaxxPosition,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for AtaxxState {
    fn default() -> Self {
        Self {
            pos: AtaxxPosition::startpos(),
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
