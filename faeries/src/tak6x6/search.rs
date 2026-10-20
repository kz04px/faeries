use super::{TakState, eval::eval, hash::hash};
use crate::{
    searchstats::SearchStats,
    tak6x6::{INF_SCORE, MATE_SCORE},
};
use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
    tak::{TakMove, TakPosition},
};
use std::cmp::max;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct TTEntry {
    pub key: u64,
    pub bestmove: Option<TakMove<6>>,
}

/// Stones placed so far; each side's first stone is placed as the opponent's.
#[must_use]
fn stones_played(pos: &TakPosition<6>) -> i32 {
    62 - pos.num_pieces[Side::Player1]
        - pos.num_pieces[Side::Player2]
        - pos.num_caps[Side::Player1]
        - pos.num_caps[Side::Player2]
}

#[must_use]
pub fn alphabeta(
    pos: &mut TakPosition<6>,
    depth: i32,
    ply_from_root: i32,
    mut alpha: i32,
    beta: i32,
    should_stop: &impl Fn(&SearchStats) -> bool,
    stats: &mut SearchStats,
    state: &mut TakState,
    pv: &mut Vec<TakMove<6>>,
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

    if depth == 0 {
        return eval(pos);
    }

    // Null move pruning
    if allow_null && depth >= 3 && ply_from_root > 0 && stones_played(pos) >= 2 && eval(pos) >= beta
    {
        pos.makenull();
        let score = -alphabeta(
            pos,
            depth - 1 - 2,
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
            return beta;
        }
    }

    let key = hash(pos);
    let mut moves = pos.legal_moves();

    // Search the hash move first
    if let Some(entry) = state.tt.get(key)
        && entry.key == key
        && let Some(ttmove) = entry.bestmove
        && let Some(idx) = moves.iter().position(|&mv| mv == ttmove)
    {
        moves[..=idx].rotate_right(1);
    }

    let mut best_score = -INF_SCORE;
    let mut bestmove = None;

    for mv in moves {
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
            break;
        }
    }

    debug_assert!(best_score > -INF_SCORE);
    debug_assert!(bestmove.is_some());

    state.tt.set(key, TTEntry { key, bestmove });

    best_score
}
