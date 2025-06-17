use tauri::{Builder, Manager};

#[tauri::command]
fn greet(name: &str) -> String {
    println!("hey {} hello", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
  tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

