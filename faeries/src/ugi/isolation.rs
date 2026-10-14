use crate::ugi::UgiGame;
use games::isolation::{IsolationMove, IsolationPosition};

impl UgiGame for IsolationPosition {
    fn as_ugi(&self, mv: &IsolationMove) -> String {
        format!("{}{}", mv.to, mv.remove)
    }
}
