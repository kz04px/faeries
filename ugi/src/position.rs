use std::{iter::Peekable, str::SplitAsciiWhitespace};

pub fn parse(stream: &mut Peekable<SplitAsciiWhitespace>, mut func: impl FnMut(&str)) {
    // Parse startpos/fen
    let fen = match stream.next() {
        Some("startpos") => "startpos".to_string(),
        Some("fen") => {
            let mut fen = String::new();
            while let Some(word) = stream.next() {
                if !fen.is_empty() {
                    fen += " ";
                }
                fen += word;

                if stream.peek() == Some(&"moves") {
                    break;
                }
            }
            fen
        }
        _ => "".to_string(),
    };

    // Set FEN
    func(&fen);
}
