use games::{ataxx::AtaxxPosition, general::bitboard::Bitboard};
use tunable::tunable;

#[tunable(min = -50, max = 100, step = 4)]
#[rustfmt::skip]
const PST: [i32; 49] = [
     20,  45,  22,   2,  19,  40,  32,
     44,  -7,  10,  -5,  15,  26,  25,
      7,   0, -16, -20, -14,  29,  25,
     23, -10, -26, -40,   6,  -8,   5,
     16,  -5,  22,  -2, -11,  -5,  11,
     47,  30,  -1,  -5,  26,  12,  51,
     39,  24,  19,   6,  22,  32,  58,
];

#[tunable(min = 0, max = 60, step = 3)]
const SURROUNDED_BONUS: i32 = 33;
// Index 0 is never read: a hole is next to at least one of our pieces
#[tunable(min = -1200, max = 200, step = 20, skip = [0])]
const HOLE_PENALTIES: [i32; 9] = [0, 16, -8, -53, -95, -209, -302, -367, -512];

#[must_use]
fn eval_side(us: Bitboard<7, 7>, them: Bitboard<7, 7>, blockers: Bitboard<7, 7>) -> i32 {
    let mut score = 0;
    let empty = !(us | them | blockers);

    // Material
    score += 100 * us.count();

    // PST
    for sq in us {
        score += PST[sq.get_index()];
    }

    // Hole penalties
    for sq in us.adjacent() & empty & them.reach2() {
        let neighbours = Bitboard::from_square(sq).adjacent() & us;
        score += HOLE_PENALTIES[neighbours.count() as usize];
    }

    // Surrounded pieces
    score += SURROUNDED_BONUS.get() * (us & !empty.adjacent()).count();

    score
}

#[must_use]
pub fn eval(pos: &AtaxxPosition<7, 7>) -> i32 {
    let mut score = 0;
    score += eval_side(pos.get_us(), pos.get_them(), pos.get_blockers());
    score -= eval_side(pos.get_them(), pos.get_us(), pos.get_blockers());
    score
}
