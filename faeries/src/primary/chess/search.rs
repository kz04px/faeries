use crate::prng::XorshiftGenerator;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::{GameResult, GameRules};
use games::general::side::Side;

pub const MAX_ROLLOUT_PLIES: i32 = 400;

pub const EXPLORATION: f64 = 0.1;

pub struct Node {
    pub mv: ChessMove,
    // The side that played `mv` to reach this node
    pub mover: Side,
    pub first_child: u32,
    pub num_children: u32,
    pub expanded: bool,
    pub visits: u32,
    // Total result for `mover`: a win is 1, a draw is 0.5, a loss is 0
    pub score: f64,
}

pub fn expand(nodes: &mut Vec<Node>, idx: usize, pos: &ChessPosition) {
    let mover = pos.get_turn();
    let first_child = nodes.len();
    if !pos.is_fifty_move() && !pos.is_threefold() {
        for mv in pos.legal_moves() {
            nodes.push(Node {
                mv,
                mover,
                first_child: 0,
                num_children: 0,
                expanded: false,
                visits: 0,
                score: 0.0,
            });
        }
    }
    nodes[idx].first_child = first_child as u32;
    nodes[idx].num_children = (nodes.len() - first_child) as u32;
    nodes[idx].expanded = true;
}

// Picks the child with the highest UCB1 value, trying unvisited children first.
#[must_use]
pub fn select_child(nodes: &[Node], idx: usize) -> usize {
    let parent = &nodes[idx];
    let first = parent.first_child as usize;
    let ln_parent = (parent.visits.max(1) as f64).ln();
    let mut best = first;
    let mut best_value = f64::NEG_INFINITY;
    for i in first..first + parent.num_children as usize {
        let child = &nodes[i];
        if child.visits == 0 {
            return i;
        }
        let n = child.visits as f64;
        let value = child.score / n + EXPLORATION * (ln_parent / n).sqrt();
        if value > best_value {
            best_value = value;
            best = i;
        }
    }
    best
}

/// Checks the invariants of a search tree whose root is `nodes[0]`.
///
/// Every iteration descends from the root and ends with one rollout. The root
/// passes it to a child, but any other node counts the rollout that first
/// visited it, so a node's visits are the sum of its children's, plus one unless it
/// is the root. The same holds for score, with each child's result seen from the
/// other side.
pub fn validate_tree(nodes: &[Node]) -> Result<(), String> {
    if nodes.is_empty() {
        return Err("the tree has no root".to_owned());
    }

    let mut has_parent = vec![false; nodes.len()];
    for (i, node) in nodes.iter().enumerate() {
        let visits = node.visits as f64;
        if !(0.0..=visits).contains(&node.score) || (node.score * 2.0).fract() != 0.0 {
            return Err(format!(
                "node {i}: score {} is not a multiple of 0.5 in [0, {}]",
                node.score, node.visits
            ));
        }

        let first = node.first_child as usize;
        let count = node.num_children as usize;
        if !node.expanded {
            if count != 0 {
                return Err(format!(
                    "node {i}: has {count} children but is not expanded"
                ));
            }
            if node.visits > 1 {
                return Err(format!(
                    "node {i}: {} visits but never expanded",
                    node.visits
                ));
            }
            continue;
        }
        if i != 0 && node.visits < 2 {
            return Err(format!(
                "node {i}: expanded with only {} visits",
                node.visits
            ));
        }
        if count == 0 {
            continue;
        }
        if first <= i || first + count > nodes.len() {
            return Err(format!(
                "node {i}: children {first}..{} are outside the tree or before it",
                first + count
            ));
        }

        let mut child_visits = 0u64;
        let mut child_score = 0.0;
        let mut child_losses = 0.0;
        for c in first..first + count {
            let child = &nodes[c];
            if has_parent[c] {
                return Err(format!("node {c}: has more than one parent"));
            }
            has_parent[c] = true;
            if child.mover != nodes[first].mover {
                return Err(format!("node {c}: its mover differs from its siblings'"));
            }
            if (i == 0) != (child.mover == node.mover) {
                return Err(format!(
                    "node {c}: its mover should {} its parent's",
                    if i == 0 { "match" } else { "differ from" }
                ));
            }
            child_visits += child.visits as u64;
            child_score += child.score;
            child_losses += child.visits as f64 - child.score;
        }

        let own_rollout = u64::from(i != 0);
        if node.visits as u64 != child_visits + own_rollout {
            return Err(format!(
                "node {i}: {} visits, but its children have {child_visits}{}",
                node.visits,
                if i == 0 { "" } else { " plus its own rollout" }
            ));
        }
        if i == 0 {
            if node.score != child_score {
                return Err(format!(
                    "root: score {}, but its children's scores sum to {child_score}",
                    node.score
                ));
            }
        } else {
            let own_score = node.score - child_losses;
            if ![0.0, 0.5, 1.0].contains(&own_score) {
                return Err(format!(
                    "node {i}: score {} leaves {own_score} for its own rollout",
                    node.score
                ));
            }
        }
    }

    if let Some(i) = (1..nodes.len()).find(|&i| !has_parent[i]) {
        return Err(format!("node {i}: has no parent"));
    }
    Ok(())
}

#[must_use]
pub fn rollout(pos: &mut ChessPosition, prng: &mut XorshiftGenerator) -> Option<GameResult> {
    let mut moves = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
    for _ in 0..MAX_ROLLOUT_PLIES {
        let count = pos.legal_moves_into(&mut moves);
        if count == 0 {
            return pos.get_result();
        }
        if pos.is_fifty_move() || pos.is_threefold() {
            return Some(GameResult::Draw);
        }
        let mv = moves[prng.next() as usize % count];
        pos.makemove(&mv);
    }
    pos.get_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(mover: Side, first_child: u32, num_children: u32, visits: u32, score: f64) -> Node {
        Node {
            mv: ChessMove::default(),
            mover,
            first_child,
            num_children,
            expanded: num_children > 0,
            visits,
            score,
        }
    }

    // A root with two children, the first of which has one child of its own
    fn small_tree() -> Vec<Node> {
        vec![
            node(Side::Player1, 1, 2, 3, 1.5),
            node(Side::Player1, 3, 1, 2, 1.0),
            node(Side::Player1, 0, 0, 1, 0.5),
            node(Side::Player2, 0, 0, 1, 0.0),
        ]
    }

    #[test]
    fn accepts_a_consistent_tree() {
        assert_eq!(validate_tree(&small_tree()), Ok(()));
    }

    #[test]
    fn rejects_inconsistent_trees() {
        let corruptions: [(&str, fn(&mut Vec<Node>)); 7] = [
            ("score above visits", |t| t[2].score = 1.5),
            ("negative score", |t| t[2].score = -0.5),
            ("score off the half-point grid", |t| t[2].score = 0.3),
            ("root visits", |t| t[0].visits = 4),
            ("inner node visits", |t| t[1].visits = 3),
            ("root score", |t| t[0].score = 1.0),
            ("inner node score", |t| t[1].score = 0.0),
        ];
        for (name, corrupt) in corruptions {
            let mut tree = small_tree();
            corrupt(&mut tree);
            assert!(validate_tree(&tree).is_err(), "{name} was accepted");
        }
    }

    #[test]
    fn rejects_broken_structure() {
        let mut tree = small_tree();
        tree[3].mover = Side::Player1;
        assert!(
            validate_tree(&tree).is_err(),
            "mover that doesn't alternate"
        );

        let mut tree = small_tree();
        tree[1].first_child = 2;
        assert!(validate_tree(&tree).is_err(), "shared child");

        let mut tree = small_tree();
        tree[1].num_children = 5;
        assert!(validate_tree(&tree).is_err(), "children outside the tree");

        let mut tree = small_tree();
        tree[2].visits = 2;
        assert!(
            validate_tree(&tree).is_err(),
            "visited leaf that was never expanded"
        );
    }
}
