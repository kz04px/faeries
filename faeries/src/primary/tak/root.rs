use super::search::negamax;
use crate::search::SearchInfo;
use crate::searchstats::SearchStats;
use games::tak::{TakMove, TakPosition};
use protocols::GoSettings;
use std::time::Instant;

const DEPTH: i32 = 2;

#[must_use]
pub fn primary(
    mut pos: TakPosition<6>,
    _settings: &GoSettings,
    info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
) -> Option<TakMove<6>> {
    let start = Instant::now();
    let mut stats = SearchStats::default();
    let mut pv = vec![];
    let score = negamax(&mut pos, DEPTH, 0, &mut stats, &mut pv);

    info_handler(
        &pos,
        &SearchInfo {
            depth: Some(DEPTH),
            seldepth: Some(stats.seldepth),
            score: Some(score),
            nodes: Some(stats.nodes),
            elapsed: Some(start.elapsed().as_millis()),
            pv: &pv,
            ..Default::default()
        },
    );

    pv.first().copied()
}
