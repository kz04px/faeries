use games::tak::TakPosition;

#[must_use]
pub fn eval(pos: &TakPosition<6>) -> i32 {
    // Stacks owned
    100 * (pos.get_us().count() - pos.get_them().count())
}
