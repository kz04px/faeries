use crate::ugi::UgiGame;
use games::chess::{ChessMove, ChessPosition};
use games::gamerules::GameRules;

impl UgiGame for ChessPosition {
    fn as_ugi(&self, mv: &ChessMove) -> String {
        self.move_to_string(mv)
    }

    fn unrecognised_command(&self, word: &str) {
        if word == "print" {
            println!("FEN: {}", self.get_fen());
            print!("{}", self);
        }
    }
}
