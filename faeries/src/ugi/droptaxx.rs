use crate::ugi::UgiGame;
use games::droptaxx::{DroptaxxMove, DroptaxxPosition};

impl UgiGame for DroptaxxPosition {
    fn as_ugi(&self, mv: &DroptaxxMove) -> String {
        format!("{}", mv.0)
    }
}
