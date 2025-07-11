use tauri::{AppHandle, Emitter};
use crate::server::ServerInfo;
use std::net::SocketAddr;
use crate::AI::hebbian_local::AI;
use crate::AI::state::MY_VEC;

pub fn process_format(
    format: [u8; 2],
    session_id: [u8; 16],
    chunk: [u8; 8],
    data_vec: [u8; 14],
    data_payload: Vec<u8>,
    addr: SocketAddr,
    port: String,
    app_handle: &AppHandle,
) -> String {
    println!("format:{:x?}", format);
    println!("data_vec:{:x?}", data_vec);

    //ほんとはSIMDでやりたい
    //こういう記法はむしろIOTで扱えるフォーマットを絞ってやる
    let received_data_display = match format {
        [0, 0] => {
            String::from_utf8_lossy(&data_payload).to_string()
        }
        [0, 1] => {
            hex::encode(&data_payload)
        }
        [0, 2] => {
            let input_vec = data_vec;
            let mut my_vec_guard = MY_VEC.lock().unwrap();
            let (new_vec, is_trusted) = AI(*my_vec_guard, input_vec);
            if is_trusted {
                *my_vec_guard = new_vec;
                //app_handle.emit("trusted_node", "学習データを更新しました").unwrap();
            } else {
                //app_handle.emit("enemy_detected", "敵シグナルを検知").unwrap();
            }
            format!("AI check result: trusted={}", is_trusted) 
        }

        [0xFF, 0xFF] => {
            println!("--- Server Discovery Signal Received ---");
            println!("  Discovered Server IP: {}", addr);
            println!("  Discovered Server PORT: {}", port);
            println!("  Session ID: {:x?}", session_id);
            println!("  Data Vec: {:x?}", data_vec);

            let server_info = ServerInfo {
                addr: addr.to_string(),
                port: port.clone(),
            };

            if let Err(e) = app_handle.emit("add_server", server_info) {
                eprintln!("Failed to emit add_server event: {}", e);
            }

            if let Ok(msg) = String::from_utf8(data_payload) {
                println!("  Signal Message: {}", msg);
                format!("Server Discovered: {} ({})", addr, msg)
            } else {
                format!("Server Discovered: {}", addr)
            }
        }

        _ => {
            format!("unsupported format: {:x?}", format)
        }
    };

    println!("data:{}", received_data_display);
    received_data_display
}

