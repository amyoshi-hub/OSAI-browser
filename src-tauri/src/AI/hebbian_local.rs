use rand::Rng;

const INPUT_SIZE: usize = 14;
const HIDDEN_SIZE: usize = 1024;
const OUTPUT_SIZE: usize = 14;
const ALPHA: f64 = 0.1;
const EPOCHS: usize = 10;
const SIMI_THRESHOLD: f64 = 0.95;

fn convert_u8_to_f64_array(input: [u8; 14]) -> [f64; 14] {
    let mut result = [0.0; 14];
    for (i, val) in input.iter().enumerate() {
        result[i] = *val as f64 / 255.0;
    }
    result
}

fn convert_f64_to_u8_array(input: [f64; 14]) -> [u8; 14] {
    let mut result = [0u8; 14];
    for (i, val) in input.iter().enumerate() {
        result[i] = (val * 255.0).clamp(0.0, 255.0) as u8;
    }
    result
}

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + f64::exp(-x))
}

fn rand_init() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-1.0..=1.0)
}

fn calc_and_return_output(input: [f64; 14]) -> [f64; 14] {
    let mut w1: Vec<Vec<f64>> = (0..HIDDEN_SIZE)
        .map(|_| (0..INPUT_SIZE).map(|_| rand_init()).collect())
        .collect();

    let mut w2: Vec<Vec<f64>> = (0..OUTPUT_SIZE)
        .map(|_| (0..HIDDEN_SIZE).map(|_| rand_init()).collect())
        .collect();

    // フォワードパス (隠れ層)
    let mut hidden = vec![0.0; HIDDEN_SIZE];
    for i in 0..HIDDEN_SIZE {
        let mut sum = 0.0;
        for j in 0..INPUT_SIZE {
            sum += w1[i][j] * input[j];
        }
        hidden[i] = sigmoid(sum);
    }

    // フォワードパス (出力層)
    let mut output = [0.0; OUTPUT_SIZE];
    for i in 0..OUTPUT_SIZE {
        let mut sum = 0.0;
        for j in 0..HIDDEN_SIZE {
            sum += w2[i][j] * hidden[j];
        }
        output[i] = sigmoid(sum);
    }

    output
}

fn check_similarity(a: [u8; 14], b: [u8; 14]) -> f64 {
    let mut match_count = 0;
    for i in 0..14 {
        if a[i] == b[i] {
            match_count += 1;
        }
    }
    match_count as f64 / 14.0
}

pub fn AI(my_vec: [u8; 14], input_vec: [u8; 14]) -> ([u8; 14], bool) {
    let input_vec_f64 = convert_u8_to_f64_array(input_vec);
    let output_f64 = calc_and_return_output(input_vec_f64);
    let output_u8 = convert_f64_to_u8_array(output_f64);

    let similarity = check_similarity(my_vec, input_vec);

    if similarity >= SIMI_THRESHOLD {
        let mut new_vec = [0u8; 14];
        for i in 0..14 {
            new_vec[i] = ((my_vec[i] as u16 + output_u8[i] as u16) / 2) as u8;
        }
        (new_vec, true)
    } else {
        println!("enemy");
        (my_vec, false)
    }
}

