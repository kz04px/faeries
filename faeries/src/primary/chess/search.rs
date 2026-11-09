use crate::prng::XorshiftGenerator;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::{GameResult, GameRules};

pub const MAX_ROLLOUT_PLIES: i32 = 400;

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
