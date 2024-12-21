#[cfg(test)]
mod tests {
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
        general::square::Square,
    };

    #[test]
    fn counters() {
        let mut pos = AtaxxPosition::startpos();
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 1);

        // Makemove g2
        pos.makemove(&AtaxxMove::Single(
            Square::<7, 7>::from_string("g2").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 1);

        // Makemove a2
        pos.makemove(&AtaxxMove::Single(
            Square::<7, 7>::from_string("a2").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 2);

        // Makemove g2g4
        pos.makemove(&AtaxxMove::Double(
            Square::<7, 7>::from_string("g2").unwrap(),
            Square::<7, 7>::from_string("g4").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 1);
        assert_eq!(pos.fullmoves, 2);

        // Makemove a2a4
        pos.makemove(&AtaxxMove::Double(
            Square::<7, 7>::from_string("a2").unwrap(),
            Square::<7, 7>::from_string("a4").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 2);
        assert_eq!(pos.fullmoves, 3);

        // Makemove pass
        pos.makemove(&AtaxxMove::Pass);
        assert_eq!(pos.halfmoves, 3);
        assert_eq!(pos.fullmoves, 3);

        // Makemove pass
        pos.makemove(&AtaxxMove::Pass);
        assert_eq!(pos.halfmoves, 4);
        assert_eq!(pos.fullmoves, 4);

        // Makemove g5
        pos.makemove(&AtaxxMove::Single(
            Square::<7, 7>::from_string("a2").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 4);

        // Makemove a5
        pos.makemove(&AtaxxMove::Single(
            Square::<7, 7>::from_string("a2").unwrap(),
        ));
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 5);
    }
}
