use crate::args::Args;
use crate::events::{Event, Sample};
use datagen::engine::{Engine, MATE_CP};
use faeries::prng::XorshiftGenerator;
use games::gamerules::{GameResult, GameRules};
use games::general::side::Side;
use protocols::GoSettings;
use std::sync::mpsc;

/// Spread a seed so neighbouring game ids give unrelated random streams.
#[must_use]
fn mix(seed: u64, id: usize) -> u64 {
    let mut z = seed.wrapping_add((id as u64 + 1).wrapping_mul(0x9e3779b97f4a7c15));
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    (z ^ (z >> 31)) | 1
}

/// Play one game from `fen`, sending its scored positions once it ends.
///
/// The opening is `args.random_plies` random moves, which are not recorded. A game that
/// stops without a result is reported with `result: None`.
pub fn play<E: Engine>(
    engine: &mut E,
    thread_id: usize,
    fen: &str,
    id: usize,
    args: &Args,
    tx: &mpsc::Sender<Event>,
) {
    tx.send(Event::GameStart {
        thread_id,
        id,
        fen: fen.to_string(),
    })
    .unwrap();

    let settings = GoSettings::from_nodes(args.nodes);
    let mut prng = XorshiftGenerator::new(mix(args.seed, id));
    let mut pos = E::Game::from_fen(fen);
    let mut samples = Vec::new();
    let mut ply = 0;

    engine.newgame();

    while !pos.is_gameover() && ply < args.max_plies {
        let mv = if ply < args.random_plies {
            let moves = pos.legal_moves();
            moves[prng.next() as usize % moves.len()]
        } else {
            let found = engine.search(&pos, &settings);
            let Some(mv) = found.mv else { break };

            if let Some(score) = found.score {
                let score = if pos.get_turn() == Side::Player1 {
                    score
                } else {
                    -score
                };
                samples.push(Sample {
                    fen: pos.get_fen(),
                    score: score.clamp(-MATE_CP, MATE_CP),
                });
            }
            mv
        };

        pos.makemove(&mv);
        ply += 1;
    }

    tx.send(Event::GameFinish {
        thread_id,
        id,
        result: pos.get_result(),
        samples,
    })
    .unwrap();
}

/// A game result as the Player 1 score: 1 for a win, 0.5 for a draw, 0 for a loss.
#[must_use]
pub fn result_string(result: GameResult) -> &'static str {
    match result {
        GameResult::Win(Side::Player1) => "1.0",
        GameResult::Win(Side::Player2) => "0.0",
        GameResult::Draw => "0.5",
    }
}
