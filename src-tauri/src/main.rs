fn main() {
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![my_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
