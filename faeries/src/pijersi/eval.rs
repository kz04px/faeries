use games::pijersi::PijersiPosition;

#[must_use]
pub fn eval(pos: &PijersiPosition) -> i32 {
    let us_lower = pos.get_lower() & pos.get_us();
    let us_upper = pos.get_upper() & pos.get_us();
    let num_us = us_lower.count() + us_upper.count();

    let them_lower = pos.get_lower() & pos.get_them();
    let them_upper = pos.get_upper() & pos.get_them();
    let num_them = them_lower.count() + them_upper.count();

    100 * (num_us - num_them)
}
