use crate::searchstats::SearchStats;
use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
};
use std::{cmp::max, time::Instant};
use ugi::go::GoSettings;

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;

#[must_use]
fn alphabeta_impl<G: GameRules>(
    pos: &mut G,
    mut alpha: i32,
    beta: i32,
    depth: i32,
    ply: i32,
    stats: &mut SearchStats,
    pv: &mut Vec<G::MoveType>,
    should_stop: &impl Fn(&SearchStats) -> bool,
    eval: &impl Fn(&G) -> i32,
) -> i32 {
    debug_assert!(alpha < beta);
    debug_assert!(ply >= 0);

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
        let score = -alphabeta_impl(
            pos,
            -beta,
            -alpha,
            depth - 1,
            ply + 1,
            stats,
            &mut next_pv,
            should_stop,
            eval,
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

#[must_use]
pub fn alphabeta<G: GameRules>(
    mut pos: G,
    settings: &GoSettings,
    info_handler: &impl Fn(
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<u64>,
        Option<u128>,
        Option<i32>,
        &Vec<G::MoveType>,
    ),
    eval: &impl Fn(&G) -> i32,
) -> Option<G::MoveType> {
    let mut bestmove = None;
    let start = Instant::now();
    let max_depth = settings.depth.unwrap_or(128);
    let mut stats = SearchStats::default();
    let us = pos.get_turn();
    let should_stop = |stats: &SearchStats| -> bool {
        if let Some(nodes) = settings.nodes {
            return stats.nodes > nodes;
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
            let to_use = time / 30;
            return start.elapsed().as_millis() >= to_use as u128;
        }

        false
    };

    // Iterative deepening
    for depth in 1..=max_depth {
        let mut pv = vec![];
        let score = alphabeta_impl(
            &mut pos,
            -1_000_000_000,
            1_000_000_000,
            depth,
            0,
            &mut stats,
            &mut pv,
            &should_stop,
            &eval,
        );
        let elapsed = Instant::now() - start;

        if depth > 1 && should_stop(&stats) {
            info_handler(
                None,
                None,
                None,
                None,
                Some(stats.nodes),
                Some(elapsed.as_millis()),
                None,
                &pv,
            );
            break;
        }

        info_handler(
            Some(depth),
            None,
            Some(score),
            None,
            Some(stats.nodes),
            Some(elapsed.as_millis()),
            None,
            &pv,
        );

        bestmove = Some(pv[0]);
    }

    bestmove
}