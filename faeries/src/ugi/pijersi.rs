use crate::state::ExtraOptions;
use crate::ugi::UgiGame;
use games::gamerules::GameRules;
use games::pijersi::{PijersiMove, PijersiPosition};
use protocols::ugi::options::Spin;

impl UgiGame for PijersiPosition {
    fn as_ugi(&self, mv: &PijersiMove) -> String {
        match mv {
            PijersiMove::SoloMove(fr, to) => fr.to_string_inverted() + &to.to_string_inverted(),
            PijersiMove::SoloStack(fr, to) => fr.to_string_inverted() + &to.to_string_inverted(),
            PijersiMove::SoloStackMove(fr, sq1, sq2) => {
                fr.to_string_inverted() + &sq1.to_string_inverted() + &sq2.to_string_inverted()
            }
            PijersiMove::StackMove(fr, to) => {
                fr.to_string_inverted() + &to.to_string_inverted() + &to.to_string_inverted()
            }
            PijersiMove::StackMoveDestack(fr, sq1, sq2) => {
                fr.to_string_inverted() + &sq1.to_string_inverted() + &sq2.to_string_inverted()
            }
            PijersiMove::StackDestack(fr, to) => {
                fr.to_string_inverted() + &fr.to_string_inverted() + &to.to_string_inverted()
            }
            PijersiMove::StackStack(fr, to) => {
                fr.to_string_inverted() + &fr.to_string_inverted() + &to.to_string_inverted()
            }
            PijersiMove::StackStackMove(fr, sq1, sq2) => {
                fr.to_string_inverted() + &sq1.to_string_inverted() + &sq2.to_string_inverted()
            }
        }
    }

    fn unrecognised_command(&self, word: &str) {
        if word == "print" {
            print!("{}", self);
            println!("FEN: {}", self.get_fen());
        }
    }
}

pub struct PijersiOptions {
    pub seed: Spin<u64>,
}

impl Default for PijersiOptions {
    fn default() -> Self {
        Self {
            seed: Spin::<u64> {
                name: "seed".to_owned(),
                min: u64::MIN,
                max: u64::MAX,
                value: 0xe50076937a9e5b1c,
            },
        }
    }
}

impl ExtraOptions for PijersiOptions {
    fn print_options(&self) {
        println!("{}", self.seed);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        if name == "seed" {
            if let Ok(num) = value.parse::<u64>() {
                self.seed.value = num.clamp(self.seed.min, self.seed.max);
            }
        }
    }
}
