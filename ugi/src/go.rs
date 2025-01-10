use std::{fmt, iter::Peekable, str::SplitAsciiWhitespace};

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

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>) -> Result<GoSettings, &'static str> {
    let mut settings = GoSettings::default();

    while let Some(word) = stream.next() {
        match word {
            // Single
            "search" => settings.kind = GoKind::Search,
            "perft" => {
                settings.kind = GoKind::Perft;
                // Sometimes the "depth" word won't be supplied
                if let Some(word) = stream.peek() {
                    if let Ok(d) = word.parse::<i32>() {
                        settings.depth = Some(d);
                        stream.next();
                    }
                }
            }
            "fastperft" => {
                settings.kind = GoKind::FastPerft;
                // Sometimes the "depth" word won't be supplied
                if let Some(word) = stream.peek() {
                    if let Ok(d) = word.parse::<i32>() {
                        settings.depth = Some(d);
                        stream.next();
                    }
                }
            }
            "split" => {
                settings.kind = GoKind::SplitPerft;
                // Sometimes the "depth" word won't be supplied
                if let Some(word) = stream.peek() {
                    if let Ok(d) = word.parse::<i32>() {
                        settings.depth = Some(d);
                        stream.next();
                    }
                }
            }
            // Double
            "p1time" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 0 {
                    return Err("Uh oh");
                }

                if settings.p1time.is_some() {
                    return Err("Uh oh");
                }

                settings.p1time = Some(time);
            }
            "p2time" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 0 {
                    return Err("Uh oh");
                }

                if settings.p2time.is_some() {
                    return Err("Uh oh");
                }

                settings.p2time = Some(time);
            }
            "p1inc" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 0 {
                    return Err("Uh oh");
                }

                if settings.p1inc.is_some() {
                    return Err("Uh oh");
                }

                settings.p1inc = Some(time);
            }
            "p2inc" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 0 {
                    return Err("Uh oh");
                }

                if settings.p2inc.is_some() {
                    return Err("Uh oh");
                }

                settings.p2inc = Some(time);
            }
            "depth" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(depth) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if depth < 1 {
                    return Err("Uh oh");
                }

                if settings.depth.is_some() {
                    return Err("Uh oh");
                }

                settings.depth = Some(depth);
            }
            "nodes" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(nodes) = word.parse::<u64>() else {
                    return Err("Uh oh");
                };

                if nodes < 1 {
                    return Err("Uh oh");
                }

                if settings.nodes.is_some() {
                    return Err("Uh oh");
                }

                settings.nodes = Some(nodes);
            }
            "movetime" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 1 {
                    return Err("Uh oh");
                }

                if settings.movetime.is_some() {
                    return Err("Uh oh");
                }

                settings.movetime = Some(time);
            }
            "movestogo" => {
                let Some(word) = stream.next() else {
                    return Err("Uh oh");
                };

                let Ok(time) = word.parse::<i32>() else {
                    return Err("Uh oh");
                };

                if time < 1 {
                    return Err("Uh oh");
                }

                if settings.movestogo.is_some() {
                    return Err("Uh oh");
                }

                settings.movestogo = Some(time);
            }
            _ => {}
        }
    }

    if settings.p1time.is_none()
        && settings.p2time.is_none()
        && settings.depth.is_none()
        && settings.nodes.is_none()
        && settings.movetime.is_none()
    {
        return Err("Uh oh");
    }

    Ok(settings)
}
