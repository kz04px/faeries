use super::search::negamax;
use crate::search::SearchInfo;
use crate::searchstats::SearchStats;
use games::ataxx::{AtaxxMove, AtaxxPosition};
use protocols::GoSettings;
use std::time::Instant;

const DEPTH: i32 = 1;

#[must_use]
pub fn primary(
    mut pos: AtaxxPosition<7, 7>,
    _settings: &GoSettings,
    info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
) -> Option<AtaxxMove<7, 7>> {
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
