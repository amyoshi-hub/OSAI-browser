use osai_lib::OSAI;
use std::io::{Write};
use std::io;

#[tokio::main]
async fn main() {

    println!("input command");
    let mut in_cmd = String::new();
    io::stdin().read_line(&mut in_cmd).unwrap();
    let cmd = in_cmd.trim();

    let  osai = OSAI::new();
    if cmd == "server"{
        let _ = osai.run().await;
    }else if cmd == "http_server"{
        OSAI::http_server().await;
    }else if cmd == "text" {
        OSAI::send_text_cli().await;
    }else{
        println!("no cmd"); 
    }
}

