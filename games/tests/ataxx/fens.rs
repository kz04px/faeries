#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    #[test]
    fn fens() {
        let tests = [
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "7/7/7/7/7/7/7 x 0 1",
            "7/7/7/7/7/7/7 o 0 1",
            "7/7/7/7/7/7/7 x 0 10",
            "7/7/7/7/7/7/7 o 0 10",
            "7/7/7/7/7/7/7 x 10 10",
            "7/7/7/7/7/7/7 o 10 10",
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/7/7/7/7/o5x o 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 0 1",
            "xo5/7/7/7/7/7/7 x 0 1",
            "x1o4/7/7/7/7/7/7 x 0 1",
            "x2o3/7/7/7/7/7/7 x 0 1",
            "x1o1x2/7/7/7/7/7/7 x 0 1",
        ];

        for fen in tests {
            assert_eq!(AtaxxPosition::from_fen(fen).get_fen(), fen);
        }
    }
}
