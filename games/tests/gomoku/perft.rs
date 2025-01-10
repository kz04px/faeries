#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, gomoku::GomokuPosition};

    #[test]
    fn perft() {
        let tests = [("startpos", vec![1, 225, 50400, 11239200])];

        for (fen, results) in tests {
            let mut pos = GomokuPosition::from_fen(fen);
            for (depth, nodes) in results.iter().enumerate() {
                let slow = pos.perft(depth as i32);
                assert_eq!(*nodes, slow, "perft mismatch depth {}", depth);
            }
        }
    }
}
