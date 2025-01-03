use crate::gamerules::GameRules;
use std::time::Instant;

pub fn perft(
    pos: &mut impl GameRules,
    depth: i32,
    info_handler: &impl Fn(i32, f32, u64),
    final_handler: &impl Fn(u64),
) {
    let start = Instant::now();
    let mut last = 0;
    for i in 1..=depth {
        let nodes = pos.perft(i);
        last = nodes;
        info_handler(i, start.elapsed().as_secs_f32(), nodes);
    }
    final_handler(last);
}

pub fn split<G: GameRules>(
    pos: &mut G,
    depth: i32,
    info_handler: &impl Fn(G::MoveType, u64),
    final_handler: &impl Fn(u64),
) {
    let mut total = 0;
    for mv in pos.legal_moves() {
        pos.makemove(&mv);
        let nodes = pos.perft(depth - 1);
        pos.undomove(&mv);
        total += nodes;
        info_handler(mv, nodes);
    }
    final_handler(total);
}
