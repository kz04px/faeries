use games::{ataxx::AtaxxPosition, general::bitboard::Bitboard};

#[rustfmt::skip]
const PST: [i32; 49] = [
    30, 20, 10, 10, 10, 20, 30,
    20, 10, 10,  5, 10, 10, 20,
    10, 10,  5,  0,  5, 10, 10,
    10,  5,  0,  0,  0,  5, 10,
    10, 10,  5,  0,  5, 10, 10,
    20, 10, 10,  5, 10, 10, 20,
    30, 20, 10, 10, 10, 20, 30,
];

const SURROUNDED_BONUS: i32 = 15;
const HOLE_PENALTIES: [i32; 9] = [0, 0, 0, 0, -100, -200, -300, -400, -500];

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
    score += SURROUNDED_BONUS * (us & !empty.adjacent()).count();

    score
}

#[must_use]
pub fn eval(pos: &AtaxxPosition<7, 7>) -> i32 {
    let mut score = 0;
    score += eval_side(pos.get_us(), pos.get_them(), pos.get_blockers());
    score -= eval_side(pos.get_them(), pos.get_us(), pos.get_blockers());
    score
}
