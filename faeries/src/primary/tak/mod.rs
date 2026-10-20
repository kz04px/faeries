pub mod eval;
pub mod hash;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::hashtable::HashTable;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};
use protocols::ugi::options::Spin;
use search::TTEntry;

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;
const DEFAULT_HASH_MB: usize = 64;

/// The 6x6 primary search, kept between searches until the engine quits.
///
/// The hash table is only allocated by `init`, which the protocols call on the first
/// "isready". Until then it has no entries, so searches run without it.
pub struct TakState {
    pub tt: HashTable<TTEntry>,
    pub hash: Spin<usize>,
}

impl Default for TakState {
    fn default() -> Self {
        Self {
            tt: HashTable::new(0),
            hash: Spin {
                name: "hash".to_owned(),
                min: 1,
                max: 65536,
                value: DEFAULT_HASH_MB,
            },
        }
    }
}

impl GameSearch for TakState {
    type Game = TakPosition<6>;

    fn init(&mut self) {
        self.tt.resize(self.hash.value);
    }

    fn newgame(&mut self) {
        self.tt.clear();
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
            // Resize the hash table now if it's allocated, otherwise on init
            if self.tt.capacity() != 0 {
                self.tt.resize(self.hash.value);
            }
        }
        true
    }

    fn search_names() -> &'static [&'static str] {
        &["primary"]
    }

    fn search(
        &mut self,
        name: &str,
        pos: TakPosition<6>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
    ) -> Option<Option<TakMove<6>>> {
        match name {
            "primary" => Some(root::primary(pos, self, settings, info_handler)),
            _ => None,
        }
    }
}

