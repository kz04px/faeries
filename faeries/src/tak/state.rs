use crate::prng;
use games::{gamerules::GameRules, tak::TakPosition};
use protocols::ugi::options::{Check, Combo, Spin};

pub enum TakPositions {
    Size3(TakPosition<3>),
    Size4(TakPosition<4>),
    Size5(TakPosition<5>),
    Size6(TakPosition<6>),
    Size7(TakPosition<7>),
    Size8(TakPosition<8>),
}

pub struct TakState {
    pub pos: TakPositions,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub size: Spin<i32>,
    pub search: Combo,
}

impl Default for TakState {
    fn default() -> Self {
        Self {
            pos: TakPositions::Size6(TakPosition::startpos()),
            prng: prng::XorshiftGenerator::new(0xe50076937a9e5b1c),
            // Options
            debug: Check {
                name: "debug".to_owned(),
                value: cfg!(debug_assertions),
            },
            size: Spin {
                name: "size".to_owned(),
                min: 3,
                max: 8,
                value: 6,
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
