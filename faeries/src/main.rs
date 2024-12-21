mod args;
mod ataxx;
mod droptaxx;
mod openings;
mod search;
mod searchstats;

use args::parse_args;
use ataxx::state::AtaxxState;
use droptaxx::state::DroptaxxState;
use games::{ataxx::AtaxxPosition, droptaxx::DroptaxxPosition, gamerules::GameRules};
use manual::Manual;
use openings::generate;
use ugi::UGI;

fn print_about() {
    println!("Faeries v{}", env!("CARGO_PKG_VERSION"));
    if cfg!(debug_assertions) {
        println!("Debug enabled");
    }
    println!("Games supported:");
    println!("- Ataxx");
    println!("- Droptaxx");
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
                    "droptaxx" => Box::new(DroptaxxState::default()),
                    _ => panic!("Unknown game"),
                };
                ugi::listen::listen(&mut state)?
            }
            "manual" => {
                let mut state: Box<dyn Manual> = match args.game.as_str() {
                    "ataxx" => Box::new(AtaxxState::default()),
                    "droptaxx" => Box::new(DroptaxxState::default()),
                    _ => panic!("Unknown game"),
                };
                manual::listen::listen(&mut state)?
            }
            "openings" => {
                let depth = stream.next().unwrap_or("1").parse::<i32>().unwrap();
                match args.game.as_str() {
                    "ataxx" => generate(&mut AtaxxPosition::startpos(), depth),
                    "droptaxx" => generate(&mut DroptaxxPosition::startpos(), depth),
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
