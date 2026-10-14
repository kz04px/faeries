use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&str, &str)) {
    match stream.next() {
        Some("name") => {}
        _ => return,
    }

    let name = stream.next();

    match stream.next() {
        Some("value") => {}
        _ => return,
    }

    let value = stream.next();

    if let (Some(name), Some(value)) = (name, value) {
        func(name, value);
    }
}
