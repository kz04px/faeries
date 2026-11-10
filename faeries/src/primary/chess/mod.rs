pub mod cache;
pub mod eval;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::search::SearchInfo;
use games::chess::{ChessMove, ChessPosition};
use protocols::ugi::options::Spin;

const DEFAULT_HASH_MB: usize = 100;

/// Chess's primary search. It keeps what it learned about positions between moves.
pub struct ChessPrimary {
    cache: cache::Cache,
    hash: Spin<usize>,
}

impl Default for ChessPrimary {
    fn default() -> Self {
        Self {
            cache: cache::Cache::with_megabytes(DEFAULT_HASH_MB),
            hash: Spin {
                name: "hash".to_owned(),
                min: 1,
                max: 65536,
                value: DEFAULT_HASH_MB,
            },
        }
    }
}

impl GameSearch for ChessPrimary {
    type Game = ChessPosition;

    fn newgame(&mut self) {
        self.cache.clear();
    }

    fn print_options(&self) {
        println!("{}", self.hash);
    }

    fn set_option(&mut self, name: &str, value: &str) -> bool {
        if name != "hash" {
            return false;
        }

        if let Ok(mb) = value.parse::<usize>() {
            self.hash.value = mb.clamp(self.hash.min, self.hash.max);
            self.cache.resize(self.hash.value);
        }
        true
    }

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
            "primary" => Some(root::primary(&mut self.cache, pos, settings, info_handler)),
            _ => None,
        }
    }
}
