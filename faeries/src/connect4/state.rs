use games::{connect4::Connect4Position, gamerules::GameRules};
use ugi::options::{Check, Combo};

pub struct Connect4State {
    pub pos: Connect4Position,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for Connect4State {
    fn default() -> Self {
        Self {
            pos: Connect4Position::startpos(),
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