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

use args::parse_args;
use games::Games;
use games::chess::ChessPosition;
use games::{
    ataxx::AtaxxPosition, connect4::Connect4Position, droptaxx::DroptaxxPosition,
    gamerules::GameRules, gomoku::GomokuPosition, isolation::IsolationPosition,
    pijersi::PijersiPosition,
};
use openings::generate;
use std::ops::DerefMut;
// Protocols
use protocols::manual::Manual;
use protocols::tei::TEI;
use protocols::uci::UCI;
use protocols::ugi::UGI;
// State
use ataxx::state::AtaxxState;
use chess::state::ChessState;
use connect4::state::Connect4State;
use droptaxx::state::DroptaxxState;
use gomoku::state::GomokuState;
use isolation::state::IsolationState;
use pijersi::state::PijersiState;
use tak::manual::ManualTakState;
use tak::tei::TEITakState;
use tak::ugi::UGITakState;

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
                Games::Ataxx => Box::new(AtaxxState::default()),
                Games::Chess => Box::new(ChessState::default()),
                Games::Connect4 => Box::new(Connect4State::default()),
                Games::Droptaxx => Box::new(DroptaxxState::default()),
                Games::Gomoku => Box::new(GomokuState::default()),
                Games::Isolation => Box::new(IsolationState::default()),
                Games::Pijersi => Box::new(PijersiState::default()),
                Games::Tak => Box::new(UGITakState::default()),
            };
            let _ = protocols::ugi::listen::listen(state.deref_mut());
        }
        "uci" => {
            let mut state: Box<dyn UCI> = match args.game.unwrap() {
                Games::Chess => Box::new(ChessState::default()),
                _ => panic!("Game must be chess"),
            };
            let get_input = |input: &mut String| std::io::stdin().read_line(input);
            let _ = protocols::uci::listen::listen(state.deref_mut(), get_input);
        }
        "tei" => {
            let mut state: Box<dyn TEI> = match args.game.unwrap() {
                Games::Tak => Box::new(TEITakState::default()),
                _ => panic!("Game must be tak"),
            };
            let get_input = |input: &mut String| std::io::stdin().read_line(input);
            let _ = protocols::tei::listen::listen(state.deref_mut(), get_input);
        }
        "manual" => {
            let mut state: Box<dyn Manual> = match args.game.unwrap() {
                Games::Ataxx => Box::new(AtaxxState::default()),
                Games::Chess => Box::new(ChessState::default()),
                Games::Connect4 => Box::new(Connect4State::default()),
                Games::Droptaxx => Box::new(DroptaxxState::default()),
                Games::Gomoku => Box::new(GomokuState::default()),
                Games::Isolation => Box::new(IsolationState::default()),
                Games::Pijersi => Box::new(PijersiState::default()),
                Games::Tak => Box::new(ManualTakState::default()),
            };
            protocols::manual::listen::listen(state.deref_mut())?
        }
        "openings" => {
            match args.game.unwrap() {
                Games::Ataxx => generate(&mut AtaxxPosition::<7, 7>::startpos(), args.depth),
                Games::Chess => generate(&mut ChessPosition::startpos(), args.depth),
                Games::Connect4 => generate(&mut Connect4Position::startpos(), args.depth),
                Games::Droptaxx => generate(&mut DroptaxxPosition::startpos(), args.depth),
                Games::Gomoku => generate(&mut GomokuPosition::startpos(), args.depth),
                Games::Isolation => generate(&mut IsolationPosition::startpos(), args.depth),
                Games::Pijersi => generate(&mut PijersiPosition::startpos(), args.depth),
                Games::Tak => todo!(),
            };
        }
        "about" => print_about(),
        "quit" => {}
        _ => {}
    }

    Ok(())
}
