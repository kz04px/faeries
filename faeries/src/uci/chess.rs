use crate::search::SearchInfo;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use crate::search::{MaybePrimary, eval::Eval};
use crate::state::ChessState;
use crate::ttperft::iterative_ttperft;
use games::chess::ChessMove;
use games::gamerules::GameRules;
use games::general::side::Side;
use games::perft;
use games::{chess::ChessPosition, gamerules::GameResult};
use protocols::GoSettings;
use protocols::uci::{UCI, UCIGameResult};

/// Size of the table ttperft uses.
const PERFT_HASH_MB: usize = 64;

pub fn info_handler(pos: &ChessPosition, info: &SearchInfo<ChessMove>) {
    print!("info");
    info.depth.inspect(|d| print!(" depth {}", d));
    info.seldepth.inspect(|d| print!(" seldepth {}", d));
    info.score.inspect(|s| print!(" score cp {}", s));
    info.mate.inspect(|d| print!(" score mate {}", d));
    info.nodes.inspect(|n| print!(" nodes {}", n));
    info.elapsed.inspect(|ms| print!(" time {}", ms));
    if let (Some(t), Some(n)) = (info.elapsed, info.nodes) {
        if t > 0 {
            print!(" nps {}", (n as u128 * 1000) / t);
        }
    }
    info.hashfull
        .inspect(|hashfull| print!(" hashfull {}", hashfull));
    if !info.pv.is_empty() {
        print!(" pv");
        let mut npos = pos.clone();
        for mv in info.pv {
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
        if self.pos.is_gameover() {
            println!("bestmove 0000");
            return;
        }

        let eval = &ChessPosition::eval;

        let bestmove = match self.search.value.as_str() {
            "primary" => match self.pos.clone().maybe_primary(settings, &info_handler) {
                Some(bestmove) => Ok(bestmove),
                None => Ok(alphabeta(self.pos.clone(), settings, &info_handler, eval)),
            },
            "random" => Ok(random(&self.pos, || self.prng.next())),
            "minimax" => Ok(minimax(self.pos.clone(), settings, &info_handler, eval)),
            "alphabeta" => Ok(alphabeta(self.pos.clone(), settings, &info_handler, eval)),
            "flatmc" => flatmc(self.pos.clone(), settings, &info_handler, || {
                self.prng.next()
            }),
            _ => panic!("Unknown search type"),
        };
        match bestmove {
            Ok(Some(mv)) => println!("bestmove {}", as_uci(&self.pos, &mv)),
            Ok(None) => println!("bestmove 0000"),
            Err(e) => {
                println!("info string error {}", e);
                println!("bestmove 0000");
            }
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
        iterative_ttperft(
            &mut self.pos,
            settings.depth.unwrap(),
            PERFT_HASH_MB,
            &ChessPosition::hash,
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
