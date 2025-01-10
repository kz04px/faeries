#[cfg(test)]
mod tests {
    use games::{droptaxx::DroptaxxPosition, gamerules::GameRules};

    #[test]
    fn fens() {
        let tests = [
            "7/7/7/7/7/7/7 x 1",
            "7/7/7/7/7/7/7 o 1",
            "7/7/7/7/7/7/7 x 1",
            "7/7/7/7/7/7/7 o 1",
            "7/7/7/7/7/7/7 x 10",
            "7/7/7/7/7/7/7 o 10",
            "x5o/7/7/7/7/7/o5x x 1",
            "x5o/7/7/7/7/7/o5x o 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 1",
        ];

        for fen in tests {
            assert_eq!(DroptaxxPosition::from_fen(fen).get_fen(), fen);
        }
    }
}
