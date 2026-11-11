use super::cache::{Edge, Entry};
use super::eval::PIECE_VALUES;
use games::chess::{ChessMove, ChessPiece, ChessPosition};
use games::gamerules::GameRules;

// Weight of the prior-guided exploration term
pub const EXPLORATION: f64 = 0.5;
// How far below its parent's average an unvisited move is assumed to score
pub const FPU_REDUCTION: f64 = 0.0;
// Centipawns of move bonus that multiply a move's prior weight by e
pub const PRIOR_SCALE: f32 = 300.0;

// The piece a move captures, counting en passant.
#[must_use]
pub fn captured(pos: &ChessPosition, mv: &ChessMove) -> Option<ChessPiece> {
    let board = &pos.board;
    match (board.piece_on_sq(mv.to), board.piece_on_sq(mv.from)) {
        (Some(piece), _) => Some(piece),
        // A pawn changing file onto an empty square is en passant
        (None, Some(ChessPiece::Pawn)) if mv.from.get_file() != mv.to.get_file() => {
            Some(ChessPiece::Pawn)
        }
        _ => None,
    }
}

// A rough guess, in centipawns, at how good a move is: captures of valuable pieces
// by cheap ones and promotions score highest, quiet moves score zero.
#[must_use]
pub fn move_bonus(pos: &ChessPosition, mv: &ChessMove) -> i32 {
    let mut bonus = 0;
    if let (Some(victim), Some(mover)) = (captured(pos, mv), pos.board.piece_on_sq(mv.from)) {
        bonus += PIECE_VALUES[victim as usize] - PIECE_VALUES[mover as usize] / 10;
    }
    if let Some(promo) = mv.promo {
        bonus += PIECE_VALUES[promo as usize] - PIECE_VALUES[ChessPiece::Pawn as usize];
    }
    bonus
}

#[must_use]
pub fn new_entry(pos: &ChessPosition) -> Entry {
    let moves = pos.legal_moves();
    let weights: Vec<f32> = moves
        .iter()
        .map(|mv| (move_bonus(pos, mv) as f32 / PRIOR_SCALE).exp())
        .collect();
    let total: f32 = weights.iter().sum();

    Entry {
        edges: moves
            .into_iter()
            .zip(weights)
            .map(|(mv, weight)| Edge {
                mv,
                prior: weight / total,
                visits: 0,
                score: 0.0,
            })
            .collect(),
        visits: 0,
    }
}

// Picks the move with the highest PUCT value: its average result plus an exploration
// bonus that is larger for moves with a high prior and few visits.
#[must_use]
pub fn select_edge(entry: &Entry) -> usize {
    let (visits, score) = entry.edges.iter().fold((0u64, 0.0), |(v, s), e| {
        (v + u64::from(e.visits), s + e.score)
    });
    let parent_mean = if visits > 0 {
        score / visits as f64
    } else {
        0.5
    };
    let unvisited = (parent_mean - FPU_REDUCTION).max(0.0);
    let sqrt_parent = (entry.visits.max(1) as f64).sqrt();

    let mut best = 0;
    let mut best_value = f64::NEG_INFINITY;
    for (i, edge) in entry.edges.iter().enumerate() {
        let n = f64::from(edge.visits);
        let mean = if edge.visits == 0 {
            unvisited
        } else {
            edge.score / n
        };
        let value = mean + EXPLORATION * f64::from(edge.prior) * sqrt_parent / (1.0 + n);
        if value > best_value {
            best_value = value;
            best = i;
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priors_form_a_distribution() {
        let pos = ChessPosition::startpos();
        let entry = new_entry(&pos);
        assert_eq!(entry.edges.len(), 20);
        let total: f32 = entry.edges.iter().map(|e| e.prior).sum();
        assert!((total - 1.0).abs() < 1e-4, "{total}");
        assert!(entry.edges.iter().all(|e| e.prior > 0.0));
    }

    #[test]
    fn captures_and_promotions_have_higher_priors_than_quiet_moves() {
        let prior = |fen: &str, uci: &str| {
            let pos = ChessPosition::from_fen(fen);
            let mv = pos.parse_movestr(uci).unwrap();
            let entry = new_entry(&pos);
            entry.edges.iter().find(|e| e.mv == mv).unwrap().prior
        };
        let fen = "4k3/8/8/3q4/8/8/8/3RK3 w - - 0 1";
        assert!(prior(fen, "d1d5") > prior(fen, "d1d2"));

        let fen = "4k3/P7/8/8/8/8/8/4K3 w - - 0 1";
        assert!(prior(fen, "a7a8q") > prior(fen, "a7a8n"));
        assert!(prior(fen, "a7a8n") > prior(fen, "e1e2"));

        // En passant counts as a capture
        let fen = "4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1";
        assert!(prior(fen, "e5d6") > prior(fen, "e5e6"));
    }

    #[test]
    fn an_unvisited_search_tries_the_highest_prior_move_first() {
        let pos = ChessPosition::from_fen("4k3/8/8/3q4/8/8/8/3RK3 w - - 0 1");
        let entry = new_entry(&pos);
        let best = &entry.edges[select_edge(&entry)];
        assert_eq!(best.mv, pos.parse_movestr("d1d5").unwrap());
    }

    #[test]
    fn visits_shift_selection_away_from_a_well_explored_move() {
        let pos = ChessPosition::startpos();
        let mut entry = new_entry(&pos);
        let first = select_edge(&entry);
        entry.edges[first].visits = 1000;
        entry.edges[first].score = 0.0;
        entry.visits = 1000;
        assert_ne!(select_edge(&entry), first);
    }
}
