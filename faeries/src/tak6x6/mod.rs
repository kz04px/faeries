pub mod eval;
pub mod hash;
pub mod root;
pub mod search;

use crate::hashtable::HashTable;
use crate::search::Primary;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};
use search::TTEntry;

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;
pub const DEFAULT_HASH_MB: usize = 64;

/// Kept between searches until the engine quits.
///
/// The hash table is only allocated by `init`, which the protocols call on the first
/// "isready". Until then it has no entries, so searches run without it.
pub struct TakState {
    pub tt: HashTable<TTEntry>,
    pub hash_mb: usize,
}

impl Default for TakState {
    fn default() -> Self {
        Self {
            tt: HashTable::new(0),
            hash_mb: DEFAULT_HASH_MB,
        }
    }
}

impl TakState {
    pub fn init(&mut self) {
        self.tt.resize(self.hash_mb);
    }

    /// Resize the hash table now if it's allocated, otherwise on `init`.
    pub fn set_hash(&mut self, mb: usize) {
        self.hash_mb = mb;
        if self.tt.capacity() != 0 {
            self.tt.resize(mb);
        }
    }

    pub fn newgame(&mut self) {
        self.tt.clear();
    }

    #[must_use]
    pub fn search(
        &mut self,
        pos: TakPosition<6>,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
    ) -> Option<TakMove<6>> {
        root::primary(pos, self, settings, info_handler)
    }
}

impl Primary for TakPosition<6> {
    fn primary(
        self,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&Self, &SearchInfo<TakMove<6>>),
    ) -> Option<TakMove<6>> {
        TakState::default().search(self, settings, info_handler)
    }
}
