#[cfg(test)]
mod tests {
    use games::{chess::ChessPosition, gamerules::GameRules};

    #[test]
    fn counters() {
        let tests = [
            ("e2e4", 0, 1),
            ("g8f6", 1, 2),
            ("e4e5", 0, 2),
            ("d7d5", 0, 3),
            ("e5d6", 0, 3),
            ("d8d6", 0, 4),
            ("g1f3", 1, 4),
            ("e7e6", 0, 5),
            ("f1e2", 1, 5),
            ("f8e7", 2, 6),
            ("e1g1", 3, 6),
            ("e8g8", 4, 7),
        ];

        let mut pos = ChessPosition::startpos();
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 1);

        for (movestr, halfmoves, fullmoves) in tests {
            let mv = pos.parse_movestr(movestr).unwrap();
            pos.makemove(&mv);
            assert_eq!(pos.halfmoves, halfmoves);
            assert_eq!(pos.fullmoves, fullmoves);
        }
    }
}
