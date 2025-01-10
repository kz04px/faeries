pub mod gtp;
pub mod manual;
pub mod tei;
pub mod uci;
pub mod ugi {
    pub use ::ugi::GoSettings;
    pub use ::ugi::ugi::*;
}

#[derive(Default, PartialEq, Debug)]
pub enum GoKind {
    #[default]
    Search,
    Perft,
    FastPerft,
    SplitPerft,
}

#[derive(Default, PartialEq, Debug)]
pub struct GoSettings {
    pub kind: GoKind,
    pub p1time: Option<i32>,
    pub p2time: Option<i32>,
    pub p1inc: Option<i32>,
    pub p2inc: Option<i32>,
    pub depth: Option<i32>,
    pub nodes: Option<u64>,
    pub movetime: Option<i32>,
    pub movestogo: Option<i32>,
}

impl From<&ugi::GoSettings> for GoSettings {
    fn from(settings: &ugi::GoSettings) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: settings.p1time,
            p2time: settings.p2time,
            p1inc: settings.p1inc,
            p2inc: settings.p2inc,
            depth: settings.depth,
            nodes: settings.nodes,
            movetime: settings.movetime,
            movestogo: settings.movestogo,
        }
    }
}

impl GoSettings {
    #[must_use]
    pub fn from_time(
        p1time: Option<i32>,
        p2time: Option<i32>,
        p1inc: Option<i32>,
        p2inc: Option<i32>,
    ) -> Self {
        Self {
            kind: GoKind::Search,
            p1time,
            p2time,
            p1inc,
            p2inc,
            depth: None,
            nodes: None,
            movetime: None,
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_depth(d: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: Some(d),
            nodes: None,
            movetime: None,
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_movetime(t: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: None,
            nodes: None,
            movetime: Some(t),
            movestogo: None,
        }
    }

    #[must_use]
    pub fn from_nodes(n: u64) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: None,
            p2time: None,
            p1inc: None,
            p2inc: None,
            depth: None,
            nodes: Some(n),
            movetime: None,
            movestogo: None,
        }
    }
}
