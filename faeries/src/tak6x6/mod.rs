pub mod eval;
pub mod root;
pub mod search;

use crate::search::Primary;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};

const INF_SCORE: i32 = 10_000_000;
const MATE_SCORE: i32 = 1_000_000;

impl Primary for TakPosition<6> {
    fn primary(
        self,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&Self, &SearchInfo<TakMove<6>>),
    ) -> Option<TakMove<6>> {
        root::primary(self, settings, info_handler)
    }
}
