use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>) -> Result<String, &'static str> {
    match stream.next() {
        Some("startpos") => Ok("startpos".to_owned()),
        Some("tps") => {
            let mut tps = String::new();
            while let Some(word) = stream.peek() {
                if *word == "moves" {
                    break;
                }

                if !tps.is_empty() {
                    tps += " ";
                }
                tps += word;

                stream.next();
            }

            if tps.is_empty() {
                Err("Uh oh")
            } else {
                Ok(tps)
            }
        }
        _ => Err("Uh oh"),
    }
}
