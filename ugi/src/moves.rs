use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&str)) {
    for movestr in stream.by_ref() {
        func(movestr);
    }
}
