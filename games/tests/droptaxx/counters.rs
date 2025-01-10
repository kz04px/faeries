#[cfg(test)]
mod tests {
    use games::{
        droptaxx::{DroptaxxMove, DroptaxxPosition},
        gamerules::GameRules,
        general::square::Square,
    };

    #[test]
    fn counters() {
        let mut pos = DroptaxxPosition::startpos();
        assert_eq!(pos.fullmoves, 1);

        // Makemove d4
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("d4").unwrap()));
        assert_eq!(pos.fullmoves, 1);

        // Makemove d5
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("d5").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Makemove d6
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("d6").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Makemove e4
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("e4").unwrap()));
        assert_eq!(pos.fullmoves, 3);

        // Makemove e5
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("e5").unwrap()));
        assert_eq!(pos.fullmoves, 3);

        // Makemove e6
        pos.makemove(&DroptaxxMove(Square::<7, 7>::from_string("e6").unwrap()));
        assert_eq!(pos.fullmoves, 4);
    }
}
