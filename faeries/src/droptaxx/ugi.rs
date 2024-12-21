use super::root::root;
use super::state::DroptaxxState;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use games::droptaxx::DroptaxxMove;
use games::gamerules::GameRules;
use games::general::side::Side;
use games::perft;
use games::{droptaxx::DroptaxxPosition, gamerules::GameResult};
use ugi::{go::GoSettings, UGIGameResult, UGI};

pub fn info_handler(
    depth: Option<i32>,
    seldepth: Option<i32>,
    score: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    elapsed: Option<u128>,
    hashfull: Option<i32>,
    pv: &Vec<DroptaxxMove>,
) {
    print!("info");
    if let Some(d) = depth {
        print!(" depth {}", d);
    }
    if let Some(d) = seldepth {
        print!(" seldepth {}", d);
    }
    if let Some(s) = score {
        print!(" score cp {}", s);
    }
    if let Some(s) = mate {
        print!(" score mate {}", s);
    }
    if let Some(n) = nodes {
        print!(" nodes {}", n);
    }
    if let Some(t) = elapsed {
        print!(" time {}", t);
    }
    if let (Some(t), Some(n)) = (elapsed, nodes) {
        if t > 0 {
            print!(" nps {}", (n as u128 * 1000) / t);
        }
    }
    if let Some(hashfull) = hashfull {
        print!(" hashfull {}", hashfull);
    }
    if !pv.is_empty() {
        print!(" pv");
        for mv in pv {
            print!(" {}", as_ugi(&mv));
        }
    }
    println!();
}

#[must_use]
pub fn as_ugi(mv: &DroptaxxMove) -> String {
    format!("{}", mv.0).to_string()
}

impl UGI for DroptaxxState {
    fn init(&mut self) {
        if self.debug.value {
            println!("info string init begin");
        }

        if self.debug.value {
            println!("info string init end");
        }

        self.uginewgame();
    }

    fn shutdown(&mut self) {
        if self.debug.value {
            println!("info string shutdown");
        }
    }

    fn name(&self) -> String {
        format!("Faeries v{}", env!("CARGO_PKG_VERSION"))
    }

    fn author(&self) -> String {
        "kz04px".to_owned()
    }

    fn uginewgame(&mut self) {
        if self.debug.value {
            println!("info string new game");
        }

        self.pos = DroptaxxPosition::from_fen("startpos");
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, fen: &str) {
        if self.debug.value {
            println!("info string set fen '{}'", fen);
        }

        self.pos.set_fen(fen);
    }

    fn moves(&mut self, movestr: &str) {
        if self.debug.value {
            println!("info string apply move '{}'", movestr);
        }

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
        if self.debug.value {
            println!("info string go {}", settings);
        }

        let eval = |pos: &DroptaxxPosition| -> i32 {
            100 * pos.get_us().count() - 100 * pos.get_them().count()
        };

        let bestmove = match self.search.value.as_str() {
            "primary" => root(self.pos, &settings, &info_handler),
            "random" => random(&self.pos),
            "minimax" => minimax(&self.pos, &settings, &info_handler, &eval),
            "alphabeta" => alphabeta(&self.pos, &settings, &info_handler, &eval),
            "flatmc" => flatmc(&self.pos, &settings, &info_handler),
            _ => panic!("Unknown search type"),
        };
        if let Some(mv) = bestmove {
            println!("bestmove {}", as_ugi(&mv));
        } else {
            println!("bestmove 0000");
        }
    }

    fn stop(&mut self) {
        if self.debug.value {
            println!("info string stop");
        }
    }

    fn print(&self) {
        print!("{}", self.pos);
    }

    fn print_options(&self) {
        println!("{}", self.debug);
        println!("{}", self.search);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        if self.debug.value {
            println!("info string set option '{}' to '{}'", name, value);
        }

        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", _) => self.search.value = value.to_owned(),
            (_, _) => {}
        }
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
            &self.pos,
            settings.depth.unwrap(),
            &info_handler,
            &final_handler,
        );
    }

    fn split(&mut self, settings: &GoSettings) {
        let info_handler = |mv: DroptaxxMove, nodes: u64| println!("{} {}", as_ugi(&mv), nodes);
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };
        perft::split(
            &self.pos,
            settings.depth.unwrap(),
            &info_handler,
            &final_handler,
        );
    }
}
