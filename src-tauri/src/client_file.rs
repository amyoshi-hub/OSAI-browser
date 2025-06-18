#[tauri::command]
async fn send_text(
    src_ip: String,
    src_port: u16,
    dst_ip: String,
    dst_port: u16,
    filename: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {

    let src_ip: Ipv4Addr = src_ip.parse().map_err(|e| format!("Invalid src_ip: {}", e))?;
    let dst_ip: Ipv4Addr = dst_ip.parse().map_err(|e| format!("Invalid dst_ip: {}", e))?;

    spawn(async move {
        let protocol = TransportProtocol::Ipv4(IpNextHeaderProtocols::Udp);
        let (mut tx, _) = match transport_channel(4096, Layer4(protocol)) {
            Ok((tx, rx)) => (tx, rx),
            Err(e) => {
                let _ = app_handle.emit("send_file_status", format!("Failed to open transport channel: {}", e));
                return;
            }
        };

        let mut file = match File::open(&filename) {
            Ok(f) => f,
            Err(e) => {
                let _ = app_handle.emit("send_file_status", format!("Failed to open file: {}", e));
                return;
            }
        };

        let mut buffer = [0u8; CHUNK_SIZE];
        let mut chunk_id = 0u32;

        loop {
            let read_bytes = match file.read(&mut buffer) {
                Ok(n) => n,
                Err(e) => {
                    let _ = app_handle.emit("send_file_status", format!("Failed to read file: {}", e));
                    return;
                }
            };

            if read_bytes == 0 {
                break;
            }

            let mut packet_buffer = vec![0u8; 8 + 16 + 8 + 2 + 14 + data.len()];
            let mut packet = build_udp_packet(
                &mut packet_buffer,
