pub mod server;
pub mod client;

/*
pub mod file_copy;
pub mod websocket;
pub mod file_server;
pub mod file_read;
pub mod http_server;
pub mod server_list;
use std::net::UdpSocket;
*/
use server::server::start_server;
//use client::send_text;
/*
use file_copy::{process_and_add_world};
use file_copy::{get_world_list, open_world};
use websocket::start_websocket_server;
use file_server::get_file_list;
use file_read::read_file_content;
use http_server::{http_server, open_url_window, fetch_file_list, request_file};
use server_list::request_server_list;
*/
mod AI;

pub struct OSAI;

impl OSAI{
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self){
        start_server("8080".to_string()).await;
        //start_websoket_server();
        //send_text();
        //get_world_list();
        //get_file_list();
        //read_file_content();
        //http_server,
        //request_server_list();
        //fetch_file_list();
        //request_file();
    }
}

