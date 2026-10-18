pub mod eval;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;

/// The 6x6 primary search. It keeps nothing between searches.
#[derive(Default)]
pub struct TakPrimary;

impl GameSearch for TakPrimary {
    type Game = TakPosition<6>;

    fn search(
        &mut self,
        pos: TakPosition<6>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
    ) -> Option<Option<TakMove<6>>> {
        Some(root::primary(pos, settings, info_handler))
    }
}

