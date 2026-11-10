use games::chess::{ChessMove, ChessPosition};
use games::gamerules::{GameResult, GameRules};

// Centipawns of material advantage that map to a value of sigmoid(1)
pub const EVAL_SCALE: f64 = 400.0;
// Centipawns for each piece, in the order pawn, knight, bishop, rook, queen, king
pub const PIECE_VALUES: [i32; 6] = [100, 300, 300, 500, 900, 0];

#[must_use]
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// How good the position is for the side to move, from 0 (lost) to 1 (won).
// Finished games score by their result, anything else by material.
#[must_use]
pub fn evaluate(pos: &ChessPosition) -> f64 {
    let mut moves = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
    if pos.legal_moves_into(&mut moves) == 0 {
        return match pos.get_result() {
            Some(GameResult::Win(side)) if side == pos.get_turn() => 1.0,
            Some(GameResult::Win(_)) => 0.0,
            _ => 0.5,
        };
    }
    if pos.is_fifty_move() || pos.is_threefold() {
        return 0.5;
    }

    let board = &pos.board;
    let pieces = [
        board.get_pawns(),
        board.get_knights(),
        board.get_bishops(),
        board.get_rooks(),
        board.get_queens(),
        board.get_kings(),
    ];
    let mut score = 0;

    // Material - us
    for (piece, bb) in pieces.iter().enumerate() {
        score += PIECE_VALUES[piece] * (*bb & board.get_us()).count();
    }
    // Material - them
    for (piece, bb) in pieces.iter().enumerate() {
        score -= PIECE_VALUES[piece] * (*bb & board.get_them()).count();
    }

    sigmoid(f64::from(score) / EVAL_SCALE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn value(fen: &str) -> f64 {
        evaluate(&ChessPosition::from_fen(fen))
    }

    #[test]
    fn material_is_counted_from_the_side_to_move() {
        let up_a_queen = "4k3/8/8/8/8/8/8/3QK3";
        let expected = sigmoid(900.0 / EVAL_SCALE);
        assert!((value(&format!("{up_a_queen} w - - 0 1")) - expected).abs() < 1e-12);
        assert!((value(&format!("{up_a_queen} b - - 0 1")) - (1.0 - expected)).abs() < 1e-12);

        // Rook and knight against bishop and two pawns
        let expected = sigmoid((500.0 + 300.0 + 300.0 - 200.0) / EVAL_SCALE);
        assert!((value("4k3/pp6/8/8/8/8/8/1N2KB1R w - - 0 1") - expected).abs() < 1e-12);
    }

    #[test]
    fn evaluation_follows_material_and_results() {
        let even = value("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        assert!((even - 0.5).abs() < 1e-12);

        let ahead = value("4k3/8/8/8/8/8/8/3QK3 w - - 0 1");
        let behind = value("4k3/8/8/8/8/8/8/3QK3 b - - 0 1");
        assert!(ahead > 0.9, "{ahead}");
        assert!((ahead + behind - 1.0).abs() < 1e-12);

        // Checkmate against the side to move, stalemate, and a fifty-move draw
        assert_eq!(value("R5k1/8/6K1/8/8/8/8/8 b - - 0 1"), 0.0);
        assert_eq!(value("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1"), 0.5);
        assert_eq!(value("4k3/8/8/8/8/8/8/R3K3 w - - 100 80"), 0.5);
    }
}
