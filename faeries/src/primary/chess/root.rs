use super::cache::Cache;
use super::qsearch::score;
use super::search::{new_entry, select_edge};
use crate::search::SearchInfo;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::GameRules;
use games::general::side::Side;
use protocols::GoSettings;
use std::time::Instant;

#[must_use]
pub fn primary(
    cache: &mut Cache,
    pos: ChessPosition,
    settings: &GoSettings,
    info_handler: &impl Fn(&ChessPosition, &SearchInfo<ChessMove>),
) -> Option<ChessMove> {
    let root_key = pos.hash();
    let root = match cache.get(root_key) {
        Some(slot) => slot,
        None => cache.insert(root_key, new_entry(&pos)),
    };
    if cache.entry(root).edges.is_empty() {
        return None;
    }

    let us = pos.get_turn();
    let mut total_games: u64 = 0;
    // The cache slot, edge index and side to move at each step of the descent
    let mut path: Vec<(usize, usize, Side)> = Vec::new();

    let start = Instant::now();
    let should_stop = |total: u64| -> bool {
        if let Some(nodes) = settings.nodes {
            return total >= nodes;
        }
        if let Some(movetime) = settings.movetime {
            return start.elapsed().as_millis() >= movetime as u128;
        }
        let our_time = if us == Side::Player1 {
            settings.p1time
        } else {
            settings.p2time
        };
        if let Some(time) = our_time {
            return start.elapsed().as_millis() >= (time / 30) as u128;
        }
        false
    };

    loop {
        if should_stop(total_games) {
            break;
        }

        let mut copy = pos.clone();
        let mut slot = root;
        cache.touch(slot);
        path.clear();

        // Descend until reaching an unvisited move or a leaf, then evaluate it
        loop {
            let entry = cache.entry(slot);
            if entry.edges.is_empty() {
                break;
            }
            let edge = select_edge(entry);
            let mover = copy.get_turn();
            copy.makemove(&entry.edges[edge].mv);
            let visited = entry.edges[edge].visits > 0;
            path.push((slot, edge, mover));
            if !visited || copy.is_fifty_move() || copy.is_threefold() {
                break;
            }
            let key = copy.hash();
            slot = match cache.get(key) {
                Some(slot) => slot,
                None => cache.insert(key, new_entry(&copy)),
            };
        }

        let to_move = copy.get_turn();
        let value = score(&copy);

        for &(slot, edge, mover) in &path {
            let entry = cache.entry_mut(slot);
            entry.visits += 1;
            let edge = &mut entry.edges[edge];
            edge.visits += 1;
            edge.score += if mover == to_move { value } else { 1.0 - value };
        }

        total_games += 1;
    }

    info_handler(
        &pos,
        &SearchInfo {
            nodes: Some(total_games),
            elapsed: Some(start.elapsed().as_millis()),
            ..Default::default()
        },
    );

    // Pick the most visited root move; break ties by score
    cache
        .entry(root)
        .edges
        .iter()
        .max_by(|a, b| a.visits.cmp(&b.visits).then(a.score.total_cmp(&b.score)))
        .map(|e| e.mv)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn search(cache: &mut Cache, pos: ChessPosition, nodes: u64) -> Option<ChessMove> {
        let settings = GoSettings {
            nodes: Some(nodes),
            ..Default::default()
        };
        primary(cache, pos, &settings, &|_, _| {})
    }

    #[test]
    fn searches_find_a_legal_move() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "6k1/5ppp/8/8/8/8/8/R3K3 w Q - 0 1",
            "4k3/8/8/8/8/8/4P3/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/R3K3 w - - 98 80",
        ];
        for fen in fens {
            let pos = ChessPosition::from_fen(fen);
            let mv = search(&mut Cache::with_megabytes(1), pos.clone(), 5000).expect(fen);
            assert!(pos.legal_moves().contains(&mv), "illegal move for {fen}");
        }
    }

    #[test]
    fn a_full_cache_keeps_searching() {
        // Room for only a handful of positions, so nearly everything is evicted
        let mut cache = Cache::new(8);
        let pos = ChessPosition::startpos();
        let mv = search(&mut cache, pos.clone(), 20000).unwrap();
        assert!(pos.legal_moves().contains(&mv));
    }

    #[test]
    fn the_next_search_starts_from_what_the_last_one_learned() {
        let mut cache = Cache::with_megabytes(16);
        let mut pos = ChessPosition::startpos();
        let mv = search(&mut cache, pos.clone(), 20000).unwrap();
        pos.makemove(&mv);
        let reply = pos.legal_moves()[0];
        pos.makemove(&reply);

        // The position after our move and a reply was reached during the search
        let slot = cache.get(pos.hash()).expect("the position was not cached");
        assert!(cache.entry(slot).visits > 0);
    }

    #[test]
    fn no_move_without_legal_moves() {
        let pos = ChessPosition::from_fen("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1");
        assert_eq!(search(&mut Cache::with_megabytes(1), pos, 100), None);
    }
}
