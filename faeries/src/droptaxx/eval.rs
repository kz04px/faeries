use games::droptaxx::DroptaxxPosition;
use games::general::bitboard::Bitboard;

#[must_use]
fn eval_side(us: Bitboard<7, 7>, _them: Bitboard<7, 7>, _blockers: Bitboard<7, 7>) -> i32 {
    // Material count
    100 * us.count()
}

#[must_use]
fn eval_us(pos: &DroptaxxPosition) -> i32 {
    eval_side(pos.get_us(), pos.get_them(), pos.get_blockers())
}

#[must_use]
fn eval_them(pos: &DroptaxxPosition) -> i32 {
    eval_side(pos.get_them(), pos.get_us(), pos.get_blockers())
}

#[must_use]
pub fn eval(pos: &DroptaxxPosition) -> i32 {
    eval_us(pos) - eval_them(pos)
}
