#[cfg(test)]
mod tests {
    use games::{
        gamerules::{GameResult, GameRules},
        general::side::Side,
        tak::TakPosition,
    };

    #[test]
    fn results() {
        let tests = [
            ("startpos", None),
            ("x6/x6/x6/x6/x6/x,1,1,1,1,1 1 1", None),
            ("x6/x6/x6/x6/x6/x,2,2,2,2,2 1 1", None),
            ("x6/x6/x6/x6/x6/1,1,1,x,1,1 1 1", None),
            ("x6/x6/x6/x6/x6/2,2,2,x,2,2 1 1", None),
            ("x6/x6/x6/x6/x6/1,1,1,1,1,x 1 1", None),
            ("x6/x6/x6/x6/x6/2,2,2,2,2,x 1 1", None),
            ("x6/x6/x6/x6/x6/1,1,1,2,2,2 1 1", None),
            // Road - horizontal
            (
                "x6/x6/x6/x6/x6/1,1,1,1,1,1 1 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "x6/x6/x6/x6/x6/2,2,2,2,2,2 1 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Road - vertical
            (
                "1,x5/1,x5/1,x5/1,x5/1,x5/1,x5 1 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "2,x5/2,x5/2,x5/2,x5/2,x5/2,x5 1 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Caps are roads
            (
                "x6/x6/x6/x6/x6/1,1,1C,1,1,1 1 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "x6/x6/x6/x6/x6/2,2,2C,2,2,2 1 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Standing aren't roads
            ("x6/x6/x6/x6/x6/1,1,1,1S,1,1 1 1", None),
            ("x6/x6/x6/x6/x6/2,2,2,2S,2,2 1 1", None),
            ("1,x5/1,x5/1,x5/1S,x5/1,x5/1,x5 1 1", None),
            ("2,x5/2,x5/2,x5/2S,x5/2,x5/2,x5 1 1", None),
            // Long road
            (
                "1,1,1,1,1,x/x4,1,x/x,1,1,1,1,x/x,1,x4/x,1,1,1,1,1/x6 1 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "2,2,2,2,2,x/x4,2,x/x,2,2,2,2,x/x,2,x4/x,2,2,2,2,2/x6 2 2",
                Some(GameResult::Win(Side::Player2)),
            ),
            // Both sides have roads
            (
                "x6/x6/x6/x6/2,2,2,2,2,2/1,1,1,1,1,1 1 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "x6/x6/x6/x6/2,2,2,2,2,2/1,1,1,1,1,1 2 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            // Filled board
            (
                "2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Draw),
            ),
            (
                "2C,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Draw),
            ),
            (
                "2S,2S,2S,2S,2S,2S/1C,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Draw),
            ),
            (
                "2S,2S,2S,2S,2S,2S/1,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "2,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "2S,2S,2S,2S,2S,2S/11S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Draw),
            ),
            (
                "22S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S/2S,2S,2S,2S,2S,2S/1S,1S,1S,1S,1S,1S 1 1",
                Some(GameResult::Draw),
            ),
            // Count flats
            (
                "x6/x6/x2,211111111111111111111111111111,122222222222222222222222222222,x2/x2,1C,x3/x6/x6 2 60",
                Some(GameResult::Draw),
            ),
            (
                "x6/x6/x2,211111111111111111111111111111,12222222222222222222222222222,x2/x2,2C,2,x2/x6/x6 1 60",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "x6/x6/x2,21111111111111111111111111111,12222222222222222222222222222,x2/x2,2C,21,x2/x6/x6 1 60",
                Some(GameResult::Win(Side::Player1)),
            ),
        ];

        for (fen, expected) in tests {
            println!("fen: {}", fen);
            let pos = TakPosition::<6>::from_fen(fen);
            assert_eq!(pos.get_result(), expected);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
