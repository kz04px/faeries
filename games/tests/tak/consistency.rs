use games::{
    gamerules::GameRules,
    tak::{PieceType, TakPosition},
};

fn check<const SIZE: usize>(pos: &mut TakPosition<SIZE>, depth: i32) {
    assert_eq!(pos.is_valid(), Ok(()));

    if depth == 0 {
        return;
    }

    for mv in pos.legal_moves() {
        let old_pos = pos.clone();
        pos.makemove(&mv);
        check(pos, depth - 1);
        pos.undomove(&mv);

        assert_eq!(pos.num_pieces, old_pos.num_pieces, "Num pieces mismatch");
        assert_eq!(pos.num_caps, old_pos.num_caps, "Num caps mismatch");
        assert_eq!(pos.turn, old_pos.turn, "Turn mismatch");
        assert_eq!(pos.fullmoves, old_pos.fullmoves, "Fullmoves mismatch");
        assert_eq!(
            pos.piece_masks[PieceType::Flat as usize],
            old_pos.piece_masks[PieceType::Flat as usize],
            "mask flat mismatch"
        );
        assert_eq!(
            pos.piece_masks[PieceType::Cap as usize],
            old_pos.piece_masks[PieceType::Cap as usize],
            "mask cap mismatch"
        );
        assert_eq!(
            pos.piece_masks[PieceType::Standing as usize],
            old_pos.piece_masks[PieceType::Standing as usize],
            "mask standing mismatch"
        );
        for x in 0..SIZE {
            for y in 0..SIZE {
                assert_eq!(pos.stacks[x][y], old_pos.stacks[x][y], "stack mismatch");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn consistency() {
        let depth = 2;
        check(&mut TakPosition::<3>::startpos(), depth);
        check(&mut TakPosition::<4>::startpos(), depth);
        check(&mut TakPosition::<5>::startpos(), depth);
        check(&mut TakPosition::<6>::startpos(), depth);
        check(&mut TakPosition::<7>::startpos(), depth);
        check(&mut TakPosition::<8>::startpos(), depth);
    }

    #[test]
    fn consistency_6x6() {
        let fens = [
            "x6/x6/x6/x6/x6/x6 1 1",
            // Side to move
            "x6/x6/x6/x6/x6/x6 2 1",
            // Counters
            "x6/x6/x6/x6/x6/x6 1 100",
            "x6/x6/x6/x6/x6/x6 2 100",
            // Flat
            "2,x5/x6/x6/x6/x6/x6 2 1",
            "x,2,x4/x6/x6/x6/x6/x6 2 1",
            "x2,2,x3/x6/x6/x6/x6/x6 2 1",
            "x3,2,x2/x6/x6/x6/x6/x6 2 1",
            "x5,2/x4,1,x/x3,2,x2/x2,1,x3/x,2,x4/1,x5 1 4",
            "x4,1,2/x3,2,1,x/x2,1,2,x2/x,2,1,x3/1,2,x4/1,x5 2 6",
            "x3,2,x,1/x,1,x3,2/x,1,1,1,1,x/x,2,2,2,2,x/x3,1,x2/x6 2 7",
            // Standing
            "2,1,x4/1S,2S,x4/x6/x6/x6/x6 1 3",
            // Caps
            "2,1,x4/1C,2C,x4/x6/x6/x6/x6 1 3",
            // Mixed
            "x2,2C,2S,2,x/x2,1C,1S,1,x/x6/x6/x6/x6 2 4",
            // Stacks
            "21,12,x4/x6/x6/x6/x6/x6 1 4",
            "211,122,x4/x6/x6/x6/x6/x6 1 6",
            "21S,12S,x4/x6/x6/x6/x6/x6 1 4",
            "211S,122S,x4/x6/x6/x6/x6/x6 1 6",
            "21C,12C,x4/x6/x6/x6/x6/x6 1 4",
            "211C,122C,x4/x6/x6/x6/x6/x6 1 6",
            "x6/x6/x6/x6/x6/1111,x5 1 1",
            "x6/x6/x6/x6/x6/1212,x5 1 1",
            "x6/x6/x6/x6/x6/2121,x5 1 1",
            "x6/x6/x6/x6/x6/1111,x5 2 1",
            "x6/x6/x6/x6/x6/1212,x5 2 1",
            "x6/x6/x6/x6/x6/2121,x5 2 1",
            // Huge stack
            "x6/x6/x6/x6/x6/1111111111,x5 1 1",
            "x6/x6/x6/x6/x6/111111111111111111,x5 1 1",
            "x6/x6/x2,121212121212121212121212121212,x3/x6/x6/x6 1 33",
            "x6/x6/x2,121212121212121212121212121212S,x3/x6/x6/x6 1 33",
            "x6/x6/x2,121212121211212121212112121212C,x3/x6/x6/x6 1 33",
            "x6/x6/x2,121212121212121212121212121212,x3/x6/x6/x6 2 33",
            "x6/x6/x2,121212121212121212121212121212S,x3/x6/x6/x6 2 33",
            "x6/x6/x2,121212121212121212121212121212C,x3/x6/x6/x6 2 33",
            // Other
            "x,2,1,x3/x,111S,22,2S,222111,1/x2,2,1S,2,2S/x,121S,1,2S,2C,2/2,x,212S,2,2,1S/x,1221C,2,x,21,x 2 29",
            "x3,12,2S,x/x,22S,22C,11,21,x/121,212,12,1121C,1212S,x/21S,1,21,211S,12S,x/x,21S,2,x3/x6 1 26",
            "1,x,1,1,1,x/1,11112C,111121C,2S,x,1/2,x,1122,2S,1,1/2,x,2S,x2,2/2,2,1S,2,2221S,2/2,x,112,x2,2 1 33",
            "1,x,1,x3/x,2,1121C,x3/1112,x,2,x3/2,2,x,2,x2/x2,1212,x3/2C,1,1,x3 1 18",
            "2,x2,1,2,1/x3,1,2C,1/x3,121C,12,1/x2,2,2,2,2/x4,1,x/x4,1,x 1 12",
            "1,x,1,x3/x2,1111212,2,x2/x,1,21,212,x,1/x2,21,2,2,2C/x2,21C,x2,2/2,x5 2 22",
            "x2,2,x2,1/x,122,121C,212,x2/x2,2,2,2,2C/1,x2,2,x,1/x2,1,x2,1/1,x2,2111112,x,1 2 24",
            "2,x,222212,x,2,12C/x,221S,x2,2,x/21,x3,1,x/22,221,1121S,x,1,x/2112,221S,x4/x,1C,1,1,x2 2 48",
            "2,x2,2,1,x/2,x,12,x,1,112S/2,21S,221C,211111,2,1/x,1S,x,22212C,x2/1S,2,2,x2,21/x,21121,2,12,2,2 2 36",
        ];

        for fen in fens {
            println!("fen: {}", fen);
            let mut pos = TakPosition::<6>::from_fen(fen);
            check(&mut pos, 2);
        }
    }
}
