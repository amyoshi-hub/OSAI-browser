use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static MY_VEC: Lazy<Mutex<[u8; 14]>> = Lazy::new(|| Mutex::new([0u8; 14]));

// 重みベクトルの初期化
pub static W1: Lazy<Mutex<Vec<Vec<f64>>>> = Lazy::new(|| {
    let w1 = (0..1024)
        .map(|_| (0..14).map(|_| rand_init()).collect())
        .collect();
    Mutex::new(w1)
});

pub static W2: Lazy<Mutex<Vec<Vec<f64>>>> = Lazy::new(|| {
    let w2 = (0..14)
        .map(|_| (0..1024).map(|_| rand_init()).collect())
        .collect();
    Mutex::new(w2)
});

// rand_init はここに置くと便利
fn rand_init() -> f64 {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    rng.gen_range(-1.0..=1.0)
}

