#[cfg(test)]
mod tests {
    use games::{
        gamerules::{GameResult, GameRules},
        general::side::Side,
        gomoku::GomokuPosition,
    };

    #[test]
    fn results() {
        let tests = [
            ("startpos", None),
            ("15/15/15/15/15/15/15/15/15/15/15/15/15/15/15 x 1", None),
            ("15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxx11 x 1", None),
            (
                "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxoooo2 x 1",
                None,
            ),
            // Right
            (
                "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxx10 x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "15/15/15/15/15/15/15/15/15/15/15/15/15/15/ooooo10 x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Up
            (
                "15/15/15/15/15/15/15/15/15/15/x14/x14/x14/x14/x14 x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "15/15/15/15/15/15/15/15/15/15/o14/o14/o14/o14/o14 x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Up right
            (
                "15/15/15/15/15/15/15/15/15/15/4x10/3x11/2x12/1x13/x14 x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "15/15/15/15/15/15/15/15/15/15/4o10/3o11/2o12/1o13/o14 x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Up left
            (
                "15/15/15/15/15/15/15/15/15/15/10x4/11x3/12x2/13x1/14x x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "15/15/15/15/15/15/15/15/15/15/10o4/11o3/12o2/13o1/14o x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Extra long
            (
                "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxxxxxxx1 x 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "15/15/15/15/15/15/15/15/15/15/15/15/15/15/oooooooooo1 x 1",
                Some(GameResult::Win(Side::Player2)),
            ),
        ];

        for (fen, expected) in tests {
            let pos = GomokuPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{}", fen);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
