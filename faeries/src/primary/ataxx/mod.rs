pub mod eval;
pub mod root;
pub mod search;

use super::GameSearch;
use crate::hashtable::HashTable;
use crate::search::SearchInfo;
use games::ataxx::{AtaxxMove, AtaxxPosition};
use protocols::ugi::options::Spin;
use search::TTEntry;

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;
pub const MAX_PLY: usize = 128;
const DEFAULT_HASH_MB: usize = 64;

pub struct AtaxxPrimary {
    pub tt: HashTable<TTEntry>,
    pub killers: [Option<AtaxxMove<7, 7>>; MAX_PLY],
    pub hash: Spin<usize>,
}

impl Default for AtaxxPrimary {
    fn default() -> Self {
        Self {
            tt: HashTable::new(0),
            killers: [None; MAX_PLY],
            hash: Spin {
                name: "hash".to_owned(),
                min: 1,
                max: 65536,
                value: DEFAULT_HASH_MB,
            },
        }
    }
}

impl GameSearch for AtaxxPrimary {
    type Game = AtaxxPosition<7, 7>;

    fn init(&mut self) {
        self.tt.resize(self.hash.value);
    }

    fn newgame(&mut self) {
        self.tt.clear();
        self.killers = [None; MAX_PLY];
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
        pos: AtaxxPosition<7, 7>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
    ) -> Option<Option<AtaxxMove<7, 7>>> {
        match name {
            "primary" => Some(root::primary(pos, self, settings, info_handler)),
            _ => None,
        }
    }
}
