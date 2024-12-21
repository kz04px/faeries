use games::{droptaxx::DroptaxxPosition, gamerules::GameRules};
use ugi::options::options;

pub struct DroptaxxState {
    pub pos: DroptaxxPosition,
    // Options
    pub debug: options::Check,
    pub search: options::Combo,
}

impl Default for DroptaxxState {
    fn default() -> Self {
        Self {
            pos: DroptaxxPosition::from_fen("startpos"),
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
