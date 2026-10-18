use games::tak::{PieceType, TakPosition};

const PST: [[i32; 6 * 6]; 3] = [
    // Flat
    [
        0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, // B
        0, 0, 0, 0, 0, 0, // C
        0, 0, 0, 0, 0, 0, // D
        0, 0, 0, 0, 0, 0, // E
        0, 0, 0, 0, 0, 0, // F
    ],
    // Standing
    [
        0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, // B
        0, 0, 0, 0, 0, 0, // C
        0, 0, 0, 0, 0, 0, // D
        0, 0, 0, 0, 0, 0, // E
        0, 0, 0, 0, 0, 0, // F
    ],
    // Cap
    [
        0, 0, 0, 0, 0, 0, // A
        0, 0, 0, 0, 0, 0, // B
        0, 0, 20, 20, 0, 0, // C
        0, 0, 20, 20, 0, 0, // D
        0, 0, 0, 0, 0, 0, // E
        0, 0, 0, 0, 0, 0, // F
    ],
];

#[must_use]
pub fn eval(pos: &TakPosition<6>) -> i32 {
    let mut score = 0;

    // Stacks owned
    score += 100 * (pos.get_us().count() - pos.get_them().count());

    // PST
    for piece in [PieceType::Flat, PieceType::Standing, PieceType::Cap] {
        let mask = pos.piece_masks[piece as usize];
        for sq in pos.get_us() & mask {
            score += PST[piece as usize][sq.get_index()];
        }
        for sq in pos.get_them() & mask {
            score -= PST[piece as usize][sq.get_index()];
        }
    }

    score
}
