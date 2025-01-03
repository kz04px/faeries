#[cfg(test)]
mod tests {
    use games::{droptaxx::DroptaxxPosition, gamerules::GameRules};

    #[test]
    fn perft() {
        let tests = [
            ("startpos", vec![1, 49, 2352, 110544]),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxx1ooo/ooooooo/ooooooo/ooooooo x 1",
                vec![1, 1, 0],
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xx3oo/ooooooo/ooooooo/ooooooo x 1",
                vec![1, 3, 6, 6, 0],
            ),
        ];

        for (fen, results) in tests {
            println!("FEN: {}", fen);
            let mut pos = DroptaxxPosition::from_fen(fen);
            for (depth, nodes) in results.iter().enumerate() {
                let result = pos.perft(depth as i32);
                assert_eq!(*nodes, result, "Perft mismatch depth {}", depth);
            }
        }
    }
}
