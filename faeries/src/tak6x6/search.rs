use super::eval::eval;
use crate::{
    searchstats::SearchStats,
    tak6x6::{INF_SCORE, MATE_SCORE},
};
use games::{
    gamerules::{GameResult, GameRules},
    tak::{TakMove, TakPosition},
};
use std::cmp::max;

#[must_use]
pub fn alphabeta(
    pos: &mut TakPosition<6>,
    depth: i32,
    ply_from_root: i32,
    mut alpha: i32,
    beta: i32,
    should_stop: &impl Fn(&SearchStats) -> bool,
    stats: &mut SearchStats,
    pv: &mut Vec<TakMove<6>>,
) -> i32 {
    debug_assert!(-INF_SCORE <= alpha);
    debug_assert!(alpha < beta);
    debug_assert!(beta <= INF_SCORE);
    debug_assert!(ply_from_root >= 0);

    if should_stop(stats) {
        return 0;
    }

    stats.nodes += 1;
    stats.seldepth = max(stats.seldepth, ply_from_root);

    match pos.get_result() {
        Some(GameResult::Win(s)) => {
            if s == pos.get_turn() {
                return MATE_SCORE - ply_from_root;
            } else {
                return -MATE_SCORE + ply_from_root;
            }
        }
        Some(GameResult::Draw) => return 0,
        None => {}
    }

    if depth == 0 {
        return eval(pos);
    }

    let mut best_score = -INF_SCORE;

    for mv in pos.legal_moves() {
        pos.makemove(&mv);
        let mut next_pv = vec![];
        let score = -alphabeta(
            pos,
            depth - 1,
            ply_from_root + 1,
            -beta,
            -alpha,
            should_stop,
            stats,
            &mut next_pv,
        );
        pos.undomove(&mv);

        if score > best_score {
            best_score = score;

            // Update PV
            *pv = vec![mv];
            for trash in next_pv {
                pv.push(trash);
            }
        }

        if score > alpha {
            alpha = score;
        }

        if alpha >= beta {
            break;
        }
    }

    debug_assert!(best_score > -INF_SCORE);

    best_score
}
