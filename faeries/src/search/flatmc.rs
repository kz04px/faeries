use games::gamerules::{GameResult, GameRules};
use rand::seq::SliceRandom;
use std::time::Instant;
use ugi::go::GoSettings;

#[must_use]
fn rollout<G: GameRules>(pos: G) -> G {
    let mut pos = pos;
    while !pos.is_gameover() {
        let moves = pos.legal_moves();
        let bestmove = moves.choose(&mut rand::thread_rng());
        if let Some(mv) = bestmove {
            pos.makemove(&mv);
        }
    }
    pos
}

#[must_use]
fn find_idx(scores: &[f32], visits: &[u64]) -> Option<usize> {
    let mut best_ratio = -1.0;
    let mut best_idx = None;
    for i in 0..scores.len() {
        let ratio = if visits[i] > 0 {
            scores[i] as f32 / visits[i] as f32
        } else {
            0.0
        };

        if ratio > best_ratio {
            best_ratio = ratio;
            best_idx = Some(i);
        }
    }
    best_idx
}

#[must_use]
pub fn flatmc<G: GameRules>(
    pos: &G,
    settings: &GoSettings,
    info_handler: impl Fn(
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<u64>,
        Option<u128>,
        Option<i32>,
        &Vec<G::MoveType>,
    ),
) -> Option<G::MoveType> {
    let start = Instant::now();
    let moves = pos.legal_moves();
    let mut scores = vec![0.0; moves.len()];
    let mut visits = vec![0; moves.len()];
    let mut iterations = 0;
    let should_stop = |iterations| -> bool {
        if let Some(n) = settings.nodes {
            return iterations >= n;
        }

        if let Some(dt) = settings.movetime {
            return start.elapsed().as_millis() >= dt as u128;
        }

        false
    };

    while !should_stop(iterations) {
        let idx = iterations as usize % moves.len();
        let npos = pos.after_move(&moves[idx]);
        let tpos = rollout(npos);
        iterations += 1;

        // Score
        let score = match tpos.get_result() {
            Some(GameResult::Win(s)) => {
                if s == pos.get_turn() {
                    1.0
                } else {
                    0.0
                }
            }
            Some(GameResult::Draw) => 0.5,
            None => panic!("uh oh"),
        };

        scores[idx] += score;
        visits[idx] += 1;

        // Print update
        let should_update =
            (iterations <= 10_000 && iterations % 1_000 == 0) || (iterations % 10_000 == 0);
        if should_update {
            let best_idx = find_idx(&scores, &visits);
            info_handler(
                None,
                None,
                None,
                None,
                Some(iterations),
                Some(start.elapsed().as_millis()),
                None,
                &vec![moves[best_idx.unwrap()]],
            );
        }
    }

    let best_idx = find_idx(&scores, &visits);
    if let Some(idx) = best_idx {
        Some(moves[idx])
    } else {
        None
    }
}
