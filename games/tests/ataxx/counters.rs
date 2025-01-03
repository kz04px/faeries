#[cfg(test)]
mod tests {
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
    };

    #[test]
    fn counters() {
        let history = [
            ("g2", 0, 1),
            ("a2", 0, 2),
            ("g2g4", 1, 2),
            ("a2a4", 2, 3),
            ("0000", 3, 3),
            ("0000", 4, 4),
            ("g5", 0, 4),
            ("a5", 0, 5),
        ];

        let mut pos = AtaxxPosition::startpos();

        for (movestr, halfmoves, fullmoves) in history {
            let mv = AtaxxMove::from_string(movestr).unwrap();
            pos.makemove(&mv);
            assert_eq!(pos.halfmoves, halfmoves);
            assert_eq!(pos.fullmoves, fullmoves);
        }

        for (movestr, halfmoves, fullmoves) in history.into_iter().rev() {
            assert_eq!(pos.halfmoves, halfmoves);
            assert_eq!(pos.fullmoves, fullmoves);
            let mv = AtaxxMove::from_string(movestr).unwrap();
            pos.undomove(&mv);
        }
    }
}
