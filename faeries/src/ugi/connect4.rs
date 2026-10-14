use crate::colour::Colour;
use crate::ugi::UgiGame;
use games::connect4::{Connect4Move, Connect4Position};
use games::gamerules::GameRules;
use games::general::side::Side;
use games::general::square::Square;

impl UgiGame for Connect4Position {
    fn as_ugi(&self, mv: &Connect4Move) -> String {
        format!("{}", mv.0)
    }

    fn unrecognised_command(&self, word: &str) {
        match word {
            "print" => print!("{}", self),
            "pprint" => pprint(self),
            _ => {}
        }
    }
}

fn pprint(pos: &Connect4Position) {
    for y in (0..6).rev() {
        for x in 0..7 {
            let sq = Square::<7, 6>::from_coords(x, y);
            if pos.get_red().is_square_set(sq) {
                print!("{}●{}", Colour::Red, Colour::Clear);
            } else if pos.get_yellow().is_square_set(sq) {
                print!("{}●{}", Colour::Yellow, Colour::Clear);
            } else {
                print!(".");
            }
        }
        println!("");
    }

    if pos.get_turn() == Side::Player1 {
        println!("Turn: {}●{}", Colour::Red, Colour::Clear);
    } else {
        println!("Turn: {}●{}", Colour::Yellow, Colour::Clear);
    }
}
