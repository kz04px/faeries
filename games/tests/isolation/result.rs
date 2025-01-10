#[cfg(test)]
mod tests {
    use games::{
        gamerules::{GameResult, GameRules},
        general::side::Side,
        isolation::IsolationPosition,
    };

    #[test]
    fn results() {
        let tests = [
            ("startpos", None),
            ("8/8/8/8/8/p......P b", None),
            ("8/8/8/8/8/p......P r", None),
            // No adjacent tiles
            ("8/8/8/8/8/p.....1P b", Some(GameResult::Win(Side::Player2))),
            ("8/8/8/8/8/p.....1P r", Some(GameResult::Win(Side::Player2))),
            ("8/8/8/8/8/p1.....P b", Some(GameResult::Win(Side::Player1))),
            ("8/8/8/8/8/p1.....P r", Some(GameResult::Win(Side::Player1))),
            ("8/8/8/8/8/p1....1P b", Some(GameResult::Win(Side::Player2))),
            ("8/8/8/8/8/p1....1P r", Some(GameResult::Win(Side::Player1))),
            ("8/8/8/8/8/6Pp b", Some(GameResult::Win(Side::Player2))),
            ("8/8/8/8/8/6Pp r", Some(GameResult::Win(Side::Player1))),
            // Last tiled square blocked
            ("8/8/8/8/8/5.Pp b", None),
            ("8/8/8/8/8/5.Pp r", Some(GameResult::Win(Side::Player1))),
            ("8/8/8/8/8/5.pP b", Some(GameResult::Win(Side::Player2))),
            ("8/8/8/8/8/5.pP r", None),
        ];

        for (fen, expected) in tests {
            let pos = IsolationPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{}", fen);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
