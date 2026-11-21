use crate::score::{S, Score};
use games::{ataxx::AtaxxPosition, general::bitboard::Bitboard};
use tunable::tunable;

#[tunable(min = -50, max = 100, step = 4)]
#[rustfmt::skip]
const PST: [Score; 49] = [
    S( 14,  20), S( 38,  60), S( 40,  13), S( -6, -15), S( 17,  28), S( 34,  38), S( 36,  53),
    S( 41,  61), S( -8,  -2), S( -7,  16), S( -9,  -4), S(  8,  11), S( 23,  23), S( 48,  34),
    S(  1,  17), S( 14,  11), S(-16,  -2), S(-22, -15), S( -7, -27), S( 35,  24), S(  6,  30),
    S( 13,  26), S(-19,  -5), S(-24, -17), S(-50, -38), S( -2,   2), S( -8, -30), S( 11,   2),
    S( 13,  37), S(  3,  -8), S( 21,  22), S(  0, -17), S(-12, -10), S(-17,   0), S( 11,  17),
    S( 43,  34), S( 35,  35), S(-10, -14), S(  7, -15), S( 27,  28), S( 10,  14), S( 69,  58),
    S( 49,  51), S( 33,  46), S( 13,  37), S( 18,   9), S( 17,  32), S( 27,  47), S( 61,  53),
];

#[tunable(min = 20, max = 1260, step = 5)]
const MATERIAL: Score = S(110, 100);
#[tunable(min = 0, max = 60, step = 3)]
const SURROUNDED_BONUS: Score = S(55, 41);
// Index 0 is never read: a hole is next to at least one of our pieces
#[tunable(min = -1200, max = 200, step = 20, skip = [0])]
const HOLE_PENALTIES: [Score; 9] = [
    S(0, 0),
    S(-16, 32),
    S(-37, 24),
    S(-90, -4),
    S(-116, -85),
    S(-233, -153),
    S(-302, -251),
    S(-311, -390),
    S(-436, -659),
];

#[must_use]
fn eval_side(us: Bitboard<7, 7>, them: Bitboard<7, 7>, blockers: Bitboard<7, 7>) -> Score {
    let mut score = Score::default();
    let empty = !(us | them | blockers);

    // Material
    score += MATERIAL.get() * us.count();

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
    let mut score = Score::default();
    score += eval_side(pos.get_us(), pos.get_them(), pos.get_blockers());
    score -= eval_side(pos.get_them(), pos.get_us(), pos.get_blockers());

    let phase = (pos.get_us() | pos.get_them()).count();
    let max_phase = (!pos.get_blockers()).count();
    score.taper(phase, max_phase)
}
