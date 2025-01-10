use super::root::primary;
use super::state::PijersiState;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use games::gamerules::GameRules;
use games::general::side::Side;
use games::perft;
use games::pijersi::PijersiMove;
use games::{gamerules::GameResult, pijersi::PijersiPosition};
use protocols::GoSettings;
use protocols::ugi::{UGI, UGIGameResult};

pub fn info_handler(
    _: &PijersiPosition,
    depth: Option<i32>,
    seldepth: Option<i32>,
    score: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    elapsed: Option<u128>,
    hashfull: Option<i32>,
    pv: &Vec<PijersiMove>,
) {
    print!("info");
    depth.inspect(|d| print!(" depth {}", d));
    seldepth.inspect(|d| print!(" seldepth {}", d));
    score.inspect(|s| print!(" score cp {}", s));
    mate.inspect(|d| print!(" score mate {}", d));
    nodes.inspect(|n| print!(" nodes {}", n));
    elapsed.inspect(|ms| print!(" time {}", ms));
    if let (Some(t), Some(n)) = (elapsed, nodes) {
        if t > 0 {
            print!(" nps {}", (n as u128 * 1000) / t);
        }
    }
    hashfull.inspect(|hashfull| print!(" hashfull {}", hashfull));
    if !pv.is_empty() {
        print!(" pv");
        for mv in pv {
            print!(" {}", as_ugi(mv));
        }
    }
    println!();
}

#[must_use]
pub fn as_ugi(mv: &PijersiMove) -> String {
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

impl UGI for PijersiState {
    fn init(&mut self) {
        self.uginewgame();
    }

    fn shutdown(&mut self) {}

    fn name(&self) -> String {
        format!("Faeries v{}", env!("CARGO_PKG_VERSION"))
    }

    fn author(&self) -> String {
        "kz04px".to_owned()
    }

    fn uginewgame(&mut self) {
        self.pos = PijersiPosition::startpos();
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, fen: &str) {
        self.pos.set_fen(fen);
    }

    fn moves(&mut self, movestr: &str) {
        let mv = self
            .pos
            .legal_moves()
            .iter()
            .find(|mv| as_ugi(mv) == movestr)
            .copied();

        if let Some(found) = mv {
            self.pos.makemove(&found);
        }
    }

    fn go(&mut self, settings: &GoSettings) {
        let eval = |pos: &PijersiPosition| -> i32 {
            let us_lower = pos.get_lower() & pos.get_us();
            let us_upper = pos.get_upper() & pos.get_us();
            let num_us = us_lower.count() + us_upper.count();

            let them_lower = pos.get_lower() & pos.get_them();
            let them_upper = pos.get_upper() & pos.get_them();
            let num_them = them_lower.count() + them_upper.count();

            100 * (num_us - num_them)
        };

        let bestmove = match self.search.value.as_str() {
            "primary" => primary(self.pos.clone(), settings, &info_handler),
            "random" => random(&self.pos, || self.prng.next()),
            "minimax" => minimax(self.pos.clone(), settings, &info_handler, &eval),
            "alphabeta" => alphabeta(self.pos.clone(), settings, &info_handler, &eval),
            "flatmc" => flatmc(self.pos.clone(), settings, &info_handler, || {
                self.prng.next()
            }),
            _ => panic!("Unknown search type"),
        };
        if let Some(mv) = bestmove {
            println!("bestmove {}", as_ugi(&mv));
        } else {
            println!("bestmove 0000");
        }
    }

    fn stop(&mut self) {}

    fn print(&self) {
        print!("{}", self.pos);
        println!("FEN: {}", self.pos.get_fen());
    }

    fn print_options(&self) {
        println!("{}", self.debug);
        println!("{}", self.search);
        println!("{}", self.seed);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", _) => self.search.value = value.to_owned(),
            ("seed", _) => {
                if let Ok(num) = value.parse::<u64>() {
                    self.seed.value = num.clamp(self.seed.min, self.seed.max);
                }
            }
            (_, _) => {}
        }
    }

    fn is_debug(&self) -> bool {
        self.debug.value
    }

    fn query_p1turn(&self) -> bool {
        self.pos.get_turn() == Side::Player1
    }

    fn query_result(&self) -> Option<UGIGameResult> {
        match self.pos.get_result() {
            Some(GameResult::Win(Side::Player1)) => Some(UGIGameResult::P1Win),
            Some(GameResult::Win(Side::Player2)) => Some(UGIGameResult::P2Win),
            Some(GameResult::Draw) => Some(UGIGameResult::Draw),
            None => None,
        }
    }

    fn perft(&mut self, settings: &GoSettings) {
        let info_handler = |depth: i32, elapsed: f32, nodes: u64| {
            print!("info");
            print!(" depth {}", depth);
            print!(" nodes {}", nodes);
            print!(" time {}", (elapsed * 1000.0) as u64);
            if elapsed > 0.0 {
                let nps = nodes as f32 / elapsed;
                print!(" nps {}", nps as u64);
            }
            println!();
        };
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };
        perft::perft(
            &mut self.pos,
            settings.depth.unwrap(),
            &info_handler,
            &final_handler,
        );
    }

    fn split(&mut self, settings: &GoSettings) {
        let info_handler = |mv: PijersiMove, nodes: u64| println!("{} {}", as_ugi(&mv), nodes);
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };
        perft::split(
            &mut self.pos,
            settings.depth.unwrap(),
            &info_handler,
            &final_handler,
        );
    }
}
