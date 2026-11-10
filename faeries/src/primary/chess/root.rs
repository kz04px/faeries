use super::eval::evaluate;
use super::search::{Node, expand, select_child, validate_tree};
use crate::search::SearchInfo;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::GameRules;
use games::general::side::Side;
use protocols::GoSettings;
use std::time::Instant;

#[must_use]
pub fn primary(
    pos: ChessPosition,
    settings: &GoSettings,
    info_handler: &impl Fn(&ChessPosition, &SearchInfo<ChessMove>),
) -> Option<ChessMove> {
    let moves = pos.legal_moves();
    if moves.is_empty() {
        return None;
    }

    let us = pos.get_turn();
    let mut nodes = vec![Node {
        mv: ChessMove::default(),
        mover: us,
        first_child: 0,
        num_children: 0,
        expanded: false,
        visits: 0,
        score: 0.0,
    }];
    let mut total_games: u64 = 0;
    let mut path: Vec<usize> = Vec::new();

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
        let mut idx = 0;
        path.clear();
        path.push(idx);

        // Descend until reaching an unvisited node or a leaf, then evaluate it
        loop {
            if !nodes[idx].expanded {
                expand(&mut nodes, idx, &copy);
            }
            if nodes[idx].num_children == 0 {
                break;
            }
            idx = select_child(&nodes, idx);
            copy.makemove(&nodes[idx].mv);
            path.push(idx);
            if nodes[idx].visits == 0 {
                break;
            }
        }

        let to_move = copy.get_turn();
        let value = evaluate(&copy);

        for &i in &path {
            let node = &mut nodes[i];
            node.visits += 1;
            node.score += if node.mover == to_move {
                value
            } else {
                1.0 - value
            };
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

    if cfg!(debug_assertions)
        && let Err(error) = validate_tree(&nodes)
    {
        panic!("invalid search tree: {error}");
    }

    // Pick the most visited root move; break ties by score
    let root = &nodes[0];
    let first = root.first_child as usize;
    nodes[first..first + root.num_children as usize]
        .iter()
        .max_by(|a, b| a.visits.cmp(&b.visits).then(a.score.total_cmp(&b.score)))
        .map(|n| n.mv)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Searching checks the tree itself when debug assertions are on
    #[test]
    fn searches_build_valid_trees() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            "6k1/5ppp/8/8/8/8/8/R3K3 w Q - 0 1",
            "4k3/8/8/8/8/8/4P3/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/R3K3 w - - 98 80",
        ];
        let settings = GoSettings {
            nodes: Some(1000),
            ..Default::default()
        };
        for fen in fens {
            let pos = ChessPosition::from_fen(fen);
            let mv = primary(pos, &settings, &|_, _| {});
            assert!(mv.is_some(), "no move for {fen}");
        }
    }

    #[test]
    fn no_move_without_legal_moves() {
        let pos = ChessPosition::from_fen("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1");
        let settings = GoSettings {
            nodes: Some(100),
            ..Default::default()
        };
        assert_eq!(primary(pos, &settings, &|_, _| {}), None);
    }
}
