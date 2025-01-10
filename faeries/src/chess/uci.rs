use super::root::primary;
use super::state::ChessState;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use games::chess::ChessMove;
use games::gamerules::GameRules;
use games::general::side::Side;
use games::perft;
use games::{chess::ChessPosition, gamerules::GameResult};
use protocols::GoSettings;
use protocols::uci::{UCI, UCIGameResult};

pub fn info_handler(
    pos: &ChessPosition,
    depth: Option<i32>,
    seldepth: Option<i32>,
    score: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    elapsed: Option<u128>,
    hashfull: Option<i32>,
    pv: &Vec<ChessMove>,
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
        let mut npos = pos.clone();
        for mv in pv {
            print!(" {}", as_uci(&npos, mv));
            npos.makemove(&mv);
        }
    }
    println!();
}

#[must_use]
pub fn as_uci(pos: &ChessPosition, mv: &ChessMove) -> String {
    format!("{}", pos.move_to_string(mv))
}

impl UCI for ChessState {
    fn init(&mut self) {
        self.ucinewgame();
    }

    fn shutdown(&mut self) {}

    fn name(&self) -> String {
        format!("Faeries v{}", env!("CARGO_PKG_VERSION"))
    }

    fn author(&self) -> String {
        "kz04px".to_owned()
    }

    fn ucinewgame(&mut self) {
        self.pos = ChessPosition::startpos();
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
            .find(|mv| as_uci(&self.pos, mv) == movestr)
            .copied();

        if let Some(found) = mv {
            self.pos.makemove(&found);
        }
    }

    fn go(&mut self, settings: &GoSettings) {
        let eval = |pos: &ChessPosition| -> i32 {
            100 * pos.board.get_us().count() - 100 * pos.board.get_them().count()
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
            println!("bestmove {}", as_uci(&self.pos, &mv));
        } else {
            println!("bestmove 0000");
        }
    }

    fn stop(&mut self) {}

    fn print(&self) {
        println!("FEN: {}", self.pos.get_fen());
        print!("{}", self.pos);
    }

    fn print_options(&self) {
        println!("{}", self.debug);
        println!("{}", self.search);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", _) => self.search.value = value.to_owned(),
            (_, _) => {}
        }
    }

    fn is_debug(&self) -> bool {
        self.debug.value
    }

    fn query_p1turn(&self) -> bool {
        self.pos.get_turn() == Side::Player1
    }

    fn query_result(&self) -> Option<UCIGameResult> {
        match self.pos.get_result() {
            Some(GameResult::Win(Side::Player1)) => Some(UCIGameResult::P1Win),
            Some(GameResult::Win(Side::Player2)) => Some(UCIGameResult::P2Win),
            Some(GameResult::Draw) => Some(UCIGameResult::Draw),
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
        let npos = self.pos.clone();
        let info_handler = |mv: ChessMove, nodes: u64| println!("{} {}", as_uci(&npos, &mv), nodes);
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
