#[cfg(test)]
mod tests {
    use games::{
        connect4::{Connect4Move, Connect4Position},
        gamerules::GameRules,
        general::file::File,
    };

    #[test]
    fn counters() {
        let mut pos = Connect4Position::startpos();
        assert_eq!(pos.fullmoves, 1);

        // Makemove d
        pos.makemove(&Connect4Move(File(3)));
        assert_eq!(pos.fullmoves, 1);

        // Makemove e
        pos.makemove(&Connect4Move(File(4)));
        assert_eq!(pos.fullmoves, 2);

        // Makemove f
        pos.makemove(&Connect4Move(File(5)));
        assert_eq!(pos.fullmoves, 2);

        // Makemove g
        pos.makemove(&Connect4Move(File(6)));
        assert_eq!(pos.fullmoves, 3);
    }
}
