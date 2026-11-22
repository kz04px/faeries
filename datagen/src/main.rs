mod args;
mod engine;
mod events;
mod fens;
mod play;

use crate::args::Args;
use engine::Engine;
use events::Event;
use faeries::state::{
    AtaxxState, ChessState, Connect4State, DroptaxxState, GomokuState, IsolationState,
    ManualTakState, PijersiState, QuoridorState,
};
use fens::get_fens;
use games::Games;
use play::{play, result_string};
use std::fs::OpenOptions;
use std::io::{BufWriter, Error, ErrorKind, Write};
use std::sync::mpsc;
use std::thread;

/// Play the self-play games and write each finished game's positions to the output file.
///
/// Every line is `<fen> | <score> | <result>`. The score is centipawns for Player 1, with
/// forced wins and losses at +-32000, and the result is 1.0, 0.5 or 0.0 for Player 1.
fn generate<E: Engine>(args: Args, fens: &[String]) -> Result<(), Error> {
    if fens.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No fens found"));
    }

    let mut probe = E::default();
    if let Some(name) = &args.search {
        probe
            .set_search(name)
            .map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
    }

    let mut out = BufWriter::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&args.output_path)?,
    );
    let (tx, rx) = mpsc::channel();
    let args = &args;
    let mut games_finished = 0;
    let mut games_kept = 0;
    let mut positions = 0;

    thread::scope(|scope| -> Result<(), Error> {
        for thread_id in 0..args.threads {
            let tx = tx.clone();

            scope.spawn(move || {
                let mut engine = E::default();
                if let Some(name) = &args.search {
                    engine.set_search(name).unwrap();
                }
                engine.init();

                if args.verbose {
                    tx.send(Event::ThreadStart { thread_id }).unwrap();
                }

                let mut game_number = thread_id;
                while game_number < args.num_games {
                    let fen = &fens[game_number % fens.len()];
                    play(&mut engine, thread_id, fen, game_number, args, &tx);
                    game_number += args.threads;
                }

                if args.verbose {
                    tx.send(Event::ThreadFinish { thread_id }).unwrap();
                }
            });
        }
        drop(tx);

        for event in &rx {
            match event {
                Event::ThreadStart { thread_id } => println!("Start thread {}", thread_id),
                Event::ThreadFinish { thread_id } => println!("Finish thread {}", thread_id),
                Event::GameStart { thread_id, id, fen } => {
                    if args.verbose {
                        println!("<Thread:{}> Start game {} fen {}", thread_id, id, fen);
                    }
                }
                Event::GameFinish {
                    thread_id,
                    id,
                    result,
                    samples,
                } => {
                    games_finished += 1;

                    let result = result.map(result_string);

                    if let Some(result) = result {
                        for sample in &samples {
                            writeln!(out, "{} | {} | {}", sample.fen, sample.score, result)?;
                        }
                        out.flush()?;
                        games_kept += 1;
                        positions += samples.len();
                    }

                    if args.verbose {
                        println!(
                            "<Thread:{}> Finish game {} result {} positions {} ({}/{})",
                            thread_id,
                            id,
                            result.unwrap_or("none"),
                            samples.len(),
                            games_finished,
                            args.num_games
                        );
                    }
                }
            }
        }

        Ok(())
    })?;

    println!(
        "Finished {} games, kept {} with {} positions",
        games_finished, games_kept, positions
    );

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = args::parse_commandline()?;
    let fens = get_fens(&args.fens_path)?;

    if args.verbose {
        println!("Verbose enabled");
        println!("game:  {}", args.game.unwrap());
        println!("fens:  {}", fens.len());
        println!("nodes: {}", args.nodes);
        println!("output: {}", args.output_path);
    }

    match args.game {
        Some(Games::Ataxx) => generate::<AtaxxState>(args, &fens),
        Some(Games::Chess) => generate::<ChessState>(args, &fens),
        Some(Games::Connect4) => generate::<Connect4State>(args, &fens),
        Some(Games::Droptaxx) => generate::<DroptaxxState>(args, &fens),
        Some(Games::Gomoku) => generate::<GomokuState>(args, &fens),
        Some(Games::Isolation) => generate::<IsolationState>(args, &fens),
        Some(Games::Pijersi) => generate::<PijersiState>(args, &fens),
        Some(Games::Quoridor) => generate::<QuoridorState>(args, &fens),
        Some(Games::Tak) => generate::<ManualTakState>(args, &fens),
        None => unreachable!(),
    }?;

    Ok(())
}
