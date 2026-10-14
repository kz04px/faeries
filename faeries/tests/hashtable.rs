#[cfg(test)]
mod hashtable {
    use faeries::hashtable::HashTable;

    #[derive(Clone, Copy, Debug, Default, PartialEq)]
    struct Entry {
        key: u64,
        value: i32,
    }

    #[test]
    fn get_returns_what_was_set() {
        let mut table = HashTable::<i32>::new(1);
        table.set(0x1234_5678_9abc_def0, 42);
        assert_eq!(table.get(0x1234_5678_9abc_def0), Some(42));
    }

    #[test]
    fn empty_table_has_default_entries() {
        let table = HashTable::<i32>::new(1);
        assert_eq!(table.get(0), Some(0));
        assert_eq!(table.get(u64::MAX), Some(0));
        assert_eq!(table.hashfull(), 0);
    }

    #[test]
    fn set_replaces_entry_with_same_hash() {
        let mut table = HashTable::<i32>::new(1);
        table.set(7, 1);
        table.set(7, 2);
        assert_eq!(table.get(7), Some(2));
    }

    #[test]
    fn colliding_hashes_share_a_slot() {
        // Hashes that differ only in their low bits share a slot
        let a = 0xffff_0000_0000_0000;
        let b = 0xffff_0000_0000_0001;
        let mut table = HashTable::<Entry>::new(1);
        table.set(a, Entry { key: a, value: 1 });
        assert_eq!(table.get(b), Some(Entry { key: a, value: 1 }));

        table.set(b, Entry { key: b, value: 2 });
        assert_eq!(table.get(a), Some(Entry { key: b, value: 2 }));
    }

    #[test]
    fn clear_discards_entries() {
        let mut table = HashTable::<i32>::new(1);
        table.set(7, 1);
        table.clear();
        assert_eq!(table.get(7), Some(0));
    }

    #[test]
    fn resize_changes_capacity_and_discards_entries() {
        let mut table = HashTable::<u64>::new(1);
        let small = table.capacity();
        table.set(7, 1);

        table.resize(2);
        assert_eq!(table.capacity(), 2 * small);
        assert_eq!(table.get(7), Some(0));
    }

    #[test]
    fn zero_sized_table_stores_nothing() {
        let mut table = HashTable::<i32>::new(0);
        assert_eq!(table.capacity(), 0);
        table.set(7, 1);
        assert_eq!(table.get(7), None);
        assert_eq!(table.hashfull(), 0);
    }

    #[test]
    fn hashfull_counts_used_slots() {
        let mut table = HashTable::<u64>::new(1);
        let capacity = table.capacity() as u64;

        // Hashes spread evenly over the first 1000 slots fill exactly half of them
        let step = u64::MAX / capacity + 1;
        for slot in (1..1000).step_by(2) {
            table.set(slot * step, slot);
        }
        assert_eq!(table.hashfull(), 500);
    }

    #[test]
    fn works_with_struct_entries() {
        #[derive(Clone, Copy, Debug, Default, PartialEq)]
        struct Entry {
            score: i16,
            depth: u8,
        }

        let mut table = HashTable::<Entry>::new(1);
        table.set(
            99,
            Entry {
                score: -5,
                depth: 3,
            },
        );
        assert_eq!(
            table.get(99),
            Some(Entry {
                score: -5,
                depth: 3
            })
        );
    }
}
