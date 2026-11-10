use games::chess::ChessMove;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::mem::size_of;

const NONE: u32 = u32::MAX;

// A typical position has about this many legal moves
const TYPICAL_MOVES: usize = 32;
// The map's share per entry: key, slot index and control byte, with room to grow
const MAP_BYTES_PER_ENTRY: usize = 24;

/// Position hashes are already well mixed, so the key is used as is.
#[derive(Default)]
struct KeyHasher(u64);

impl Hasher for KeyHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, _: &[u8]) {
        unreachable!("only u64 keys are hashed");
    }

    fn write_u64(&mut self, key: u64) {
        self.0 = key;
    }
}

/// The search results for one legal move from a position.
pub struct Edge {
    pub mv: ChessMove,
    // How promising the move looks before any search, as a probability among the position's moves
    pub prior: f32,
    pub visits: u32,
    // Total result for the side playing `mv`: a win is 1, a draw is 0.5, a loss is 0
    pub score: f64,
}

/// What the search knows about one position.
pub struct Entry {
    pub edges: Vec<Edge>,
    pub visits: u32,
}

struct Slot {
    key: u64,
    entry: Entry,
    newer: u32,
    older: u32,
}

/// Search results for positions, keyed by position hash. When full, the least
/// recently used position is replaced.
pub struct Cache {
    map: HashMap<u64, u32, BuildHasherDefault<KeyHasher>>,
    slots: Vec<Slot>,
    newest: u32,
    oldest: u32,
    capacity: usize,
}

impl Cache {
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::default(),
            slots: Vec::new(),
            newest: NONE,
            oldest: NONE,
            capacity: capacity.max(1),
        }
    }

    /// A cache using about `mb` megabytes once full.
    #[must_use]
    pub fn with_megabytes(mb: usize) -> Self {
        Self::new(Self::capacity_for(mb))
    }

    /// Resize to use about `mb` megabytes once full, discarding every entry.
    pub fn resize(&mut self, mb: usize) {
        self.clear();
        self.slots = Vec::new();
        self.capacity = Self::capacity_for(mb);
    }

    fn capacity_for(mb: usize) -> usize {
        let per_entry = size_of::<Slot>() + TYPICAL_MOVES * size_of::<Edge>() + MAP_BYTES_PER_ENTRY;
        mb * 1024 * 1024 / per_entry
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.slots.clear();
        self.newest = NONE;
        self.oldest = NONE;
    }

    /// The slot holding `key`, marked as most recently used.
    pub fn get(&mut self, key: u64) -> Option<usize> {
        let slot = *self.map.get(&key)?;
        self.touch(slot as usize);
        Some(slot as usize)
    }

    /// Stores `entry` for `key` as the most recently used, replacing the least
    /// recently used one if the cache is full. `key` must not already be present.
    pub fn insert(&mut self, key: u64, entry: Entry) -> usize {
        let slot = if self.slots.len() < self.capacity {
            self.slots.push(Slot {
                key,
                entry,
                newer: NONE,
                older: NONE,
            });
            self.slots.len() - 1
        } else {
            let slot = self.oldest as usize;
            self.unlink(slot);
            self.map.remove(&self.slots[slot].key);
            self.slots[slot].key = key;
            self.slots[slot].entry = entry;
            slot
        };
        self.map.insert(key, slot as u32);
        self.link_newest(slot);
        slot
    }

    /// Marks the slot as most recently used.
    pub fn touch(&mut self, slot: usize) {
        if self.newest != slot as u32 {
            self.unlink(slot);
            self.link_newest(slot);
        }
    }

    #[must_use]
    pub fn entry(&self, slot: usize) -> &Entry {
        &self.slots[slot].entry
    }

    pub fn entry_mut(&mut self, slot: usize) -> &mut Entry {
        &mut self.slots[slot].entry
    }

    fn unlink(&mut self, slot: usize) {
        let (newer, older) = (self.slots[slot].newer, self.slots[slot].older);
        if newer == NONE {
            self.newest = older;
        } else {
            self.slots[newer as usize].older = older;
        }
        if older == NONE {
            self.oldest = newer;
        } else {
            self.slots[older as usize].newer = newer;
        }
    }

    fn link_newest(&mut self, slot: usize) {
        self.slots[slot].newer = NONE;
        self.slots[slot].older = self.newest;
        if self.newest == NONE {
            self.oldest = slot as u32;
        } else {
            self.slots[self.newest as usize].newer = slot as u32;
        }
        self.newest = slot as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(visits: u32) -> Entry {
        Entry {
            edges: Vec::new(),
            visits,
        }
    }

    #[test]
    fn finds_what_was_inserted() {
        let mut cache = Cache::new(4);
        let slot = cache.insert(10, entry(7));
        assert_eq!(cache.get(10), Some(slot));
        assert_eq!(cache.entry(slot).visits, 7);
        assert_eq!(cache.get(11), None);
    }

    #[test]
    fn replaces_the_least_recently_used_when_full() {
        let mut cache = Cache::new(3);
        for key in 1..=3 {
            cache.insert(key, entry(key as u32));
        }
        // Using 1 makes 2 the oldest
        assert!(cache.get(1).is_some());
        cache.insert(4, entry(4));
        assert!(cache.get(2).is_none());
        for key in [1, 3, 4] {
            let slot = cache.get(key).unwrap();
            assert_eq!(cache.entry(slot).visits, key as u32);
        }
    }

    #[test]
    fn touching_protects_a_slot() {
        let mut cache = Cache::new(2);
        let a = cache.insert(1, entry(1));
        cache.insert(2, entry(2));
        cache.touch(a);
        cache.insert(3, entry(3));
        assert!(cache.get(1).is_some());
        assert!(cache.get(2).is_none());
    }

    #[test]
    fn clearing_and_resizing_forget_everything() {
        let mut cache = Cache::new(4);
        cache.insert(1, entry(1));
        cache.clear();
        assert!(cache.get(1).is_none());
        cache.insert(2, entry(2));
        cache.resize(1);
        assert!(cache.get(2).is_none());
        let slot = cache.insert(3, entry(3));
        assert_eq!(cache.get(3), Some(slot));
    }

    #[test]
    fn capacity_follows_the_size_asked_for() {
        let small = Cache::with_megabytes(10);
        let large = Cache::with_megabytes(100);
        assert!(large.capacity > 9 * small.capacity);
        assert!(large.capacity < 11 * small.capacity);
    }
}
