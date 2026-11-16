use games::ataxx::AtaxxPosition;

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

#[must_use]
pub fn eval(pos: &AtaxxPosition<7, 7>) -> i32 {
    let mut score = 100 * (pos.get_us().count() - pos.get_them().count());

    for sq in pos.get_us() {
        score += PST[sq.get_index()];
    }
    for sq in pos.get_them() {
        score -= PST[sq.get_index()];
    }

    score
}
