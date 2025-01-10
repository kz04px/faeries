//! The Go Text Protocol, which is what Go programs are spoken to in.
//!
//! GTP is not UGI with different words, and what it does differently is what this
//! trait has to expose:
//!
//! - **The controller says whose move it is.** `play white d4` is a legal thing to
//!   send when Black has just played, and a handicap game opens with several Black
//!   moves in a row. Every move command carries a colour, so an engine whose rules
//!   only ever move the side to move has to null the turn over.
//! - **The game is a record, not a position.** There is no FEN here: the controller
//!   builds the game up with `play` and takes it back with `undo`, so an engine has
//!   to be able to unwind.
//! - **Nothing may be written to stdout but responses.** A response is `=` for
//!   success or `?` for failure, the id the command was numbered with if it was
//!   numbered, the text, and a blank line. An engine that prints `info` lines while
//!   it searches will be read as talking nonsense.
//! - **Failure is a normal answer.** An illegal move or a board size the engine
//!   cannot play is a `?` response and the session carries on, which is why the
//!   fallible commands here return `Result<_, String>`: the error is the text the
//!   controller is shown.
//!
//! Vertices are passed through as strings rather than parsed here. Their spelling is
//! standard — a column letter with `I` missing, then a row counting from the bottom
//! — but turning one into a move is a thing only the game knows how to do, and the
//! same is true of what makes a move illegal.

use std::fmt;

pub mod listen;

/// The standard commands, which every implementor answers. `list_commands` and
/// `known_command` are answers about this list, plus whatever `extra_commands` adds.
pub const COMMANDS: [&str; 17] = [
    "protocol_version",
    "name",
    "version",
    "known_command",
    "list_commands",
    "quit",
    "boardsize",
    "clear_board",
    "komi",
    "play",
    "genmove",
    "undo",
    "showboard",
    "final_score",
    "final_status_list",
    "time_settings",
    "time_left",
];

/// The version of the protocol this speaks. There has not been a third.
pub const PROTOCOL_VERSION: i32 = 2;

/// A colour, as the controller names it. Which of the engine's two sides this is
/// belongs to the engine: Black moves first.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GTPColour {
    Black,
    White,
}

impl GTPColour {
    /// Parse a colour. A single letter is as common in the wild as the full word,
    /// and case is not significant anywhere in GTP.
    pub fn from_string(word: &str) -> Result<Self, &'static str> {
        match word.to_ascii_lowercase().as_str() {
            "b" | "black" => Ok(Self::Black),
            "w" | "white" => Ok(Self::White),
            _ => Err("invalid color"),
        }
    }
}

impl fmt::Display for GTPColour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Black => write!(f, "black"),
            Self::White => write!(f, "white"),
        }
    }
}

pub trait GTP {
    fn init(&mut self) {}

    fn shutdown(&mut self) {}

    #[must_use]
    fn name(&self) -> String;

    #[must_use]
    fn version(&self) -> String;

    /// Set the board size, and start a new game on it. Controllers send this even
    /// when the size is the one already up, and expect a clear board either way.
    ///
    /// An engine built for one size says so rather than pretending: the error text
    /// GTP expects for that is `unacceptable size`.
    fn boardsize(&mut self, size: i32) -> Result<(), String>;

    fn clear_board(&mut self);

    /// Komi in points, which is conventionally a half point so that the game cannot
    /// be drawn. It is a setting rather than part of the game, so it survives
    /// `clear_board`.
    fn komi(&mut self, komi: f32);

    /// Play a move for a colour, which need not be the one whose turn it is.
    ///
    /// The vertex is unparsed: it is a point such as `D4`, or `pass`. An unreadable
    /// one and an unplayable one are both failures, and GTP has one error for both.
    fn play(&mut self, colour: GTPColour, vertex: &str) -> Result<(), String>;

    /// Choose a move for a colour, play it, and name it. The answer is a vertex,
    /// `pass`, or `resign`.
    #[must_use]
    fn genmove(&mut self, colour: GTPColour) -> String;

    /// Take back the last move. Failing when there is nothing to take back is the
    /// point of the `Result`; GTP's text for it is `cannot undo`.
    fn undo(&mut self) -> Result<(), String>;

    /// The board, drawn however the engine likes. It is for a person to read.
    #[must_use]
    fn showboard(&self) -> String;

    /// The final score, as `B+7.5`, `W+0.5`, or `0` for a tie.
    #[must_use]
    fn final_score(&self) -> String;

    /// The stones with a given status — `alive`, `dead` or `seki` — as vertices.
    /// Anything else is `invalid status`.
    fn final_status_list(&self, status: &str) -> Result<String, String>;

    /// The clock the game is to be played to, in seconds. Byo-yomi is a promise
    /// about what the clock will do later; an engine may take it or leave it.
    fn time_settings(&mut self, main: i32, byoyomi: i32, stones: i32);

    /// What is left on a side's clock, in seconds. `stones` is how many moves are
    /// left in the current byo-yomi period, and is zero outside one.
    fn time_left(&mut self, colour: GTPColour, time: i32, stones: i32);

    /// Commands this engine answers beyond the standard ones.
    ///
    /// GTP has no `setoption`, so an engine with options has nowhere to put them but
    /// commands of its own, which are conventionally named after the program. They
    /// are listed by `list_commands` alongside the rest and answered by `custom`.
    #[must_use]
    fn extra_commands(&self) -> Vec<String> {
        vec![]
    }

    /// Answer one of the above. The default rejects everything, which is what an
    /// engine that added none of them wants.
    fn custom(&mut self, _command: &str, _args: &[&str]) -> Result<String, String> {
        Err("unknown command".to_owned())
    }
}
