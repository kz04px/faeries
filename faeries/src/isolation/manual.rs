use super::{root::root, state::IsolationState, ugi::as_ugi};
use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
    isolation::IsolationMove,
};
use manual::{Manual, ManualGameResult};
use ugi::go::GoSettings;

pub fn info_handler(
    depth: Option<i32>,
    seldepth: Option<i32>,
    score: Option<i32>,
    mate: Option<i32>,
    nodes: Option<u64>,
    elapsed: Option<u128>,
    hashfull: Option<i32>,
    pv: &Vec<IsolationMove>,
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

pub fn empty_handler(
    _: Option<i32>,
    _: Option<i32>,
    _: Option<i32>,
    _: Option<i32>,
    _: Option<u64>,
    _: Option<u128>,
    _: Option<i32>,
    _: &Vec<IsolationMove>,
) {
}

impl Manual for IsolationState {
    fn print(&self) {
        println!("{}", self.pos);
    }

    fn is_gameover(&self) -> bool {
        self.pos.is_gameover()
    }

    fn makemove(&mut self, movestr: &str) -> bool {
        let found = self
            .pos
            .legal_moves()
            .into_iter()
            .find(|mv| as_ugi(mv) == movestr);

        if let Some(mv) = found {
            self.pos.makemove(&mv);
            true
        } else {
            false
        }
    }

    fn play(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);
        let bestmove = root(self.pos.clone(), &settings, &empty_handler);
        if let Some(mv) = bestmove {
            self.pos.makemove(&mv);
        }
    }

    fn hint(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);
        let _ = root(self.pos.clone(), &settings, &info_handler);
    }

    fn get_result(&self) -> Option<ManualGameResult> {
        match self.pos.get_result() {
            Some(GameResult::Win(Side::Player1)) => Some(ManualGameResult::P1Win),
            Some(GameResult::Win(Side::Player2)) => Some(ManualGameResult::P2Win),
            Some(GameResult::Draw) => Some(ManualGameResult::Draw),
            None => todo!(),
        }
    }
}