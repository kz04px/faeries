use games::{ataxx::AtaxxPosition, gamerules::GameRules};
use ugi::options::options;

pub struct AtaxxState {
    pub pos: AtaxxPosition,
    // Options
    pub debug: options::Check,
    pub search: options::Combo,
}

impl Default for AtaxxState {
    fn default() -> Self {
        Self {
            pos: AtaxxPosition::from_fen("startpos"),
            // Options
            debug: options::Check {
                name: "debug".to_string(),
                value: cfg!(debug_assertions),
            },
            search: options::Combo {
                name: "search".to_string(),
                value: "primary".to_string(),
                options: vec![
                    "primary".to_string(),
                    "random".to_string(),
                    "minimax".to_string(),
                    "alphabeta".to_string(),
                    "flatmc".to_string(),
                ],
            },
        }
    }
}
