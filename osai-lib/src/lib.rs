pub struct OSAI{
    pub use_gui: bool,
}

impl OSAI {
    pub fn new() -> Self {
        OSAI { use_gui: true }
    }
    pub fn with_gui(mut self) -> Self{
        self.use_gui = true;
        self
    }
    pub fn with_cui(mut self) -> Self{
        self.use_gui = false;
        self
    }

    pub async fn run(self){
        use crate::tauri::handlers::*;
        
        let builder = tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                start_server,
                send_text,
                process_and_add_world,
                get_world_list,
                open_world,
                start_websocket_server,
                get_file_list,
                read_file_content,
                http_server,
                open_url_window,
                fetch_file_list,
                request_file,
                request_server_list,
            ]);
        builder.run(tauri::generate_context!()).expect("error while running tauri application");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
