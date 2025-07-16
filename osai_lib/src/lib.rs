use std::io::{Write};
use std::io;


pub mod server;
pub mod client;

/*
pub mod file_copy;
pub mod websocket;
pub mod file_server;
pub mod file_read;
pub mod server_list;
use std::net::UdpSocket;
*/
use server::server::start_server;
use client::client::send_text;
/*
use file_copy::{process_and_add_world};
use file_copy::{get_world_list, open_world};
use websocket::start_websocket_server;
use file_server::get_file_list;
use file_read::read_file_content;
use http_server::{http_server, open_url_window, fetch_file_list, request_file};
use server_list::request_server_list;
*/
mod ai;

pub struct OSAI;

impl OSAI{
    pub fn new() -> Self {
        Self
    }

    pub async fn send_text_cli(){
        let mut dst_ip = String::new();
        let mut dst_port = String::new();
        let mut text = String::new();
        println!("sendTo:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut dst_ip).unwrap();
        println!("sendPort:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut dst_port).unwrap();
        println!("sendText:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut text).unwrap();

        let dst_ip = dst_ip.trim().to_string();
        let dst_port = dst_port.trim().parse::<u16>().unwrap_or(8080);
        let text = text.trim().to_string();

        send_text(dst_ip, dst_port, text).await;
    }

    pub async fn run(&self) -> Result<(), String>{
        let _ = start_server("8080".to_string()).await?;
        
        //start_websoket_server();
        //send_text();
        //get_world_list();
        //get_file_list();
        //read_file_content();
        //http_server,
        //request_server_list();
        //fetch_file_list();
        //request_file();
        Ok(())
    }
}

