use super::cache::{Edge, Entry};
use games::chess::ChessPosition;
use games::gamerules::GameRules;

pub const EXPLORATION: f64 = 0.1;

#[must_use]
pub fn new_entry(pos: &ChessPosition) -> Entry {
    Entry {
        edges: pos
            .legal_moves()
            .into_iter()
            .map(|mv| Edge {
                mv,
                visits: 0,
                score: 0.0,
            })
            .collect(),
        visits: 0,
    }
}

// Picks the move with the highest UCB1 value, trying unvisited moves first.
#[must_use]
pub fn select_edge(entry: &Entry) -> usize {
    let ln_parent = (entry.visits.max(1) as f64).ln();
    let mut best = 0;
    let mut best_value = f64::NEG_INFINITY;
    for (i, edge) in entry.edges.iter().enumerate() {
        if edge.visits == 0 {
            return i;
        }
        let n = edge.visits as f64;
        let value = edge.score / n + EXPLORATION * (ln_parent / n).sqrt();
        if value > best_value {
            best_value = value;
            best = i;
        }
    }
    best
}
