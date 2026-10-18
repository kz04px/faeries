pub mod root;

use super::GameSearch;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};

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

