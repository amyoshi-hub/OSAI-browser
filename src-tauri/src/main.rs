use tauri::{Builder};
pub mod server;
pub mod server_signal;
pub mod client;
pub mod file_copy;
pub mod websocket;

use server::start_server;
use client::send_text;
use file_copy::{process_and_add_world};
use file_copy::{get_world_list, open_world};
use websocket::start_websocket_server;


#[tokio::main]
async fn main() {

    println!("App start");

    Builder::default()
        // generate_handler!に複数のコマンドを渡せます
        .invoke_handler(tauri::generate_handler![
            start_server,
            send_text,
            process_and_add_world,
            get_world_list,
//            read_text_file,
            open_world,
            start_websocket_server,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
