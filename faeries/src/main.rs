mod args;
mod colour;
mod openings;
mod prng;
mod search;
mod searchstats;
// Games
mod ataxx;
mod chess;
mod connect4;
mod droptaxx;
mod gomoku;
mod isolation;
mod pijersi;
mod tak;

use args::{GameType, parse_args};
use ataxx::state::AtaxxState;
use chess::state::ChessState;
use connect4::state::Connect4State;
use droptaxx::state::DroptaxxState;
use games::{
    ataxx::AtaxxPosition, connect4::Connect4Position, droptaxx::DroptaxxPosition,
    gamerules::GameRules, gomoku::GomokuPosition, isolation::IsolationPosition,
    pijersi::PijersiPosition,
};
use gomoku::state::GomokuState;
use isolation::state::IsolationState;
use openings::generate;
use pijersi::state::PijersiState;
use protocols::manual::Manual;
use protocols::uci::UCI;
use protocols::ugi::UGI;
use std::ops::DerefMut;
use tak::state::TakState;

fn print_about() {
    println!("Faeries v{}", env!("CARGO_PKG_VERSION"));
    if cfg!(debug_assertions) {
        println!("Debug enabled");
    }
    println!("Games supported:");
    println!("- Ataxx");
    println!("- Chess");
    println!("- Connect4");
    println!("- Droptaxx");
    println!("- Gomoku");
    println!("- Isolation");
    println!("- Pijersi");
    println!("- Tak");
}

fn main() -> std::io::Result<()> {
    let args = parse_args()?;
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)?;
    match input.as_str().trim_end() {
        "ugi" => {
            let mut state: Box<dyn UGI> = match args.game.unwrap() {
                GameType::Ataxx => Box::new(AtaxxState::default()),
                GameType::Chess => Box::new(ChessState::default()),
                GameType::Connect4 => Box::new(Connect4State::default()),
                GameType::Droptaxx => Box::new(DroptaxxState::default()),
                GameType::Gomoku => Box::new(GomokuState::default()),
                GameType::Isolation => Box::new(IsolationState::default()),
                GameType::Pijersi => Box::new(PijersiState::default()),
                GameType::Tak => Box::new(TakState::default()),
            };
            let get_input = |input: &mut String| std::io::stdin().read_line(input);
            let _ = protocols::ugi::listen::listen(state.deref_mut(), get_input);
        }
        "uci" => {
            let mut state: Box<dyn UCI> = match args.game.unwrap() {
                GameType::Chess => Box::new(ChessState::default()),
                _ => panic!("Game must be chess"),
            };
            let get_input = |input: &mut String| std::io::stdin().read_line(input);
            let _ = protocols::uci::listen::listen(state.deref_mut(), get_input);
        }
        "manual" => {
            let mut state: Box<dyn Manual> = match args.game.unwrap() {
                GameType::Ataxx => Box::new(AtaxxState::default()),
                GameType::Chess => Box::new(ChessState::default()),
                GameType::Connect4 => Box::new(Connect4State::default()),
                GameType::Droptaxx => Box::new(DroptaxxState::default()),
                GameType::Gomoku => Box::new(GomokuState::default()),
                GameType::Isolation => Box::new(IsolationState::default()),
                GameType::Pijersi => Box::new(PijersiState::default()),
                GameType::Tak => Box::new(TakState::default()),
            };
            protocols::manual::listen::listen(state.deref_mut())?
        }
        "openings" => {
            match args.game.unwrap() {
                GameType::Ataxx => generate(&mut AtaxxPosition::startpos(), args.depth),
                GameType::Chess => todo!(),
                GameType::Connect4 => generate(&mut Connect4Position::startpos(), args.depth),
                GameType::Droptaxx => generate(&mut DroptaxxPosition::startpos(), args.depth),
                GameType::Gomoku => generate(&mut GomokuPosition::startpos(), args.depth),
                GameType::Isolation => generate(&mut IsolationPosition::startpos(), args.depth),
                GameType::Pijersi => generate(&mut PijersiPosition::startpos(), args.depth),
                GameType::Tak => todo!(),
            };
        }
        "about" => print_about(),
        "quit" => {}
        _ => {}
    }

    Ok(())
}
