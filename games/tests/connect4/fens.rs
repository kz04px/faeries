#[cfg(test)]
mod tests {
    use games::{connect4::Connect4Position, gamerules::GameRules};

    #[test]
    fn fens() {
        let tests = [
            "7/7/7/7/7/7 r 1",
            "7/7/7/7/7/7 y 1",
            "7/7/7/7/7/7 r 100",
            "7/7/7/7/7/7 y 100",
            "7/7/7/7/7/3r3 y 1",
            "7/7/7/7/7/3y3 r 1",
            "7/7/7/7/7/3ry2 r 1",
        ];

        for fen in tests {
            assert_eq!(Connect4Position::from_fen(fen).get_fen(), fen);
        }
    }
}
