#[cfg(test)]
mod tests {
    use games::{
        gamerules::GameRules,
        general::square::Square,
        isolation::{IsolationMove, IsolationPosition},
    };

    #[test]
    fn counters() {
        let mut pos = IsolationPosition::startpos();
        assert_eq!(pos.fullmoves, 1);

        // Makemove g3a1
        pos.makemove(&IsolationMove {
            to: Square::<8, 6>::from_string("g3").unwrap(),
            remove: Square::<8, 6>::from_string("a1").unwrap(),
        });
        assert_eq!(pos.fullmoves, 1);

        // Makemove b4b1
        pos.makemove(&IsolationMove {
            to: Square::<8, 6>::from_string("b4").unwrap(),
            remove: Square::<8, 6>::from_string("b1").unwrap(),
        });
        assert_eq!(pos.fullmoves, 2);

        // Makemove f2c1
        pos.makemove(&IsolationMove {
            to: Square::<8, 6>::from_string("f2").unwrap(),
            remove: Square::<8, 6>::from_string("c1").unwrap(),
        });
        assert_eq!(pos.fullmoves, 2);

        // Makemove c5d1
        pos.makemove(&IsolationMove {
            to: Square::<8, 6>::from_string("c5").unwrap(),
            remove: Square::<8, 6>::from_string("d1").unwrap(),
        });
        assert_eq!(pos.fullmoves, 3);
    }
}
