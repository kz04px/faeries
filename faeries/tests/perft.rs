#[cfg(test)]
mod perft {
    use faeries::{hashtable::HashTable, ttperft::ttperft};
    use games::{chess::ChessPosition, gamerules::GameRules};

    // Positions and node counts from https://www.chessprogramming.org/Perft_Results
    const POSITIONS: [(&str, &[u64]); 5] = [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            &[20, 400, 8_902, 197_281],
        ),
        (
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            &[48, 2_039, 97_862],
        ),
        (
            "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
            &[14, 191, 2_812, 43_238],
        ),
        (
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            &[6, 264, 9_467],
        ),
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            &[44, 1_486, 62_379],
        ),
    ];

    fn check(mb: usize) {
        for (fen, counts) in POSITIONS {
            let mut pos = ChessPosition::from_fen(fen);
            let mut table = HashTable::new(mb);
            for (i, &expected) in counts.iter().enumerate() {
                let depth = i as i32 + 1;
                let nodes = ttperft(&mut pos, depth, &mut table, &ChessPosition::hash);
                assert_eq!(nodes, expected, "{fen} depth {depth}");
            }
        }
    }

    #[test]
    fn ttperft_matches_known_counts() {
        check(64);
    }

    #[test]
    fn ttperft_with_small_table() {
        check(1);
    }

    #[test]
    fn ttperft_without_table() {
        check(0);
    }
}
