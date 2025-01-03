#[cfg(test)]
mod tests {
    use games::{
        ataxx::AtaxxPosition,
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    #[test]
    fn results() {
        let tests = [
            ("7/7/7/7/7/7/7 x 0 1", Some(GameResult::Draw)),
            ("startpos", None),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxx1ooo/ooooooo/ooooooo/ooooooo x 0 1",
                None,
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo o 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo x 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo o 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "7/7/7/7/-------/-------/oooxxxx x 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "7/7/7/7/-------/-------/ooooxxx x 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "7/7/7/7/-------/-------/ooo-xxx x 0 1",
                Some(GameResult::Draw),
            ),
            ("x5o/7/7/7/7/7/o5x x 99 1", None),
            ("x5o/7/7/7/7/7/o5x x 100 1", Some(GameResult::Draw)),
            ("x5o/7/7/7/7/7/o5x x 101 1", Some(GameResult::Draw)),
        ];

        for (fen, expected) in tests {
            let pos = AtaxxPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{}", fen);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
