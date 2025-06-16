use tokio::net::UdpSocket;
use tokio::task;
use tauri::Builder;

#[tauri::command]
async fn get_title_from_webview(window: tauri::Window) -> String {
    "Example Title from Rust".to_string()
}

#[tokio::main]
async fn main() {
    let server_address = "127.0.0.1:8081".to_string(); // ポート番号を指定
    task::spawn(async move { start_udp_server(server_address).await });

    println!("メインスレッドは他の処理を続行...");

    Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn start_udp_server(address: String) {
    let socket = UdpSocket::bind(address).await.expect("Couldn't bind to address");
    println!("UDPサーバーが待機中...");

    let mut buf = [0u8; 1024];
    loop {
        match socket.recv_from(&mut buf).await {
            Ok((size, src)) => {
                let received_data = String::from_utf8_lossy(&buf[..size]);
                println!("{} から受信: {}", src, received_data);

                // 必要に応じてレスポンスを送信
                let response = "受信完了".as_bytes();
                if let Err(e) = socket.send_to(response, src).await {
                    eprintln!("レスポンス送信エラー: {}", e);
                }
            }
            Err(e) => eprintln!("受信エラー: {}", e),
        }
    }
}

