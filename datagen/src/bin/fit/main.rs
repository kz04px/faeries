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

    let (data, skipped, too_large) = features::load(&args.data_path, args.max_score)?;
    if data.fens.is_empty() {
        return Err(invalid("No positions in the data".to_string()));
    }
    println!(
        "Positions: {} ({} lines skipped, {} over the max score)",
        data.fens.len(),
        skipped,
        too_large
    );
    println!("Parameters: {}", params.len());

    let positions: Vec<E::Game> = data.fens.iter().map(|f| E::Game::from_fen(f)).collect();
    let features = features::extract::<E>(&positions, &params, args.threads).map_err(invalid)?;
    let (mean, worst) = features::check::<E>(&positions, &params, &features);
    println!(
        "Linear model vs eval: mean error {:.3}cp, worst {:.1}cp",
        mean, worst
    );

    // Positions the fit doesn't train on
    let test = match &args.test_path {
        Some(path) => {
            let (test, skipped, too_large) = features::load(path, args.max_score)?;
            if test.fens.is_empty() {
                return Err(invalid("No positions in the test data".to_string()));
            }
            println!(
                "Test positions: {} ({} lines skipped, {} over the max score)",
                test.fens.len(),
                skipped,
                too_large
            );
            let positions: Vec<E::Game> = test.fens.iter().map(|f| E::Game::from_fen(f)).collect();
            let features =
                features::extract::<E>(&positions, &params, args.threads).map_err(invalid)?;
            Some((test, features))
        }
        None => None,
    };

    let k = args
        .k
        .unwrap_or_else(|| train::fit_k(&features, &data.results));
    println!("k: {:.2}", k);
    println!("Eval weight: {}", args.eval_weight);

    let targets = train::targets(&data.scores, &data.results, args.eval_weight, k);
    let defaults: Vec<f64> = params.iter().map(|p| p.default as f64).collect();
    // A held value can't move: its range is just its default
    let is_held = |name: &str| {
        args.hold.iter().any(|h| {
            name == h
                || name
                    .strip_prefix(h.as_str())
                    .is_some_and(|r| r.starts_with('_'))
        })
    };
    for h in &args.hold {
        if !params
            .iter()
            .any(|p| is_held(&p.name) && p.name.starts_with(h.as_str()))
        {
            return Err(invalid(format!("No tunable named '{}' to hold", h)));
        }
    }
    let bounds: Vec<(f64, f64)> = params
        .iter()
        .map(|p| {
            if is_held(&p.name) {
                (p.default as f64, p.default as f64)
            } else {
                (p.min as f64, p.max as f64)
            }
        })
        .collect();
    // Where the fit starts: the defaults, or zero for everything that can move
    let start: Vec<f64> = if args.zero_init {
        defaults
            .iter()
            .zip(&bounds)
            .map(|(&d, &(lo, hi))| if lo == hi { d } else { 0f64.clamp(lo, hi) })
            .collect()
    } else {
        defaults.clone()
    };
    let held = params.iter().filter(|p| is_held(&p.name)).count();
    if held > 0 {
        println!("Held at their defaults: {}", held);
    }
    // Values the fit keeps adding up to zero
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (j, param) in params.iter().enumerate() {
        if let Some(group) = param.mean_group {
            groups.resize(groups.len().max(group + 1), Vec::new());
            groups[group].push(j);
        }
    }
    let test = test.map(|(data, features)| {
        let targets = train::targets(&data.scores, &data.results, args.eval_weight, k);
        (features, targets)
    });
    let test_loss = |weights: &[f64]| -> Option<f64> {
        test.as_ref()
            .map(|(f, t)| train::loss(f, t, weights, k, args.threads))
    };
    let show = |loss: f64, test: Option<f64>| match test {
        Some(test) => format!("train {:.6} test {:.6}", loss, test),
        None => format!("train {:.6}", loss),
    };
    println!(
        "Defaults: {}",
        show(
            train::loss(&features, &targets, &defaults, k, args.threads),
            test_loss(&defaults)
        )
    );
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
        &groups,
        &settings,
        |epoch, loss, weights| {
            if epoch % print_every == 0 {
                println!("Epoch {:>5} {}", epoch, show(loss, test_loss(weights)));
            }
        },
    );
    println!(
        "Loss: {:.6} -> {:.6} (the start is {})",
        fitted.start_loss,
        fitted.end_loss,
        if args.zero_init {
            "every free value at zero"
        } else {
            "the defaults with zero-mean tables"
        }
    );

    let mut report = String::new();
    let rounded = train::round_to_integers(&fitted.weights, &bounds, &groups);
    let rounded_weights: Vec<f64> = rounded.iter().map(|&v| v as f64).collect();
    println!(
        "Fitted, rounded: {}",
        show(
            train::loss(&features, &targets, &rounded_weights, k, args.threads),
            test_loss(&rounded_weights)
        )
    );
    for (param, value) in params.iter().zip(&rounded) {
        writeln!(report, "{}, {}, {}", param.name, param.default, value).unwrap();
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
