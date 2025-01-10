#[cfg(test)]
mod tests {
    use games::{
        droptaxx::DroptaxxPosition,
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    #[test]
    fn results() {
        let tests = [
            ("7/7/7/7/7/7/7 x 1", None),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxx1ooo/ooooooo/ooooooo/ooooooo x 1",
                None,
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo o 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "xxxxxxx/xxxxxxx/xxxxxxx/xxxoooo/ooooooo/ooooooo/ooooooo o 1",
                Some(GameResult::Win(Side::Player2)),
            ),
        ];

        for (fen, expected) in tests {
            let pos = DroptaxxPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
