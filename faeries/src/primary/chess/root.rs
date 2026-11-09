use super::search::rollout;
use crate::prng::XorshiftGenerator;
use crate::search::SearchInfo;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::{GameResult, GameRules};
use games::general::side::Side;
use protocols::GoSettings;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

#[must_use]
pub fn primary(
    pos: ChessPosition,
    settings: &GoSettings,
    info_handler: &impl Fn(&ChessPosition, &SearchInfo<ChessMove>),
) -> Option<ChessMove> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut prng = XorshiftGenerator::new(seed | 1);

    let moves = pos.legal_moves();
    if moves.is_empty() {
        return None;
    }

    let us = pos.get_turn();
    let num_moves = moves.len();
    // Scores are in half-points: a win is 2, a draw is 1, a loss is 0.
    let mut scores = vec![0u32; num_moves];
    let mut games = vec![0u32; num_moves];
    let mut total_games: u64 = 0;

    let start = Instant::now();
    let should_stop = |total: u64| -> bool {
        if let Some(nodes) = settings.nodes {
            return total >= nodes;
        }
        if let Some(movetime) = settings.movetime {
            return start.elapsed().as_millis() >= movetime as u128;
        }
        let our_time = if us == Side::Player1 {
            settings.p1time
        } else {
            settings.p2time
        };
        if let Some(time) = our_time {
            return start.elapsed().as_millis() >= (time / 30) as u128;
        }
        false
    };

    let mut idx = 0usize;
    loop {
        if should_stop(total_games) {
            break;
        }

        let mv = moves[idx % num_moves];
        let mut copy = pos.clone();
        copy.makemove(&mv);

        let result = rollout(&mut copy, &mut prng);
        games[idx % num_moves] += 1;
        scores[idx % num_moves] += match result {
            Some(GameResult::Win(side)) if side == us => 2,
            Some(GameResult::Win(_)) => 0,
            _ => 1,
        };

        idx = idx.wrapping_add(1);
        total_games += 1;
    }

    info_handler(
        &pos,
        &SearchInfo {
            nodes: Some(total_games),
            elapsed: Some(start.elapsed().as_millis()),
            ..Default::default()
        },
    );

    // Pick the move with the highest score rate; break ties by most games played
    moves
        .iter()
        .enumerate()
        .max_by_key(|(i, _)| {
            let g = games[*i].max(1);
            (scores[*i] * 1000 / g, games[*i])
        })
        .map(|(_, mv)| *mv)
}
