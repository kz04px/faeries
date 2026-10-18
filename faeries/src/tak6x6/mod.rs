pub mod root;

use crate::search::Primary;
use crate::search::SearchInfo;
use games::tak::{TakMove, TakPosition};

impl Primary for TakPosition<6> {
    fn primary(
        self,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&Self, &SearchInfo<TakMove<6>>),
    ) -> Option<TakMove<6>> {
        root::primary(self, settings, info_handler)
    }
}
