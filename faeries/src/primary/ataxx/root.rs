use super::{AtaxxPrimary, MAX_PLY, order::MoveBuffer, search::alphabeta};
use crate::searchstats::SearchStats;
use crate::{primary::ataxx::INF_SCORE, search::SearchInfo};
use games::{
    ataxx::{AtaxxMove, AtaxxPosition},
    gamerules::GameRules,
    general::side::Side,
};
use protocols::GoSettings;
use std::time::Instant;

const MAX_DEPTH: i32 = MAX_PLY as i32;

#[must_use]
pub fn primary(
    mut pos: AtaxxPosition<7, 7>,
    state: &mut AtaxxPrimary,
    settings: &GoSettings,
    info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
) -> Option<AtaxxMove<7, 7>> {
    let start = Instant::now();
    let mut bestmove = None;
    state.killers = [None; MAX_PLY];
    let mut buffers: Vec<MoveBuffer> = (0..MAX_PLY).map(|_| MoveBuffer::default()).collect();
    let mut stats = SearchStats::default();
    let max_depth = settings.depth.unwrap_or(MAX_DEPTH);
    let us = pos.get_turn();
    let clock_budget = if us == Side::Player1 {
        settings.p1time.map(|time| (time / 30) as u128)
    } else {
        settings.p2time.map(|time| (time / 30) as u128)
    };
    let should_stop = |stats: &SearchStats| -> bool {
        if let Some(nodes) = settings.nodes {
            return stats.nodes > nodes;
        }

        if let Some(movetime) = settings.movetime {
            return start.elapsed().as_millis() >= movetime as u128;
        }

        if let Some(budget) = clock_budget {
            return start.elapsed().as_millis() >= budget;
        }

        false
    };

    let mut previous_time: Option<f64> = None;
    for depth in 1..=max_depth {
        let depth_start = Instant::now();
        let mut pv = vec![];
        // Depth 1 always finishes so there's a move to play
        let score = if depth == 1 {
            alphabeta(
                &mut pos,
                depth,
                0,
                -INF_SCORE,
                INF_SCORE,
                &|_| false,
                &mut stats,
                state,
                &mut pv,
                false,
                &mut buffers,
            )
        } else {
            alphabeta(
                &mut pos,
                depth,
                0,
                -INF_SCORE,
                INF_SCORE,
                &should_stop,
                &mut stats,
                state,
                &mut pv,
                false,
                &mut buffers,
            )
        };

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

        // Predict if there's enough time left for the next depth to finish
        let depth_time = depth_start.elapsed().as_secs_f64() * 1000.0;
        if let Some(budget) = clock_budget {
            let growth =
                previous_time.map_or(2.0, |previous| (depth_time / previous).clamp(1.0, 6.0));
            let predicted_end = start.elapsed().as_secs_f64() * 1000.0 + depth_time * growth;
            if predicted_end >= budget as f64 {
                break;
            }
        }
        previous_time = Some(depth_time.max(f64::EPSILON));
    }

    bestmove
}
