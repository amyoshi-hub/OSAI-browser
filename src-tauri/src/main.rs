#[derive(Default)]
struct App {
  webview_window: Option<(Window, WebView)>,
}

impl ApplicationHandler for App {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let window = event_loop.create_window(Window::default_attributes()).unwrap();
    let webview = WebViewBuilder::new()
      .with_url("https://tauri.app")
      .build(&window)
      .unwrap();

    self.webview_window = Some((window, webview));
  }

  fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, event: WindowEvent) {}

  // Advance GTK event loop <!----- IMPORTANT
  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    #[cfg(target_os = "linux")]
    while gtk::events_pending() {
      gtk::main_iteration_do(false);
    }
  }
}

pub let event_loop = EventLoop::new().unwrap();
pub let mut app = App::default();
event_loop.run_app(&mut app).unwrap();

