use datagen::engine::Engine;
use games::gamerules::GameRules;
use games::general::side::Side;
use std::io;
use std::thread;
use tunable::Param;

/// How far a parameter is moved to measure what it adds to the eval. A multiple of any
/// phase divisor that matters, so the taper's rounding is lost in the size of the step.
const PROBE: i32 = 1 << 16;

/// Positions to compare the linear model with the real eval on.
const CHECK_POSITIONS: usize = 2000;

/// The scored positions of a datagen file.
pub struct Data {
    pub fens: Vec<String>,
    /// Centipawns for Player 1.
    pub scores: Vec<f64>,
    /// 1, 0.5 or 0 for Player 1.
    pub results: Vec<f64>,
}

/// Read `<fen> | <score> | <result>` lines, skipping any that don't parse.
pub fn load(path: &str) -> io::Result<(Data, usize)> {
    let mut data = Data {
        fens: Vec::new(),
        scores: Vec::new(),
        results: Vec::new(),
    };
    let mut skipped = 0;

    for line in std::fs::read_to_string(path)?.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split('|').map(str::trim).collect();
        let parsed = match parts[..] {
            [fen, score, result] => score
                .parse::<f64>()
                .ok()
                .zip(result.parse::<f64>().ok())
                .map(|(s, r)| (fen, s, r)),
            _ => None,
        };
        match parsed {
            Some((fen, score, result)) => {
                data.fens.push(fen.to_string());
                data.scores.push(score);
                data.results.push(result);
            }
            None => skipped += 1,
        }
    }

    Ok((data, skipped))
}

/// Every position's eval as a constant plus what each parameter adds to it, all for
/// Player 1: `eval = constant + sum(value * param)`. Rows are sparse.
pub struct Features {
    pub constant: Vec<f64>,
    /// The eval at the parameters' defaults.
    pub base: Vec<f64>,
    pub offsets: Vec<usize>,
    pub index: Vec<u32>,
    pub value: Vec<f32>,
}

impl Features {
    #[must_use]
    pub fn len(&self) -> usize {
        self.constant.len()
    }

    #[must_use]
    pub fn row(&self, i: usize) -> (&[u32], &[f32]) {
        let range = self.offsets[i]..self.offsets[i + 1];
        (&self.index[range.clone()], &self.value[range])
    }

    /// The eval of position `i` with these parameter values.
    #[must_use]
    pub fn predict(&self, i: usize, weights: &[f64]) -> f64 {
        let (index, value) = self.row(i);
        self.constant[i]
            + index
                .iter()
                .zip(value)
                .map(|(&j, &v)| v as f64 * weights[j as usize])
                .sum::<f64>()
    }
}

fn sign(side: Side) -> i64 {
    if side == Side::Player1 { 1 } else { -1 }
}

/// The eval of every position for the side to move, over `threads` threads.
fn eval_all<E: Engine>(positions: &[E::Game], threads: usize) -> Vec<i64>
where
    E::Game: Sync,
{
    let chunk = positions.len().div_ceil(threads).max(1);
    thread::scope(|scope| {
        let workers: Vec<_> = positions
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|pos| E::eval(pos).expect("The game has no tunable eval") as i64)
                        .collect::<Vec<i64>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().unwrap())
            .collect()
    })
}

/// Measure what each of `params` adds to the eval of each position.
///
/// A parameter is moved by [`PROBE`] and every position evaluated again, so the eval only
/// needs to be linear in its parameters. The parameters are left at their defaults.
pub fn extract<E: Engine>(
    positions: &[E::Game],
    params: &[Param],
    threads: usize,
) -> Result<Features, String>
where
    E::Game: Sync,
{
    if positions.is_empty() {
        return Err("No positions".to_string());
    }
    if E::eval(&positions[0]).is_none() {
        return Err("The game has no tunable eval".to_string());
    }

    let signs: Vec<i64> = positions.iter().map(|p| sign(p.get_turn())).collect();
    let base_stm = eval_all::<E>(positions, threads);
    let base: Vec<f64> = base_stm
        .iter()
        .zip(&signs)
        .map(|(e, s)| (e * s) as f64)
        .collect();

    // Parameter by parameter, as each move leaks a little memory
    let mut columns: Vec<Vec<(u32, f32)>> = Vec::with_capacity(params.len());
    for param in params {
        param.set_unclamped(param.default + PROBE);
        let moved = eval_all::<E>(positions, threads);
        param.restore();

        let mut column = Vec::new();
        for (i, ((m, b), s)) in moved.iter().zip(&base_stm).zip(&signs).enumerate() {
            if m != b {
                column.push((i as u32, ((m - b) * s) as f32 / PROBE as f32));
            }
        }
        columns.push(column);
    }

    // Rows from columns
    let mut offsets = vec![0usize; positions.len() + 1];
    for column in &columns {
        for &(i, _) in column {
            offsets[i as usize + 1] += 1;
        }
    }
    for i in 0..positions.len() {
        offsets[i + 1] += offsets[i];
    }
    let mut next = offsets.clone();
    let mut index = vec![0u32; offsets[positions.len()]];
    let mut value = vec![0f32; index.len()];
    for (j, column) in columns.iter().enumerate() {
        for &(i, v) in column {
            let at = next[i as usize];
            index[at] = j as u32;
            value[at] = v;
            next[i as usize] += 1;
        }
    }

    let mut features = Features {
        constant: base.clone(),
        base,
        offsets,
        index,
        value,
    };
    let defaults: Vec<f64> = params.iter().map(|p| p.default as f64).collect();
    // The part of the eval that isn't from a parameter
    for i in 0..features.len() {
        let from_params = features.predict(i, &defaults) - features.constant[i];
        features.constant[i] -= from_params;
    }

    Ok(features)
}

/// How far the linear model is from the real eval with the parameters moved about, as the
/// mean and the largest error in centipawns.
pub fn check<E: Engine>(positions: &[E::Game], params: &[Param], features: &Features) -> (f64, f64)
where
    E::Game: Sync,
{
    let n = positions.len().min(CHECK_POSITIONS);
    let weights: Vec<f64> = params
        .iter()
        .enumerate()
        .map(|(j, p)| (p.default + p.step * ((j % 5) as i32 - 2)) as f64)
        .collect();

    for (param, w) in params.iter().zip(&weights) {
        param.set_unclamped(*w as i32);
    }
    let real = eval_all::<E>(&positions[..n], 1);
    for param in params {
        param.restore();
    }

    let mut total = 0.0;
    let mut worst: f64 = 0.0;
    for i in 0..n {
        let error = (features.predict(i, &weights)
            - (real[i] * sign(positions[i].get_turn())) as f64)
            .abs();
        total += error;
        worst = worst.max(error);
    }
    (total / n as f64, worst)
}
