// src/server.rs
use tokio::net::UdpSocket;
use tokio::task;
use tauri::Emitter;
//use rand::Rng; // session_id のランダム生成用

//use crate::client;
use crate::server_signal;

// parse_packet も同じファイルに移動したと仮定
fn parse_packet(payload: &[u8]) -> Option<([u8; 16], [u8; 8], [u8; 2], [u8; 14], Vec<u8>)> {
    if payload.len() < 40 { // 最低限のヘッダー長を確認
        return None;
    }
    let session_id = payload[0..16].try_into().ok()?;
    let chunk = payload[16..24].try_into().ok()?;
    let format = payload[24..26].try_into().ok()?;
    let data_vec = payload[26..40].try_into().ok()?;
    let data = payload[40..].to_vec();

    Some((session_id, chunk, format, data_vec, data))
}


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

    // --- サーバー起動シグナルの送信 ---
    let mut send_buf = [0u8; 1024]; // 送信用のバッファ
    let local_port = port.parse::<u16>().map_err(|e| format!("Invalid port number: {}", e))?;
    
    // server_signal モジュールからパケット構築関数を呼び出し
    let packet_len = server_signal::build_server_announce_packet(
        &mut send_buf,
        local_port, // 送信元ポート
        local_port, // 送信先ポート (自分自身を検出するテスト用。実際はブロードキャスト先ポートなど)
    );

    // シグナル送信用のソケットを別に用意 (task::spawn が socket を move するため)
    let send_socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await
        .map_err(|e| format!("Failed to create send socket for signal: {}", e))?;
    
    let broadcast_addr = format!("255.255.255.255:{}", local_port); // ブロードキャストアドレスの例
    send_socket.set_broadcast(true).map_err(|e| format!("Failed to set broadcast: {}", e))?; // ブロードキャスト有効化

    // シグナルを送信
    send_socket.send_to(&send_buf[..packet_len], &broadcast_addr).await
        .map_err(|e| format!("Failed to send server announce signal: {}", e))?;
    
    println!("Server announce signal sent to {}", broadcast_addr);
    // ------------------------------------

    task::spawn(async move {
        let mut buf = [0; 1024];
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((len, addr)) => {
                    match parse_packet(&buf[..len]) {
                        Some((session_id, chunk, format, data_vec, data_payload)) => {
                            let received_data_display: String; // 表示用の文字列を先に定義

                            match format {
                                [0, 0] => {
                                    received_data_display = String::from_utf8_lossy(&data_payload).to_string();
                                },
                                [0, 1] => {
                                    received_data_display = hex::encode(&data_payload);
                                },
                                
                                [0xFF, 0xFF] => {
                                    println!("--- Server Discovery Signal Received ---");
                                    println!("  Discovered Server IP: {}", addr); // 送信元IPアドレスを表示
                                    println!("  Session ID: {:x?}", session_id); // シグナルを送ってきたセッションIDも確認用に表示
                                    println!("  Data Vec (for this signal, might be all zeros/FFs): {:x?}", data_vec); // data_vecも確認用に表示
                                    
                                    // シグナルパケットの data_payload に含まれるメッセージを表示
                                    if let Ok(msg) = String::from_utf8(data_payload.to_vec()) {
                                        println!("  Signal Message: {}", msg);
                                        received_data_display = format!("Server Discovered: {} ({})", addr, msg);
                                    } else {
                                        received_data_display = format!("Server Discovered: {}", addr);
                                    }
                                },
                                
                                _ => {
                                    received_data_display = format!("unsupported format: {:x?}", format);
                                },
                            };
                            
                            // 各フィールドのログ出力は、display 変数に集約したため不要になる可能性あり
                            // 例えば `received_data_display` だけ emit するなら、これらはデバッグ時にだけ有効化する
                            println!("session_id:{:x?}", session_id);
                            println!("chunk:{:x?}", chunk);
                            println!("format:{:x?}", format);
                            println!("data_vec:{:x?}", data_vec);
                            println!("data:{}", received_data_display);

                            if let Err(e) = app_handle.emit("udp_data_received", format!("{} from {}", received_data_display, addr)) {
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
    }); // `task::spawn` の閉じカッコ

    Ok(format!("Server started successfully on {}", address))
}
