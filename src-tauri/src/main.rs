use tauri::{Builder};
mod server;
mod client;
use server::start_server;
use client::send_text;

#[tokio::main]
async fn main() {

    println!("App start");

    Builder::default()
        // generate_handler!に複数のコマンドを渡せます
        .invoke_handler(tauri::generate_handler![start_server, send_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
