#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    fn check(pos: &mut AtaxxPosition, depth: i32) {
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
            assert_eq!(pos.blockers, old_pos.blockers);
            assert_eq!(pos.turn, old_pos.turn);
            assert_eq!(pos.halfmoves, old_pos.halfmoves);
            assert_eq!(pos.fullmoves, old_pos.fullmoves);
        }
    }

    #[test]
    fn consistent() {
        let fens = [
            "startpos",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "x5o/7/2-1-2/3-3/2-1-2/7/o5x x 0 1",
            "x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1",
            "7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1",
            "7/7/7/2x1o2/7/7/7 x 0 1",
        ];

        for fen in fens {
            let mut pos = AtaxxPosition::from_fen(fen);
            check(&mut pos, 3);
        }
    }
}
