// src/main.rs (例)
#[tauri::command]
fn get_title_from_webview(window: tauri::WebviewWindow) -> String {
    // ここでwindowを使ってWebViewのタイトルを取得したり、
    // JavaScriptを実行して情報を取得したりするロジックを記述
    // 例えば、content_webview.eval("document.title") など
    // ただし、この場合はiframe内のタイトルを取得することになります
    "Example Title from Rust".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_title_from_webview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
