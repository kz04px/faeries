mod args;
mod ataxx;
mod connect4;
mod droptaxx;
mod gomoku;
mod isolation;
mod openings;
mod search;
mod searchstats;

use args::parse_args;
use ataxx::state::AtaxxState;
use connect4::state::Connect4State;
use droptaxx::state::DroptaxxState;
use games::{
    ataxx::AtaxxPosition, connect4::Connect4Position, droptaxx::DroptaxxPosition,
    gamerules::GameRules, gomoku::GomokuPosition, isolation::IsolationPosition,
};
use gomoku::state::GomokuState;
use isolation::state::IsolationState;
use manual::Manual;
use openings::generate;
use std::ops::DerefMut;
use ugi::UGI;

fn print_about() {
    println!("Faeries v{}", env!("CARGO_PKG_VERSION"));
    if cfg!(debug_assertions) {
        println!("Debug enabled");
    }
    println!("Games supported:");
    println!("- Ataxx");
    println!("- Connect4");
    println!("- Droptaxx");
    println!("- Gomoku");
    println!("- Isolation");
}

fn main() -> std::io::Result<()> {
    let args = parse_args();
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    let mut stream = input.split_ascii_whitespace().peekable();
    while let Some(word) = stream.next() {
        match word {
            "ugi" => {
                let mut state: Box<dyn UGI> = match args.game.as_str() {
                    "ataxx" => Box::new(AtaxxState::default()),
                    "connect4" => Box::new(Connect4State::default()),
                    "droptaxx" => Box::new(DroptaxxState::default()),
                    "gomoku" => Box::new(GomokuState::default()),
                    "isolation" => Box::new(IsolationState::default()),
                    _ => panic!("Unknown game"),
                };
                let get_input = |input: &mut String| std::io::stdin().read_line(input);
                let _ = ugi::listen::listen(state.deref_mut(), get_input);
            }
            "manual" => {
                let mut state: Box<dyn Manual> = match args.game.as_str() {
                    "ataxx" => Box::new(AtaxxState::default()),
                    "connect4" => Box::new(Connect4State::default()),
                    "droptaxx" => Box::new(DroptaxxState::default()),
                    "gomoku" => Box::new(GomokuState::default()),
                    "isolation" => Box::new(IsolationState::default()),
                    _ => panic!("Unknown game"),
                };
                manual::listen::listen(&mut state)?
            }
            "openings" => {
                let depth = stream.next().unwrap_or("1").parse::<i32>().unwrap();
                match args.game.as_str() {
                    "ataxx" => generate(&mut AtaxxPosition::startpos(), depth),
                    "connect4" => generate(&mut Connect4Position::startpos(), depth),
                    "droptaxx" => generate(&mut DroptaxxPosition::startpos(), depth),
                    "gomoku" => generate(&mut GomokuPosition::startpos(), depth),
                    "isolation" => generate(&mut IsolationPosition::startpos(), depth),
                    _ => panic!("Unknown game"),
                };
            }
            "about" => print_about(),
            "quit" => {}
            _ => println!("Unknown protocol"),
        }
    }

    Ok(())
}
