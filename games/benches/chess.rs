#![feature(test)]

extern crate test;

use games::{chess::ChessPosition, gamerules::GameRules};

fn run_perft() {
    let fens = ["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"];

    for fen in fens {
        let mut pos = ChessPosition::from_fen(fen);
        std::hint::black_box(pos.perft(6));
    }
}

#[cfg(test)]
mod chess {
    use super::*;
    use test::Bencher;

    #[bench]
    fn perft(b: &mut Bencher) {
        b.iter(|| run_perft());
    }
}
