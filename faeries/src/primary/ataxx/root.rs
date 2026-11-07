use crate::prng::XorshiftGenerator;
use crate::search::SearchInfo;
use crate::search::random::random;
use games::ataxx::{AtaxxMove, AtaxxPosition};
use protocols::GoSettings;
use std::time::{SystemTime, UNIX_EPOCH};

#[must_use]
pub fn primary(
    pos: AtaxxPosition<7, 7>,
    _settings: &GoSettings,
    _info_handler: &impl Fn(&AtaxxPosition<7, 7>, &SearchInfo<AtaxxMove<7, 7>>),
) -> Option<AtaxxMove<7, 7>> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    // Xorshift never leaves a zero state
    let mut prng = XorshiftGenerator::new(seed | 1);
    random(&pos, || prng.next())
}
