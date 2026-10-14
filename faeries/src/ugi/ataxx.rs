use crate::colour::Colour;
use crate::ugi::UgiGame;
use games::ataxx::{AtaxxMove, AtaxxPosition};
use games::gamerules::GameRules;
use games::general::side::Side;
use games::general::square::Square;

impl UgiGame for AtaxxPosition<7, 7> {
    fn as_ugi(&self, mv: &AtaxxMove<7, 7>) -> String {
        match mv {
            AtaxxMove::Pass => "0000".to_owned(),
            AtaxxMove::Single(to) => format!("{}", to),
            AtaxxMove::Double(from, to) => format!("{}{}", from, to),
        }
    }

    fn unrecognised_command(&self, word: &str) {
        match word {
            "print" => {
                print!("{}", self);
                println!("FEN: {}", self.get_fen());
            }
            "pprint" => pprint(self),
            _ => {}
        }
    }
}

fn pprint(pos: &AtaxxPosition<7, 7>) {
    let printer = |piece: &str, x: i32, y: i32| {
        let sq = Square::<7, 7>::from_coords(x, y);
        let is_light = y % 2 == x % 2;

        let background = if pos.get_blockers().is_square_set(sq) {
            Colour::BlackBackground
        } else if is_light {
            Colour::MagentaBackground
        } else {
            Colour::BlueBackground
        };

        let foreground = if pos.get_black().is_square_set(sq) {
            Colour::Black
        } else {
            Colour::WhiteBright
        };

        if pos.get_both().is_square_set(sq) {
            print!("{}{}{}{}", background, foreground, piece, Colour::Clear);
        } else {
            print!("{}   {}", background, Colour::Clear);
        };
    };

    for y in (0..7).rev() {
        for x in 0..7 {
            printer("◢█◣", x, y);
        }
        println!("");
        for x in 0..7 {
            printer("███", x, y);
        }
        println!("");
        for x in 0..7 {
            printer("◥█◤", x, y);
        }
        println!("");
    }

    if pos.get_turn() == Side::Player1 {
        println!("Turn: {}●{}", Colour::Black, Colour::Clear);
    } else {
        println!("Turn: {}●{}", Colour::WhiteBright, Colour::Clear);
    }
}
