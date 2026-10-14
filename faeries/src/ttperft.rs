use crate::hashtable::HashTable;
use games::gamerules::GameRules;
use std::time::Instant;

/// Node count of a subtree searched to exactly `depth`.
#[derive(Clone, Copy, Default, PartialEq)]
pub struct PerftEntry {
    key: u64,
    nodes: u64,
    depth: i32,
}

/// Transposition Table Perft
pub fn ttperft<G: GameRules>(
    pos: &mut G,
    depth: i32,
    table: &mut HashTable<PerftEntry>,
    hash: &impl Fn(&G) -> u64,
) -> u64 {
    // Counting these directly is cheaper than probing the table
    if depth <= 1 {
        return pos.perft(depth);
    }

    let key = hash(pos);
    if let Some(entry) = table.get(key)
        && entry.key == key
        && entry.depth == depth
    {
        return entry.nodes;
    }

    let mut nodes = 0;
    for mv in pos.legal_moves() {
        pos.makemove(&mv);
        nodes += ttperft(pos, depth - 1, table, hash);
        pos.undomove(&mv);
    }

    table.set(key, PerftEntry { key, nodes, depth });
    nodes
}

/// Run ttperft at each depth up to `depth`, sharing one table of `mb` megabytes.
pub fn iterative_ttperft<G: GameRules>(
    pos: &mut G,
    depth: i32,
    mb: usize,
    hash: &impl Fn(&G) -> u64,
    info_handler: &impl Fn(i32, f32, u64),
    final_handler: &impl Fn(u64),
) {
    let start = Instant::now();
    let mut table = HashTable::new(mb);
    let mut last = 0;
    for i in 1..=depth {
        last = ttperft(pos, i, &mut table, hash);
        info_handler(i, start.elapsed().as_secs_f32(), last);
    }
    final_handler(last);
}
