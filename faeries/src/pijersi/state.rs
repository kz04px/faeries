use crate::prng;
use games::{gamerules::GameRules, pijersi::PijersiPosition};
use std::u64;
use protocols::ugi::options::{Check, Combo, Spin};

pub struct PijersiState {
    pub pos: PijersiPosition,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub search: Combo,
    pub seed: Spin<u64>,
}

impl Default for PijersiState {
    fn default() -> Self {
        Self {
            pos: PijersiPosition::startpos(),
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
            seed: Spin::<u64> {
                name: "seed".to_owned(),
                min: u64::MIN,
                max: u64::MAX,
                value: 0xe50076937a9e5b1c,
            },
        }
    }
}
