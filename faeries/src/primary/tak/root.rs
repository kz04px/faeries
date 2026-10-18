use crate::prng::XorshiftGenerator;
use crate::search::SearchInfo;
use crate::search::random::random;
use games::tak::{TakMove, TakPosition};
use protocols::GoSettings;
use std::time::{SystemTime, UNIX_EPOCH};

#[must_use]
pub fn primary(
    pos: TakPosition<6>,
    _settings: &GoSettings,
    _info_handler: &impl Fn(&TakPosition<6>, &SearchInfo<TakMove<6>>),
) -> Option<TakMove<6>> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    // Xorshift never leaves a zero state
    let mut prng = XorshiftGenerator::new(seed | 1);
    random(&pos, || prng.next())
}
