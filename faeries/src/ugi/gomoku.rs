use crate::ugi::UgiGame;
use games::gomoku::{GomokuMove, GomokuPosition};

impl UgiGame for GomokuPosition {
    fn as_ugi(&self, mv: &GomokuMove) -> String {
        format!("{}", mv.0)
    }
}
