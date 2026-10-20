use super::TakState;
use super::search::alphabeta;
use crate::searchstats::SearchStats;
use crate::{search::SearchInfo, tak6x6::INF_SCORE};
use games::{
    gamerules::GameRules,
    general::side::Side,
    tak::{TakMove, TakPosition},
};
use protocols::GoSettings;
use std::time::Instant;

const MAX_DEPTH: i32 = 128;

#[must_use]
pub fn primary(
    mut pos: TakPosition<6>,
    state: &mut TakState,
    settings: &GoSettings,
    info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
) -> Option<TakMove<6>> {
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
        let score = alphabeta(
            &mut pos,
            depth,
            0,
            -INF_SCORE,
            INF_SCORE,
            &should_stop,
            &mut stats,
            state,
            &mut pv,
            true,
        );

        if depth > 1 && should_stop(&stats) {
            info_handler(
                &pos,
                &SearchInfo {
                    nodes: Some(stats.nodes),
                    elapsed: Some(start.elapsed().as_millis()),
                    ..Default::default()
                },
            );
            break;
        }

        bestmove = Some(pv[0]);

        info_handler(
            &pos,
            &SearchInfo {
                depth: Some(depth),
                seldepth: Some(stats.seldepth),
                score: Some(score),
                nodes: Some(stats.nodes),
                elapsed: Some(start.elapsed().as_millis()),
                hashfull: Some(state.tt.hashfull()),
                pv: &pv,
                ..Default::default()
            },
        );
    }

    bestmove
}
