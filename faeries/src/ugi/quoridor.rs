use crate::ugi::UgiGame;
use games::gamerules::GameRules;
use games::general::square::Direction;
use games::quoridor::{QuoridorMove, QuoridorPosition};

impl UgiGame for QuoridorPosition {
    fn as_ugi(&self, mv: &QuoridorMove) -> String {
        match mv {
            QuoridorMove::Move(dir) => direction_name(*dir).to_owned(),
            QuoridorMove::Jump(forward, side) => {
                format!("{}{}", direction_name(*forward), direction_name(*side))
            }
            QuoridorMove::VWall(sq) => format!("{sq}v"),
            QuoridorMove::HWall(sq) => format!("{sq}h"),
        }
    }

    fn unrecognised_command(&self, word: &str) {
        if word == "print" {
            println!("FEN: {}", self.get_fen());
        }
    }
}

fn direction_name(dir: Direction) -> &'static str {
    match dir {
        Direction::Up => "up",
        Direction::Down => "down",
        Direction::Left => "left",
        Direction::Right => "right",
    }
}
