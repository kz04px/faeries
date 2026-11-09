pub mod eval;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::search::SearchInfo;
use games::ataxx::{AtaxxMove, AtaxxPosition};

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;

/// Ataxx's primary search. It keeps nothing between searches.
#[derive(Default)]
pub struct AtaxxPrimary;

impl GameSearch for AtaxxPrimary {
    type Game = AtaxxPosition<7, 7>;

    fn search_names() -> &'static [&'static str] {
        &["primary"]
    }

    fn search(
        &mut self,
        name: &str,
        pos: AtaxxPosition<7, 7>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
    ) -> Option<Option<AtaxxMove<7, 7>>> {
        match name {
            "primary" => Some(root::primary(pos, settings, info_handler)),
            _ => None,
        }
    }
}

