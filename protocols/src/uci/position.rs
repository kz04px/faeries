use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>) -> Result<String, &'static str> {
    match stream.next() {
        Some("startpos") => Ok("startpos".to_owned()),
        Some("fen") => {
            let mut fen = String::new();
            while let Some(word) = stream.peek() {
                if *word == "moves" {
                    break;
                }

                if !fen.is_empty() {
                    fen += " ";
                }
                fen += word;

                stream.next();
            }

            if fen.is_empty() {
                Err("Uh oh")
            } else {
                Ok(fen)
            }
        }
        _ => Err("Uh oh"),
    }
}
