pub mod ataxx;

use crate::search::SearchInfo;
use games::gamerules::GameRules;
use protocols::GoSettings;
use std::marker::PhantomData;

/// A game's custom searches, kept between searches until the engine quits.
pub trait GameSearch: Default {
    type Game: GameRules;

    /// Called on the first "isready".
    fn init(&mut self) {}

    fn newgame(&mut self) {}

    fn print_options(&self) {}

    /// Returns whether the option belongs to this search.
    fn set_option(&mut self, _name: &str, _value: &str) -> bool {
        false
    }

    /// The search names this impl handles. Used to populate the search option.
    fn search_names() -> &'static [&'static str] {
        &[]
    }

    /// The best move for the named search, or `None` if this game doesn't handle that name.
    #[must_use]
    fn search(
        &mut self,
        name: &str,
        pos: Self::Game,
        settings: &GoSettings,
        info_handler: &impl Fn(&Self::Game, &SearchInfo<<Self::Game as GameRules>::MoveType>),
    ) -> Option<Option<<Self::Game as GameRules>::MoveType>>;
}

/// For games without custom searches.
pub struct NoSearch<G>(PhantomData<G>);

impl<G> Default for NoSearch<G> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<G: GameRules> GameSearch for NoSearch<G> {
    type Game = G;

    fn search(
        &mut self,
        _name: &str,
        _pos: G,
        _settings: &GoSettings,
        _info_handler: &impl Fn(&G, &SearchInfo<G::MoveType>),
    ) -> Option<Option<G::MoveType>> {
        None
    }
}
