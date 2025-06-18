use tokio::net::UdpSocket;
use tokio::task;
//use tauri::{Builder, Manager};
use tauri::Emitter;
//use net_arc::{netConfig, UdpArc};

#[tauri::command]
pub async fn start_server(app_handle: tauri::AppHandle, ip: String, port: String) -> Result<String, String> {
    let address = format!("{}:{}", ip, port);
    println!("Attempting to bind UDP server to: {}", address);

    let socket = match UdpSocket::bind(&address).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to bind socket to {}: {}", address, e);
            return Err(format!("Failed to bind socket: {}", e));
        }
    };
    println!("UDP Server successfully bound to: {}", address);

    // Frontendにイベントを送信する例 (オプション)
    app_handle.emit("server_status", format!("Server started on {}", address))
        .map_err(|e| format!("Failed to emit event: {}", e))?;

    task::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    match parse_packet(&buf[..len]) {
                        Some((session_id, chunk, format, data_vec, data)) => {
                            let received_data = match &format {
                                b"00" => String::from_utf8_lossy(&data).to_string(),
                                b"01" => hex::encode(&data),
                                _ => format!("unsupported format: {:x?}", format), // セミコロンを追加
                            };
                            println!("session_id:{:x?}", session_id);
                            println!("chunk:{:x?}", chunk);
                            println!("format:{:x?}", format);
                            println!("data_vec:{:x?}", data_vec);
                            println!("data:{:x?}", data.len());

                            if let Err(e) = app_handle.emit("udp_data_received", format!("{} from {}", received_data, addr)) {
                                eprintln!("Failed to emit UDP data event: {}", e);
                            }
                        }
                        None => {
                            eprintln!("Failed to parse data");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    break;
                }
            }
        }
    }); // `task::spawn` の閉じカッコを正しい位置に配置

    Ok(format!("Server started successfully on {}", address)) // 正しく閉じる
}

fn parse_packet(payload: &[u8]) -> Option<([u8; 16], [u8; 8], [u8; 2], [u8; 14], Vec<u8>)> {
    let session_id = payload[0..16].try_into().ok()?;
    let chunk = payload[16..24].try_into().ok()?;
    let format = payload[24..26].try_into().ok()?;
    let data_vec = payload[26..40].try_into().ok()?; // 修正: 正しいスライス範囲
    let data = payload[40..].to_vec();

    Some((session_id, chunk, format, data_vec, data))
}

