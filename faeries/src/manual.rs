use crate::search::alphabeta::alphabeta;
use crate::search::eval::Eval;
use crate::search::{MaybePrimary, SearchInfo};
use crate::state::{ExtraOptions, GameState};
use crate::ugi::{UgiGame, info_handler};
use games::{
    gamerules::{GameResult, GameRules},
    general::side::Side,
};
use protocols::{
    GoSettings,
    manual::{Manual, ManualGameResult},
};

pub fn empty_handler<G: GameRules>(_: &G, _: &SearchInfo<G::MoveType>) {}

impl<G: MaybePrimary + Eval, X: ExtraOptions> GameState<G, X> {
    /// The game's primary search, or alphabeta if it doesn't have one.
    fn search(
        &self,
        settings: &GoSettings,
        info_handler: &impl Fn(&G, &SearchInfo<G::MoveType>),
    ) -> Option<G::MoveType> {
        match self.pos.clone().maybe_primary(settings, info_handler) {
            Some(bestmove) => bestmove,
            None => alphabeta(self.pos.clone(), settings, info_handler, &G::eval),
        }
    }
}

impl<G: UgiGame + MaybePrimary + Eval, X: ExtraOptions> Manual for GameState<G, X> {
    fn print(&self) {
        self.pos.manual_print();
    }

    fn is_gameover(&self) -> bool {
        self.pos.is_gameover()
    }

    fn makemove(&mut self, movestr: &str) -> bool {
        let found = self
            .pos
            .legal_moves()
            .into_iter()
            .find(|mv| self.pos.as_ugi(mv) == movestr);

        if let Some(mv) = found {
            self.pos.makemove(&mv);
            true
        } else {
            false
        }
    }

    fn makenull(&mut self) {
        self.pos.makenull();
    }

    fn play(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);
        let bestmove = self.search(&settings, &empty_handler::<G>);
        if let Some(mv) = bestmove {
            self.pos.makemove(&mv);
        }
    }

    fn hint(&mut self, depth: i32) {
        let settings = GoSettings::from_depth(depth);
        let _ = self.search(&settings, &info_handler::<G>);
    }

    fn get_result(&self) -> Option<ManualGameResult> {
        match self.pos.get_result() {
            Some(GameResult::Win(Side::Player1)) => Some(ManualGameResult::P1Win),
            Some(GameResult::Win(Side::Player2)) => Some(ManualGameResult::P2Win),
            Some(GameResult::Draw) => Some(ManualGameResult::Draw),
            None => None,
        }
    }
}
