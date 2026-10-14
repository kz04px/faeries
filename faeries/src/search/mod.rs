pub mod alphabeta;
pub mod eval;
pub mod flatmc;
pub mod minimax;
pub mod random;

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
