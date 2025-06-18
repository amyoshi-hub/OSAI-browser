use tauri::{Builder, Manager};
use tauri::Listener;

#[tauri::command]
fn greet(name: &str) -> String {
    println!("hey {} hello", name);
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();

            let _id = handle.listen("front-to-back", |event| {
                // ★★★ ここを修正します ★★★
                // コンパイラが 'event.payload()' を '&str' と言うので、そのまま受け取る
                let raw_payload: &str = event.payload();

                // '&str' なので、'Option' として扱うのではなく、'is_empty()' でチェック
                let payload_content: String;
                if raw_payload.is_empty() {
                    payload_content = "No payload (empty string)".to_string();
                } else {
                    payload_content = raw_payload.to_string(); // &str を String に変換
                }

                println!(
                    "got front-to-back with payload {:?}",
                    payload_content
                );
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
