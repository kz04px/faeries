pub mod root;

use super::GameSearch;
use crate::search::SearchInfo;
use games::ataxx::{AtaxxMove, AtaxxPosition};

/// Ataxx's primary search. It keeps nothing between searches.
#[derive(Default)]
pub struct AtaxxPrimary;

impl GameSearch for AtaxxPrimary {
    type Game = AtaxxPosition<7, 7>;

    fn search(
        &mut self,
        pos: AtaxxPosition<7, 7>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
    ) -> Option<Option<AtaxxMove<7, 7>>> {
        Some(root::primary(pos, settings, info_handler))
    }
}

