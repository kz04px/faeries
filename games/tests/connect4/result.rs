#[cfg(test)]
mod tests {
    use games::{
        connect4::Connect4Position,
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    #[test]
    fn results() {
        let tests = [
            ("7/7/7/7/7/7 r 1", None),
            ("7/7/7/7/7/rrrr3 r 1", Some(GameResult::Win(Side::Player1))),
            ("7/7/r6/r6/r6/r6 r 1", Some(GameResult::Win(Side::Player1))),
            ("7/7/7/7/7/yyyy3 y 1", Some(GameResult::Win(Side::Player2))),
            ("7/7/y6/y6/y6/y6 y 1", Some(GameResult::Win(Side::Player2))),
            (
                "rryyrry/yyrryyr/rryyrry/yyrryyr/rryyrry/yyrryyr y 1",
                Some(GameResult::Draw),
            ),
        ];

        for (fen, expected) in tests {
            let pos = Connect4Position::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{}", fen);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
