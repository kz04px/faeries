#[cfg(test)]
mod tests {
    use games::{connect4::Connect4Position, gamerules::GameRules};

    fn check(pos: &mut Connect4Position, depth: i32) {
        assert_eq!(pos.is_valid(), Ok(()));

        if depth == 0 {
            return;
        }

        for mv in pos.legal_moves() {
            let old_pos = pos.clone();
            pos.makemove(&mv);
            check(pos, depth - 1);
            pos.undomove(&mv);
            assert_eq!(pos.pieces, old_pos.pieces);
            assert_eq!(pos.turn, old_pos.turn);
            assert_eq!(pos.fullmoves, old_pos.fullmoves);
        }
    }

    #[test]
    fn consistent() {
        let fens = ["startpos"];

        for fen in fens {
            let mut pos = Connect4Position::from_fen(fen);
            check(&mut pos, 3);
        }
    }
}
