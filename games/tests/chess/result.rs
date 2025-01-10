#[cfg(test)]
mod tests {
    use games::{
        chess::ChessPosition,
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    #[test]
    fn results() {
        let tests = [
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                None,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 99",
                None,
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 100",
                Some(GameResult::Draw),
            ),
            (
                "1R2k3/8/4K3/8/8/8/8/8 b - - 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "8/8/8/8/8/4k3/8/1r2K3 w - - 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
        ];

        for (fen, expected) in tests {
            let pos = ChessPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{}", fen);
            assert_eq!(pos.is_gameover(), expected.is_some());
        }
    }
}
