use games::gamerules::{GameResult, GameRules};

pub enum Event<G: GameRules> {
    ThreadStart {
        thread_id: usize,
    },
    ThreadFinish {
        thread_id: usize,
    },
    GameStart {
        thread_id: usize,
        id: usize,
        fen: String,
    },
    GameFinish {
        thread_id: usize,
        id: usize,
        result: Option<GameResult>,
    },
    Move {
        thread_id: usize,
        id: usize,
        mv: G::MoveType,
    },
}
