pub mod ataxx;
pub mod chess;
pub mod connect4;
pub mod droptaxx;
pub mod gomoku;
pub mod isolation;
pub mod pijersi;
pub mod tak;

use crate::search::MaybePrimary;
use crate::search::SearchInfo;
use crate::search::alphabeta::alphabeta;
use crate::search::eval::Eval;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use crate::state::{ExtraOptions, GameState};
use games::gamerules::{GameResult, GameRules};
use games::general::side::Side;
use games::perft;
use protocols::ugi::{GoSettings, UGI, UGIGameResult};
use std::fmt::Display;

/// How a game's moves and boards appear over UGI.
pub trait UgiGame: GameRules + Display {
    /// Format a move played from this position.
    #[must_use]
    fn as_ugi(&self, mv: &Self::MoveType) -> String;

    /// Handle a command that isn't part of UGI.
    fn unrecognised_command(&self, word: &str) {
        if word == "print" {
            print!("{}", self);
        }
    }

    /// The board shown by the manual protocol's "print" command.
    fn manual_print(&self) {
        println!("{}", self);
    }
}

pub fn info_handler<G: UgiGame>(pos: &G, info: &SearchInfo<G::MoveType>) {
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
            print!(" {}", npos.as_ugi(mv));
            npos.makemove(mv);
        }
    }
    println!();
}

impl<G: UgiGame + MaybePrimary + Eval, X: ExtraOptions> UGI for GameState<G, X> {
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
        self.pos = G::startpos();
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
            .find(|mv| self.pos.as_ugi(mv) == movestr)
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

        let settings = &protocols::GoSettings::from(settings);
        let info_handler = &info_handler::<G>;
        let eval = &G::eval;

        let bestmove = match self.search.value.as_str() {
            "primary" => match self.pos.clone().maybe_primary(settings, info_handler) {
                Some(bestmove) => Ok(bestmove),
                None => Ok(alphabeta(self.pos.clone(), settings, info_handler, eval)),
            },
            "random" => Ok(random(&self.pos, || self.prng.next())),
            "minimax" => Ok(minimax(self.pos.clone(), settings, info_handler, eval)),
            "alphabeta" => Ok(alphabeta(self.pos.clone(), settings, info_handler, eval)),
            "flatmc" => flatmc(self.pos.clone(), settings, info_handler, || {
                self.prng.next()
            }),
            _ => panic!("Unknown search type"),
        };
        match bestmove {
            Ok(Some(mv)) => println!("bestmove {}", self.pos.as_ugi(&mv)),
            Ok(None) => println!("bestmove 0000"),
            Err(e) => {
                println!("info string error {}", e);
                println!("bestmove 0000");
            }
        }
    }

    fn stop(&mut self) {}

    fn unrecognised_command(
        &mut self,
        word: &str,
        _args: &mut std::iter::Peekable<std::str::SplitAsciiWhitespace>,
    ) {
        self.pos.unrecognised_command(word);
    }

    fn print_options(&self) {
        println!("{}", self.debug);
        println!("{}", self.search);
        self.extra.print_options();
    }

    fn movelist(&self) {
        for mv in self.pos.legal_moves() {
            println!("{}", self.pos.as_ugi(&mv));
        }
    }

    fn set_option(&mut self, name: &str, value: &str) {
        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", _) => self.search.value = value.to_owned(),
            (_, _) => self.extra.set_option(name, value),
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

    fn perft(&mut self, depth: u32) {
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
        perft::perft(&mut self.pos, depth as i32, &info_handler, &final_handler);
    }

    fn split(&mut self, depth: u32) {
        let root = self.pos.clone();
        let info_handler = |mv: G::MoveType, nodes: u64| println!("{} {}", root.as_ugi(&mv), nodes);
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };
        perft::split(&mut self.pos, depth as i32, &info_handler, &final_handler);
    }
}
