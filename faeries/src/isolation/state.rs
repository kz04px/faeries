use games::{gamerules::GameRules, isolation::IsolationPosition};
use ugi::options::{Check, Combo};

pub struct IsolationState {
    pub pos: IsolationPosition,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for IsolationState {
    fn default() -> Self {
        Self {
            pos: IsolationPosition::startpos(),
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
