use games::ataxx::AtaxxPosition;

#[must_use]
pub fn eval(pos: &AtaxxPosition<7, 7>) -> i32 {
    100 * (pos.get_us().count() - pos.get_them().count())
}
