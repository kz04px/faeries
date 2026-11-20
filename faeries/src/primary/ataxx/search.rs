use super::{AtaxxPrimary, eval::eval, order::MovePicker};
use crate::{
    primary::ataxx::{INF_SCORE, MATE_SCORE, MAX_PLY},
    searchstats::SearchStats,
};
use games::{
    ataxx::{AtaxxMove, AtaxxPosition},
    gamerules::{GameResult, GameRules},
};
use std::cmp::max;

const NULL_MOVE_MIN_DEPTH: i32 = 3;
const NULL_MOVE_REDUCTION: i32 = 2;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum Bound {
    #[default]
    Exact,
    Lower,
    Upper,
}

#[derive(Clone, Copy, Default, PartialEq)]
pub struct TTEntry {
    pub key: u64,
    pub bestmove: Option<AtaxxMove<7, 7>>,
    pub score: i32,
    pub depth: u8,
    pub bound: Bound,
}

const MATE_THRESHOLD: i32 = MATE_SCORE - MAX_PLY as i32;

fn score_to_tt(score: i32, ply_from_root: i32) -> i32 {
    if score >= MATE_THRESHOLD {
        score + ply_from_root
    } else if score <= -MATE_THRESHOLD {
        score - ply_from_root
    } else {
        score
    }
}

fn score_from_tt(score: i32, ply_from_root: i32) -> i32 {
    if score >= MATE_THRESHOLD {
        score - ply_from_root
    } else if score <= -MATE_THRESHOLD {
        score + ply_from_root
    } else {
        score
    }
}

#[must_use]
pub fn alphabeta(
    pos: &mut AtaxxPosition<7, 7>,
    depth: i32,
    ply_from_root: i32,
    mut alpha: i32,
    beta: i32,
    should_stop: &impl Fn(&SearchStats) -> bool,
    stats: &mut SearchStats,
    state: &mut AtaxxPrimary,
    pv: &mut Vec<AtaxxMove<7, 7>>,
    allow_null: bool,
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

    if depth <= 0 {
        return eval(pos);
    }

    let key = pos.get_hash();
    let tt_entry = state.tt.get(key).filter(|entry| entry.key == key);

    // TT bounds pruning
    if ply_from_root > 0
        && let Some(entry) = tt_entry
        && i32::from(entry.depth) >= depth
    {
        let score = score_from_tt(entry.score, ply_from_root);
        match entry.bound {
            Bound::Exact => return score,
            Bound::Lower if score >= beta => return score,
            Bound::Upper if score <= alpha => return score,
            _ => {}
        }
    }

    let original_alpha = alpha;
    let mut best_score = -INF_SCORE;
    let mut bestmove = None;

    let static_eval = eval(pos);

    // Null move pruning
    if allow_null
        && depth >= NULL_MOVE_MIN_DEPTH
        && beta.abs() < MATE_THRESHOLD
        && pos.get_empty().count() > 4
        && static_eval >= beta
    {
        pos.makenull();
        let score = -alphabeta(
            pos,
            depth - 1 - NULL_MOVE_REDUCTION,
            ply_from_root + 1,
            -beta,
            -beta + 1,
            should_stop,
            stats,
            state,
            &mut vec![],
            false,
        );
        pos.undonull();

        if score >= beta {
            return score;
        }
    }

    let picker = MovePicker::new(
        pos,
        tt_entry.and_then(|entry| entry.bestmove),
        state.killers[ply_from_root as usize],
    );

    for mv in picker {
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
            state,
            &mut next_pv,
            true,
        );
        pos.undomove(&mv);

        if score > best_score {
            best_score = score;
            bestmove = Some(mv);

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
            state.killers[ply_from_root as usize] = Some(mv);
            break;
        }
    }

    debug_assert!(best_score > -INF_SCORE);
    debug_assert!(bestmove.is_some());

    // Don't store entries in the TT if the search is over
    if should_stop(stats) {
        return best_score;
    }

    let bound = if best_score >= beta {
        Bound::Lower
    } else if best_score <= original_alpha {
        Bound::Upper
    } else {
        Bound::Exact
    };

    state.tt.set(
        key,
        TTEntry {
            key,
            bestmove,
            score: score_to_tt(best_score, ply_from_root),
            depth: depth as u8,
            bound,
        },
    );

    best_score
}
