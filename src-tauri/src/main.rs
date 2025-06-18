use tokio::net::UdpSocket;
use tokio::task;
use tauri::{Builder, Manager};
use tauri::Emitter;

#[tauri::command]
fn rust_code() -> String {
    "This is a message from Rust".to_string()
}

#[tauri::command]
async fn start_server(app_handle: tauri::AppHandle, ip: String, port: String) -> Result<String, String> {
    let address = format!("{}:{}", ip, port);
    println!("Attempting to bind UDP server to: {}", address); // ここも修正

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
                    let received_data = String::from_utf8_lossy(&buf[..len]);
                    println!("Received data: {} from {}", received_data, addr);
                    // 必要であれば、受信したデータをフロントエンドに送り返す
                    if let Err(e) = app_handle.emit("udp_data_received", format!("{} from {}", received_data, addr)) {
                        eprintln!("Failed to emit UDP data event: {}", e);
                    }
                },
                Err(e) => {
                    eprintln!("Failed to receive data: {}", e);
                    // エラー発生時にループを抜けるか、エラー回復を試みるか検討
                    break;
                }
            }
        }
    });

    Ok(format!("Server started successfully on {}", address))
}

#[tokio::main]
async fn main() {

    println!("App start");

    Builder::default()
        // generate_handler!に複数のコマンドを渡せます
        .invoke_handler(tauri::generate_handler![rust_code, start_server])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
