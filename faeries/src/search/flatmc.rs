use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
};
use protocols::GoSettings;
use std::time::Instant;

#[must_use]
fn find_idx(scores: &[f32], visits: &[u64]) -> Option<usize> {
    let mut best_ratio = -1.0;
    let mut best_idx = None;
    for i in 0..scores.len() {
        let ratio = if visits[i] > 0 {
            scores[i] / visits[i] as f32
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
    mut pos: G,
    settings: &GoSettings,
    info_handler: &impl Fn(
        &G,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<u64>,
        Option<u128>,
        Option<i32>,
        &Vec<G::MoveType>,
    ),
    mut random_generator: impl FnMut() -> u64,
) -> Option<G::MoveType> {
    let start = Instant::now();
    let root_moves = pos.legal_moves();
    let mut scores = vec![0.0; root_moves.len()];
    let mut visits = vec![0; root_moves.len()];
    let mut iterations = 0;
    let us = pos.get_turn();
    let should_stop = |iterations| -> bool {
        if let Some(n) = settings.nodes {
            return iterations >= n;
        }

        if let Some(dt) = settings.movetime {
            return start.elapsed().as_millis() >= dt as u128;
        }

        let our_time = if us == Side::Player1 {
            settings.p1time
        } else {
            settings.p2time
        };

        if let Some(time) = our_time {
            let to_use = time / 30;
            return start.elapsed().as_millis() >= to_use as u128;
        }

        false
    };

    let us = pos.get_turn();
    loop {
        let mut history = vec![];

        // Cycle through legal moves
        let idx = iterations as usize % root_moves.len();
        pos.makemove(&root_moves[idx]);

        // Rollout
        while !pos.is_gameover() {
            let moves = pos.legal_moves();
            let move_idx = random_generator() as usize % moves.len();
            let bestmove = Some(moves[move_idx]);
            if let Some(mv) = bestmove {
                history.push(mv);
                pos.makemove(&mv);
            }
        }

        // Score
        let score = match pos.get_result() {
            Some(GameResult::Win(side)) => {
                if side == us {
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

        // Roll back
        for mv in history.into_iter().rev() {
            pos.undomove(&mv);
        }
        pos.undomove(&root_moves[idx]);

        // Finished
        iterations += 1;

        let is_last = should_stop(iterations);
        let should_update = (iterations % 10 == 0 && iterations <= 100)
            || (iterations % 100 == 0 && iterations <= 1_000)
            || (iterations % 1_000 == 0 && iterations <= 10_000)
            || (iterations % 10_000 == 0 && iterations <= 100_000)
            || (iterations % 100_000 == 0 && iterations <= 1_000_000)
            || (iterations % 1_000_000 == 0);

        // Print update
        if should_update || is_last {
            let best_idx = find_idx(&scores, &visits);
            info_handler(
                &pos,
                None,
                None,
                None,
                None,
                Some(iterations),
                Some(start.elapsed().as_millis()),
                None,
                &vec![root_moves[best_idx.unwrap()]],
            );
        }

        if is_last {
            break;
        }
    }

    let best_idx = find_idx(&scores, &visits);
    if let Some(idx) = best_idx {
        Some(root_moves[idx])
    } else {
        None
    }
}
