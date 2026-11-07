use super::eval::eval;
use crate::{
    primary::ataxx::{INF_SCORE, MATE_SCORE},
    searchstats::SearchStats,
};
use games::{
    ataxx::{AtaxxMove, AtaxxPosition},
    gamerules::{GameResult, GameRules},
};
use std::cmp::max;

#[must_use]
pub fn negamax(
    pos: &mut AtaxxPosition<7, 7>,
    depth: i32,
    ply_from_root: i32,
    stats: &mut SearchStats,
    pv: &mut Vec<AtaxxMove<7, 7>>,
) -> i32 {
    debug_assert!(ply_from_root >= 0);

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
        let score = -negamax(pos, depth - 1, ply_from_root + 1, stats, &mut next_pv);
        pos.undomove(&mv);

        if score > best_score {
            best_score = score;

            // Update PV
            *pv = vec![mv];
            for trash in next_pv {
                pv.push(trash);
            }
        }
    }

    debug_assert!(best_score > -INF_SCORE);

    best_score
}
