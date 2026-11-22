use games::gamerules::GameResult;

/// A position the search scored during a game.
pub struct Sample {
    pub fen: String,
    /// Centipawns from Player 1's point of view.
    pub score: i32,
}

pub enum Event {
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
        samples: Vec<Sample>,
    },
}
