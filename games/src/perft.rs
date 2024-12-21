use crate::gamerules::GameRules;
use std::time::Instant;

#[must_use]
pub fn perft_impl(pos: &impl GameRules, depth: i32) -> u64 {
    match depth {
        0 => 1,
        1 => pos.count_moves(),
        _ => {
            let mut nodes = 0;
            pos.move_generator(|mv| {
                let npos = pos.after_move(&mv);
                nodes += perft_impl(&npos, depth - 1);
            });
            nodes
        }
    }
}

pub fn perft(
    pos: &impl GameRules,
    depth: i32,
    info_handler: impl Fn(i32, f32, u64),
    final_handler: impl Fn(u64),
) {
    let start = Instant::now();
    let mut last = 0;
    for i in 1..=depth {
        let nodes = perft_impl(pos, i);
        last = nodes;
        info_handler(i, start.elapsed().as_secs_f32(), nodes);
    }
    final_handler(last);
}

pub fn split<G: GameRules>(
    pos: &G,
    depth: i32,
    info_handler: impl Fn(G::MoveType, u64),
    final_handler: impl Fn(u64),
) {
    let mut total = 0;
    pos.move_generator(|mv| {
        let npos = pos.after_move(&mv);
        let nodes = perft_impl(&npos, depth - 1);
        total += nodes;
        info_handler(mv, nodes);
    });
    final_handler(total);
}
