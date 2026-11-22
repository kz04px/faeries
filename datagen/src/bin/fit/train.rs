use crate::features::Features;
use std::thread;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// The mean squared error between `sigmoid(eval / k)` and the targets, and its gradient with
/// respect to each parameter.
fn loss_and_gradient(
    features: &Features,
    targets: &[f64],
    weights: &[f64],
    k: f64,
    threads: usize,
) -> (f64, Vec<f64>) {
    let n = features.len();
    let chunk = n.div_ceil(threads).max(1);

    let parts: Vec<(f64, Vec<f64>)> = thread::scope(|scope| {
        let workers: Vec<_> = (0..n)
            .step_by(chunk)
            .map(|start| {
                scope.spawn(move || {
                    let mut loss = 0.0;
                    let mut gradient = vec![0.0; weights.len()];
                    for i in start..(start + chunk).min(n) {
                        let p = sigmoid(features.predict(i, weights) / k);
                        let error = p - targets[i];
                        loss += error * error;
                        let slope = 2.0 * error * p * (1.0 - p) / k;
                        let (index, value) = features.row(i);
                        for (&j, &v) in index.iter().zip(value) {
                            gradient[j as usize] += slope * v as f64;
                        }
                    }
                    (loss, gradient)
                })
            })
            .collect();
        workers.into_iter().map(|w| w.join().unwrap()).collect()
    });

    let mut loss = 0.0;
    let mut gradient = vec![0.0; weights.len()];
    for (l, g) in parts {
        loss += l;
        for (total, part) in gradient.iter_mut().zip(g) {
            *total += part;
        }
    }
    let n = n as f64;
    (loss / n, gradient.into_iter().map(|g| g / n).collect())
}

/// The mean squared error of the game results against `sigmoid(eval / k)` at the defaults.
fn result_loss(features: &Features, results: &[f64], k: f64) -> f64 {
    let total: f64 = features
        .base
        .iter()
        .zip(results)
        .map(|(e, r)| (sigmoid(e / k) - r).powi(2))
        .sum();
    total / features.len() as f64
}

/// The `k` that best fits the game results to the eval at the defaults.
#[must_use]
pub fn fit_k(features: &Features, results: &[f64]) -> f64 {
    // The loss has one minimum in log k
    let (mut lo, mut hi) = (1f64.ln(), 5000f64.ln());
    for _ in 0..60 {
        let a = lo + (hi - lo) / 3.0;
        let b = hi - (hi - lo) / 3.0;
        if result_loss(features, results, a.exp()) < result_loss(features, results, b.exp()) {
            hi = b;
        } else {
            lo = a;
        }
    }
    ((lo + hi) / 2.0).exp()
}

/// What the eval should say: `eval_weight` of the search score and the rest of the game
/// result, as win probabilities.
#[must_use]
pub fn targets(scores: &[f64], results: &[f64], eval_weight: f64, k: f64) -> Vec<f64> {
    scores
        .iter()
        .zip(results)
        .map(|(s, r)| eval_weight * sigmoid(s / k) + (1.0 - eval_weight) * r)
        .collect()
}

pub struct Settings {
    pub k: f64,
    pub epochs: usize,
    pub lr: f64,
    pub threads: usize,
}

pub struct Fitted {
    pub weights: Vec<f64>,
    pub start_loss: f64,
    pub end_loss: f64,
}

/// Adam on the whole data set, from `start`, with every weight kept inside its range. The
/// step shrinks linearly to a tenth of `lr`.
pub fn fit(
    features: &Features,
    targets: &[f64],
    start: &[f64],
    bounds: &[(f64, f64)],
    settings: &Settings,
    progress: impl Fn(usize, f64),
) -> Fitted {
    const BETA1: f64 = 0.9;
    const BETA2: f64 = 0.999;
    const EPSILON: f64 = 1e-8;

    let mut weights = start.to_vec();
    let mut m = vec![0.0; weights.len()];
    let mut v = vec![0.0; weights.len()];
    let mut start_loss = 0.0;

    for epoch in 0..settings.epochs {
        let (loss, gradient) =
            loss_and_gradient(features, targets, &weights, settings.k, settings.threads);
        if epoch == 0 {
            start_loss = loss;
        }
        progress(epoch, loss);

        let t = (epoch + 1) as i32;
        let progress_done = epoch as f64 / settings.epochs as f64;
        let lr = settings.lr * (1.0 - 0.9 * progress_done);
        for j in 0..weights.len() {
            m[j] = BETA1 * m[j] + (1.0 - BETA1) * gradient[j];
            v[j] = BETA2 * v[j] + (1.0 - BETA2) * gradient[j] * gradient[j];
            let m_hat = m[j] / (1.0 - BETA1.powi(t));
            let v_hat = v[j] / (1.0 - BETA2.powi(t));
            weights[j] -= lr * m_hat / (v_hat.sqrt() + EPSILON);
            weights[j] = weights[j].clamp(bounds[j].0, bounds[j].1);
        }
    }

    let (end_loss, _) =
        loss_and_gradient(features, targets, &weights, settings.k, settings.threads);
    Fitted {
        weights,
        start_loss,
        end_loss,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Evals of `constant + 3 * a + 2 * b` for a few positions, with the true weights
    /// `a = 50`, `b = -30`.
    fn synthetic() -> (Features, Vec<f64>) {
        let rows: [(f64, [(u32, f32); 2]); 6] = [
            (0.0, [(0, 1.0), (1, 0.0)]),
            (10.0, [(0, 2.0), (1, 1.0)]),
            (-5.0, [(0, 0.0), (1, 3.0)]),
            (0.0, [(0, 3.0), (1, 2.0)]),
            (20.0, [(0, 1.0), (1, 4.0)]),
            (0.0, [(0, 4.0), (1, 1.0)]),
        ];
        let mut f = Features {
            constant: vec![],
            base: vec![],
            offsets: vec![0],
            index: vec![],
            value: vec![],
        };
        let mut scores = vec![];
        for (c, row) in rows {
            f.constant.push(c);
            f.base.push(c);
            for (j, v) in row {
                if v != 0.0 {
                    f.index.push(j);
                    f.value.push(v);
                }
            }
            f.offsets.push(f.index.len());
            scores.push(c + 50.0 * row[0].1 as f64 - 30.0 * row[1].1 as f64);
        }
        (f, scores)
    }

    #[test]
    fn fits_the_search_scores() {
        let (features, scores) = synthetic();
        let results = vec![0.5; scores.len()];
        let k = 100.0;
        let targets = targets(&scores, &results, 1.0, k);
        let settings = Settings {
            k,
            epochs: 4000,
            lr: 2.0,
            threads: 2,
        };
        let fitted = fit(
            &features,
            &targets,
            &[0.0, 0.0],
            &[(-500.0, 500.0), (-500.0, 500.0)],
            &settings,
            |_, _| {},
        );
        assert!(fitted.end_loss < fitted.start_loss);
        assert!(
            (fitted.weights[0] - 50.0).abs() < 2.0,
            "{:?}",
            fitted.weights
        );
        assert!(
            (fitted.weights[1] + 30.0).abs() < 2.0,
            "{:?}",
            fitted.weights
        );
    }

    #[test]
    fn weights_stay_in_range() {
        let (features, scores) = synthetic();
        let results = vec![0.5; scores.len()];
        let targets = targets(&scores, &results, 1.0, 100.0);
        let settings = Settings {
            k: 100.0,
            epochs: 500,
            lr: 5.0,
            threads: 1,
        };
        let fitted = fit(
            &features,
            &targets,
            &[0.0, 0.0],
            &[(-10.0, 10.0), (-10.0, 10.0)],
            &settings,
            |_, _| {},
        );
        assert!(fitted.weights.iter().all(|w| w.abs() <= 10.0));
    }

    #[test]
    fn gradient_matches_finite_differences() {
        let (features, scores) = synthetic();
        let targets = targets(&scores, &vec![1.0; scores.len()], 0.5, 100.0);
        let weights = [10.0, -4.0];
        let (_, g) = loss_and_gradient(&features, &targets, &weights, 100.0, 3);
        for j in 0..2 {
            let mut up = weights;
            let mut down = weights;
            up[j] += 1e-3;
            down[j] -= 1e-3;
            let (lu, _) = loss_and_gradient(&features, &targets, &up, 100.0, 1);
            let (ld, _) = loss_and_gradient(&features, &targets, &down, 100.0, 1);
            assert!((g[j] - (lu - ld) / 2e-3).abs() < 1e-8, "{}", j);
        }
    }
}
