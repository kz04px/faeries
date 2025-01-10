mod args;
mod events;
mod fens;
mod play;

use crate::args::Args;
use fens::get_fens;
use games::{
    Games,
    ataxx::AtaxxPosition,
    chess::ChessPosition,
    connect4::Connect4Position,
    droptaxx::DroptaxxPosition,
    gamerules::GameRules,
    gomoku::GomokuPosition,
    isolation::IsolationPosition,
    pijersi::PijersiPosition,
    tak::TakPosition,
};
use protocols::ugi::UGI;
use std::io::Error;

#[must_use]
fn move_string<G: GameRules + UGI>(_pos: &G, _mv: &G::MoveType) -> String {
    "0000".to_string()
}

fn generate<G: GameRules>(_args: Args, _fens: &Vec<String>) -> Result<(), Error> {
    // let mut workers = vec![];
    // let (tx, rx) = mpsc::channel();
    // let num_fens = fens.len();

    // // Create workers
    // for i in 0..args.threads {
    //     let ntx: mpsc::Sender<Event<AtaxxPosition>> = tx.clone();
    //     // let ntx: mpsc::Sender<Event<G>> = tx.clone();
    //     let gg = fens.clone();

    //     workers.push(thread::spawn(move || {
    //         if args.verbose {
    //             tx.send(Event::ThreadStart { thread_id: i }).unwrap();
    //         }

    //         let mut game_number = i;

    //         while game_number < args.num_games {
    //             let fen_idx = game_number % num_fens;
    //             play(
    //                 i,
    //                 &gg[fen_idx],
    //                 game_number,
    //                 &GoSettings::from_nodes(args.nodes),
    //                 &ntx,
    //             );
    //             game_number += args.threads;
    //         }

    //         if args.verbose {
    //             tx.send(Event::ThreadFinish { thread_id: i }).unwrap();
    //         }
    //     }));
    // }

    // let mut games_completed = 0;

    // for event in &rx {
    //     match event {
    //         Event::ThreadStart { thread_id } => {
    //             if args.verbose {
    //                 println!("Start thread {}", thread_id)
    //             }
    //         }
    //         Event::ThreadFinish { thread_id } => {
    //             if args.verbose {
    //                 println!("Finish thread {}", thread_id)
    //             }
    //         }
    //         Event::GameStart { thread_id, id, fen } => {
    //             println!("<Thread:{}> Start game {} fen {}", thread_id, id, fen)
    //         }
    //         Event::GameFinish {
    //             thread_id,
    //             id,
    //             result,
    //         } => {
    //             games_completed += 1;
    //             println!(
    //                 "<Thread:{}> Finish game {} result {}",
    //                 thread_id,
    //                 id,
    //                 match result {
    //                     Some(GameResult::Win(Side::Player1)) => "1-0",
    //                     Some(GameResult::Win(Side::Player2)) => "0-1",
    //                     Some(GameResult::Draw) => "1/2-1/2",
    //                     None => "None",
    //                 }
    //             );
    //             if games_completed >= args.num_games {
    //                 break;
    //             }
    //         }
    //         _ => {}
    //     }
    // }

    // // Wait
    // for worker in workers {
    //     let _ = worker.join();
    // }

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
    }

    let _success = match args.game {
        Some(Games::Ataxx) => generate::<AtaxxPosition<7, 7>>(args, &fens),
        Some(Games::Chess) => generate::<ChessPosition>(args, &fens),
        Some(Games::Connect4) => generate::<Connect4Position>(args, &fens),
        Some(Games::Droptaxx) => generate::<DroptaxxPosition>(args, &fens),
        Some(Games::Gomoku) => generate::<GomokuPosition>(args, &fens),
        Some(Games::Isolation) => generate::<IsolationPosition>(args, &fens),
        Some(Games::Pijersi) => generate::<PijersiPosition>(args, &fens),
        Some(Games::Tak) => generate::<TakPosition<6>>(args, &fens),
        None => todo!(),
    };

    Ok(())
}
