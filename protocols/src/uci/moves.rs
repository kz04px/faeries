use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, func: impl FnMut(&str)) {
    stream.for_each(func)
}
