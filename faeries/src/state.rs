use crate::prng;
use crate::ugi::pijersi::PijersiOptions;
use games::ataxx::AtaxxPosition;
use games::chess::ChessPosition;
use games::connect4::Connect4Position;
use games::droptaxx::DroptaxxPosition;
use games::gamerules::GameRules;
use games::gomoku::GomokuPosition;
use games::isolation::IsolationPosition;
use games::pijersi::PijersiPosition;
use games::tak::TakPosition;
use protocols::ugi::options::{Check, Combo};

/// Options a game has on top of the shared ones.
pub trait ExtraOptions: Default {
    fn print_options(&self) {}

    fn set_option(&mut self, _name: &str, _value: &str) {}
}

impl ExtraOptions for () {}

pub struct GameState<G: GameRules, X: ExtraOptions = ()> {
    pub pos: G,
    pub prng: prng::XorshiftGenerator,
    // Options
    pub debug: Check,
    pub search: Combo,
    pub extra: X,
}

impl<G: GameRules, X: ExtraOptions> GameState<G, X> {
    /// Swap in a position of another type, keeping the options and prng.
    #[must_use]
    pub fn with_pos<H: GameRules>(self, pos: H) -> GameState<H, X> {
        GameState {
            pos,
            prng: self.prng,
            debug: self.debug,
            search: self.search,
            extra: self.extra,
        }
    }
}

impl<G: GameRules, X: ExtraOptions> Default for GameState<G, X> {
    fn default() -> Self {
        Self {
            pos: G::startpos(),
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
            extra: X::default(),
        }
    }
}

pub type AtaxxState = GameState<AtaxxPosition<7, 7>>;
pub type ChessState = GameState<ChessPosition>;
pub type Connect4State = GameState<Connect4Position>;
pub type DroptaxxState = GameState<DroptaxxPosition>;
pub type GomokuState = GameState<GomokuPosition>;
pub type IsolationState = GameState<IsolationPosition>;
pub type PijersiState = GameState<PijersiPosition, PijersiOptions>;
pub type ManualTakState = GameState<TakPosition<6>>;
