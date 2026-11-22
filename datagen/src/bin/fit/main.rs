//! Fits a game's tunables to the data `datagen` writes, by gradient descent.
//!
//! Each tunable's effect on the eval is measured once for every position by moving it and
//! evaluating again, so the eval code needs nothing beyond its `#[tunable]` tags and the game
//! search's `eval` hook. Training then runs on those effects alone.

mod args;
mod features;
mod train;

use args::Args;
use datagen::engine::Engine;
use faeries::state::{
    AtaxxState, ChessState, Connect4State, DroptaxxState, GomokuState, IsolationState,
    ManualTakState, PijersiState, QuoridorState,
};
use games::{Games, gamerules::GameRules};
use std::fmt::Write as _;
use std::io::{Error, ErrorKind};

fn fit<E: Engine>(args: &Args) -> Result<(), Error>
where
    E::Game: Sync,
{
    let invalid = |msg: String| Error::new(ErrorKind::InvalidInput, msg);

    // Tunables live under the game's primary search
    let prefix = format!("faeries::primary::{}", args.game.to_string().to_lowercase());
    let params = tunable::params(&prefix);
    if params.is_empty() {
        return Err(invalid(format!("No tunables under {}", prefix)));
    }

    let (data, skipped) = features::load(&args.data_path)?;
    if data.fens.is_empty() {
        return Err(invalid("No positions in the data".to_string()));
    }
    println!("Positions: {} ({} lines skipped)", data.fens.len(), skipped);
    println!("Parameters: {}", params.len());

    let positions: Vec<E::Game> = data.fens.iter().map(|f| E::Game::from_fen(f)).collect();
    let features = features::extract::<E>(&positions, &params, args.threads).map_err(invalid)?;
    let (mean, worst) = features::check::<E>(&positions, &params, &features);
    println!(
        "Linear model vs eval: mean error {:.3}cp, worst {:.1}cp",
        mean, worst
    );

    let k = args
        .k
        .unwrap_or_else(|| train::fit_k(&features, &data.results));
    println!("k: {:.2}", k);
    println!("Eval weight: {}", args.eval_weight);

    let targets = train::targets(&data.scores, &data.results, args.eval_weight, k);
    let start: Vec<f64> = params.iter().map(|p| p.default as f64).collect();
    let bounds: Vec<(f64, f64)> = params
        .iter()
        .map(|p| (p.min as f64, p.max as f64))
        .collect();
    let settings = train::Settings {
        k,
        epochs: args.epochs,
        lr: args.lr,
        threads: args.threads,
    };
    let print_every = (args.epochs / 10).max(1);
    let fitted = train::fit(
        &features,
        &targets,
        &start,
        &bounds,
        &settings,
        |epoch, loss| {
            if epoch % print_every == 0 {
                println!("Epoch {:>5} loss {:.6}", epoch, loss);
            }
        },
    );
    println!("Loss: {:.6} -> {:.6}", fitted.start_loss, fitted.end_loss);

    let mut report = String::new();
    for (param, weight) in params.iter().zip(&fitted.weights) {
        writeln!(
            report,
            "{}, {}, {}",
            param.name,
            param.default,
            weight.round() as i32
        )
        .unwrap();
    }
    println!("name, default, fitted");
    print!("{}", report);
    if let Some(path) = &args.output_path {
        std::fs::write(path, report)?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let args = args::parse_commandline()?;

    match args.game {
        Games::Ataxx => fit::<AtaxxState>(&args),
        Games::Chess => fit::<ChessState>(&args),
        Games::Connect4 => fit::<Connect4State>(&args),
        Games::Droptaxx => fit::<DroptaxxState>(&args),
        Games::Gomoku => fit::<GomokuState>(&args),
        Games::Isolation => fit::<IsolationState>(&args),
        Games::Pijersi => fit::<PijersiState>(&args),
        Games::Quoridor => fit::<QuoridorState>(&args),
        Games::Tak => fit::<ManualTakState>(&args),
    }
}
