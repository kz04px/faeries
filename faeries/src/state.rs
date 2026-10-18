use crate::primary::{GameSearch, NoSearch};
use crate::prng;
use crate::ugi::pijersi::PijersiOptions;
use games::ataxx::AtaxxPosition;
use games::chess::ChessPosition;
use games::connect4::Connect4Position;
use games::droptaxx::DroptaxxPosition;
use games::gomoku::GomokuPosition;
use games::gamerules::GameRules;
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

pub struct GameState<G, S: GameSearch<Game = G> = NoSearch<G>, X: ExtraOptions = ()> {
    pub pos: G,
    pub game_search: S,
    pub prng: prng::XorshiftGenerator,
    /// Whether `init` has run, so a new game search must be initialised too.
    pub initialised: bool,
    /// Options given to the game search, replayed on a new one.
    pub search_options: Vec<(String, String)>,
    // Options
    pub debug: Check,
    pub search: Combo,
    pub extra: X,
}

impl<G: GameRules, S: GameSearch<Game = G>, X: ExtraOptions> GameState<G, S, X> {
    /// Swap in a position of another type, keeping the options and prng.
    ///
    /// The new game gets a fresh NoSearch with the same options replayed.
    #[must_use]
    pub fn with_pos<H: GameRules>(self, pos: H) -> GameState<H, NoSearch<H>, X> {
        GameState {
            pos,
            game_search: NoSearch::default(),
            prng: self.prng,
            initialised: self.initialised,
            search_options: self.search_options,
            debug: self.debug,
            search: self.search,
            extra: self.extra,
        }
    }

    /// Pass an option to the game search, returning whether it was one of its options.
    pub fn set_search_option(&mut self, name: &str, value: &str) -> bool {
        if self.game_search.set_option(name, value) {
            self.search_options.retain(|(n, _)| n != name);
            self.search_options.push((name.to_owned(), value.to_owned()));
            true
        } else {
            false
        }
    }
}

impl<G: GameRules, S: GameSearch<Game = G>, X: ExtraOptions> Default for GameState<G, S, X> {
    fn default() -> Self {
        let custom: &[&str] = S::search_names();
        let default_search = custom.first().copied().unwrap_or("alphabeta");
        let mut options: Vec<String> = custom.iter().map(|s| s.to_string()).collect();
        options.extend(["random", "minimax", "alphabeta", "flatmc"].map(str::to_owned));

        Self {
            pos: G::startpos(),
            game_search: S::default(),
            prng: prng::XorshiftGenerator::new(0xe50076937a9e5b1c),
            initialised: false,
            search_options: vec![],
            // Options
            debug: Check {
                name: "debug".to_owned(),
                value: cfg!(debug_assertions),
            },
            search: Combo {
                name: "search".to_owned(),
                value: default_search.to_owned(),
                options,
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
pub type PijersiState = GameState<PijersiPosition, NoSearch<PijersiPosition>, PijersiOptions>;
pub type ManualTakState = GameState<TakPosition<6>, crate::primary::tak::TakPrimary>;
