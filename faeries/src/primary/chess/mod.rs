pub mod eval;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::search::SearchInfo;
use games::chess::{ChessMove, ChessPosition};

/// Chess's primary search. It keeps nothing between searches.
#[derive(Default)]
pub struct ChessPrimary;

impl GameSearch for ChessPrimary {
    type Game = ChessPosition;

    fn search_names() -> &'static [&'static str] {
        &["primary"]
    }

    fn search(
        &mut self,
        name: &str,
        pos: ChessPosition,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&ChessPosition, &SearchInfo<ChessMove>),
    ) -> Option<Option<ChessMove>> {
        match name {
            "primary" => Some(root::primary(pos, settings, info_handler)),
            _ => None,
        }
    }
}
