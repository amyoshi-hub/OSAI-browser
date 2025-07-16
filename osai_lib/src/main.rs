use osai_lib::OSAI;
use std::io::{Write};
use std::io;

#[tokio::main]
async fn main() {

    println!("input command");
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd).unwrap();

    let  osai = OSAI::new();
    if cmd.trim() == "server"{
        let _ = osai.run().await;
    }else{
        OSAI::send_text_cli().await;
    }
}

