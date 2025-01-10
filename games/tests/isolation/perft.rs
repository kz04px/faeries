#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, isolation::IsolationPosition};

    #[test]
    fn perft() {
        let tests = [
            ("startpos", vec![1, 225, 47300, 10998540]),
            ("8/8/8/8/8/pP6 b", vec![1, 0]),
            ("8/8/8/8/8/p.P5 b", vec![1, 1, 0]),
            ("8/8/8/8/8/p..P4 b", vec![1, 2, 1, 0]),
            ("8/8/8/8/8/p...P3 b", vec![1, 3, 4, 2]),
            ("8/8/8/8/8/p....P2 b", vec![1, 4, 9, 16, 6, 0]),
        ];

        for (fen, results) in tests {
            println!("FEN: {}", fen);
            let mut pos = IsolationPosition::from_fen(fen);
            for (depth, nodes) in results.iter().enumerate() {
                let slow = pos.perft(depth as i32);
                assert_eq!(*nodes, slow, "Slow perft mismatch depth {}", depth);
            }
        }
    }
}
