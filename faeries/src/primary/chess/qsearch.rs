use super::eval::{material, sigmoid, terminal, EVAL_SCALE};
use super::search::{captured, move_bonus};
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::GameRules;

// Captures deeper than this are not followed
const MAX_DEPTH: u32 = 32;
const INFINITY: i32 = 1_000_000;

// How good the position is for the side to move, from 0 (lost) to 1 (won). Finished
// games score by their result, anything else by material once the captures
// available to both sides have been played out.
#[must_use]
pub fn score(pos: &ChessPosition) -> f64 {
    terminal(pos).unwrap_or_else(|| {
        let cp = quiesce(pos, -INFINITY, INFINITY, 0);
        sigmoid(f64::from(cp) / EVAL_SCALE)
    })
}

// Material in centipawns for the side to move after captures and promotions have
// been played out. Each side may stop capturing whenever that is better for it.
fn quiesce(pos: &ChessPosition, mut alpha: i32, beta: i32, depth: u32) -> i32 {
    let stand_pat = material(pos);
    if depth >= MAX_DEPTH || stand_pat >= beta {
        return stand_pat.min(beta);
    }
    alpha = alpha.max(stand_pat);

    let mut moves = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
    let count = pos.legal_moves_into(&mut moves);
    let mut tactical: Vec<(i32, ChessMove)> = moves[..count]
        .iter()
        .filter(|mv| mv.promo.is_some() || captured(pos, mv).is_some())
        .map(|mv| (move_bonus(pos, mv), *mv))
        .collect();
    tactical.sort_unstable_by_key(|&(bonus, _)| -bonus);

    for (_, mv) in tactical {
        let mut child = pos.clone();
        child.makemove(&mv);
        let value = -quiesce(&child, -beta, -alpha, depth + 1);
        if value >= beta {
            return beta;
        }
        alpha = alpha.max(value);
    }
    alpha
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pos(fen: &str) -> ChessPosition {
        ChessPosition::from_fen(fen)
    }

    #[test]
    fn quiet_positions_score_by_material() {
        let startpos = ChessPosition::startpos();
        assert!((score(&startpos) - 0.5).abs() < 1e-12);
        let up_a_queen = pos("4k3/8/8/8/8/8/8/3QK3 w - - 0 1");
        assert!((score(&up_a_queen) - sigmoid(900.0 / EVAL_SCALE)).abs() < 1e-12);
    }

    #[test]
    fn a_pending_recapture_is_played_out() {
        // The queen on c5 is en prise to the pawn, so black is not really behind
        let hanging = pos("4k3/8/3p4/2Q5/8/8/8/4K3 b - - 0 1");
        assert!(sigmoid(f64::from(material(&hanging)) / EVAL_SCALE) < 0.2);
        let expected = sigmoid(100.0 / EVAL_SCALE);
        assert!(
            (score(&hanging) - expected).abs() < 1e-12,
            "{}",
            score(&hanging)
        );
    }

    #[test]
    fn a_defended_pawn_is_not_worth_taking() {
        // Taking the pawn on c5 loses the queen to the pawn on d6, so white stays put
        let fen = "4k3/8/3p4/2p5/8/8/8/2Q1K3 w - - 0 1";
        let expected = sigmoid(700.0 / EVAL_SCALE);
        assert!((score(&pos(fen)) - expected).abs() < 1e-12);
    }

    #[test]
    fn an_undefended_piece_can_be_taken() {
        // Material is +300 now and +800 once the queen takes the rook
        let fen = "4k3/8/8/r2p4/8/8/8/Q3K3 w - - 0 1";
        let expected = sigmoid(800.0 / EVAL_SCALE);
        assert!((score(&pos(fen)) - expected).abs() < 1e-12);
    }

    #[test]
    fn finished_games_score_by_their_result() {
        assert_eq!(score(&pos("R5k1/8/6K1/8/8/8/8/8 b - - 0 1")), 0.0);
        assert_eq!(score(&pos("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1")), 0.5);
        assert_eq!(score(&pos("4k3/8/8/8/8/8/8/R3K3 w - - 100 80")), 0.5);
    }
}
