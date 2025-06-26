use rand::Rng;
use std::f64::consts::E;

const INPUT_SIZE: usize = 2;
const HIDDEN_SIZE: usize = 3;
const OUTPUT_SIZE: usize = 1;
const EPOCHS: usize = 10000;
const ALPHA: f64 = 0.5;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn rnd() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-1.0..1.0)
}

fn shared_error(index: usize, errors: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut count = 0;

    for i in index.saturating_sub(1)..=(index + 1).min(errors.len() - 1) {
        sum += errors[i];
        count += 1;
    }
    sum / count as f64
}

fn main() {
    let inputs = [
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
    ];
    let targets = [0.0, 1.0, 1.0, 0.0];

    // Initialize weights: input -> hidden
    let mut w1: [[f64; INPUT_SIZE]; HIDDEN_SIZE] = [[0.0; INPUT_SIZE]; HIDDEN_SIZE];
    for i in 0..HIDDEN_SIZE {
        for j in 0..INPUT_SIZE {
            w1[i][j] = rnd();
        }
    }

    // Initialize weights: hidden -> output
    let mut w2: [f64; HIDDEN_SIZE] = [0.0; HIDDEN_SIZE];
    for j in 0..HIDDEN_SIZE {
        w2[j] = rnd();
    }

    // Training
    for _ in 0..EPOCHS {
        for (n, &input) in inputs.iter().enumerate() {
            // Forward pass
            let mut hidden = [0.0; HIDDEN_SIZE];
            for i in 0..HIDDEN_SIZE {
                hidden[i] = (0..INPUT_SIZE).map(|j| w1[i][j] * input[j]).sum::<f64>();
                hidden[i] = sigmoid(hidden[i]);
            }

            let output = sigmoid(hidden.iter().zip(w2.iter()).map(|(h, w)| h * w).sum::<f64>());
            let error_out = targets[n] - output;

            // Calculate hidden errors
            let hidden_errors: Vec<f64> = w2.iter().map(|&w| error_out * w).collect();

            // Update w2 (hidden -> output)
            for j in 0..HIDDEN_SIZE {
                w2[j] += ALPHA * error_out * hidden[j];
            }

            // Update w1 (input -> hidden) with shared errors
            for i in 0..HIDDEN_SIZE {
                let shared = shared_error(i, &hidden_errors);
                for j in 0..INPUT_SIZE {
                    w1[i][j] += ALPHA * shared * input[j];
                }
            }
        }
    }

    // Test
    println!("Result after training:");
    for &input in &inputs {
        let mut hidden = [0.0; HIDDEN_SIZE];
        for i in 0..HIDDEN_SIZE {
            hidden[i] = (0..INPUT_SIZE).map(|j| w1[i][j] * input[j]).sum::<f64>();
            hidden[i] = sigmoid(hidden[i]);
        }

        let output = sigmoid(hidden.iter().zip(w2.iter()).map(|(h, w)| h * w).sum::<f64>());
        println!("in: {:.0} {:.0} -> out: {:.3}", input[0], input[1], output);
    }
}

