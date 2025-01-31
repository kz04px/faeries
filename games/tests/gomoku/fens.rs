#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, gomoku::GomokuPosition};

    #[test]
    fn fens() {
        let tests = [
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/15 x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/15 o 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/15 x 100",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/15 o 100",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x14 x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/o14 x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x13o x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/o13x x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x4o3x4o x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/o4x3o4x x 1",
        ];

        for fen in tests {
            let pos = GomokuPosition::from_fen(fen);
            assert_eq!(pos.get_fen(), fen);
        }
    }
}
