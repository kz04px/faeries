use games::{general::side::Side, tak::TakPosition};

/// splitmix64 finaliser
#[must_use]
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// Hash of the board and side to move, computed from scratch.
///
/// Reserves aren't hashed: pieces never leave the board, so the board determines them.
#[must_use]
pub fn hash(pos: &TakPosition<6>) -> u64 {
    let mut hash = mix((pos.turn == Side::Player1) as u64);

    for sq in pos.side_masks[0] | pos.side_masks[1] {
        let stack = &pos.stacks[sq.get_file()][sq.get_rank()];
        let height = stack.get_height() as usize;
        let top = pos.get_piece_on(sq).unwrap();

        let mut colours = 0u64;
        for idx in 0..height {
            colours |= (stack.is_player1(idx) as u64) << idx;
        }

        let meta = sq.get_index() as u64 | (height as u64) << 8 | (top as u64) << 16;
        hash = mix(mix(hash ^ meta) ^ colours);
    }

    hash
}
