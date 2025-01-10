use super::{root::primary, state::TakState, ugi::as_ugi};
use crate::tak::state::TakPositions::*;
use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
    tak::{TakMove, TakPosition},
};
use protocols::{
    GoSettings,
    manual::{Manual, ManualGameResult},
};

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

pub fn empty_handler<const SIZE: usize>(
    _: &TakPosition<SIZE>,
    _: Option<i32>,
    _: Option<i32>,
    _: Option<i32>,
    _: Option<i32>,
    _: Option<u64>,
    _: Option<u128>,
    _: Option<i32>,
    _: &Vec<TakMove<SIZE>>,
) {
}

impl Manual for TakState {
    fn print(&self) {
        match &self.pos {
            Size3(pos) => println!("{}", pos),
            Size4(pos) => println!("{}", pos),
            Size5(pos) => println!("{}", pos),
            Size6(pos) => println!("{}", pos),
            Size7(pos) => println!("{}", pos),
            Size8(pos) => println!("{}", pos),
        }
    }

    fn is_gameover(&self) -> bool {
        match &self.pos {
            Size3(pos) => pos.is_gameover(),
            Size4(pos) => pos.is_gameover(),
            Size5(pos) => pos.is_gameover(),
            Size6(pos) => pos.is_gameover(),
            Size7(pos) => pos.is_gameover(),
            Size8(pos) => pos.is_gameover(),
        }
    }

    fn makemove(&mut self, movestr: &str) -> bool {
        match &mut self.pos {
            Size3(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
            Size4(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
            Size5(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
            Size6(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
            Size7(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
            Size8(pos) => {
                let found = pos
                    .legal_moves()
                    .into_iter()
                    .find(|mv| as_ugi(&pos, mv) == movestr);

                if let Some(mv) = found {
                    pos.makemove(&mv);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn makenull(&mut self) {
        match &mut self.pos {
            Size3(pos) => pos.makenull(),
            Size4(pos) => pos.makenull(),
            Size5(pos) => pos.makenull(),
            Size6(pos) => pos.makenull(),
            Size7(pos) => pos.makenull(),
            Size8(pos) => pos.makenull(),
        }
    }

    fn play(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);

        match &mut self.pos {
            Size3(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
            Size4(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
            Size5(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
            Size6(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
            Size7(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
            Size8(pos) => {
                let bestmove = primary(pos.clone(), &settings, &empty_handler);
                if let Some(mv) = bestmove {
                    pos.makemove(&mv);
                }
            }
        }
    }

    fn hint(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);
        match &self.pos {
            Size3(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
            Size4(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
            Size5(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
            Size6(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
            Size7(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
            Size8(pos) => {
                let _ = primary(pos.clone(), &settings, &info_handler);
            }
        }
    }

    fn get_result(&self) -> Option<ManualGameResult> {
        let result = match &self.pos {
            Size3(pos) => pos.get_result(),
            Size4(pos) => pos.get_result(),
            Size5(pos) => pos.get_result(),
            Size6(pos) => pos.get_result(),
            Size7(pos) => pos.get_result(),
            Size8(pos) => pos.get_result(),
        };

        match result {
            Some(GameResult::Win(Side::Player1)) => Some(ManualGameResult::P1Win),
            Some(GameResult::Win(Side::Player2)) => Some(ManualGameResult::P2Win),
            Some(GameResult::Draw) => Some(ManualGameResult::Draw),
            None => None,
        }
    }
}
