use crate::prng::XorshiftGenerator;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::{GameResult, GameRules};

pub const MAX_ROLLOUT_PLIES: i32 = 400;

// Picks the move with the highest score rate, trying each move once first.
// Scores are in half-points: a win is 2, a draw is 1, a loss is 0.
#[must_use]
pub fn select_best(scores: &[u32], games: &[u32]) -> usize {
    let mut best = 0;
    let mut best_rate = f64::NEG_INFINITY;
    for i in 0..games.len() {
        if games[i] == 0 {
            return i;
        }
        let rate = scores[i] as f64 / (2.0 * games[i] as f64);
        if rate > best_rate {
            best_rate = rate;
            best = i;
        }
    }
    best
}

#[must_use]
pub fn rollout(pos: &mut ChessPosition, prng: &mut XorshiftGenerator) -> Option<GameResult> {
    let mut moves = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
    for _ in 0..MAX_ROLLOUT_PLIES {
        let count = pos.legal_moves_into(&mut moves);
        if count == 0 {
            return pos.get_result();
        }
        if pos.is_fifty_move() || pos.is_threefold() {
            return Some(GameResult::Draw);
        }
        let mv = moves[prng.next() as usize % count];
        pos.makemove(&mv);
    }
    pos.get_result()
}
