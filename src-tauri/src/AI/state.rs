use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static MY_VEC: Lazy<Mutex<[u8; 14]>> = Lazy::new(|| Mutex::new([0u8; 14]));

