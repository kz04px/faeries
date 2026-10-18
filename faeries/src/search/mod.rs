pub mod alphabeta;
pub mod eval;
pub mod flatmc;
pub mod minimax;
pub mod random;

use games::ataxx::AtaxxPosition;
use games::chess::ChessPosition;
use games::connect4::Connect4Position;
use games::droptaxx::DroptaxxPosition;
use games::gamerules::GameRules;
use games::gomoku::GomokuPosition;
use games::isolation::IsolationPosition;
use games::pijersi::PijersiPosition;
use games::tak::TakPosition;
use std::fmt;

/// Progress reported by a search. Fields a search doesn't track are `None`.
pub struct SearchInfo<'a, M> {
    pub depth: Option<i32>,
    pub seldepth: Option<i32>,
    pub score: Option<i32>,
    pub mate: Option<i32>,
    pub nodes: Option<u64>,
    /// Milliseconds since the search started.
    pub elapsed: Option<u128>,
    pub hashfull: Option<i32>,
    pub pv: &'a [M],
}

impl<M> Default for SearchInfo<'_, M> {
    fn default() -> Self {
        Self {
            depth: None,
            seldepth: None,
            score: None,
            mate: None,
            nodes: None,
            elapsed: None,
            hashfull: None,
            pv: &[],
        }
    }
}

/// A game's own search, used by the "primary" search option.
pub trait Primary: GameRules {
    #[must_use]
    fn primary(
        self,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&Self, &SearchInfo<Self::MoveType>),
    ) -> Option<Self::MoveType>;
}

/// A game the protocols can play, with or without a primary search.
pub trait MaybePrimary: GameRules {
    /// The game's primary search, or `None` if it doesn't have one.
    #[must_use]
    fn maybe_primary(
        self,
        _settings: &protocols::GoSettings,
        _info_handler: &impl Fn(&Self, &SearchInfo<Self::MoveType>),
    ) -> Option<Option<Self::MoveType>> {
        None
    }
}

impl<G: Primary> MaybePrimary for G {
    fn maybe_primary(
        self,
        settings: &protocols::GoSettings,
        info_handler: &impl Fn(&Self, &SearchInfo<Self::MoveType>),
    ) -> Option<Option<Self::MoveType>> {
        Some(self.primary(settings, info_handler))
    }
}

// Games without a primary search
impl MaybePrimary for AtaxxPosition<7, 7> {}
impl MaybePrimary for ChessPosition {}
impl MaybePrimary for Connect4Position {}
impl MaybePrimary for DroptaxxPosition {}
impl MaybePrimary for GomokuPosition {}
impl MaybePrimary for IsolationPosition {}
impl MaybePrimary for PijersiPosition {}
impl MaybePrimary for TakPosition<3> {}
impl MaybePrimary for TakPosition<4> {}
impl MaybePrimary for TakPosition<5> {}
impl MaybePrimary for TakPosition<7> {}
impl MaybePrimary for TakPosition<8> {}

/// Settings a search can't run with.
#[derive(Debug, PartialEq)]
pub enum SearchError {
    /// None of the given settings would ever stop the search.
    NoLimit(&'static str),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SearchError::NoLimit(expected) => write!(f, "search needs one of: {}", expected),
        }
    }
}
