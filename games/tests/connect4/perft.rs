#[cfg(test)]
mod tests {
    use games::{connect4::Connect4Position, gamerules::GameRules, perft::perft_impl};

    #[test]
    fn perft() {
        let tests = [
            ("startpos", vec![1, 7, 49]),
            ("7/7/7/7/7/rrrr3 r 1", vec![1, 0]),
            ("7/7/7/7/7/rrrr3 y 1", vec![1, 0]),
            ("7/7/7/7/7/yyyy3 r 1", vec![1, 0]),
            ("7/7/7/7/7/yyyy3 y 1", vec![1, 0]),
            ("7/7/7/7/7/rrr4 r 1", vec![1, 7, 42]),
            ("7/7/7/7/7/yyy4 y 1", vec![1, 7, 42]),
            ("7/7/7/7/7/yyy4 r 1", vec![1, 7, 49, 301]),
            ("7/7/7/7/7/rrr4 y 1", vec![1, 7, 49, 301]),
        ];

        for (fen, results) in tests {
            println!("FEN: {}", fen);
            let pos = Connect4Position::from_fen(fen);
            for (depth, nodes) in results.iter().enumerate() {
                let result = perft_impl(&pos, depth as i32);
                assert_eq!(*nodes, result, "Perft mismatch depth {}", depth);
            }
        }
    }
}
