use std::mem::size_of;

/// A fixed-size table of entries looked up by a 64 bit hash, as used for a
/// transposition table.
///
/// Different hashes can share a slot, so entries store their own hash and the
/// caller checks it before using the data.
pub struct HashTable<T: Copy + Default + PartialEq> {
    entries: Vec<T>,
}

impl<T: Copy + Default + PartialEq> HashTable<T> {
    /// A table using at most `mb` megabytes.
    #[must_use]
    pub fn new(mb: usize) -> Self {
        let mut table = Self { entries: vec![] };
        table.resize(mb);
        table
    }

    /// Resize to use at most `mb` megabytes, discarding every entry.
    pub fn resize(&mut self, mb: usize) {
        let num_entries = mb * 1024 * 1024 / size_of::<T>();
        self.entries = vec![T::default(); num_entries];
    }

    /// Discard every entry.
    pub fn clear(&mut self) {
        self.entries.fill(T::default());
    }

    /// Number of slots in the table.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.entries.len()
    }

    /// The entry in `hash`'s slot, which may belong to another hash. `None` if the
    /// table has no slots.
    #[must_use]
    pub fn get(&self, hash: u64) -> Option<T> {
        self.entries.get(self.index(hash)).copied()
    }

    /// Store `data` in `hash`'s slot, replacing whatever was there.
    pub fn set(&mut self, hash: u64, data: T) {
        let idx = self.index(hash);
        if let Some(entry) = self.entries.get_mut(idx) {
            *entry = data;
        }
    }

    /// How full the table is, in permille, estimated from the first 1000 slots.
    #[must_use]
    pub fn hashfull(&self) -> i32 {
        let sample = &self.entries[..self.entries.len().min(1000)];
        if sample.is_empty() {
            return 0;
        }

        let used = sample.iter().filter(|&&entry| entry != T::default()).count();
        (used * 1000 / sample.len()) as i32
    }

    /// Map a hash onto a slot, without needing a power of two table size.
    ///
    /// Uses "fastrange" multiply-shift reduction
    fn index(&self, hash: u64) -> usize {
        ((hash as u128 * self.entries.len() as u128) >> 64) as usize
    }
}
