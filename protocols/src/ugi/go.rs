use crate::{GoKind, GoSettings};
use std::{iter::Peekable, str::SplitAsciiWhitespace};

#[must_use]
fn show_or<T>(value: Option<T>) -> String
where
    T: ToString,
{
    if let Some(n) = value {
        n.to_string()
    } else {
        "None".to_string()
    }
}

#[must_use]
pub fn to_ugi_string(settings: &GoSettings) -> String {
    format!(
        "go {} p1time {} p2time {} p1inc {} p2inc {} depth {} nodes {} movetime {} movestogo {}",
        match settings.kind {
            GoKind::Search => "search",
            GoKind::Perft => "perft",
            GoKind::FastPerft => "fastperft",
            GoKind::SplitPerft => "split",
        },
        show_or(settings.p1time),
        show_or(settings.p2time),
        show_or(settings.p1inc),
        show_or(settings.p2inc),
        show_or(settings.depth),
        show_or(settings.nodes),
        show_or(settings.movetime),
        show_or(settings.movestogo),
    )
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
