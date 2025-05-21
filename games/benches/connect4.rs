#![feature(test)]

extern crate test;

use games::{connect4::Connect4Position, gamerules::GameRules};

fn run_perft() {
    let fens: [&str; 49] = [
        "7/7/7/7/7/ry5 r",
        "7/7/7/7/7/r1y4 r",
        "7/7/7/7/7/r2y3 r",
        "7/7/7/7/7/r3y2 r",
        "7/7/7/7/7/r4y1 r",
        "7/7/7/7/7/r5y r",
        "7/7/7/7/y6/r6 r",
        "7/7/7/7/7/yr5 r",
        "7/7/7/7/7/1ry4 r",
        "7/7/7/7/7/1r1y3 r",
        "7/7/7/7/7/1r2y2 r",
        "7/7/7/7/7/1r3y1 r",
        "7/7/7/7/7/1r4y r",
        "7/7/7/7/1y5/1r5 r",
        "7/7/7/7/7/y1r4 r",
        "7/7/7/7/7/1yr4 r",
        "7/7/7/7/7/2ry3 r",
        "7/7/7/7/7/2r1y2 r",
        "7/7/7/7/7/2r2y1 r",
        "7/7/7/7/7/2r3y r",
        "7/7/7/7/2y4/2r4 r",
        "7/7/7/7/7/y2r3 r",
        "7/7/7/7/7/1y1r3 r",
        "7/7/7/7/7/2yr3 r",
        "7/7/7/7/7/3ry2 r",
        "7/7/7/7/7/3r1y1 r",
        "7/7/7/7/7/3r2y r",
        "7/7/7/7/3y3/3r3 r",
        "7/7/7/7/7/y3r2 r",
        "7/7/7/7/7/1y2r2 r",
        "7/7/7/7/7/2y1r2 r",
        "7/7/7/7/7/3yr2 r",
        "7/7/7/7/7/4ry1 r",
        "7/7/7/7/7/4r1y r",
        "7/7/7/7/4y2/4r2 r",
        "7/7/7/7/7/y4r1 r",
        "7/7/7/7/7/1y3r1 r",
        "7/7/7/7/7/2y2r1 r",
        "7/7/7/7/7/3y1r1 r",
        "7/7/7/7/7/4yr1 r",
        "7/7/7/7/7/5ry r",
        "7/7/7/7/5y1/5r1 r",
        "7/7/7/7/7/y5r r",
        "7/7/7/7/7/1y4r r",
        "7/7/7/7/7/2y3r r",
        "7/7/7/7/7/3y2r r",
        "7/7/7/7/7/4y1r r",
        "7/7/7/7/7/5yr r",
        "7/7/7/7/6y/6r r",
    ];

    for fen in fens {
        let mut pos = Connect4Position::from_fen(fen);
        std::hint::black_box(pos.perft(6));
    }
}

#[cfg(test)]
mod connect4 {
    use super::*;
    use test::Bencher;

    #[bench]
    fn perft(b: &mut Bencher) {
        b.iter(|| run_perft());
    }
}
