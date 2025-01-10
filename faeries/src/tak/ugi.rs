use super::root::primary;
use super::state::TakPositions;
use super::state::TakState;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use crate::tak::state::TakPositions::*;
use games::gamerules::GameRules;
use games::general::side::Side;
use games::perft;
use games::tak::Dir;
use games::tak::TakMove;
use games::{gamerules::GameResult, tak::TakPosition};
use protocols::GoSettings;
use protocols::ugi::{UGI, UGIGameResult};

pub fn info_handler<const SIZE: usize>(
    pos: &TakPosition<SIZE>,
    depth: Option<i32>,
    seldepth: Option<i32>,
    score: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    elapsed: Option<u128>,
    hashfull: Option<i32>,
    pv: &Vec<TakMove<SIZE>>,
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
            print!(" {}", as_ugi(pos, mv));
        }
    }
    println!();
}

#[must_use]
pub fn as_ugi<const SIZE: usize>(_pos: &TakPosition<SIZE>, mv: &TakMove<SIZE>) -> String {
    match mv {
        TakMove::Drop(sq, kind) => format!(
            "{}{}",
            match kind {
                games::tak::PieceType::Flat => "",
                games::tak::PieceType::Standing => "S",
                games::tak::PieceType::Cap => "C",
            },
            sq
        ),
        TakMove::Spread(sq, dir, height, coverage, is_crush) => {
            debug_assert!(*height > 0);
            debug_assert!(*height as usize <= SIZE);
            debug_assert!(*coverage != 0);

            format!(
                "{}{}{}{}{}",
                if *height > 1 {
                    height.to_string()
                } else {
                    "".to_string()
                },
                sq,
                match dir {
                    Dir::Up => "+",
                    Dir::Down => "-",
                    Dir::Left => "<",
                    Dir::Right => ">",
                },
                {
                    let mut gg = String::new();

                    let mut head = 0;
                    let mut tail = 0;

                    while ((coverage >> head) & 1) == 0 {
                        head += 1;
                        tail += 1;
                    }

                    while head < *height {
                        head += 1;

                        if ((coverage >> head) & 1) == 1 {
                            gg += &(head - tail).to_string();
                            tail = head;
                        }
                    }

                    if head > tail {
                        gg += &(head - tail).to_string();
                    }

                    if gg.len() != 1 { gg } else { "".to_string() }
                },
                if *is_crush { "*" } else { "" }
            )
        }
    }
}

impl UGI for TakState {
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
        self.position("startpos");
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, fen: &str) {
        match &mut self.pos {
            Size3(pos) => pos.set_fen(fen),
            Size4(pos) => pos.set_fen(fen),
            Size5(pos) => pos.set_fen(fen),
            Size6(pos) => pos.set_fen(fen),
            Size7(pos) => pos.set_fen(fen),
            Size8(pos) => pos.set_fen(fen),
        }
    }

    fn moves(&mut self, movestr: &str) {
        match &mut self.pos {
            Size3(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            Size4(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            Size5(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            Size6(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            Size7(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            Size8(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
        };
    }

    fn go(&mut self, settings: &GoSettings) {
        match &self.pos {
            Size3(pos) => {
                let eval = |_pos: &TakPosition<3>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            Size4(pos) => {
                let eval = |_pos: &TakPosition<4>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            Size5(pos) => {
                let eval = |_pos: &TakPosition<5>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            Size6(pos) => {
                let eval = |_pos: &TakPosition<6>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            Size7(pos) => {
                let eval = |_pos: &TakPosition<7>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            Size8(pos) => {
                let eval = |_pos: &TakPosition<8>| -> i32 { 0 };
                let bestmove = match self.search.value.as_str() {
                    "primary" => primary(pos.clone(), settings, &info_handler),
                    "random" => random(&pos.clone(), || self.prng.next()),
                    "minimax" => minimax(pos.clone(), settings, &info_handler, &eval),
                    "alphabeta" => alphabeta(pos.clone(), settings, &info_handler, &eval),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                if let Some(mv) = bestmove {
                    println!("bestmove {}", as_ugi(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
        }
    }

    fn stop(&mut self) {}

    fn print(&self) {
        match &self.pos {
            Size3(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
            Size4(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
            Size5(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
            Size6(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
            Size7(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
            Size8(pos) => {
                println!("FEN: {}", pos.get_fen());
                print!("{}", pos);
            }
        }
    }

    fn print_options(&self) {
        println!("{}", self.debug);
        println!("{}", self.size);
        println!("{}", self.search);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("size", _) => {
                self.size.value = value.parse().unwrap();
                self.pos = match self.size.value {
                    3 => TakPositions::Size3(TakPosition::startpos()),
                    4 => TakPositions::Size4(TakPosition::startpos()),
                    5 => TakPositions::Size5(TakPosition::startpos()),
                    6 => TakPositions::Size6(TakPosition::startpos()),
                    7 => TakPositions::Size7(TakPosition::startpos()),
                    8 => TakPositions::Size8(TakPosition::startpos()),
                    _ => panic!("Unsupported size"),
                };
            }
            ("search", _) => self.search.value = value.to_owned(),
            (_, _) => {}
        }
    }

    fn is_debug(&self) -> bool {
        self.debug.value
    }

    fn query_p1turn(&self) -> bool {
        match &self.pos {
            Size3(pos) => pos.get_turn() == Side::Player1,
            Size4(pos) => pos.get_turn() == Side::Player1,
            Size5(pos) => pos.get_turn() == Side::Player1,
            Size6(pos) => pos.get_turn() == Side::Player1,
            Size7(pos) => pos.get_turn() == Side::Player1,
            Size8(pos) => pos.get_turn() == Side::Player1,
        }
    }

    fn query_result(&self) -> Option<UGIGameResult> {
        let result = match &self.pos {
            Size3(pos) => pos.get_result(),
            Size4(pos) => pos.get_result(),
            Size5(pos) => pos.get_result(),
            Size6(pos) => pos.get_result(),
            Size7(pos) => pos.get_result(),
            Size8(pos) => pos.get_result(),
        };

        match result {
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

        match &mut self.pos {
            Size3(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
            Size4(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
            Size5(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
            Size6(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
            Size7(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
            Size8(pos) => perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler),
        };
    }

    fn split(&mut self, settings: &GoSettings) {
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };

        match &mut self.pos {
            Size3(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<3>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            Size4(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<4>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            Size5(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<5>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            Size6(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<6>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            Size7(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<7>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            Size8(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<8>, nodes: u64| println!("{} {}", as_ugi(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
        }
    }

    fn movelist(&self) {
        let mut count = 0;

        match &self.pos {
            Size3(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
            Size4(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
            Size5(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
            Size6(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
            Size7(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
            Size8(pos) => pos.move_generator(|mv| {
                println!("{}", as_ugi(&pos, &mv));
                count += 1;
                false
            }),
        }

        println!("Total: {}", count);
    }
}
