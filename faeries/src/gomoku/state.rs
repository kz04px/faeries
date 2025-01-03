use games::{gomoku::GomokuPosition, gamerules::GameRules};
use ugi::options::{Check, Combo};

pub struct GomokuState {
    pub pos: GomokuPosition,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for GomokuState {
    fn default() -> Self {
        Self {
            pos: GomokuPosition::startpos(),
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
