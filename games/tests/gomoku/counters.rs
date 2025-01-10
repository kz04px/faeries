#[cfg(test)]
mod tests {
    use games::{
        gamerules::GameRules,
        general::square::Square,
        gomoku::{GomokuMove, GomokuPosition},
    };

    #[test]
    fn counters() {
        let mut pos = GomokuPosition::startpos();
        assert_eq!(pos.fullmoves, 1);

        // Makemove a1
        pos.makemove(&GomokuMove(Square::<15, 15>::from_string("a1").unwrap()));
        assert_eq!(pos.fullmoves, 1);

        // Makemove b1
        pos.makemove(&GomokuMove(Square::<15, 15>::from_string("b1").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Makemove c1
        pos.makemove(&GomokuMove(Square::<15, 15>::from_string("c1").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Makemove d1
        pos.makemove(&GomokuMove(Square::<15, 15>::from_string("d1").unwrap()));
        assert_eq!(pos.fullmoves, 3);

        // Undomove d1
        pos.undomove(&GomokuMove(Square::<15, 15>::from_string("d1").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Undomove c1
        pos.undomove(&GomokuMove(Square::<15, 15>::from_string("c1").unwrap()));
        assert_eq!(pos.fullmoves, 2);

        // Undomove b1
        pos.undomove(&GomokuMove(Square::<15, 15>::from_string("b1").unwrap()));
        assert_eq!(pos.fullmoves, 1);

        // Undomove a1
        pos.undomove(&GomokuMove(Square::<15, 15>::from_string("a1").unwrap()));
        assert_eq!(pos.fullmoves, 1);
    }
}
