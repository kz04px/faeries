use crate::prng::XorshiftGenerator;
use crate::search::SearchInfo;
use crate::search::random::random;
use games::chess::{ChessMove, ChessPosition};
use protocols::GoSettings;
use std::time::{SystemTime, UNIX_EPOCH};

#[must_use]
pub fn primary(
    pos: ChessPosition,
    _settings: &GoSettings,
    _info_handler: &impl Fn(&ChessPosition, &SearchInfo<ChessMove>),
) -> Option<ChessMove> {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let mut prng = XorshiftGenerator::new(seed | 1);
    random(&pos, || prng.next())
}
