use super::{COMMANDS, GTP, GTPColour, PROTOCOL_VERSION};

/// Strip a line down to the command in it.
///
/// The spec asks for control characters to be dropped, tabs to become spaces, and
/// everything from a `#` onwards to be discarded as a comment. Splitting on
/// whitespace afterwards deals with the rest, including the line ending.
#[must_use]
pub fn preprocess(line: &str) -> String {
    line.chars()
        .take_while(|&c| c != '#')
        .map(|c| if c == '\t' { ' ' } else { c })
        .filter(|c| !c.is_control())
        .collect()
}

/// Write a response: `=` or `?`, the id if the command carried one, the text, and
/// the blank line that ends it.
///
/// There is no space between the prefix and the id, and none after the prefix when
/// the response is empty — a controller reading strictly will not have it.
fn respond(id: Option<i32>, prefix: char, text: &str) {
    match (id, text.is_empty()) {
        (Some(id), true) => println!("{}{}\n", prefix, id),
        (Some(id), false) => println!("{}{} {}\n", prefix, id, text),
        (None, true) => println!("{}\n", prefix),
        (None, false) => println!("{} {}\n", prefix, text),
    }
}

/// Every command this session answers.
#[must_use]
fn commands(state: &dyn GTP) -> Vec<String> {
    COMMANDS
        .iter()
        .map(|c| (*c).to_owned())
        .chain(state.extra_commands())
        .collect()
}

/// An integer argument, or the error GTP gives for a command it could not read.
fn number(args: &[&str], idx: usize) -> Result<i32, String> {
    args.get(idx)
        .ok_or("syntax error")?
        .parse::<i32>()
        .map_err(|_| "syntax error".to_owned())
}

/// A colour argument.
fn colour(args: &[&str], idx: usize) -> Result<GTPColour, String> {
    GTPColour::from_string(args.get(idx).ok_or("syntax error")?).map_err(|e| e.to_owned())
}

/// Answer one command. The reply is the response text, or the text of the failure.
///
/// `quit` is not here: it is the one command that has to stop the loop, and it is
/// dealt with by the caller.
pub fn handle(state: &mut dyn GTP, command: &str, args: &[&str]) -> Result<String, String> {
    match command {
        "protocol_version" => Ok(PROTOCOL_VERSION.to_string()),
        "name" => Ok(state.name()),
        "version" => Ok(state.version()),
        "known_command" => {
            let word = args.first().ok_or("syntax error")?;
            Ok(commands(state).iter().any(|c| c == word).to_string())
        }
        "list_commands" => Ok(commands(state).join("\n")),
        "boardsize" => {
            state.boardsize(number(args, 0)?)?;
            Ok(String::new())
        }
        "clear_board" => {
            state.clear_board();
            Ok(String::new())
        }
        "komi" => {
            let komi = args
                .first()
                .ok_or("syntax error")?
                .parse::<f32>()
                .map_err(|_| "syntax error")?;
            state.komi(komi);
            Ok(String::new())
        }
        "play" => {
            let colour = colour(args, 0)?;
            let vertex = args.get(1).ok_or("syntax error")?;
            state.play(colour, vertex)?;
            Ok(String::new())
        }
        "genmove" => Ok(state.genmove(colour(args, 0)?)),
        "undo" => {
            state.undo()?;
            Ok(String::new())
        }
        "showboard" => Ok(state.showboard()),
        "final_score" => Ok(state.final_score()),
        "final_status_list" => state.final_status_list(args.first().ok_or("syntax error")?),
        // Byo-yomi is optional in the command as well as in the clock: controllers
        // that do not use it send the main time alone.
        "time_settings" => {
            let main = number(args, 0)?;
            state.time_settings(
                main,
                number(args, 1).unwrap_or(0),
                number(args, 2).unwrap_or(0),
            );
            Ok(String::new())
        }
        "time_left" => {
            let colour = colour(args, 0)?;
            let time = number(args, 1)?;
            state.time_left(colour, time, number(args, 2).unwrap_or(0));
            Ok(String::new())
        }
        // `quit` reaches this only when something other than `listen` is driving.
        // Answering it rather than rejecting it keeps `list_commands` honest.
        "quit" => Ok(String::new()),
        _ => state.custom(command, args),
    }
}

pub fn listen(
    state: &mut dyn GTP,
    mut read_input: impl FnMut(&mut String) -> std::io::Result<usize>,
) -> std::io::Result<()> {
    state.init();

    let mut input = String::new();

    loop {
        input.clear();

        // End of input is the controller having gone away, which is a quit that was
        // never typed.
        if read_input(&mut input)? == 0 {
            break;
        }

        let line = preprocess(&input);
        let mut words = line.split_ascii_whitespace().peekable();

        // A command may be numbered, and the number comes back with the response so
        // that the controller can tell which command it is an answer to.
        let id = match words.peek().map(|word| word.parse::<i32>()) {
            Some(Ok(id)) => {
                words.next();
                Some(id)
            }
            _ => None,
        };

        // A line with nothing in it is not a command and is not answered.
        let Some(command) = words.next() else {
            continue;
        };
        let args = words.collect::<Vec<_>>();

        if command == "quit" {
            respond(id, '=', "");
            break;
        }

        match handle(state, command, &args) {
            Ok(text) => respond(id, '=', &text),
            Err(e) => respond(id, '?', &e),
        }
    }

    state.shutdown();

    Ok(())
}
