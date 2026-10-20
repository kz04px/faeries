use crate::prng;
use crate::search::alphabeta::alphabeta;
use crate::search::flatmc::flatmc;
use crate::search::minimax::minimax;
use crate::search::random::random;
use crate::search::{MaybePrimary, eval::Eval};
use crate::tak6x6::{DEFAULT_HASH_MB, TakState};
use crate::ugi::{UgiGame, info_handler};
use games::gamerules::GameRules;
use games::perft;
use games::tak::TakMove;
use games::tak::TakPosition;
use protocols::GoSettings;
use protocols::tei::TEI;
use protocols::ugi::options::{Check, Combo, Spin};

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
    pub primary: TakState,
    // Options
    pub debug: Check,
    pub search: Combo,
    pub hash: Spin<usize>,
}

impl Default for TEITakState {
    fn default() -> Self {
        Self {
            pos: TakPositions::Size6(TakPosition::startpos()),
            prng: prng::XorshiftGenerator::new(0xe50076937a9e5b1c),
            primary: TakState::default(),
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
            hash: Spin {
                name: "hash".to_owned(),
                min: 1,
                max: 65536,
                value: DEFAULT_HASH_MB,
            },
        }
    }
}

impl TEI for TEITakState {
    fn init(&mut self) {
        self.primary.init();
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
        self.primary.newgame();
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
        // The crush marker is optional, so match moves with or without it
        let movestr = movestr.trim_end_matches('*');
        match &mut self.pos {
            TakPositions::Size3(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size4(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size5(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size6(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size7(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
            TakPositions::Size8(pos) => {
                let mv = pos
                    .legal_moves()
                    .iter()
                    .find(|mv| pos.as_ugi(mv).trim_end_matches('*') == movestr)
                    .copied();

                if let Some(found) = mv {
                    pos.makemove(&found);
                }
            }
        };
    }

    fn go(&mut self, settings: &GoSettings) {
        let is_gameover = match &self.pos {
            TakPositions::Size3(pos) => pos.is_gameover(),
            TakPositions::Size4(pos) => pos.is_gameover(),
            TakPositions::Size5(pos) => pos.is_gameover(),
            TakPositions::Size6(pos) => pos.is_gameover(),
            TakPositions::Size7(pos) => pos.is_gameover(),
            TakPositions::Size8(pos) => pos.is_gameover(),
        };
        if is_gameover {
            println!("bestmove 0000");
            return;
        }

        match &self.pos {
            TakPositions::Size3(pos) => {
                let eval = &TakPosition::<3>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => match pos.clone().maybe_primary(settings, &info_handler) {
                        Some(bestmove) => Ok(bestmove),
                        None => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    },
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
                }
            }
            TakPositions::Size4(pos) => {
                let eval = &TakPosition::<4>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => match pos.clone().maybe_primary(settings, &info_handler) {
                        Some(bestmove) => Ok(bestmove),
                        None => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    },
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
                }
            }
            TakPositions::Size5(pos) => {
                let eval = &TakPosition::<5>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => match pos.clone().maybe_primary(settings, &info_handler) {
                        Some(bestmove) => Ok(bestmove),
                        None => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    },
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
                }
            }
            TakPositions::Size6(pos) => {
                let eval = &TakPosition::<6>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => Ok(self.primary.search(pos.clone(), settings, &info_handler)),
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
                }
            }
            TakPositions::Size7(pos) => {
                let eval = &TakPosition::<7>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => match pos.clone().maybe_primary(settings, &info_handler) {
                        Some(bestmove) => Ok(bestmove),
                        None => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    },
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
                }
            }
            TakPositions::Size8(pos) => {
                let eval = &TakPosition::<8>::eval;
                let bestmove = match self.search.value.as_str() {
                    "primary" => match pos.clone().maybe_primary(settings, &info_handler) {
                        Some(bestmove) => Ok(bestmove),
                        None => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    },
                    "random" => Ok(random(&pos.clone(), || self.prng.next())),
                    "minimax" => Ok(minimax(pos.clone(), settings, &info_handler, eval)),
                    "alphabeta" => Ok(alphabeta(pos.clone(), settings, &info_handler, eval)),
                    "flatmc" => flatmc(pos.clone(), settings, &info_handler, || self.prng.next()),
                    _ => panic!("Unknown search type"),
                };
                match bestmove {
                    Ok(Some(mv)) => println!("bestmove {}", pos.as_ugi(&mv)),
                    Ok(None) => println!("bestmove 0000"),
                    Err(e) => {
                        println!("info string error {}", e);
                        println!("bestmove 0000");
                    }
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
        println!("{}", self.hash);
    }

    fn set_option(&mut self, name: &str, value: &str) {
        match (name, value) {
            ("debug", "true") => self.debug.value = true,
            ("debug", "false") => self.debug.value = false,
            ("search", _) => self.search.value = value.to_owned(),
            ("hash", _) => {
                if let Ok(mb) = value.parse::<usize>() {
                    self.hash.value = mb.clamp(self.hash.min, self.hash.max);
                    self.primary.set_hash(self.hash.value);
                }
            }
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
                    |mv: TakMove<3>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size4(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<4>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size5(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<5>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size6(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<6>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size7(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<7>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
            TakPositions::Size8(pos) => {
                let fudge = pos.clone();
                let info_handler =
                    |mv: TakMove<8>, nodes: u64| println!("{} {}", fudge.as_ugi(&mv), nodes);
                perft::split(pos, settings.depth.unwrap(), &info_handler, &final_handler);
            }
        }
    }

    fn movelist(&self) {
        let mut count = 0;

        match &self.pos {
            TakPositions::Size3(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
            TakPositions::Size4(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
            TakPositions::Size5(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
            TakPositions::Size6(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
            TakPositions::Size7(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
            TakPositions::Size8(pos) => pos.move_generator(|mv| {
                println!("{}", pos.as_ugi(&mv));
                count += 1;
                false
            }),
        }

        println!("Total: {}", count);
    }
}
