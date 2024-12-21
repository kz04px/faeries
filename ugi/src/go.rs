use std::{fmt, iter::Peekable, str::SplitAsciiWhitespace};

#[derive(Default, PartialEq)]
pub enum GoKind {
    #[default]
    Search,
    Perft,
    FastPerft,
    SplitPerft,
}

#[derive(Default)]
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

impl GoSettings {
    #[must_use]
    pub fn from_time(p1time: i32, p2time: i32, p1inc: i32, p2inc: i32) -> Self {
        Self {
            kind: GoKind::Search,
            p1time: Some(p1time),
            p2time: Some(p2time),
            p1inc: Some(p1inc),
            p2inc: Some(p2inc),
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

impl fmt::Display for GoSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            GoKind::Search => write!(f, "search")?,
            GoKind::Perft => write!(f, "perft")?,
            GoKind::FastPerft => write!(f, "fastperft")?,
            GoKind::SplitPerft => write!(f, "split")?,
        }

        if let Some(n) = self.p1time {
            write!(f, " p1time {}", n)?;
        } else {
            write!(f, " p1time None")?;
        }

        if let Some(n) = self.p2time {
            write!(f, " p2time {}", n)?;
        } else {
            write!(f, " p2time None")?;
        }

        if let Some(n) = self.p1inc {
            write!(f, " p1inc {}", n)?;
        } else {
            write!(f, " p1inc None")?;
        }

        if let Some(n) = self.p2inc {
            write!(f, " p2inc {}", n)?;
        } else {
            write!(f, " p2inc None")?;
        }

        if let Some(n) = self.depth {
            write!(f, " depth {}", n)?;
        } else {
            write!(f, " depth None")?;
        }

        if let Some(n) = self.nodes {
            write!(f, " nodes {}", n)?;
        } else {
            write!(f, " nodes None")?;
        }

        if let Some(n) = self.movetime {
            write!(f, " movetime {}", n)?;
        } else {
            write!(f, " movetime None")?;
        }

        if let Some(n) = self.movestogo {
            write!(f, " movestogo {}", n)?;
        } else {
            write!(f, " movestogo None")?;
        }

        Ok(())
    }
}

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&GoSettings)) {
    let mut settings = GoSettings::default();

    while let Some(word) = stream.next() {
        match word {
            // Single
            "search" => settings.kind = GoKind::Search,
            "perft" => settings.kind = GoKind::Perft,
            "fastperft" => settings.kind = GoKind::FastPerft,
            "split" => settings.kind = GoKind::SplitPerft,
            // Double
            "p1time" => settings.p1time = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "p2time" => settings.p2time = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "p1inc" => settings.p1inc = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "p2inc" => settings.p2inc = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "depth" => settings.depth = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "nodes" => settings.nodes = stream.next().unwrap_or("0").parse::<u64>().ok(),
            "movetime" => settings.movetime = stream.next().unwrap_or("0").parse::<i32>().ok(),
            "movestogo" => settings.movestogo = stream.next().unwrap_or("0").parse::<i32>().ok(),
            _ => {
                // Sometimes the "depth" word won't be supplied
                settings.depth = word.parse::<i32>().ok();
            }
        }
    }

    func(&settings);
}
