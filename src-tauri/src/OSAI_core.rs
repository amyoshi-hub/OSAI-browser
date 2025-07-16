use tauri::{Builder};
pub mod server;
pub mod server_signal;
pub mod client;
pub mod file_copy;
pub mod websocket;
pub mod file_server;
pub mod file_read;
pub mod http_server;
pub mod server_list;

use server::start_server;
use client::send_text;
use file_copy::{process_and_add_world};
use file_copy::{get_world_list, open_world};
use websocket::start_websocket_server;
use file_server::get_file_list;
use file_read::read_file_content;
use http_server::{http_server, open_url_window, fetch_file_list, request_file};
use server_list::request_server_list;
mod AI;
mod format_handler;

trait OSAI{

}

impl OSAI{
            let port = String::new();
            //pub async fn start_server(app_handle: tauri::AppHandle, port: String) ->これでもこのまま呼べる？
            start_server(port),
            send_text,
            process_and_add_world,
            get_world_list,
            open_world,
            start_websocket_server,
            get_file_list,
            read_file_content,
            http_server,
            open_url_window,
            fetch_file_list,
            request_file,
            request_server_list,
}

