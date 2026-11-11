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

// The result of a finished game from the side to move's view, from 0 (lost) to 1 (won).
// None while the game is still going.
#[must_use]
pub fn terminal(pos: &ChessPosition) -> Option<f64> {
    let mut moves = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
    if pos.legal_moves_into(&mut moves) == 0 {
        return Some(match pos.get_result() {
            Some(GameResult::Win(side)) if side == pos.get_turn() => 1.0,
            Some(GameResult::Win(_)) => 0.0,
            _ => 0.5,
        });
    }
    if pos.is_fifty_move() || pos.is_threefold() {
        return Some(0.5);
    }
    None
}

// Material balance in centipawns, from the side to move's view.
#[must_use]
pub fn material(pos: &ChessPosition) -> i32 {
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
    for (piece, bb) in pieces.iter().enumerate() {
        score += PIECE_VALUES[piece] * (*bb & board.get_us()).count();
        score -= PIECE_VALUES[piece] * (*bb & board.get_them()).count();
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn material_of(fen: &str) -> i32 {
        material(&ChessPosition::from_fen(fen))
    }

    fn terminal_of(fen: &str) -> Option<f64> {
        terminal(&ChessPosition::from_fen(fen))
    }

    #[test]
    fn material_is_counted_from_the_side_to_move() {
        let up_a_queen = "4k3/8/8/8/8/8/8/3QK3";
        assert_eq!(material_of(&format!("{up_a_queen} w - - 0 1")), 900);
        assert_eq!(material_of(&format!("{up_a_queen} b - - 0 1")), -900);

        // Rook and knight against bishop and two pawns
        assert_eq!(
            material_of("4k3/pp6/8/8/8/8/8/1N2KB1R w - - 0 1"),
            500 + 300 + 300 - 200
        );
        assert_eq!(
            material_of("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"),
            0
        );
    }

    #[test]
    fn sigmoid_maps_material_to_a_win_probability() {
        assert!((sigmoid(0.0) - 0.5).abs() < 1e-12);
        assert!(sigmoid(900.0 / EVAL_SCALE) > 0.9);
        assert!((sigmoid(2.0) + sigmoid(-2.0) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn finished_games_have_a_result() {
        // Checkmate against the side to move, stalemate, and a fifty-move draw
        assert_eq!(terminal_of("R5k1/8/6K1/8/8/8/8/8 b - - 0 1"), Some(0.0));
        assert_eq!(terminal_of("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1"), Some(0.5));
        assert_eq!(terminal_of("4k3/8/8/8/8/8/8/R3K3 w - - 100 80"), Some(0.5));
        assert_eq!(terminal_of("4k3/8/8/8/8/8/8/R3K3 w - - 0 1"), None);
    }
}
