#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, isolation::IsolationPosition, perft::perft_impl};

    #[test]
    fn perft() {
        let tests = [
            ("startpos", vec![1, 225, 47300, 10998540]),
            ("8/8/8/8/8/pP6 b", vec![1, 0]),
            ("8/8/8/8/8/p.P5 b", vec![1, 1, 0]),
            ("8/8/8/8/8/p..P4 b", vec![1, 2, 1, 0]),
            ("8/8/8/8/8/p...P3 b", vec![1, 3, 4, 2]),
            ("8/8/8/8/8/p....P2 b", vec![1, 4, 9, 16, 6, 0]),
            // (
            //     "7/7/7/p..5/...5/..P5 b",
            //     vec![1, 21, 288, 5050, 57984, 293868, 682336, 478136, 0],
            // ),
        ];

        for (fen, results) in tests {
            println!("FEN: {}", fen);
            let pos = IsolationPosition::from_fen(fen);
            for (depth, nodes) in results.iter().enumerate() {
                let slow = perft_impl(&pos, depth as i32);
                assert_eq!(*nodes, slow, "Slow perft mismatch depth {}", depth);

                // let fast = pos.fast_perft(depth as i32);
                // assert_eq!(*nodes, fast, "Fast perft mismatch depth {}", depth);
            }
        }
    }
}
