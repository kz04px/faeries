use super::eval::eval;
use crate::searchstats::SearchStats;
use games::{
    gamerules::{GameResult, GameRules},
    pijersi::{PijersiMove, PijersiPosition},
};
use std::cmp::max;

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;

#[must_use]
pub fn negamax(
    pos: &mut PijersiPosition,
    depth: i32,
    ply: i32,
    should_stop: &impl Fn(&SearchStats) -> bool,
    stats: &mut SearchStats,
    pv: &mut Vec<PijersiMove>,
) -> i32 {
    if should_stop(stats) {
        return 0;
    }

    stats.nodes += 1;
    stats.seldepth = max(stats.seldepth, ply);

    match pos.get_result() {
        Some(GameResult::Win(s)) => {
            if s == pos.get_turn() {
                return MATE_SCORE - ply;
            } else {
                return -MATE_SCORE + ply;
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
        let score = -negamax(pos, depth - 1, ply + 1, should_stop, stats, &mut next_pv);
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
