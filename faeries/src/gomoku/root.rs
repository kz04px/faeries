use super::search::negamax;
use crate::searchstats::SearchStats;
use games::{
    gamerules::GameRules,
    general::side::Side,
    gomoku::{GomokuMove, GomokuPosition},
};
use std::time::Instant;
use ugi::go::GoSettings;

const MAX_DEPTH: i32 = 128;

#[must_use]
pub fn root(
    mut pos: GomokuPosition,
    settings: &GoSettings,
    info_handler: &impl Fn(
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<i32>,
        Option<u64>,
        Option<u128>,
        Option<i32>,
        &Vec<GomokuMove>,
    ),
) -> Option<GomokuMove> {
    let start = Instant::now();
    let mut bestmove = None;
    let mut stats = SearchStats::default();
    let max_depth = settings.depth.unwrap_or(MAX_DEPTH);
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

    for depth in 1..=max_depth {
        let mut pv = vec![];
        let score = negamax(&mut pos, depth, 0, &should_stop, &mut stats, &mut pv);

        if depth > 1 && should_stop(&stats) {
            info_handler(
                None,
                None,
                None,
                None,
                Some(stats.nodes),
                Some(start.elapsed().as_millis()),
                None,
                &vec![],
            );
            break;
        }

        bestmove = Some(pv[0]);

        info_handler(
            Some(depth),
            Some(stats.seldepth),
            Some(score),
            None,
            Some(stats.nodes),
            Some(start.elapsed().as_millis()),
            None,
            &pv,
        );
    }

    bestmove
}