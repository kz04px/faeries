#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, gomoku::GomokuPosition};

    fn check(pos: &mut GomokuPosition, depth: i32) {
        assert_eq!(pos.is_valid(), Ok(()));

        if depth == 0 {
            return;
        }

        for mv in pos.legal_moves() {
            let old_pos = pos.clone();
            pos.makemove(&mv);
            check(pos, depth - 1);
            pos.undomove(&mv);
            assert_eq!(pos.board, old_pos.board);
            assert_eq!(pos.turn, old_pos.turn);
            assert_eq!(pos.fullmoves, old_pos.fullmoves);
        }
    }

    #[test]
    fn consistent() {
        let fens = ["startpos"];

        for fen in fens {
            let mut pos = GomokuPosition::from_fen(fen);
            check(&mut pos, 2);
        }
    }
}
