use super::root::primary;
use crate::prng;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use games::gamerules::GameRules;
use games::perft;
use games::tak::Dir;
use games::tak::TakMove;
use games::tak::TakPosition;
use protocols::GoSettings;
use protocols::tei::TEI;
use protocols::ugi::options::{Check, Combo};

pub enum TakPositions {
    Size3(TakPosition<3>),
    Size4(TakPosition<4>),
    Size5(TakPosition<5>),
    Size6(TakPosition<6>),
    Size7(TakPosition<7>),
    Size8(TakPosition<8>),
}

pub struct TEITakState {
    pub pos: TakPositions,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub search: Combo,
}

impl Default for TEITakState {
    fn default() -> Self {
        Self {
            pos: TakPositions::Size6(TakPosition::startpos()),
            prng: prng::XorshiftGenerator::new(0xe50076937a9e5b1c),
            // Options
            debug: Check {
                name: "debug".to_owned(),
                value: cfg!(debug_assertions),
            },
            search: Combo {
                name: "search".to_owned(),
                value: "primary".to_owned(),
                options: vec![
                    "primary".to_owned(),
                    "random".to_owned(),
                    "minimax".to_owned(),
                    "alphabeta".to_owned(),
                    "flatmc".to_owned(),
                ],
            },
        }
    }
}

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
            print!(" {}", as_tei(pos, mv));
        }
    }
    println!();
}

#[must_use]
pub fn as_tei<const SIZE: usize>(_pos: &TakPosition<SIZE>, mv: &TakMove<SIZE>) -> String {
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

impl TEI for TEITakState {
    fn init(&mut self) {
        self.teinewgame(6, 0);
    }

    fn shutdown(&mut self) {}

    fn name(&self) -> String {
        format!("Faeries v{}", env!("CARGO_PKG_VERSION"))
    }

    fn author(&self) -> String {
        "kz04px".to_owned()
    }

    fn version(&self) -> String {
        "1.0.0".to_owned()
    }

    fn teinewgame(&mut self, size: i32, _halfkomi: i32) {
        match size {
            3 => self.pos = TakPositions::Size3(TakPosition::<3>::startpos()),
            4 => self.pos = TakPositions::Size4(TakPosition::<4>::startpos()),
            5 => self.pos = TakPositions::Size5(TakPosition::<5>::startpos()),
            6 => self.pos = TakPositions::Size6(TakPosition::<6>::startpos()),
            7 => self.pos = TakPositions::Size7(TakPosition::<7>::startpos()),
            8 => self.pos = TakPositions::Size8(TakPosition::<8>::startpos()),
            _ => panic!("Unsupported board size"),
        }
    }

    fn isready(&mut self) {
        println!("readyok");
    }

    fn position(&mut self, tps: &str) {
        match &mut self.pos {
            TakPositions::Size3(pos) => pos.set_fen(tps),
            TakPositions::Size4(pos) => pos.set_fen(tps),
            TakPositions::Size5(pos) => pos.set_fen(tps),
            TakPositions::Size6(pos) => pos.set_fen(tps),
            TakPositions::Size7(pos) => pos.set_fen(tps),
            TakPositions::Size8(pos) => pos.set_fen(tps),
        }
    }

    fn moves(&mut self, movestr: &str) {
        match &mut self.pos {
            TakPositions::Size3(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size4(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size5(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size6(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size7(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size8(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| as_tei(&pos, mv) == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
        };
    }

    fn go(&mut self, settings: &GoSettings) {
        match &self.pos {
            TakPositions::Size3(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            TakPositions::Size4(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            TakPositions::Size5(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            TakPositions::Size6(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            TakPositions::Size7(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
            TakPositions::Size8(pos) => {
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
                    println!("bestmove {}", as_tei(&pos, &mv));
                } else {
                    println!("bestmove 0000");
                }
            }
        }
    }

    fn stop(&mut self) {}

    fn print(&self) {
        match &self.pos {
            TakPositions::Size3(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
            TakPositions::Size4(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
            TakPositions::Size5(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
            TakPositions::Size6(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
            TakPositions::Size7(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
            TakPositions::Size8(pos) => {
                println!("tps: {}", pos.get_fen());
                print!("{}", pos);
            }
        }
    }

    fn print_size_komi(&self) {
        println!("size 3 4 5 6 7 8 halfkomi 0");
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
            TakPositions::Size3(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
            TakPositions::Size4(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
            TakPositions::Size5(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
            TakPositions::Size6(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
            TakPositions::Size7(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
            TakPositions::Size8(pos) => {
                perft::perft(pos, settings.depth.unwrap(), &info_handler, &final_handler)
            }
        };
    }

    fn split(&mut self, settings: &GoSettings) {
        let final_handler = |nodes: u64| {
            println!("nodes {}", nodes);
        };

        match &mut self.pos {
            TakPositions::Size3(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<3>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size4(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<4>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size5(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<5>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size6(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<6>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size7(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<7>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size8(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<8>, nodes: u64| println!("{} {}", as_tei(&fudge, &mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
        }
    }

    fn movelist(&self) {
        let mut count = 0;

        match &self.pos {
            TakPositions::Size3(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
            TakPositions::Size4(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
            TakPositions::Size5(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
            TakPositions::Size6(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
            TakPositions::Size7(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
            TakPositions::Size8(pos) => pos.move_generator(|mv| {
                println!("{}", as_tei(&pos, &mv));
                count += 1;
                false
            }),
        }

        println!("Total: {}", count);
    }
}
